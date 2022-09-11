#![feature(generic_associated_types)]

use macros::versions;
// use std::mem;
use core::fmt::Debug;
use core::fmt::Error;
use core::fmt::Formatter;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::num::NonZeroU64;
use core::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
// use core::ops::FnOnce;

// struct GpuBox<T>(usize, PhantomData<T>); // Box<T> equivalent
// struct GpuRef<'a, T>(usize, PhantomData<&'a T>); // &'a T equivalent
// struct GpuRefMut<'a, T>(usize, PhantomData<&'a mut T>); // &'a mut T equivalent
//
// struct GpuVec<T>(usize, PhantomData<T>); // Box<[T]> equivalent

#[repr(transparent)]
struct GPUPointer<'a, T>(NonZeroU64, PhantomData<&'a T>);

impl<'a, T> Debug for GPUPointer<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!(
            "{:#x} ({})",
            self.0,
            core::any::type_name::<T>()
        ))
    }
}

#[repr(transparent)]
struct GPUWeakPointer<T>(NonZeroU64, PhantomData<T>);

impl<T> GPUWeakPointer<T> {
    // The third argument is a type inference hack
    unsafe fn offset<U>(&self, off: usize, _: *const U) -> GPUWeakPointer<U> {
        GPUWeakPointer::<U>(self.0.checked_add(off as u64).unwrap(), PhantomData)
    }
}

#[repr(transparent)]
#[allow(dead_code)]
struct GPURawPointer(NonZeroU64);

#[macro_export]
macro_rules! inner_ptr {
    ($gpuva:expr, $($f:tt)*) => ({
        fn uninit_from<T: GPUStruct>(_: &GPUWeakPointer<T>) -> MaybeUninit<T::Raw<'static>> {
            core::mem::MaybeUninit::uninit()
        }
        let tmp = uninit_from($gpuva);
        let outer = tmp.as_ptr();
        let p: *const _ = unsafe { core::ptr::addr_of!((*outer).$($f)*) };
        let inner = p as *const u8;
        let off = unsafe { inner.offset_from(outer as *const u8) };
        unsafe { $gpuva.offset(off.try_into().unwrap(), p) }
    })
}

trait GPUStruct: 'static {
    type Raw<'a>: Debug;
}

struct GPUObject<T: GPUStruct> {
    raw: Box<T::Raw<'static>>,
    gpu_ptr: GPUWeakPointer<T>,
    inner: T,
}

impl<T: GPUStruct> GPUObject<T> {
    fn new(inner: T, callback: impl for<'a> FnOnce(&'a T) -> T::Raw<'a>) -> GPUObject<T> {
        let r = Box::new(unsafe { std::mem::transmute_copy(&callback(&inner)) });
        GPUObject::<T> {
            raw: r,
            gpu_ptr: GPUWeakPointer::<T>(NonZeroU64::new(1).unwrap(), PhantomData),
            inner,
        }
    }

    fn new_prealloc(
        inner_cb: impl FnOnce(&GPUWeakPointer<T>) -> T,
        raw_cb: impl for<'a> FnOnce(&'a T) -> T::Raw<'a>,
    ) -> GPUObject<T> {
        let gpu_ptr = GPUWeakPointer::<T>(NonZeroU64::new(1).unwrap(), PhantomData);
        let inner = inner_cb(&gpu_ptr);
        let r = Box::new(unsafe { std::mem::transmute_copy(&raw_cb(&inner)) });
        GPUObject::<T> {
            raw: r,
            gpu_ptr,
            inner,
        }
    }

    fn gpu_pointer(&self) -> GPUPointer<'_, T> {
        GPUPointer(self.gpu_ptr.0, PhantomData)
    }

    fn with_mut<RetVal>(
        &mut self,
        callback: impl for<'a> FnOnce(&'a mut <T as GPUStruct>::Raw<'a>, &'a mut T) -> RetVal,
    ) -> RetVal {
        unsafe {
            let ptr: *mut T::Raw<'static> = &mut *self.raw;
            callback(&mut *ptr, &mut *(&mut self.inner as *mut _))
        }
    }

    fn with<RetVal>(
        &self,
        callback: impl for<'a> FnOnce(&'a <T as GPUStruct>::Raw<'a>, &'a T) -> RetVal,
    ) -> RetVal {
        unsafe {
            let ptr: *const T::Raw<'static> = &*self.raw;
            callback(&*ptr, &*(&self.inner as *const _))
        }
    }
}

impl<T: GPUStruct + Debug> Debug for GPUObject<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct(core::any::type_name::<T>())
            .field("raw", &format_args!("{:#X?}", &self.raw))
            .field("inner", &format_args!("{:#X?}", &self.inner))
            .finish()
    }
}

struct GPUArray<T: Copy> {
    raw: Box<Vec<T>>,
    gpu_ptr: NonZeroU64,
}

impl<T: Copy> GPUArray<T> {
    fn new(data: Vec<T>) -> GPUArray<T> {
        GPUArray::<T> {
            raw: Box::new(data),
            gpu_ptr: NonZeroU64::new(1).unwrap(),
        }
    }

    fn gpu_pointer(&self) -> GPUPointer<'_, &'_ [T]> {
        GPUPointer(self.gpu_ptr, PhantomData)
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        (*self.raw).len()
    }

    #[allow(dead_code)]
    fn as_slice(&self) -> &[T] {
        &self.raw
    }

    #[allow(dead_code)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.raw
    }
}

impl<T: GPUStruct> Drop for GPUObject<T> {
    fn drop(&mut self) {
        println!("Dropping {}", core::any::type_name::<T>(),);
    }
}

impl<T: Copy + Debug> Debug for GPUArray<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct(core::any::type_name::<T>())
            .field("array", &format_args!("{:#X?}", &self.raw))
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct RawBufferManagerBlockControl<'a> {
    total: AtomicU32,
    _t: PhantomData<&'a ()>,
}

#[derive(Debug)]
struct BufferManagerBlockControl {}

impl GPUStruct for BufferManagerBlockControl {
    type Raw<'a> = RawBufferManagerBlockControl<'a>;
}

#[versions(AGX)]
#[derive(Debug)]
#[repr(C)]
struct RawBufferManagerInfo<'a> {
    gpu_counter: u32,
    unk_4: u32,
    #[ver(V < V13_0b4)]
    unk_4_0: u32,
    block_count: AtomicU32,
    block_ctl_addr: GPUPointer<'a, BufferManagerBlockControl>,
}

#[versions(AGX)]
#[derive(Debug)]
struct BufferManagerInfo {
    #[allow(dead_code)]
    block_control: GPUObject<BufferManagerBlockControl>,
}

#[versions(AGX)]
impl GPUStruct for BufferManagerInfo::ver {
    type Raw<'a> = RawBufferManagerInfo::ver<'a>;
}

#[versions(AGX)]
#[derive(Debug)]
#[repr(C)]
struct RawTAJob<'a> {
    buffer_mgr: GPUPointer<'a, BufferManagerInfo::ver>,
    micro_sequence: GPUPointer<'a, &'a [u8]>,
    foo: u64,
}

#[versions(AGX)]
#[derive(Debug)]
struct TAJob {
    #[allow(dead_code)]
    buffer_mgr: Arc<GPUObject<BufferManagerInfo::ver>>,
    #[allow(dead_code)]
    micro_sequence: GPUMicroSequence,
}

#[versions(AGX)]
impl GPUStruct for TAJob::ver {
    type Raw<'a> = RawTAJob::ver<'a>;
}

type GPUMicroSequence = GPUArray<u8>;

struct GPUMicroSequenceBuilder {
    ops: Vec<u8>,
}

impl GPUMicroSequenceBuilder {
    fn new() -> GPUMicroSequenceBuilder {
        GPUMicroSequenceBuilder { ops: Vec::new() }
    }

    fn add<T: GPUMSOp>(&mut self, op: T) {
        let p: *const T = &op;
        let p: *const u8 = p as *const u8;
        let s: &[u8] = unsafe { std::slice::from_raw_parts(p, std::mem::size_of::<T>()) };
        self.ops.extend(s);
    }

    fn build(self) -> GPUMicroSequence {
        GPUArray::<u8>::new(self.ops)
    }
}

trait GPUMSOp {}

#[repr(C, packed(4))]
struct SomeOp {
    magic: u8,
    foo_p: GPUWeakPointer<u64>,
}

impl SomeOp {
    const MAGIC: u8 = 4;
}

impl GPUMSOp for SomeOp {}

#[allow(dead_code)]
#[versions(AGX)]
struct GPUDriver {}

#[allow(dead_code)]
#[versions(AGX)]
impl GPUDriver::ver {
    fn run() {
        let ctl = GPUObject::new(BufferManagerBlockControl {}, |_inner| {
            RawBufferManagerBlockControl {
                total: AtomicU32::new(0),
                ..Default::default()
            }
        });

        let ctl2 =
            GPUObject::<BufferManagerBlockControl>::new(BufferManagerBlockControl {}, |_inner| {
                RawBufferManagerBlockControl {
                    total: AtomicU32::new(0),
                    ..Default::default()
                }
            });

        dbg!(&ctl2);

        let mut mgr = GPUObject::<BufferManagerInfo::ver>::new(
            BufferManagerInfo::ver { block_control: ctl },
            |inner| RawBufferManagerInfo::ver {
                gpu_counter: 0,
                unk_4: 0,
                #[ver(V < V13_0b4)]
                unk_4_0: 1,
                block_count: AtomicU32::new(0),
                block_ctl_addr: inner.block_control.gpu_pointer(),
            },
        );

        dbg!(&mgr);

        mgr.with_mut(|raw, inner| {
            raw.gpu_counter = 2;
            inner.block_control.with(|raw, _inner| {
                raw.total.fetch_add(1, Ordering::Relaxed);
            });
        });

        let arc = Arc::new(mgr);

        let ta = GPUObject::<TAJob::ver>::new_prealloc(
            |p| {
                let mut ms = GPUMicroSequenceBuilder::new();
                ms.add(SomeOp {
                    magic: SomeOp::MAGIC,
                    foo_p: inner_ptr!(p, foo),
                });
                TAJob::ver {
                    buffer_mgr: arc.clone(),
                    micro_sequence: ms.build(),
                }
            },
            |inner| RawTAJob::ver {
                buffer_mgr: inner.buffer_mgr.gpu_pointer(),
                micro_sequence: inner.micro_sequence.gpu_pointer(),
                foo: 0x1234,
            },
        );

        arc.with(|_raw, inner| {
            inner.block_control.with(|raw, _inner| {
                raw.total.fetch_add(1, Ordering::Relaxed);
            });
        });

        dbg!(&ta);
    }
}

fn main() {
    GPUDriverG13GV13_0b4::run();
    GPUDriverG13GV12_3::run();
}
