#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AXIMC master 0 packing functionality register"]
    pub aximc_m0_fn_mod2: crate::Reg<aximc_m0_fn_mod2::AXIMC_M0_FN_MOD2_SPEC>,
    _reserved1: [u8; 0xd8],
    #[doc = "0xdc - AXIMC master 0 read priority register"]
    pub aximc_m0_read_qos: crate::Reg<aximc_m0_read_qos::AXIMC_M0_READ_QOS_SPEC>,
    #[doc = "0xe0 - AXIMC master 0 issuing capability override functionality register"]
    pub aximc_m0_fn_mod: crate::Reg<aximc_m0_fn_mod::AXIMC_M0_FN_MOD_SPEC>,
    #[doc = "0xe4 - AXIMC master 0 write priority register"]
    pub aximc_m0_write_qos: crate::Reg<aximc_m0_write_qos::AXIMC_M0_WRITE_QOS_SPEC>,
    _reserved4: [u8; 0x0f18],
    #[doc = "0x1000 - AXIMC master 1 packing functionality register"]
    pub aximc_m1_fn_mod2: crate::Reg<aximc_m1_fn_mod2::AXIMC_M1_FN_MOD2_SPEC>,
    _reserved5: [u8; 0xd8],
    #[doc = "0x10dc - AXIMC master 1 read priority register"]
    pub aximc_m1_read_qos: crate::Reg<aximc_m1_read_qos::AXIMC_M1_READ_QOS_SPEC>,
    #[doc = "0x10e0 - AXIMC master 1 write priority register"]
    pub aximc_m1_write_qos: crate::Reg<aximc_m1_write_qos::AXIMC_M1_WRITE_QOS_SPEC>,
    #[doc = "0x10e4 - AXIMC master 1 issuing capability override functionality register"]
    pub aximc_m1_fn_mod: crate::Reg<aximc_m1_fn_mod::AXIMC_M1_FN_MOD_SPEC>,
    _reserved8: [u8; 0x0ee8],
    #[doc = "0x1fd0 - AXIMC peripheral ID4 register"]
    pub aximc_periph_id_4: crate::Reg<aximc_periph_id_4::AXIMC_PERIPH_ID_4_SPEC>,
    #[doc = "0x1fd4 - AXIMC peripheral ID5 register"]
    pub aximc_periph_id_5: crate::Reg<aximc_periph_id_5::AXIMC_PERIPH_ID_5_SPEC>,
    #[doc = "0x1fd8 - AXIMC peripheral ID6 register"]
    pub aximc_periph_id_6: crate::Reg<aximc_periph_id_6::AXIMC_PERIPH_ID_6_SPEC>,
    #[doc = "0x1fdc - AXIMC peripheral ID7 register"]
    pub aximc_periph_id_7: crate::Reg<aximc_periph_id_7::AXIMC_PERIPH_ID_7_SPEC>,
    #[doc = "0x1fe0 - AXIMC peripheral ID0 register"]
    pub aximc_periph_id_0: crate::Reg<aximc_periph_id_0::AXIMC_PERIPH_ID_0_SPEC>,
    #[doc = "0x1fe4 - AXIMC peripheral ID1 register"]
    pub aximc_periph_id_1: crate::Reg<aximc_periph_id_1::AXIMC_PERIPH_ID_1_SPEC>,
    #[doc = "0x1fe8 - AXIMC peripheral ID2 register"]
    pub aximc_periph_id_2: crate::Reg<aximc_periph_id_2::AXIMC_PERIPH_ID_2_SPEC>,
    #[doc = "0x1fec - AXIMC peripheral ID3 register"]
    pub aximc_periph_id_3: crate::Reg<aximc_periph_id_3::AXIMC_PERIPH_ID_3_SPEC>,
    #[doc = "0x1ff0 - AXIMC component ID0 register"]
    pub aximc_comp_id_0: crate::Reg<aximc_comp_id_0::AXIMC_COMP_ID_0_SPEC>,
    #[doc = "0x1ff4 - AXIMC component ID1 register"]
    pub aximc_comp_id_1: crate::Reg<aximc_comp_id_1::AXIMC_COMP_ID_1_SPEC>,
    #[doc = "0x1ff8 - AXIMC component ID2 register"]
    pub aximc_comp_id_2: crate::Reg<aximc_comp_id_2::AXIMC_COMP_ID_2_SPEC>,
    #[doc = "0x1ffc - AXIMC component ID3 register"]
    pub aximc_comp_id_3: crate::Reg<aximc_comp_id_3::AXIMC_COMP_ID_3_SPEC>,
    #[doc = "0x2000 - AXIMC master 2 packing functionality register"]
    pub aximc_m2_fn_mod2: crate::Reg<aximc_m2_fn_mod2::AXIMC_M2_FN_MOD2_SPEC>,
    _reserved21: [u8; 0xd8],
    #[doc = "0x20dc - AXIMC master 2 read priority register"]
    pub aximc_m2_read_qos: crate::Reg<aximc_m2_read_qos::AXIMC_M2_READ_QOS_SPEC>,
    #[doc = "0x20e0 - AXIMC master 2 write priority register"]
    pub aximc_m2_write_qos: crate::Reg<aximc_m2_write_qos::AXIMC_M2_WRITE_QOS_SPEC>,
    #[doc = "0x20e4 - AXIMC master 2 issuing capability override functionality register"]
    pub aximc_m2_fn_mod: crate::Reg<aximc_m2_fn_mod::AXIMC_M2_FN_MOD_SPEC>,
    _reserved24: [u8; 0x0f18],
    #[doc = "0x3000 - AXIMC master 5 packing functionality register"]
    pub aximc_m5_fn_mod2: crate::Reg<aximc_m5_fn_mod2::AXIMC_M5_FN_MOD2_SPEC>,
    _reserved25: [u8; 0xd8],
    #[doc = "0x30dc - AXIMC master 5 read priority register"]
    pub aximc_m5_read_qos: crate::Reg<aximc_m5_read_qos::AXIMC_M5_READ_QOS_SPEC>,
    #[doc = "0x30e0 - AXIMC master 5 write priority register"]
    pub aximc_m5_write_qos: crate::Reg<aximc_m5_write_qos::AXIMC_M5_WRITE_QOS_SPEC>,
    #[doc = "0x30e4 - AXIMC master 5 issuing capability override functionality register"]
    pub aximc_m5_fn_mod: crate::Reg<aximc_m5_fn_mod::AXIMC_M5_FN_MOD_SPEC>,
    _reserved28: [u8; 0x0ff4],
    #[doc = "0x40dc - AXIMC master 3 read priority register"]
    pub aximc_m3_read_qos: crate::Reg<aximc_m3_read_qos::AXIMC_M3_READ_QOS_SPEC>,
    #[doc = "0x40e0 - AXIMC master 3 write priority register"]
    pub aximc_m3_write_qos: crate::Reg<aximc_m3_write_qos::AXIMC_M3_WRITE_QOS_SPEC>,
    #[doc = "0x40e4 - AXIMC master 3 packing functionality register"]
    pub aximc_m3_fn_mod: crate::Reg<aximc_m3_fn_mod::AXIMC_M3_FN_MOD_SPEC>,
    _reserved31: [u8; 0x0ff4],
    #[doc = "0x50dc - AXIMC master 7 read priority register"]
    pub aximc_m7_read_qos: crate::Reg<aximc_m7_read_qos::AXIMC_M7_READ_QOS_SPEC>,
    #[doc = "0x50e0 - AXIMC master 7 write priority register"]
    pub aximc_m7_write_qos: crate::Reg<aximc_m7_write_qos::AXIMC_M7_WRITE_QOS_SPEC>,
    #[doc = "0x50e4 - AXIMC master 7 issuing capability override functionality register"]
    pub aximc_m7_fn_mod: crate::Reg<aximc_m7_fn_mod::AXIMC_M7_FN_MOD_SPEC>,
    _reserved34: [u8; 0x0ff4],
    #[doc = "0x60dc - AXIMC master 8 read priority register"]
    pub aximc_m8_read_qos: crate::Reg<aximc_m8_read_qos::AXIMC_M8_READ_QOS_SPEC>,
    #[doc = "0x60e0 - AXIMC master 8 write priority register"]
    pub aximc_m8_write_qos: crate::Reg<aximc_m8_write_qos::AXIMC_M8_WRITE_QOS_SPEC>,
    #[doc = "0x60e4 - AXIMC master 8 issuing capability override functionality register"]
    pub aximc_m8_fn_mod: crate::Reg<aximc_m8_fn_mod::AXIMC_M8_FN_MOD_SPEC>,
    _reserved37: [u8; 0x1f18],
    #[doc = "0x8000 - AXIMC master 4 packing functionality register"]
    pub aximc_m4_fn_mod2: crate::Reg<aximc_m4_fn_mod2::AXIMC_M4_FN_MOD2_SPEC>,
    _reserved38: [u8; 0xd8],
    #[doc = "0x80dc - AXIMC master 4 read priority register"]
    pub aximc_m4_read_qos: crate::Reg<aximc_m4_read_qos::AXIMC_M4_READ_QOS_SPEC>,
    #[doc = "0x80e0 - AXIMC master 4 write priority register"]
    pub aximc_m4_write_qos: crate::Reg<aximc_m4_write_qos::AXIMC_M4_WRITE_QOS_SPEC>,
    #[doc = "0x80e4 - AXIMC master 4 packing functionality register"]
    pub aximc_m4_fn_mod: crate::Reg<aximc_m4_fn_mod::AXIMC_M4_FN_MOD_SPEC>,
    _reserved41: [u8; 0x0ff4],
    #[doc = "0x90dc - AXIMC master 9 read priority register"]
    pub aximc_m9_read_qos: crate::Reg<aximc_m9_read_qos::AXIMC_M9_READ_QOS_SPEC>,
    #[doc = "0x90e0 - AXIMC master 9 write priority register"]
    pub aximc_m9_write_qos: crate::Reg<aximc_m9_write_qos::AXIMC_M9_WRITE_QOS_SPEC>,
    #[doc = "0x90e4 - AXIMC master 9 issuing capability override functionality register"]
    pub aximc_m9_fn_mod: crate::Reg<aximc_m9_fn_mod::AXIMC_M9_FN_MOD_SPEC>,
    _reserved44: [u8; 0x0ff4],
    #[doc = "0xa0dc - AXIMC master 10 read priority register"]
    pub aximc_m10_read_qos: crate::Reg<aximc_m10_read_qos::AXIMC_M10_READ_QOS_SPEC>,
    #[doc = "0xa0e0 - AXIMC master 10 write priority register"]
    pub aximc_m10_write_qos: crate::Reg<aximc_m10_write_qos::AXIMC_M10_WRITE_QOS_SPEC>,
    #[doc = "0xa0e4 - AXIMC master 10 issuing capability override functionality register"]
    pub aximc_m10_fn_mod: crate::Reg<aximc_m10_fn_mod::AXIMC_M10_FN_MOD_SPEC>,
    _reserved47: [u8; 0x0f18],
    #[doc = "0xb000 - AXIMC master 6 packing functionality register"]
    pub aximc_m6_fn_mod2: crate::Reg<aximc_m6_fn_mod2::AXIMC_M6_FN_MOD2_SPEC>,
    _reserved48: [u8; 0xd8],
    #[doc = "0xb0dc - AXIMC master 6 read priority register"]
    pub aximc_m6_read_qos: crate::Reg<aximc_m6_read_qos::AXIMC_M6_READ_QOS_SPEC>,
    #[doc = "0xb0e0 - AXIMC master 6 write priority register"]
    pub aximc_m6_write_qos: crate::Reg<aximc_m6_write_qos::AXIMC_M6_WRITE_QOS_SPEC>,
    #[doc = "0xb0e4 - AXIMC master 6 issuing capability override functionality register"]
    pub aximc_m6_fn_mod: crate::Reg<aximc_m6_fn_mod::AXIMC_M6_FN_MOD_SPEC>,
    _reserved51: [u8; 0x0003_6f40],
    #[doc = "0x42028 - AXIMC master 0 AHB conversion override functionality register"]
    pub aximc_m0_fn_mod_ahb: crate::Reg<aximc_m0_fn_mod_ahb::AXIMC_M0_FN_MOD_AHB_SPEC>,
    _reserved52: [u8; 0x0ffc],
    #[doc = "0x43028 - AXIMC master 1 AHB conversion override functionality register"]
    pub aximc_m1_fn_mod_ahb: crate::Reg<aximc_m1_fn_mod_ahb::AXIMC_M1_FN_MOD_AHB_SPEC>,
    _reserved53: [u8; 0x0ffc],
    #[doc = "0x44028 - AXIMC master 2 AHB conversion override functionality register"]
    pub aximc_m2_fn_mod_ahb: crate::Reg<aximc_m2_fn_mod_ahb::AXIMC_M2_FN_MOD_AHB_SPEC>,
    _reserved54: [u8; 0x0ffc],
    #[doc = "0x45028 - AXIMC master 5 AHB conversion override functionality register"]
    pub aximc_m5_fn_mod_ahb: crate::Reg<aximc_m5_fn_mod_ahb::AXIMC_M5_FN_MOD_AHB_SPEC>,
    _reserved55: [u8; 0x5000],
    #[doc = "0x4a02c - AXIMC long burst capability inhibition register"]
    pub aximc_fn_mod_lb: crate::Reg<aximc_fn_mod_lb::AXIMC_FN_MOD_LB_SPEC>,
    _reserved56: [u8; 0x2ff8],
    #[doc = "0x4d028 - AXIMC master 6 AHB conversion override functionality register"]
    pub aximc_m6_fn_mod_ahb: crate::Reg<aximc_m6_fn_mod_ahb::AXIMC_M6_FN_MOD_AHB_SPEC>,
}
#[doc = "AXIMC_M0_FN_MOD2 register accessor: an alias for `Reg<AXIMC_M0_FN_MOD2_SPEC>`"]
pub type AXIMC_M0_FN_MOD2 = crate::Reg<aximc_m0_fn_mod2::AXIMC_M0_FN_MOD2_SPEC>;
#[doc = "AXIMC master 0 packing functionality register"]
pub mod aximc_m0_fn_mod2;
#[doc = "AXIMC_M0_READ_QOS register accessor: an alias for `Reg<AXIMC_M0_READ_QOS_SPEC>`"]
pub type AXIMC_M0_READ_QOS = crate::Reg<aximc_m0_read_qos::AXIMC_M0_READ_QOS_SPEC>;
#[doc = "AXIMC master 0 read priority register"]
pub mod aximc_m0_read_qos;
#[doc = "AXIMC_M0_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M0_WRITE_QOS_SPEC>`"]
pub type AXIMC_M0_WRITE_QOS = crate::Reg<aximc_m0_write_qos::AXIMC_M0_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 0 write priority register"]
pub mod aximc_m0_write_qos;
#[doc = "AXIMC_M0_FN_MOD register accessor: an alias for `Reg<AXIMC_M0_FN_MOD_SPEC>`"]
pub type AXIMC_M0_FN_MOD = crate::Reg<aximc_m0_fn_mod::AXIMC_M0_FN_MOD_SPEC>;
#[doc = "AXIMC master 0 issuing capability override functionality register"]
pub mod aximc_m0_fn_mod;
#[doc = "AXIMC_M1_FN_MOD2 register accessor: an alias for `Reg<AXIMC_M1_FN_MOD2_SPEC>`"]
pub type AXIMC_M1_FN_MOD2 = crate::Reg<aximc_m1_fn_mod2::AXIMC_M1_FN_MOD2_SPEC>;
#[doc = "AXIMC master 1 packing functionality register"]
pub mod aximc_m1_fn_mod2;
#[doc = "AXIMC_M1_READ_QOS register accessor: an alias for `Reg<AXIMC_M1_READ_QOS_SPEC>`"]
pub type AXIMC_M1_READ_QOS = crate::Reg<aximc_m1_read_qos::AXIMC_M1_READ_QOS_SPEC>;
#[doc = "AXIMC master 1 read priority register"]
pub mod aximc_m1_read_qos;
#[doc = "AXIMC_M1_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M1_WRITE_QOS_SPEC>`"]
pub type AXIMC_M1_WRITE_QOS = crate::Reg<aximc_m1_write_qos::AXIMC_M1_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 1 write priority register"]
pub mod aximc_m1_write_qos;
#[doc = "AXIMC_M1_FN_MOD register accessor: an alias for `Reg<AXIMC_M1_FN_MOD_SPEC>`"]
pub type AXIMC_M1_FN_MOD = crate::Reg<aximc_m1_fn_mod::AXIMC_M1_FN_MOD_SPEC>;
#[doc = "AXIMC master 1 issuing capability override functionality register"]
pub mod aximc_m1_fn_mod;
#[doc = "AXIMC_M2_FN_MOD2 register accessor: an alias for `Reg<AXIMC_M2_FN_MOD2_SPEC>`"]
pub type AXIMC_M2_FN_MOD2 = crate::Reg<aximc_m2_fn_mod2::AXIMC_M2_FN_MOD2_SPEC>;
#[doc = "AXIMC master 2 packing functionality register"]
pub mod aximc_m2_fn_mod2;
#[doc = "AXIMC_M2_READ_QOS register accessor: an alias for `Reg<AXIMC_M2_READ_QOS_SPEC>`"]
pub type AXIMC_M2_READ_QOS = crate::Reg<aximc_m2_read_qos::AXIMC_M2_READ_QOS_SPEC>;
#[doc = "AXIMC master 2 read priority register"]
pub mod aximc_m2_read_qos;
#[doc = "AXIMC_M2_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M2_WRITE_QOS_SPEC>`"]
pub type AXIMC_M2_WRITE_QOS = crate::Reg<aximc_m2_write_qos::AXIMC_M2_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 2 write priority register"]
pub mod aximc_m2_write_qos;
#[doc = "AXIMC_M2_FN_MOD register accessor: an alias for `Reg<AXIMC_M2_FN_MOD_SPEC>`"]
pub type AXIMC_M2_FN_MOD = crate::Reg<aximc_m2_fn_mod::AXIMC_M2_FN_MOD_SPEC>;
#[doc = "AXIMC master 2 issuing capability override functionality register"]
pub mod aximc_m2_fn_mod;
#[doc = "AXIMC_M5_FN_MOD2 register accessor: an alias for `Reg<AXIMC_M5_FN_MOD2_SPEC>`"]
pub type AXIMC_M5_FN_MOD2 = crate::Reg<aximc_m5_fn_mod2::AXIMC_M5_FN_MOD2_SPEC>;
#[doc = "AXIMC master 5 packing functionality register"]
pub mod aximc_m5_fn_mod2;
#[doc = "AXIMC_M5_READ_QOS register accessor: an alias for `Reg<AXIMC_M5_READ_QOS_SPEC>`"]
pub type AXIMC_M5_READ_QOS = crate::Reg<aximc_m5_read_qos::AXIMC_M5_READ_QOS_SPEC>;
#[doc = "AXIMC master 5 read priority register"]
pub mod aximc_m5_read_qos;
#[doc = "AXIMC_M5_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M5_WRITE_QOS_SPEC>`"]
pub type AXIMC_M5_WRITE_QOS = crate::Reg<aximc_m5_write_qos::AXIMC_M5_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 5 write priority register"]
pub mod aximc_m5_write_qos;
#[doc = "AXIMC_M5_FN_MOD register accessor: an alias for `Reg<AXIMC_M5_FN_MOD_SPEC>`"]
pub type AXIMC_M5_FN_MOD = crate::Reg<aximc_m5_fn_mod::AXIMC_M5_FN_MOD_SPEC>;
#[doc = "AXIMC master 5 issuing capability override functionality register"]
pub mod aximc_m5_fn_mod;
#[doc = "AXIMC_M3_READ_QOS register accessor: an alias for `Reg<AXIMC_M3_READ_QOS_SPEC>`"]
pub type AXIMC_M3_READ_QOS = crate::Reg<aximc_m3_read_qos::AXIMC_M3_READ_QOS_SPEC>;
#[doc = "AXIMC master 3 read priority register"]
pub mod aximc_m3_read_qos;
#[doc = "AXIMC_M3_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M3_WRITE_QOS_SPEC>`"]
pub type AXIMC_M3_WRITE_QOS = crate::Reg<aximc_m3_write_qos::AXIMC_M3_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 3 write priority register"]
pub mod aximc_m3_write_qos;
#[doc = "AXIMC_M3_FN_MOD register accessor: an alias for `Reg<AXIMC_M3_FN_MOD_SPEC>`"]
pub type AXIMC_M3_FN_MOD = crate::Reg<aximc_m3_fn_mod::AXIMC_M3_FN_MOD_SPEC>;
#[doc = "AXIMC master 3 packing functionality register"]
pub mod aximc_m3_fn_mod;
#[doc = "AXIMC_M7_READ_QOS register accessor: an alias for `Reg<AXIMC_M7_READ_QOS_SPEC>`"]
pub type AXIMC_M7_READ_QOS = crate::Reg<aximc_m7_read_qos::AXIMC_M7_READ_QOS_SPEC>;
#[doc = "AXIMC master 7 read priority register"]
pub mod aximc_m7_read_qos;
#[doc = "AXIMC_M7_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M7_WRITE_QOS_SPEC>`"]
pub type AXIMC_M7_WRITE_QOS = crate::Reg<aximc_m7_write_qos::AXIMC_M7_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 7 write priority register"]
pub mod aximc_m7_write_qos;
#[doc = "AXIMC_M7_FN_MOD register accessor: an alias for `Reg<AXIMC_M7_FN_MOD_SPEC>`"]
pub type AXIMC_M7_FN_MOD = crate::Reg<aximc_m7_fn_mod::AXIMC_M7_FN_MOD_SPEC>;
#[doc = "AXIMC master 7 issuing capability override functionality register"]
pub mod aximc_m7_fn_mod;
#[doc = "AXIMC_M8_READ_QOS register accessor: an alias for `Reg<AXIMC_M8_READ_QOS_SPEC>`"]
pub type AXIMC_M8_READ_QOS = crate::Reg<aximc_m8_read_qos::AXIMC_M8_READ_QOS_SPEC>;
#[doc = "AXIMC master 8 read priority register"]
pub mod aximc_m8_read_qos;
#[doc = "AXIMC_M8_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M8_WRITE_QOS_SPEC>`"]
pub type AXIMC_M8_WRITE_QOS = crate::Reg<aximc_m8_write_qos::AXIMC_M8_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 8 write priority register"]
pub mod aximc_m8_write_qos;
#[doc = "AXIMC_M8_FN_MOD register accessor: an alias for `Reg<AXIMC_M8_FN_MOD_SPEC>`"]
pub type AXIMC_M8_FN_MOD = crate::Reg<aximc_m8_fn_mod::AXIMC_M8_FN_MOD_SPEC>;
#[doc = "AXIMC master 8 issuing capability override functionality register"]
pub mod aximc_m8_fn_mod;
#[doc = "AXIMC_M4_FN_MOD2 register accessor: an alias for `Reg<AXIMC_M4_FN_MOD2_SPEC>`"]
pub type AXIMC_M4_FN_MOD2 = crate::Reg<aximc_m4_fn_mod2::AXIMC_M4_FN_MOD2_SPEC>;
#[doc = "AXIMC master 4 packing functionality register"]
pub mod aximc_m4_fn_mod2;
#[doc = "AXIMC_M4_READ_QOS register accessor: an alias for `Reg<AXIMC_M4_READ_QOS_SPEC>`"]
pub type AXIMC_M4_READ_QOS = crate::Reg<aximc_m4_read_qos::AXIMC_M4_READ_QOS_SPEC>;
#[doc = "AXIMC master 4 read priority register"]
pub mod aximc_m4_read_qos;
#[doc = "AXIMC_M4_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M4_WRITE_QOS_SPEC>`"]
pub type AXIMC_M4_WRITE_QOS = crate::Reg<aximc_m4_write_qos::AXIMC_M4_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 4 write priority register"]
pub mod aximc_m4_write_qos;
#[doc = "AXIMC_M4_FN_MOD register accessor: an alias for `Reg<AXIMC_M4_FN_MOD_SPEC>`"]
pub type AXIMC_M4_FN_MOD = crate::Reg<aximc_m4_fn_mod::AXIMC_M4_FN_MOD_SPEC>;
#[doc = "AXIMC master 4 packing functionality register"]
pub mod aximc_m4_fn_mod;
#[doc = "AXIMC_M9_READ_QOS register accessor: an alias for `Reg<AXIMC_M9_READ_QOS_SPEC>`"]
pub type AXIMC_M9_READ_QOS = crate::Reg<aximc_m9_read_qos::AXIMC_M9_READ_QOS_SPEC>;
#[doc = "AXIMC master 9 read priority register"]
pub mod aximc_m9_read_qos;
#[doc = "AXIMC_M9_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M9_WRITE_QOS_SPEC>`"]
pub type AXIMC_M9_WRITE_QOS = crate::Reg<aximc_m9_write_qos::AXIMC_M9_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 9 write priority register"]
pub mod aximc_m9_write_qos;
#[doc = "AXIMC_M9_FN_MOD register accessor: an alias for `Reg<AXIMC_M9_FN_MOD_SPEC>`"]
pub type AXIMC_M9_FN_MOD = crate::Reg<aximc_m9_fn_mod::AXIMC_M9_FN_MOD_SPEC>;
#[doc = "AXIMC master 9 issuing capability override functionality register"]
pub mod aximc_m9_fn_mod;
#[doc = "AXIMC_M10_READ_QOS register accessor: an alias for `Reg<AXIMC_M10_READ_QOS_SPEC>`"]
pub type AXIMC_M10_READ_QOS = crate::Reg<aximc_m10_read_qos::AXIMC_M10_READ_QOS_SPEC>;
#[doc = "AXIMC master 10 read priority register"]
pub mod aximc_m10_read_qos;
#[doc = "AXIMC_M10_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M10_WRITE_QOS_SPEC>`"]
pub type AXIMC_M10_WRITE_QOS = crate::Reg<aximc_m10_write_qos::AXIMC_M10_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 10 write priority register"]
pub mod aximc_m10_write_qos;
#[doc = "AXIMC_M10_FN_MOD register accessor: an alias for `Reg<AXIMC_M10_FN_MOD_SPEC>`"]
pub type AXIMC_M10_FN_MOD = crate::Reg<aximc_m10_fn_mod::AXIMC_M10_FN_MOD_SPEC>;
#[doc = "AXIMC master 10 issuing capability override functionality register"]
pub mod aximc_m10_fn_mod;
#[doc = "AXIMC_M6_FN_MOD2 register accessor: an alias for `Reg<AXIMC_M6_FN_MOD2_SPEC>`"]
pub type AXIMC_M6_FN_MOD2 = crate::Reg<aximc_m6_fn_mod2::AXIMC_M6_FN_MOD2_SPEC>;
#[doc = "AXIMC master 6 packing functionality register"]
pub mod aximc_m6_fn_mod2;
#[doc = "AXIMC_M6_READ_QOS register accessor: an alias for `Reg<AXIMC_M6_READ_QOS_SPEC>`"]
pub type AXIMC_M6_READ_QOS = crate::Reg<aximc_m6_read_qos::AXIMC_M6_READ_QOS_SPEC>;
#[doc = "AXIMC master 6 read priority register"]
pub mod aximc_m6_read_qos;
#[doc = "AXIMC_M6_WRITE_QOS register accessor: an alias for `Reg<AXIMC_M6_WRITE_QOS_SPEC>`"]
pub type AXIMC_M6_WRITE_QOS = crate::Reg<aximc_m6_write_qos::AXIMC_M6_WRITE_QOS_SPEC>;
#[doc = "AXIMC master 6 write priority register"]
pub mod aximc_m6_write_qos;
#[doc = "AXIMC_M6_FN_MOD register accessor: an alias for `Reg<AXIMC_M6_FN_MOD_SPEC>`"]
pub type AXIMC_M6_FN_MOD = crate::Reg<aximc_m6_fn_mod::AXIMC_M6_FN_MOD_SPEC>;
#[doc = "AXIMC master 6 issuing capability override functionality register"]
pub mod aximc_m6_fn_mod;
#[doc = "AXIMC_PERIPH_ID_4 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_4_SPEC>`"]
pub type AXIMC_PERIPH_ID_4 = crate::Reg<aximc_periph_id_4::AXIMC_PERIPH_ID_4_SPEC>;
#[doc = "AXIMC peripheral ID4 register"]
pub mod aximc_periph_id_4;
#[doc = "AXIMC_PERIPH_ID_5 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_5_SPEC>`"]
pub type AXIMC_PERIPH_ID_5 = crate::Reg<aximc_periph_id_5::AXIMC_PERIPH_ID_5_SPEC>;
#[doc = "AXIMC peripheral ID5 register"]
pub mod aximc_periph_id_5;
#[doc = "AXIMC_PERIPH_ID_6 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_6_SPEC>`"]
pub type AXIMC_PERIPH_ID_6 = crate::Reg<aximc_periph_id_6::AXIMC_PERIPH_ID_6_SPEC>;
#[doc = "AXIMC peripheral ID6 register"]
pub mod aximc_periph_id_6;
#[doc = "AXIMC_PERIPH_ID_7 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_7_SPEC>`"]
pub type AXIMC_PERIPH_ID_7 = crate::Reg<aximc_periph_id_7::AXIMC_PERIPH_ID_7_SPEC>;
#[doc = "AXIMC peripheral ID7 register"]
pub mod aximc_periph_id_7;
#[doc = "AXIMC_PERIPH_ID_0 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_0_SPEC>`"]
pub type AXIMC_PERIPH_ID_0 = crate::Reg<aximc_periph_id_0::AXIMC_PERIPH_ID_0_SPEC>;
#[doc = "AXIMC peripheral ID0 register"]
pub mod aximc_periph_id_0;
#[doc = "AXIMC_PERIPH_ID_1 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_1_SPEC>`"]
pub type AXIMC_PERIPH_ID_1 = crate::Reg<aximc_periph_id_1::AXIMC_PERIPH_ID_1_SPEC>;
#[doc = "AXIMC peripheral ID1 register"]
pub mod aximc_periph_id_1;
#[doc = "AXIMC_PERIPH_ID_2 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_2_SPEC>`"]
pub type AXIMC_PERIPH_ID_2 = crate::Reg<aximc_periph_id_2::AXIMC_PERIPH_ID_2_SPEC>;
#[doc = "AXIMC peripheral ID2 register"]
pub mod aximc_periph_id_2;
#[doc = "AXIMC_PERIPH_ID_3 register accessor: an alias for `Reg<AXIMC_PERIPH_ID_3_SPEC>`"]
pub type AXIMC_PERIPH_ID_3 = crate::Reg<aximc_periph_id_3::AXIMC_PERIPH_ID_3_SPEC>;
#[doc = "AXIMC peripheral ID3 register"]
pub mod aximc_periph_id_3;
#[doc = "AXIMC_COMP_ID_0 register accessor: an alias for `Reg<AXIMC_COMP_ID_0_SPEC>`"]
pub type AXIMC_COMP_ID_0 = crate::Reg<aximc_comp_id_0::AXIMC_COMP_ID_0_SPEC>;
#[doc = "AXIMC component ID0 register"]
pub mod aximc_comp_id_0;
#[doc = "AXIMC_COMP_ID_1 register accessor: an alias for `Reg<AXIMC_COMP_ID_1_SPEC>`"]
pub type AXIMC_COMP_ID_1 = crate::Reg<aximc_comp_id_1::AXIMC_COMP_ID_1_SPEC>;
#[doc = "AXIMC component ID1 register"]
pub mod aximc_comp_id_1;
#[doc = "AXIMC_COMP_ID_2 register accessor: an alias for `Reg<AXIMC_COMP_ID_2_SPEC>`"]
pub type AXIMC_COMP_ID_2 = crate::Reg<aximc_comp_id_2::AXIMC_COMP_ID_2_SPEC>;
#[doc = "AXIMC component ID2 register"]
pub mod aximc_comp_id_2;
#[doc = "AXIMC_COMP_ID_3 register accessor: an alias for `Reg<AXIMC_COMP_ID_3_SPEC>`"]
pub type AXIMC_COMP_ID_3 = crate::Reg<aximc_comp_id_3::AXIMC_COMP_ID_3_SPEC>;
#[doc = "AXIMC component ID3 register"]
pub mod aximc_comp_id_3;
#[doc = "AXIMC_M0_FN_MOD_AHB register accessor: an alias for `Reg<AXIMC_M0_FN_MOD_AHB_SPEC>`"]
pub type AXIMC_M0_FN_MOD_AHB = crate::Reg<aximc_m0_fn_mod_ahb::AXIMC_M0_FN_MOD_AHB_SPEC>;
#[doc = "AXIMC master 0 AHB conversion override functionality register"]
pub mod aximc_m0_fn_mod_ahb;
#[doc = "AXIMC_M1_FN_MOD_AHB register accessor: an alias for `Reg<AXIMC_M1_FN_MOD_AHB_SPEC>`"]
pub type AXIMC_M1_FN_MOD_AHB = crate::Reg<aximc_m1_fn_mod_ahb::AXIMC_M1_FN_MOD_AHB_SPEC>;
#[doc = "AXIMC master 1 AHB conversion override functionality register"]
pub mod aximc_m1_fn_mod_ahb;
#[doc = "AXIMC_M2_FN_MOD_AHB register accessor: an alias for `Reg<AXIMC_M2_FN_MOD_AHB_SPEC>`"]
pub type AXIMC_M2_FN_MOD_AHB = crate::Reg<aximc_m2_fn_mod_ahb::AXIMC_M2_FN_MOD_AHB_SPEC>;
#[doc = "AXIMC master 2 AHB conversion override functionality register"]
pub mod aximc_m2_fn_mod_ahb;
#[doc = "AXIMC_M5_FN_MOD_AHB register accessor: an alias for `Reg<AXIMC_M5_FN_MOD_AHB_SPEC>`"]
pub type AXIMC_M5_FN_MOD_AHB = crate::Reg<aximc_m5_fn_mod_ahb::AXIMC_M5_FN_MOD_AHB_SPEC>;
#[doc = "AXIMC master 5 AHB conversion override functionality register"]
pub mod aximc_m5_fn_mod_ahb;
#[doc = "AXIMC_M6_FN_MOD_AHB register accessor: an alias for `Reg<AXIMC_M6_FN_MOD_AHB_SPEC>`"]
pub type AXIMC_M6_FN_MOD_AHB = crate::Reg<aximc_m6_fn_mod_ahb::AXIMC_M6_FN_MOD_AHB_SPEC>;
#[doc = "AXIMC master 6 AHB conversion override functionality register"]
pub mod aximc_m6_fn_mod_ahb;
#[doc = "AXIMC_FN_MOD_LB register accessor: an alias for `Reg<AXIMC_FN_MOD_LB_SPEC>`"]
pub type AXIMC_FN_MOD_LB = crate::Reg<aximc_fn_mod_lb::AXIMC_FN_MOD_LB_SPEC>;
#[doc = "AXIMC long burst capability inhibition register"]
pub mod aximc_fn_mod_lb;
