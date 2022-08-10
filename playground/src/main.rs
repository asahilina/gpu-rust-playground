use macros::versions;
use std::mem;

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct AGXFWBufferManagerInfo {
    gpu_counter: u32,
    unk_4: u32,
    last_id: u32,
    cur_id: u32,
    unk_10: u32,
    gpu_counter2: u32,
    unk_18: u32,

    #[ver(V < V13_0b4)]
    unk_1c: u32,

    page_list_addr: u64,
    page_list_size: u32,
    page_count: u32,
    unk_30: u32,
    block_count: u32,
    unk_38: u32,
    block_list_addr: u64,
    block_ctl_addr: u64,
    last_page: u32,
    gpu_page_ptr1: u32,
    gpu_page_ptr2: u32,
    unk_58: u32,
    block_size: u32,
    unk_60: u64,
    counter_addr: u64,
    unk_70: u32,
    unk_74: u32,
    unk_78: u32,
    unk_7c: u32,
    unk_80: u32,
    unk_84: u32,
    unk_88: u32,
    unk_8c: u32,
    unk_90: [u8; 0x30],
}

#[versions(AGX)]
impl Default for AGXFWBufferManagerInfo {
    fn default() -> AGXFWBufferManagerInfo::ver {
        unsafe { std::mem::zeroed() }
    }
}

trait AGXBufferManager {
    fn set_size(&mut self, pages: u32);
}

#[versions(AGX)]
#[derive(Default, Debug)]
struct AGXBufferManager {
    info: AGXFWBufferManagerInfo::ver,
}

#[versions(AGX)]
impl AGXBufferManager for AGXBufferManager {
    fn set_size(&mut self, pages: u32) {
        self.info.page_count = pages;
        #[ver(V < V13_0b4)]
        {
            self.info.unk_1c = 1;
        }

        #[ver(V < V13_0b4)]
        self.info.unk_1c = 1;
    }
}

fn main() {
    println!(
        "Sizeof AGXFWBufferManagerInfoG13GV12_3: {}",
        mem::size_of::<AGXFWBufferManagerInfoG13GV12_3>()
    );

    let mut mgr: AGXBufferManagerG13GV12_3 = Default::default();

    mgr.set_size(0x100);
    dbg!(mgr);

    let mut mgr2: AGXBufferManagerG13GV13_0b4 = Default::default();

    mgr2.set_size(0x100);
    dbg!(mgr2);
}
