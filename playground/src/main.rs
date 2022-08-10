use macros::versions;
use std::mem;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct ChannelInfo {
    state_addr: u64,
    ringbuffer_addr: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct ChannelInfoSet {
    _TA_0: ChannelInfo,
    _3D_0: ChannelInfo,
    _CL_0: ChannelInfo,
    _TA_1: ChannelInfo,
    _3D_1: ChannelInfo,
    _CL_1: ChannelInfo,
    _TA_2: ChannelInfo,
    _3D_2: ChannelInfo,
    _CL_2: ChannelInfo,
    _TA_3: ChannelInfo,
    _3D_3: ChannelInfo,
    _CL_3: ChannelInfo,
    DevCtrl: ChannelInfo,
    Event: ChannelInfo,
    FWLog: ChannelInfo,
    KTrace: ChannelInfo,
    Stats: ChannelInfo,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_FWStatus {
    fwctl_channel: ChannelInfo,
    halt_count: u32,
    __pad0: [u8; 0xc],
    halted: u32,
    __pad1: [u8; 0xc],
    resume: u32,
    __pad2: [u8; 0xc],
    unk_40: u32,
    __pad3: [u8; 0xc],
    unk_ctr: u32,
    __pad4: [u8; 0xc],
    unk_60: u32,
    __pad5: [u8; 0xc],
    unk_70: u32,
    __pad6: [u8; 0xc],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct AGXHWDataShared1 {
    unk_0: u32,
    unk_4: u32,
    unk_8: u32,
    unk_c: u32,
    unk_10: u32,
    unk_14: u32,
    unk_18: u32,
    unk_1c: u32,
    unk_20: [u8; 0x26],
    unk_46: [u8; 0x6],
    unk_4c: [u8; 0x58],
    unk_a4: u32,
    unk_a8: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct AGXHWDataShared2 {
    unk_ac: u32,
    unk_b0: u32,
    unk_b4: [u8; 0x18],
    unk_cc: [u8; 0x8],
    unk_d4: [u8; 0x10],
    unk_e4: [u8; 0x8],
    unk_ec: [u8; 0x4c8],
    unk_5b4: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct AGXHWDataA130Extra {
    unk_0: [u8; 0x38],
    unk_38: u32,
    unk_3c: u32,
    unk_40: u32,
    unk_44: u32,
    unk_48: u32,
    unk_4c: u32,
    unk_50: u32,
    unk_54: u32,
    unk_58: u32,
    unk_5c: u32,
    unk_60: f32,
    unk_64: f32,
    unk_68: f32,
    unk_6c: f32,
    unk_70: f32,
    unk_74: f32,
    unk_78: f32,
    unk_7c: f32,
    unk_80: f32,
    unk_84: f32,
    unk_88: u32,
    unk_8c: u32,
    unk_90: u32,
    unk_94: u32,
    unk_98: u32,
    unk_9c: f32,
    unk_a0: u32,
    unk_a4: u32,
    unk_a8: u32,
    unk_ac: u32,
    unk_b0: u32,
    unk_b4: u32,
    unk_b8: u32,
    unk_bc: u32,
    unk_c0: u32,
    unk_c4: f32,
    unk_c8: [u8; 0x4c],
    unk_114: f32,
    unk_118: u32,
    unk_11c: u32,
    unk_120: u32,
    unk_124: u32,
    unk_128: u32,
    unk_12c: [u8; 0x8c],
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct AGXHWDataA {
    unk_0: u32,
    unk_4: u32,

    #[ver(V >= V13_0b4)]
    unk_8_0: u32,

    unk_8: u32,
    pwr_status: u32,
    unk_10: f32,
    unk_14: u32,
    unk_18: u32,
    unk_1c: u32,
    unk_20: u32,
    unk_24: u32,
    actual_pstate: u32,
    tgt_pstate: u32,
    unk_30: u32,
    cur_pstate: u32,
    unk_38: u32,

    #[ver(V >= V13_0b4)]
    unk_3c_0: u32,

    unk_3c: u32,
    unk_40: u32,
    unk_44: u32,
    unk_48: u32,
    unk_4c: u32,
    freq_mhz: f32,
    unk_54: [u8; 0x20],

    #[ver(V >= V13_0b4)]
    unk_74_0: u32,

    unk_74: [f32; 0x10],
    unk_b4: [u8; 0x100],
    unk_1b4: u32,
    temp_c: u32,
    avg_power_mw: u32,
    update_ts: u64,
    unk_1c8: u32,
    unk_1cc: [u8; 0x478],
    pad_644: [u8; 0x8],
    unk_64c: u32,
    unk_650: u32,
    pad_654: u32,
    unk_658: f32,
    pad_65c: u32,
    unk_660: f32,
    pad_664: u32,
    unk_668: f32,
    pad_66c: u32,
    unk_670: u32,
    unk_674: f32,
    unk_678: f32,
    pad_67c: u32,
    unk_680: u32,
    unk_684: u32,
    unk_688: u32,
    unk_68c: u32,
    pad_690: u32,
    unk_694: u32,
    unk_698: u32,
    pad_69c: [u8; 0x18],
    unk_6b4: u32,

    #[ver(V >= V13_0b4)]
    unk_6b8_0: [u8; 0x10],

    unk_6b8: u32,
    unk_6bc: u32,
    pad_6c0: [u8; 0x14],
    unk_6d4: u32,
    unk_6d8: u32,
    pad_6dc: u32,
    unk_6e0: f32,
    pad_6e4: u32,
    unk_6e8: f32,
    pad_6ec: u32,
    unk_6f0: f32,
    pad_6f4: u32,
    unk_6f8: u32,
    unk_6fc: f32,
    unk_700: f32,
    pad_704: u32,
    unk_708: u32,
    unk_70c: u32,
    unk_710: u32,
    unk_714: u32,
    pad_718: u32,
    unk_71c: f32,
    unk_720: u32,
    cur_power_mw_2: u32,
    unk_728: u32,
    unk_72c: u32,

    #[ver(V >= V13_0b4)]
    unk_730_0: u32,

    #[ver(V >= V13_0b4)]
    unk_730_4: u32,

    #[ver(V >= V13_0b4)]
    unk_730_8: u32,

    #[ver(V >= V13_0b4)]
    unk_730_c: u32,

    unk_730: f32,
    unk_734: u32,
    unk_738: u32,
    unk_73c: u32,
    unk_740: u32,
    unk_744: u32,
    unk_748: [f32; 0x4],
    unk_758: u32,
    unk_75c: u32,
    pad_760: u32,
    unk_764: u32,
    unk_768: u32,
    unk_76c: u32,
    pad_770: u32,
    unk_774: u32,
    unk_778: u32,
    unk_77c: u32,
    unk_780: f32,
    unk_784: f32,
    unk_788: f32,
    unk_78c: f32,
    unk_790: f32,
    unk_794: f32,
    unk_798: f32,
    unk_79c: f32,
    unk_7a0: f32,
    unk_7a4: f32,
    unk_7a8: f32,
    unk_7ac: u32,
    unk_7b0: u32,
    unk_7b4: u32,
    pad_7b8: u32,
    use_percent: f32,
    unk_7c0: u32,
    pad_7c4: [u8; 0x18],
    unk_7dc: u32,

    #[ver(V >= V13_0b4)]
    unk_7e0_0: [u8; 0x10],

    unk_7e0: u32,
    pad_7e4: u32,
    unk_7e8: [u8; 0x14],
    unk_7fc: f32,
    unk_800: f32,
    unk_804: f32,
    unk_808: u32,
    pad_80c: u32,
    unk_810: u32,
    pad_814: u32,
    unk_818: u32,
    unk_81c: u32,
    pad_820: u32,
    unk_824: f32,
    unk_828: u32,
    unk_82c: u32,
    unk_830: f32,
    unk_834: f32,
    unk_838: u32,
    unk_83c: u32,
    pad_840: [u8; 0x2c],
    unk_86c: u32,
    unk_870: u32,
    unk_874: u32,
    unk_878: u32,
    unk_87c: u32,
    unk_880: u32,
    unk_884: u32,
    pad_888: u32,
    unk_88c: u32,
    pad_890: u32,
    unk_894: f32,
    pad_898: u32,
    unk_89c: f32,
    pad_8a0: u32,
    unk_8a4: u32,
    unk_8a8: f32,
    unk_8ac: f32,
    pad_8b0: u32,
    unk_8b4: u32,
    unk_8b8: u32,
    unk_8bc: u32,
    unk_8c0: u32,
    unk_8c4: u32,
    unk_8c8: u32,
    unk_8cc: u32,
    pad_8d0: [u8; 0x14],

    #[ver(V >= V13_0b4)]
    unk_8e4_0: [u8; 0x10],

    unk_8e4: u32,
    unk_8e8: u32,
    unk_8ec: u32,
    unk_8f0: u32,
    unk_8f4: u32,
    pad_8f8: u32,
    pad_8fc: u32,
    unk_900: [u8; 0x294],
    unk_b94: u32,
    freq_with_off: u32,
    unk_b9c: u32,
    unk_ba0: u64,
    unk_ba8: u64,
    unk_bb0: u32,
    unk_bb4: u32,
    pad_bb8: [u8; 0x74],
    unk_c2c: u32,
    unk_c30: u32,
    unk_c34: u32,
    unk_c38: u32,
    unk_c3c: u32,
    unk_c40: u32,
    unk_c44: f32,
    unk_c48: f32,
    unk_c4c: f32,
    unk_c50: u32,
    unk_c54: u32,

    #[ver(V >= V13_0b4)]
    unk_c58_0: u32,

    #[ver(V >= V13_0b4)]
    unk_c58_4: u32,

    unk_c58: f32,
    unk_c5c: u32,
    unk_c60: u32,
    unk_c64: u32,
    unk_c68: u32,

    #[ver(V >= V13_0b4)]
    unk_c6c_0: u32,

    #[ver(V >= V13_0b4)]
    unk_c6c_4: u32,

    unk_c6c: f32,
    unk_c70: f32,
    pad_c74: u32,
    unk_c78: u32,
    unk_c7c: u32,
    unk_c80: u32,
    unk_c84: u32,
    unk_c88: u32,
    unk_c8c: u32,
    unk_c90: [u8; 0x60],

    #[ver(V >= V13_0b4)]
    unk_cf0_0: [u8; 0x20],

    unk_cf0: u32,
    unk_cf4: u32,
    unk_cf8: u32,
    unk_cfc: u32,
    unk_d00: u32,
    unk_d04: f32,
    unk_d08: u32,
    unk_d0c: f32,
    unk_d10: u32,
    unk_d14: f32,
    unk_d18: u32,
    unk_d1c: u32,
    unk_d20: f32,
    unk_d24: f32,
    unk_d28: u32,
    unk_d2c: u32,
    unk_d30: u32,
    unk_d34: u32,
    unk_d38: u32,
    unk_d3c: u32,
    unk_d40: f32,
    unk_d44: u32,
    unk_d48: u32,
    unk_d4c: u32,
    unk_d50: u32,

    #[ver(V >= V13_0b4)]
    unk_d54_0: u32,

    #[ver(V >= V13_0b4)]
    unk_d54_4: [u8; 0xc],

    unk_d54: [u8; 0x10],
    unk_d64: u32,
    unk_d68: [u8; 0x24],
    unk_d8c: u32,
    unk_d90: u32,
    unk_d94: u32,
    unk_d98: u32,
    unk_d9c: f32,
    unk_da0: u32,
    unk_da4: f32,
    unk_da8: u32,
    unk_dac: f32,
    unk_db0: u32,
    unk_db4: u32,
    unk_db8: f32,
    unk_dbc: f32,
    unk_dc0: u32,
    unk_dc4: u32,
    unk_dc8: u32,
    unk_dcc: u32,
    unk_dd0: [u8; 0x40],

    #[ver(V >= V13_0b4)]
    unk_e10_0: AGXHWDataA130Extra,

    unk_e10: [u8; 0x20],
    pad_e30: [u8; 0x7e0],
    unk_1610: [u8; 0x28],

    #[ver(V < V13_0b4)]
    unk_1638: [u8; 0x8],

    unk_1640: [u8; 0x2000],
    unk_3640: u32,
    hws1: AGXHWDataShared1,

    #[ver(V >= V13_0b4)]
    unk_pad1: [u8; 0x20],

    hws2: AGXHWDataShared2,
    unk_3bfc: u32,
    unk_3c00: [u8; 0xa0],
    unk_3ca0: u64,
    unk_3ca8: u64,
    unk_3cb0: u64,
    ts_last_idle: u64,
    ts_last_poweron: u64,
    ts_last_poweroff: u64,
    unk_3cd0: u64,
    unk_3cd8: u64,

    #[ver(V >= V13_0b4)]
    unk_3ce0_0: u32,

    unk_3ce0: [u8; 0x40],
    unk_3d20: [u8; 0x4c],

    #[ver(V >= V13_0b4)]
    unk_3d6c: [u8; 0x38],

}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct IOMapping {
    phys_addr: u64,
    virt_addr: u64,
    size: u32,
    range_size: u32,
    readwrite: u64,
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct AGXHWDataB {

    #[ver(V < V13_0b4)]
    unk_0: u64,

    unk_8: u64,

    #[ver(V < V13_0b4)]
    unk_10: u64,

    unk_18: u64,
    unk_20: u64,
    unk_28: u64,
    unk_30: u64,
    unkptr_38: u64,
    pad_40: [u8; 0x20],

    #[ver(V < V13_0b4)]
    yuv_matrices: [[[i16; 0x4]; 0x3]; 0xf],

    #[ver(V >= V13_0b4)]
    yuv_matrices: [[[i16; 0x4]; 0x3]; 0x3f],

    pad_1c8: [u8; 0x8],
    io_mappings: [IOMapping; 0x14],

    #[ver(V >= V13_0b4)]
    unk_450_0: [u8; 0x68],

    chip_id: u32,
    unk_454: u32,
    unk_458: u32,
    unk_45c: u32,
    unk_460: u32,
    unk_464: u32,
    unk_468: u32,
    unk_46c: u32,
    unk_470: u32,
    unk_474: u32,
    unk_478: u32,
    unk_47c: u32,
    unk_480: u32,
    unk_484: u32,
    unk_488: u32,
    unk_48c: u32,
    unk_490: u32,
    unk_494: u32,
    pad_498: [u8; 0x4],
    unk_49c: u32,
    unk_4a0: u32,
    unk_4a4: u32,
    pad_4a8: [u8; 0x4],
    unk_4ac: u32,
    pad_4b0: [u8; 0x8],
    unk_4b8: u32,
    unk_4bc: [u8; 0x4],
    unk_4c0: u32,
    unk_4c4: u32,
    unk_4c8: u32,
    unk_4cc: u32,
    unk_4d0: u32,
    unk_4d4: u32,
    unk_4d8: [u8; 0x4],
    unk_4dc: u32,
    unk_4e0: u64,
    unk_4e8: u32,
    unk_4ec: u32,
    unk_4f0: u32,
    unk_4f4: u32,
    unk_4f8: u32,
    unk_4fc: u32,
    unk_500: u32,

    #[ver(V >= V13_0b4)]
    unk_504_0: u32,

    unk_504: u32,
    unk_508: u32,
    unk_50c: u32,
    unk_510: u32,
    unk_514: u32,
    unk_518: u32,
    unk_51c: u32,
    unk_520: u32,
    unk_524: u32,
    unk_528: u32,
    unk_52c: u32,
    unk_530: u32,
    unk_534: u32,
    unk_538: u32,

    #[ver(V >= V13_0b4)]
    unk_53c_0: u32,

    unk_53c: u32,
    unk_540: u32,
    unk_544: u32,
    unk_548: u32,
    unk_54c: u32,
    unk_550: u32,
    unk_554: u32,
    unk_558: u32,
    unk_55c: u32,
    unk_560: u32,
    unk_564: u32,
    unk_568: u32,
    max_pstate: u32,

    #[ver(V < V13_0b4)]
    num_pstates: u32,

    frequencies: [u32; 0x10],
    voltages: [[u32; 0x8]; 0x10],
    voltages_sram: [[u32; 0x8]; 0x10],
    unk_9b4: [f32; 0x10],
    unk_9f4: [u32; 0x10],
    perf_levels: [u32; 0x10],
    unk_a74: u32,
    unk_a78: u32,
    unk_a7c: u32,
    unk_a80: u32,
    unk_a84: u32,
    unk_a88: u32,
    unk_a8c: u32,
    pad_a90: [u8; 0x24],

    #[ver(V < V13_0b4)]
    min_volt: u32,

    #[ver(V < V13_0b4)]
    unk_ab8: u32,

    #[ver(V < V13_0b4)]
    unk_abc: u32,

    #[ver(V < V13_0b4)]
    unk_ac0: u32,

    #[ver(V >= V13_0b4)]
    unk_ac4_0: [u8; 0x1f0],

    pad_ac4: [u8; 0x8],
    unk_acc: u32,
    unk_ad0: u32,
    pad_ad4: [u8; 0x10],
    unk_ae4: [u32; 0x4],
    pad_af4: [u8; 0x4],
    unk_af8: u32,
    pad_afc: [u8; 0x8],
    unk_b04: u32,
    unk_b08: u32,
    unk_b0c: u32,
    unk_b10: u32,
    pad_b14: [u8; 0x8],
    unk_b1c: u32,
    unk_b20: u32,
    unk_b24: u32,
    unk_b28: u32,
    unk_b2c: u32,
    unk_b30: u32,
    unk_b34: u32,

    #[ver(V >= V13_0b4)]
    unk_b38_0: u32,

    #[ver(V >= V13_0b4)]
    unk_b38_4: u32,

    unk_b38: [u64; 0x6],
    unk_b68: u32,

    #[ver(V >= V13_0b4)]
    unk_b6c: [u8; 0xd0],

    #[ver(V >= V13_0b4)]
    unk_c3c: u32,

}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_GPUQueueStatsTA {
    busy: u32,
    unk_4: u32,
    cur_cmdqueue: u64,
    cur_count: u32,
    unk_14: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_GPUStatsTA {
    unk_4: u32,
    queues: [InitData_GPUQueueStatsTA; 0x4],
    unk_68: [u8; 0x8],
    unk_70: u32,
    unk_74: u32,
    unk_timestamp: u64,
    unk_80: [u8; 0x40],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_GPUQueueStats3D {
    busy: u32,
    cur_cmdqueue: u64,
    unk_c: u32,
    unk_10: u32,
    unk_14: [u8; 0x14],
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_GPUStats3D {
    unk_0: [u8; 0x18],
    queues: [InitData_GPUQueueStats3D; 0x4],
    unk_68: u32,
    cur_cmdqueue: u64,
    unk_74: [u8; 0x44],

    #[ver(V < V13_0b4)]
    unk_ac: [u8; 0x38],

    tvb_overflows_1: u32,
    tvb_overflows_2: u32,
    unk_f8: u32,
    unk_fc: u32,
    cur_stamp_id: i32,
    unk_104: [u8; 0x14],
    unk_118: i32,
    unk_11c: u32,
    unk_120: u32,
    unk_124: [u8; 0x1c],
    unk_140: u32,
    unk_144: u32,
    unk_timestamp: u64,
    unk_150: [u8; 0x68],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_GPUGlobalStatsTA {
    total_cmds: u32,
    stats: InitData_GPUStatsTA,
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_GPUGlobalStats3D {
    total_cmds: u32,
    unk_4: u32,
    stats: InitData_GPUStats3D::ver,
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_RegionB {
    channels: ChannelInfoSet,
    pad_110: [u8; 0x50],
    unk_160: u64,
    unk_168: u64,
    stats_ta_addr: u64,
    stats_3d_addr: u64,
    stats_cp_addr: u64,
    hwdata_a_addr: u64,
    unkptr_190: u64,
    unkptr_198: u64,
    hwdata_b_addr: u64,
    hwdata_b_addr2: u64,
    fwlog_ring2: u64,
    unkptr_1b8: u64,
    unkptr_1c0: u64,
    unkptr_1c8: u64,
    pad_1d0: [u8; 0x44],
    buffer_mgr_ctl_addr: u64,
    buffer_mgr_ctl_addr2: u64,
    pad_224: [u8; 0x685c],
    unk_6a80: u32,
    gpu_idle: u32,
    unkpad_6a88: [u8; 0x14],
    unk_6a9c: u32,
    unk_ctr0: u32,
    unk_ctr1: u32,
    unk_6aa8: u32,
    unk_6aac: u32,
    unk_ctr2: u32,
    unk_6ab4: u32,
    unk_6ab8: u32,
    unk_6abc: u32,
    unk_6ac0: u32,
    unk_6ac4: u32,
    unk_ctr3: u32,
    unk_6acc: u32,
    unk_6ad0: u32,
    unk_6ad4: u32,
    unk_6ad8: u32,
    unk_6adc: u32,
    unk_6ae0: u32,
    unk_6ae4: u32,
    unk_6ae8: u32,
    unk_6aec: u32,
    unk_6af0: u32,
    unk_ctr4: u32,
    unk_ctr5: u32,
    unk_6afc: u32,
    pad_6b00: [u8; 0x38],
    unk_6b38: u32,
    pad_6b3c: [u8; 0x84],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_PendingStamp {
    info: u32,
    wait_value: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_FaultInfo {
    unk_0: u32,
    unk_4: u32,
    queue_uuid: u32,
    unk_c: u32,
    unk_10: u32,
    unk_14: u32,
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData_RegionC {
    unk_0: [u8; 0x28],

    #[ver(V >= V13_0b4)]
    unk_28_0: u32,

    unk_28: u32,

    #[ver(V >= V13_0b4)]
    unk_2c_0: u32,

    unk_2c: u32,
    unk_30: u32,
    unk_34: u32,
    unk_38: [u8; 0x1c],
    unk_54: u16,
    unk_56: u16,
    unk_58: u16,
    unk_5a: u32,
    unk_5e: u32,
    unk_62: u32,

    #[ver(V >= V13_0b4)]
    unk_66_0: [u8; 0xc],

    unk_66: u32,
    unk_6a: [u8; 0x16],
    unk_80: [u8; 0xf80],
    unk_1000: [u8; 0x7000],
    unk_8000: [u8; 0x900],

    #[ver(V >= V13_0b4)]
    unk_8900_0: u32,

    unk_8900: u32,
    unk_atomic: u32,
    unk_8908: u32,
    unk_890c: u32,
    unk_8910: u32,
    unk_8914: u32,
    unk_8918: u32,
    unk_891c: u32,
    unk_8920: u32,
    unk_8924: u32,
    unk_8928: u32,
    unk_892c: f32,
    unk_8930: f32,
    unk_8934: u32,
    unk_8938: u32,
    unk_893c: u32,
    unk_8940: u32,
    unk_8944: u32,
    unk_8948: u32,
    unk_894c: u32,
    unk_8950: [u8; 0x6c],

    #[ver(V >= V13_0b4)]
    unk_89bc_0: [u8; 0x3c],

    unk_89bc: u32,
    unk_89c0: u32,
    unk_89c4: i32,
    unk_89c8: u32,
    unk_89cc: f32,
    unk_89d0: f32,
    unk_89d4: [u8; 0xc],
    unk_89e0: u32,
    unk_89e4: u32,
    unk_89e8: f32,
    unk_89ec: f32,
    unk_89f0: u32,

    #[ver(V >= V13_0b4)]
    unk_89f4_0: [u8; 0x8],

    #[ver(V >= V13_0b4)]
    unk_89f4_8: u32,

    #[ver(V >= V13_0b4)]
    unk_89f4_c: [u8; 0x50],

    hws1: AGXHWDataShared1,
    hws2: AGXHWDataShared2,
    unk_8fac: [u8; 0x60],

    #[ver(V >= V13_0b4)]
    unk_900c_0: [u8; 0x28],

    unk_900c: u32,

    #[ver(V >= V13_0b4)]
    unk_9010_0: u32,

    #[ver(V >= V13_0b4)]
    unk_9010_4: [u8; 0x10],

    unk_9010: [u8; 0x30],
    unk_9040: u32,
    unk_9044: [u8; 0xbc],
    unk_a000: [u8; 0x6000],
    unk_10000: [u8; 0xe50],
    unk_10e50: u32,
    unk_10e54: [u8; 0x2c],

    #[ver(V >= V13_0b4)]
    unk_10e80_0: [u8; 0xed4],

    #[ver(V >= V13_0b4)]
    unk_10e80_ed0: u32,

    #[ver(V >= V13_0b4)]
    unk_10e80_ed4: [u8; 0x2c],

    unk_10e80: u32,
    unk_10e84: u32,
    unk_10e88: [u8; 0x188],
    idle_ts: u64,
    idle_unk: u64,
    unk_11020: u32,
    unk_11024: u32,
    unk_11028: u32,

    #[ver(V >= V13_0b4)]
    unk_1102c_0: u32,

    #[ver(V >= V13_0b4)]
    unk_1102c_4: u32,

    #[ver(V >= V13_0b4)]
    unk_1102c_8: u32,

    #[ver(V >= V13_0b4)]
    unk_1102c_c: u32,

    #[ver(V >= V13_0b4)]
    unk_1102c_10: u32,

    unk_1102c: u32,
    idle_to_off_timeout_ms: u32,
    unk_11034: u32,
    unk_11038: u32,
    pending_stamps: [InitData_PendingStamp; 0x110],
    unk_117bc: u32,
    fault_info: InitData_FaultInfo,
    counter: u32,
    unk_118dc: u32,

    #[ver(V >= V13_0b4)]
    unk_118e0_0: [u8; 0x9c],

    unk_118e0: u32,

    #[ver(V >= V13_0b4)]
    unk_118e4_0: u32,

    unk_118e4: [u8; 0x1c],
    unk_11900: [u8; 0x440],

    #[ver(V >= V13_0b4)]
    unk_11d40: [u8; 0x19c],

    #[ver(V >= V13_0b4)]
    unk_11edc: u32,

    #[ver(V >= V13_0b4)]
    unk_11ee0: [u8; 0x1c],

    #[ver(V >= V13_0b4)]
    unk_11efc: u32,

}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct UatLevelInfo {
    unk_3: u8,
    unk_1: u8,
    unk_2: u8,
    index_shift: u8,
    num_entries: u16,
    unk_4: u16,
    unk_8: u64,
    unk_10: u64,
    index_mask: u64,
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct InitData {

    #[ver(V >= V13_0b4)]
    ver_info: [u16; 0x4],

    regionA_addr: u64,
    unk_8: u32,
    unk_c: u32,
    regionB_addr: u64,
    regionC_addr: u64,
    fw_status_addr: u64,
    uat_page_size: u16,
    uat_page_bits: u8,
    uat_num_levels: u8,
    uat_level_info: [UatLevelInfo; 0x3],
    pad_8c: [u8; 0x14],
    host_mapped_fw_allocations: u32,
    __pad0: [u8; 0x1000],
}

#[versions(AGX)]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
struct BufferManagerInfo {
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
impl Default for BufferManagerInfo::ver {
    fn default() -> BufferManagerInfo::ver {
        unsafe { std::mem::zeroed() }
    }
}

trait AGXBufferManager {
    fn set_size(&mut self, pages: u32);
}

#[versions(AGX)]
#[derive(Default, Debug)]
struct AGXBufferManager {
    info: BufferManagerInfo::ver,
}

#[versions(AGX)]
impl AGXBufferManager for AGXBufferManager::ver {
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
        mem::size_of::<BufferManagerInfoG13GV12_3>()
    );

    let mut mgr: AGXBufferManagerG13GV12_3 = Default::default();

    mgr.set_size(0x100);
    dbg!(mgr);

    let mut mgr2: AGXBufferManagerG13GV13_0b4 = Default::default();

    mgr2.set_size(0x100);
    dbg!(mgr2);
}
