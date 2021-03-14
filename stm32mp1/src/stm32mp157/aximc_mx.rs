#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AXIMC master 0 packing functionality register"]
    pub aximc_m0_fn_mod2: AXIMC_M0_FN_MOD2,
    _reserved1: [u8; 216usize],
    #[doc = "0xdc - AXIMC master 0 read priority register"]
    pub aximc_m0_read_qos: AXIMC_M0_READ_QOS,
    #[doc = "0xe0 - AXIMC master 0 issuing capability override functionality register"]
    pub aximc_m0_fn_mod: AXIMC_M0_FN_MOD,
    #[doc = "0xe4 - AXIMC master 0 write priority register"]
    pub aximc_m0_write_qos: AXIMC_M0_WRITE_QOS,
    _reserved4: [u8; 3864usize],
    #[doc = "0x1000 - AXIMC master 1 packing functionality register"]
    pub aximc_m1_fn_mod2: AXIMC_M1_FN_MOD2,
    _reserved5: [u8; 216usize],
    #[doc = "0x10dc - AXIMC master 1 read priority register"]
    pub aximc_m1_read_qos: AXIMC_M1_READ_QOS,
    #[doc = "0x10e0 - AXIMC master 1 write priority register"]
    pub aximc_m1_write_qos: AXIMC_M1_WRITE_QOS,
    #[doc = "0x10e4 - AXIMC master 1 issuing capability override functionality register"]
    pub aximc_m1_fn_mod: AXIMC_M1_FN_MOD,
    _reserved8: [u8; 3816usize],
    #[doc = "0x1fd0 - AXIMC peripheral ID4 register"]
    pub aximc_periph_id_4: AXIMC_PERIPH_ID_4,
    #[doc = "0x1fd4 - AXIMC peripheral ID5 register"]
    pub aximc_periph_id_5: AXIMC_PERIPH_ID_5,
    #[doc = "0x1fd8 - AXIMC peripheral ID6 register"]
    pub aximc_periph_id_6: AXIMC_PERIPH_ID_6,
    #[doc = "0x1fdc - AXIMC peripheral ID7 register"]
    pub aximc_periph_id_7: AXIMC_PERIPH_ID_7,
    #[doc = "0x1fe0 - AXIMC peripheral ID0 register"]
    pub aximc_periph_id_0: AXIMC_PERIPH_ID_0,
    #[doc = "0x1fe4 - AXIMC peripheral ID1 register"]
    pub aximc_periph_id_1: AXIMC_PERIPH_ID_1,
    #[doc = "0x1fe8 - AXIMC peripheral ID2 register"]
    pub aximc_periph_id_2: AXIMC_PERIPH_ID_2,
    #[doc = "0x1fec - AXIMC peripheral ID3 register"]
    pub aximc_periph_id_3: AXIMC_PERIPH_ID_3,
    #[doc = "0x1ff0 - AXIMC component ID0 register"]
    pub aximc_comp_id_0: AXIMC_COMP_ID_0,
    #[doc = "0x1ff4 - AXIMC component ID1 register"]
    pub aximc_comp_id_1: AXIMC_COMP_ID_1,
    #[doc = "0x1ff8 - AXIMC component ID2 register"]
    pub aximc_comp_id_2: AXIMC_COMP_ID_2,
    #[doc = "0x1ffc - AXIMC component ID3 register"]
    pub aximc_comp_id_3: AXIMC_COMP_ID_3,
    #[doc = "0x2000 - AXIMC master 2 packing functionality register"]
    pub aximc_m2_fn_mod2: AXIMC_M2_FN_MOD2,
    _reserved21: [u8; 216usize],
    #[doc = "0x20dc - AXIMC master 2 read priority register"]
    pub aximc_m2_read_qos: AXIMC_M2_READ_QOS,
    #[doc = "0x20e0 - AXIMC master 2 write priority register"]
    pub aximc_m2_write_qos: AXIMC_M2_WRITE_QOS,
    #[doc = "0x20e4 - AXIMC master 2 issuing capability override functionality register"]
    pub aximc_m2_fn_mod: AXIMC_M2_FN_MOD,
    _reserved24: [u8; 3864usize],
    #[doc = "0x3000 - AXIMC master 5 packing functionality register"]
    pub aximc_m5_fn_mod2: AXIMC_M5_FN_MOD2,
    _reserved25: [u8; 216usize],
    #[doc = "0x30dc - AXIMC master 5 read priority register"]
    pub aximc_m5_read_qos: AXIMC_M5_READ_QOS,
    #[doc = "0x30e0 - AXIMC master 5 write priority register"]
    pub aximc_m5_write_qos: AXIMC_M5_WRITE_QOS,
    #[doc = "0x30e4 - AXIMC master 5 issuing capability override functionality register"]
    pub aximc_m5_fn_mod: AXIMC_M5_FN_MOD,
    _reserved28: [u8; 4084usize],
    #[doc = "0x40dc - AXIMC master 3 read priority register"]
    pub aximc_m3_read_qos: AXIMC_M3_READ_QOS,
    #[doc = "0x40e0 - AXIMC master 3 write priority register"]
    pub aximc_m3_write_qos: AXIMC_M3_WRITE_QOS,
    #[doc = "0x40e4 - AXIMC master 3 packing functionality register"]
    pub aximc_m3_fn_mod: AXIMC_M3_FN_MOD,
    _reserved31: [u8; 4084usize],
    #[doc = "0x50dc - AXIMC master 7 read priority register"]
    pub aximc_m7_read_qos: AXIMC_M7_READ_QOS,
    #[doc = "0x50e0 - AXIMC master 7 write priority register"]
    pub aximc_m7_write_qos: AXIMC_M7_WRITE_QOS,
    #[doc = "0x50e4 - AXIMC master 7 issuing capability override functionality register"]
    pub aximc_m7_fn_mod: AXIMC_M7_FN_MOD,
    _reserved34: [u8; 4084usize],
    #[doc = "0x60dc - AXIMC master 8 read priority register"]
    pub aximc_m8_read_qos: AXIMC_M8_READ_QOS,
    #[doc = "0x60e0 - AXIMC master 8 write priority register"]
    pub aximc_m8_write_qos: AXIMC_M8_WRITE_QOS,
    #[doc = "0x60e4 - AXIMC master 8 issuing capability override functionality register"]
    pub aximc_m8_fn_mod: AXIMC_M8_FN_MOD,
    _reserved37: [u8; 7960usize],
    #[doc = "0x8000 - AXIMC master 4 packing functionality register"]
    pub aximc_m4_fn_mod2: AXIMC_M4_FN_MOD2,
    _reserved38: [u8; 216usize],
    #[doc = "0x80dc - AXIMC master 4 read priority register"]
    pub aximc_m4_read_qos: AXIMC_M4_READ_QOS,
    #[doc = "0x80e0 - AXIMC master 4 write priority register"]
    pub aximc_m4_write_qos: AXIMC_M4_WRITE_QOS,
    #[doc = "0x80e4 - AXIMC master 4 packing functionality register"]
    pub aximc_m4_fn_mod: AXIMC_M4_FN_MOD,
    _reserved41: [u8; 4084usize],
    #[doc = "0x90dc - AXIMC master 9 read priority register"]
    pub aximc_m9_read_qos: AXIMC_M9_READ_QOS,
    #[doc = "0x90e0 - AXIMC master 9 write priority register"]
    pub aximc_m9_write_qos: AXIMC_M9_WRITE_QOS,
    #[doc = "0x90e4 - AXIMC master 9 issuing capability override functionality register"]
    pub aximc_m9_fn_mod: AXIMC_M9_FN_MOD,
    _reserved44: [u8; 4084usize],
    #[doc = "0xa0dc - AXIMC master 10 read priority register"]
    pub aximc_m10_read_qos: AXIMC_M10_READ_QOS,
    #[doc = "0xa0e0 - AXIMC master 10 write priority register"]
    pub aximc_m10_write_qos: AXIMC_M10_WRITE_QOS,
    #[doc = "0xa0e4 - AXIMC master 10 issuing capability override functionality register"]
    pub aximc_m10_fn_mod: AXIMC_M10_FN_MOD,
    _reserved47: [u8; 3864usize],
    #[doc = "0xb000 - AXIMC master 6 packing functionality register"]
    pub aximc_m6_fn_mod2: AXIMC_M6_FN_MOD2,
    _reserved48: [u8; 216usize],
    #[doc = "0xb0dc - AXIMC master 6 read priority register"]
    pub aximc_m6_read_qos: AXIMC_M6_READ_QOS,
    #[doc = "0xb0e0 - AXIMC master 6 write priority register"]
    pub aximc_m6_write_qos: AXIMC_M6_WRITE_QOS,
    #[doc = "0xb0e4 - AXIMC master 6 issuing capability override functionality register"]
    pub aximc_m6_fn_mod: AXIMC_M6_FN_MOD,
    _reserved51: [u8; 225088usize],
    #[doc = "0x42028 - AXIMC master 0 AHB conversion override functionality register"]
    pub aximc_m0_fn_mod_ahb: AXIMC_M0_FN_MOD_AHB,
    _reserved52: [u8; 4092usize],
    #[doc = "0x43028 - AXIMC master 1 AHB conversion override functionality register"]
    pub aximc_m1_fn_mod_ahb: AXIMC_M1_FN_MOD_AHB,
    _reserved53: [u8; 4092usize],
    #[doc = "0x44028 - AXIMC master 2 AHB conversion override functionality register"]
    pub aximc_m2_fn_mod_ahb: AXIMC_M2_FN_MOD_AHB,
    _reserved54: [u8; 4092usize],
    #[doc = "0x45028 - AXIMC master 5 AHB conversion override functionality register"]
    pub aximc_m5_fn_mod_ahb: AXIMC_M5_FN_MOD_AHB,
    _reserved55: [u8; 20480usize],
    #[doc = "0x4a02c - AXIMC long burst capability inhibition register"]
    pub aximc_fn_mod_lb: AXIMC_FN_MOD_LB,
    _reserved56: [u8; 12280usize],
    #[doc = "0x4d028 - AXIMC master 6 AHB conversion override functionality register"]
    pub aximc_m6_fn_mod_ahb: AXIMC_M6_FN_MOD_AHB,
}
#[doc = "AXIMC master 0 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m0_fn_mod2](aximc_m0_fn_mod2) module"]
pub type AXIMC_M0_FN_MOD2 = crate::Reg<u32, _AXIMC_M0_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M0_FN_MOD2;
#[doc = "`read()` method returns [aximc_m0_fn_mod2::R](aximc_m0_fn_mod2::R) reader structure"]
impl crate::Readable for AXIMC_M0_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [aximc_m0_fn_mod2::W](aximc_m0_fn_mod2::W) writer structure"]
impl crate::Writable for AXIMC_M0_FN_MOD2 {}
#[doc = "AXIMC master 0 packing functionality register"]
pub mod aximc_m0_fn_mod2;
#[doc = "AXIMC master 0 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m0_read_qos](aximc_m0_read_qos) module"]
pub type AXIMC_M0_READ_QOS = crate::Reg<u32, _AXIMC_M0_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M0_READ_QOS;
#[doc = "`read()` method returns [aximc_m0_read_qos::R](aximc_m0_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M0_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m0_read_qos::W](aximc_m0_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M0_READ_QOS {}
#[doc = "AXIMC master 0 read priority register"]
pub mod aximc_m0_read_qos;
#[doc = "AXIMC master 0 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m0_write_qos](aximc_m0_write_qos) module"]
pub type AXIMC_M0_WRITE_QOS = crate::Reg<u32, _AXIMC_M0_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M0_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m0_write_qos::R](aximc_m0_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M0_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m0_write_qos::W](aximc_m0_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M0_WRITE_QOS {}
#[doc = "AXIMC master 0 write priority register"]
pub mod aximc_m0_write_qos;
#[doc = "AXIMC master 0 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m0_fn_mod](aximc_m0_fn_mod) module"]
pub type AXIMC_M0_FN_MOD = crate::Reg<u32, _AXIMC_M0_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M0_FN_MOD;
#[doc = "`read()` method returns [aximc_m0_fn_mod::R](aximc_m0_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M0_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m0_fn_mod::W](aximc_m0_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M0_FN_MOD {}
#[doc = "AXIMC master 0 issuing capability override functionality register"]
pub mod aximc_m0_fn_mod;
#[doc = "AXIMC master 1 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m1_fn_mod2](aximc_m1_fn_mod2) module"]
pub type AXIMC_M1_FN_MOD2 = crate::Reg<u32, _AXIMC_M1_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M1_FN_MOD2;
#[doc = "`read()` method returns [aximc_m1_fn_mod2::R](aximc_m1_fn_mod2::R) reader structure"]
impl crate::Readable for AXIMC_M1_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [aximc_m1_fn_mod2::W](aximc_m1_fn_mod2::W) writer structure"]
impl crate::Writable for AXIMC_M1_FN_MOD2 {}
#[doc = "AXIMC master 1 packing functionality register"]
pub mod aximc_m1_fn_mod2;
#[doc = "AXIMC master 1 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m1_read_qos](aximc_m1_read_qos) module"]
pub type AXIMC_M1_READ_QOS = crate::Reg<u32, _AXIMC_M1_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M1_READ_QOS;
#[doc = "`read()` method returns [aximc_m1_read_qos::R](aximc_m1_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M1_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m1_read_qos::W](aximc_m1_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M1_READ_QOS {}
#[doc = "AXIMC master 1 read priority register"]
pub mod aximc_m1_read_qos;
#[doc = "AXIMC master 1 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m1_write_qos](aximc_m1_write_qos) module"]
pub type AXIMC_M1_WRITE_QOS = crate::Reg<u32, _AXIMC_M1_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M1_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m1_write_qos::R](aximc_m1_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M1_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m1_write_qos::W](aximc_m1_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M1_WRITE_QOS {}
#[doc = "AXIMC master 1 write priority register"]
pub mod aximc_m1_write_qos;
#[doc = "AXIMC master 1 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m1_fn_mod](aximc_m1_fn_mod) module"]
pub type AXIMC_M1_FN_MOD = crate::Reg<u32, _AXIMC_M1_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M1_FN_MOD;
#[doc = "`read()` method returns [aximc_m1_fn_mod::R](aximc_m1_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M1_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m1_fn_mod::W](aximc_m1_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M1_FN_MOD {}
#[doc = "AXIMC master 1 issuing capability override functionality register"]
pub mod aximc_m1_fn_mod;
#[doc = "AXIMC master 2 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m2_fn_mod2](aximc_m2_fn_mod2) module"]
pub type AXIMC_M2_FN_MOD2 = crate::Reg<u32, _AXIMC_M2_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M2_FN_MOD2;
#[doc = "`read()` method returns [aximc_m2_fn_mod2::R](aximc_m2_fn_mod2::R) reader structure"]
impl crate::Readable for AXIMC_M2_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [aximc_m2_fn_mod2::W](aximc_m2_fn_mod2::W) writer structure"]
impl crate::Writable for AXIMC_M2_FN_MOD2 {}
#[doc = "AXIMC master 2 packing functionality register"]
pub mod aximc_m2_fn_mod2;
#[doc = "AXIMC master 2 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m2_read_qos](aximc_m2_read_qos) module"]
pub type AXIMC_M2_READ_QOS = crate::Reg<u32, _AXIMC_M2_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M2_READ_QOS;
#[doc = "`read()` method returns [aximc_m2_read_qos::R](aximc_m2_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M2_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m2_read_qos::W](aximc_m2_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M2_READ_QOS {}
#[doc = "AXIMC master 2 read priority register"]
pub mod aximc_m2_read_qos;
#[doc = "AXIMC master 2 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m2_write_qos](aximc_m2_write_qos) module"]
pub type AXIMC_M2_WRITE_QOS = crate::Reg<u32, _AXIMC_M2_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M2_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m2_write_qos::R](aximc_m2_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M2_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m2_write_qos::W](aximc_m2_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M2_WRITE_QOS {}
#[doc = "AXIMC master 2 write priority register"]
pub mod aximc_m2_write_qos;
#[doc = "AXIMC master 2 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m2_fn_mod](aximc_m2_fn_mod) module"]
pub type AXIMC_M2_FN_MOD = crate::Reg<u32, _AXIMC_M2_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M2_FN_MOD;
#[doc = "`read()` method returns [aximc_m2_fn_mod::R](aximc_m2_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M2_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m2_fn_mod::W](aximc_m2_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M2_FN_MOD {}
#[doc = "AXIMC master 2 issuing capability override functionality register"]
pub mod aximc_m2_fn_mod;
#[doc = "AXIMC master 5 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m5_fn_mod2](aximc_m5_fn_mod2) module"]
pub type AXIMC_M5_FN_MOD2 = crate::Reg<u32, _AXIMC_M5_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M5_FN_MOD2;
#[doc = "`read()` method returns [aximc_m5_fn_mod2::R](aximc_m5_fn_mod2::R) reader structure"]
impl crate::Readable for AXIMC_M5_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [aximc_m5_fn_mod2::W](aximc_m5_fn_mod2::W) writer structure"]
impl crate::Writable for AXIMC_M5_FN_MOD2 {}
#[doc = "AXIMC master 5 packing functionality register"]
pub mod aximc_m5_fn_mod2;
#[doc = "AXIMC master 5 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m5_read_qos](aximc_m5_read_qos) module"]
pub type AXIMC_M5_READ_QOS = crate::Reg<u32, _AXIMC_M5_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M5_READ_QOS;
#[doc = "`read()` method returns [aximc_m5_read_qos::R](aximc_m5_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M5_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m5_read_qos::W](aximc_m5_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M5_READ_QOS {}
#[doc = "AXIMC master 5 read priority register"]
pub mod aximc_m5_read_qos;
#[doc = "AXIMC master 5 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m5_write_qos](aximc_m5_write_qos) module"]
pub type AXIMC_M5_WRITE_QOS = crate::Reg<u32, _AXIMC_M5_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M5_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m5_write_qos::R](aximc_m5_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M5_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m5_write_qos::W](aximc_m5_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M5_WRITE_QOS {}
#[doc = "AXIMC master 5 write priority register"]
pub mod aximc_m5_write_qos;
#[doc = "AXIMC master 5 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m5_fn_mod](aximc_m5_fn_mod) module"]
pub type AXIMC_M5_FN_MOD = crate::Reg<u32, _AXIMC_M5_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M5_FN_MOD;
#[doc = "`read()` method returns [aximc_m5_fn_mod::R](aximc_m5_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M5_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m5_fn_mod::W](aximc_m5_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M5_FN_MOD {}
#[doc = "AXIMC master 5 issuing capability override functionality register"]
pub mod aximc_m5_fn_mod;
#[doc = "AXIMC master 3 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m3_read_qos](aximc_m3_read_qos) module"]
pub type AXIMC_M3_READ_QOS = crate::Reg<u32, _AXIMC_M3_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M3_READ_QOS;
#[doc = "`read()` method returns [aximc_m3_read_qos::R](aximc_m3_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M3_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m3_read_qos::W](aximc_m3_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M3_READ_QOS {}
#[doc = "AXIMC master 3 read priority register"]
pub mod aximc_m3_read_qos;
#[doc = "AXIMC master 3 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m3_write_qos](aximc_m3_write_qos) module"]
pub type AXIMC_M3_WRITE_QOS = crate::Reg<u32, _AXIMC_M3_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M3_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m3_write_qos::R](aximc_m3_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M3_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m3_write_qos::W](aximc_m3_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M3_WRITE_QOS {}
#[doc = "AXIMC master 3 write priority register"]
pub mod aximc_m3_write_qos;
#[doc = "AXIMC master 3 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m3_fn_mod](aximc_m3_fn_mod) module"]
pub type AXIMC_M3_FN_MOD = crate::Reg<u32, _AXIMC_M3_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M3_FN_MOD;
#[doc = "`read()` method returns [aximc_m3_fn_mod::R](aximc_m3_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M3_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m3_fn_mod::W](aximc_m3_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M3_FN_MOD {}
#[doc = "AXIMC master 3 packing functionality register"]
pub mod aximc_m3_fn_mod;
#[doc = "AXIMC master 7 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m7_read_qos](aximc_m7_read_qos) module"]
pub type AXIMC_M7_READ_QOS = crate::Reg<u32, _AXIMC_M7_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M7_READ_QOS;
#[doc = "`read()` method returns [aximc_m7_read_qos::R](aximc_m7_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M7_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m7_read_qos::W](aximc_m7_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M7_READ_QOS {}
#[doc = "AXIMC master 7 read priority register"]
pub mod aximc_m7_read_qos;
#[doc = "AXIMC master 7 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m7_write_qos](aximc_m7_write_qos) module"]
pub type AXIMC_M7_WRITE_QOS = crate::Reg<u32, _AXIMC_M7_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M7_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m7_write_qos::R](aximc_m7_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M7_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m7_write_qos::W](aximc_m7_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M7_WRITE_QOS {}
#[doc = "AXIMC master 7 write priority register"]
pub mod aximc_m7_write_qos;
#[doc = "AXIMC master 7 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m7_fn_mod](aximc_m7_fn_mod) module"]
pub type AXIMC_M7_FN_MOD = crate::Reg<u32, _AXIMC_M7_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M7_FN_MOD;
#[doc = "`read()` method returns [aximc_m7_fn_mod::R](aximc_m7_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M7_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m7_fn_mod::W](aximc_m7_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M7_FN_MOD {}
#[doc = "AXIMC master 7 issuing capability override functionality register"]
pub mod aximc_m7_fn_mod;
#[doc = "AXIMC master 8 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m8_read_qos](aximc_m8_read_qos) module"]
pub type AXIMC_M8_READ_QOS = crate::Reg<u32, _AXIMC_M8_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M8_READ_QOS;
#[doc = "`read()` method returns [aximc_m8_read_qos::R](aximc_m8_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M8_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m8_read_qos::W](aximc_m8_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M8_READ_QOS {}
#[doc = "AXIMC master 8 read priority register"]
pub mod aximc_m8_read_qos;
#[doc = "AXIMC master 8 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m8_write_qos](aximc_m8_write_qos) module"]
pub type AXIMC_M8_WRITE_QOS = crate::Reg<u32, _AXIMC_M8_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M8_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m8_write_qos::R](aximc_m8_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M8_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m8_write_qos::W](aximc_m8_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M8_WRITE_QOS {}
#[doc = "AXIMC master 8 write priority register"]
pub mod aximc_m8_write_qos;
#[doc = "AXIMC master 8 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m8_fn_mod](aximc_m8_fn_mod) module"]
pub type AXIMC_M8_FN_MOD = crate::Reg<u32, _AXIMC_M8_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M8_FN_MOD;
#[doc = "`read()` method returns [aximc_m8_fn_mod::R](aximc_m8_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M8_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m8_fn_mod::W](aximc_m8_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M8_FN_MOD {}
#[doc = "AXIMC master 8 issuing capability override functionality register"]
pub mod aximc_m8_fn_mod;
#[doc = "AXIMC master 4 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m4_fn_mod2](aximc_m4_fn_mod2) module"]
pub type AXIMC_M4_FN_MOD2 = crate::Reg<u32, _AXIMC_M4_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M4_FN_MOD2;
#[doc = "`read()` method returns [aximc_m4_fn_mod2::R](aximc_m4_fn_mod2::R) reader structure"]
impl crate::Readable for AXIMC_M4_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [aximc_m4_fn_mod2::W](aximc_m4_fn_mod2::W) writer structure"]
impl crate::Writable for AXIMC_M4_FN_MOD2 {}
#[doc = "AXIMC master 4 packing functionality register"]
pub mod aximc_m4_fn_mod2;
#[doc = "AXIMC master 4 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m4_read_qos](aximc_m4_read_qos) module"]
pub type AXIMC_M4_READ_QOS = crate::Reg<u32, _AXIMC_M4_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M4_READ_QOS;
#[doc = "`read()` method returns [aximc_m4_read_qos::R](aximc_m4_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M4_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m4_read_qos::W](aximc_m4_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M4_READ_QOS {}
#[doc = "AXIMC master 4 read priority register"]
pub mod aximc_m4_read_qos;
#[doc = "AXIMC master 4 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m4_write_qos](aximc_m4_write_qos) module"]
pub type AXIMC_M4_WRITE_QOS = crate::Reg<u32, _AXIMC_M4_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M4_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m4_write_qos::R](aximc_m4_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M4_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m4_write_qos::W](aximc_m4_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M4_WRITE_QOS {}
#[doc = "AXIMC master 4 write priority register"]
pub mod aximc_m4_write_qos;
#[doc = "AXIMC master 4 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m4_fn_mod](aximc_m4_fn_mod) module"]
pub type AXIMC_M4_FN_MOD = crate::Reg<u32, _AXIMC_M4_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M4_FN_MOD;
#[doc = "`read()` method returns [aximc_m4_fn_mod::R](aximc_m4_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M4_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m4_fn_mod::W](aximc_m4_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M4_FN_MOD {}
#[doc = "AXIMC master 4 packing functionality register"]
pub mod aximc_m4_fn_mod;
#[doc = "AXIMC master 9 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m9_read_qos](aximc_m9_read_qos) module"]
pub type AXIMC_M9_READ_QOS = crate::Reg<u32, _AXIMC_M9_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M9_READ_QOS;
#[doc = "`read()` method returns [aximc_m9_read_qos::R](aximc_m9_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M9_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m9_read_qos::W](aximc_m9_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M9_READ_QOS {}
#[doc = "AXIMC master 9 read priority register"]
pub mod aximc_m9_read_qos;
#[doc = "AXIMC master 9 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m9_write_qos](aximc_m9_write_qos) module"]
pub type AXIMC_M9_WRITE_QOS = crate::Reg<u32, _AXIMC_M9_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M9_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m9_write_qos::R](aximc_m9_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M9_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m9_write_qos::W](aximc_m9_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M9_WRITE_QOS {}
#[doc = "AXIMC master 9 write priority register"]
pub mod aximc_m9_write_qos;
#[doc = "AXIMC master 9 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m9_fn_mod](aximc_m9_fn_mod) module"]
pub type AXIMC_M9_FN_MOD = crate::Reg<u32, _AXIMC_M9_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M9_FN_MOD;
#[doc = "`read()` method returns [aximc_m9_fn_mod::R](aximc_m9_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M9_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m9_fn_mod::W](aximc_m9_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M9_FN_MOD {}
#[doc = "AXIMC master 9 issuing capability override functionality register"]
pub mod aximc_m9_fn_mod;
#[doc = "AXIMC master 10 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m10_read_qos](aximc_m10_read_qos) module"]
pub type AXIMC_M10_READ_QOS = crate::Reg<u32, _AXIMC_M10_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M10_READ_QOS;
#[doc = "`read()` method returns [aximc_m10_read_qos::R](aximc_m10_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M10_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m10_read_qos::W](aximc_m10_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M10_READ_QOS {}
#[doc = "AXIMC master 10 read priority register"]
pub mod aximc_m10_read_qos;
#[doc = "AXIMC master 10 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m10_write_qos](aximc_m10_write_qos) module"]
pub type AXIMC_M10_WRITE_QOS = crate::Reg<u32, _AXIMC_M10_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M10_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m10_write_qos::R](aximc_m10_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M10_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m10_write_qos::W](aximc_m10_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M10_WRITE_QOS {}
#[doc = "AXIMC master 10 write priority register"]
pub mod aximc_m10_write_qos;
#[doc = "AXIMC master 10 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m10_fn_mod](aximc_m10_fn_mod) module"]
pub type AXIMC_M10_FN_MOD = crate::Reg<u32, _AXIMC_M10_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M10_FN_MOD;
#[doc = "`read()` method returns [aximc_m10_fn_mod::R](aximc_m10_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M10_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m10_fn_mod::W](aximc_m10_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M10_FN_MOD {}
#[doc = "AXIMC master 10 issuing capability override functionality register"]
pub mod aximc_m10_fn_mod;
#[doc = "AXIMC master 6 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m6_fn_mod2](aximc_m6_fn_mod2) module"]
pub type AXIMC_M6_FN_MOD2 = crate::Reg<u32, _AXIMC_M6_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M6_FN_MOD2;
#[doc = "`read()` method returns [aximc_m6_fn_mod2::R](aximc_m6_fn_mod2::R) reader structure"]
impl crate::Readable for AXIMC_M6_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [aximc_m6_fn_mod2::W](aximc_m6_fn_mod2::W) writer structure"]
impl crate::Writable for AXIMC_M6_FN_MOD2 {}
#[doc = "AXIMC master 6 packing functionality register"]
pub mod aximc_m6_fn_mod2;
#[doc = "AXIMC master 6 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m6_read_qos](aximc_m6_read_qos) module"]
pub type AXIMC_M6_READ_QOS = crate::Reg<u32, _AXIMC_M6_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M6_READ_QOS;
#[doc = "`read()` method returns [aximc_m6_read_qos::R](aximc_m6_read_qos::R) reader structure"]
impl crate::Readable for AXIMC_M6_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m6_read_qos::W](aximc_m6_read_qos::W) writer structure"]
impl crate::Writable for AXIMC_M6_READ_QOS {}
#[doc = "AXIMC master 6 read priority register"]
pub mod aximc_m6_read_qos;
#[doc = "AXIMC master 6 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m6_write_qos](aximc_m6_write_qos) module"]
pub type AXIMC_M6_WRITE_QOS = crate::Reg<u32, _AXIMC_M6_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M6_WRITE_QOS;
#[doc = "`read()` method returns [aximc_m6_write_qos::R](aximc_m6_write_qos::R) reader structure"]
impl crate::Readable for AXIMC_M6_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [aximc_m6_write_qos::W](aximc_m6_write_qos::W) writer structure"]
impl crate::Writable for AXIMC_M6_WRITE_QOS {}
#[doc = "AXIMC master 6 write priority register"]
pub mod aximc_m6_write_qos;
#[doc = "AXIMC master 6 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m6_fn_mod](aximc_m6_fn_mod) module"]
pub type AXIMC_M6_FN_MOD = crate::Reg<u32, _AXIMC_M6_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M6_FN_MOD;
#[doc = "`read()` method returns [aximc_m6_fn_mod::R](aximc_m6_fn_mod::R) reader structure"]
impl crate::Readable for AXIMC_M6_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [aximc_m6_fn_mod::W](aximc_m6_fn_mod::W) writer structure"]
impl crate::Writable for AXIMC_M6_FN_MOD {}
#[doc = "AXIMC master 6 issuing capability override functionality register"]
pub mod aximc_m6_fn_mod;
#[doc = "AXIMC peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_4](aximc_periph_id_4) module"]
pub type AXIMC_PERIPH_ID_4 = crate::Reg<u32, _AXIMC_PERIPH_ID_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_4;
#[doc = "`read()` method returns [aximc_periph_id_4::R](aximc_periph_id_4::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_4 {}
#[doc = "AXIMC peripheral ID4 register"]
pub mod aximc_periph_id_4;
#[doc = "AXIMC peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_5](aximc_periph_id_5) module"]
pub type AXIMC_PERIPH_ID_5 = crate::Reg<u32, _AXIMC_PERIPH_ID_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_5;
#[doc = "`read()` method returns [aximc_periph_id_5::R](aximc_periph_id_5::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_5 {}
#[doc = "AXIMC peripheral ID5 register"]
pub mod aximc_periph_id_5;
#[doc = "AXIMC peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_6](aximc_periph_id_6) module"]
pub type AXIMC_PERIPH_ID_6 = crate::Reg<u32, _AXIMC_PERIPH_ID_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_6;
#[doc = "`read()` method returns [aximc_periph_id_6::R](aximc_periph_id_6::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_6 {}
#[doc = "AXIMC peripheral ID6 register"]
pub mod aximc_periph_id_6;
#[doc = "AXIMC peripheral ID7 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_7](aximc_periph_id_7) module"]
pub type AXIMC_PERIPH_ID_7 = crate::Reg<u32, _AXIMC_PERIPH_ID_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_7;
#[doc = "`read()` method returns [aximc_periph_id_7::R](aximc_periph_id_7::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_7 {}
#[doc = "AXIMC peripheral ID7 register"]
pub mod aximc_periph_id_7;
#[doc = "AXIMC peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_0](aximc_periph_id_0) module"]
pub type AXIMC_PERIPH_ID_0 = crate::Reg<u32, _AXIMC_PERIPH_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_0;
#[doc = "`read()` method returns [aximc_periph_id_0::R](aximc_periph_id_0::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_0 {}
#[doc = "AXIMC peripheral ID0 register"]
pub mod aximc_periph_id_0;
#[doc = "AXIMC peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_1](aximc_periph_id_1) module"]
pub type AXIMC_PERIPH_ID_1 = crate::Reg<u32, _AXIMC_PERIPH_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_1;
#[doc = "`read()` method returns [aximc_periph_id_1::R](aximc_periph_id_1::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_1 {}
#[doc = "AXIMC peripheral ID1 register"]
pub mod aximc_periph_id_1;
#[doc = "AXIMC peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_2](aximc_periph_id_2) module"]
pub type AXIMC_PERIPH_ID_2 = crate::Reg<u32, _AXIMC_PERIPH_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_2;
#[doc = "`read()` method returns [aximc_periph_id_2::R](aximc_periph_id_2::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_2 {}
#[doc = "AXIMC peripheral ID2 register"]
pub mod aximc_periph_id_2;
#[doc = "AXIMC peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_3](aximc_periph_id_3) module"]
pub type AXIMC_PERIPH_ID_3 = crate::Reg<u32, _AXIMC_PERIPH_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_PERIPH_ID_3;
#[doc = "`read()` method returns [aximc_periph_id_3::R](aximc_periph_id_3::R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_3 {}
#[doc = "AXIMC peripheral ID3 register"]
pub mod aximc_periph_id_3;
#[doc = "AXIMC component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_comp_id_0](aximc_comp_id_0) module"]
pub type AXIMC_COMP_ID_0 = crate::Reg<u32, _AXIMC_COMP_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_COMP_ID_0;
#[doc = "`read()` method returns [aximc_comp_id_0::R](aximc_comp_id_0::R) reader structure"]
impl crate::Readable for AXIMC_COMP_ID_0 {}
#[doc = "AXIMC component ID0 register"]
pub mod aximc_comp_id_0;
#[doc = "AXIMC component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_comp_id_1](aximc_comp_id_1) module"]
pub type AXIMC_COMP_ID_1 = crate::Reg<u32, _AXIMC_COMP_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_COMP_ID_1;
#[doc = "`read()` method returns [aximc_comp_id_1::R](aximc_comp_id_1::R) reader structure"]
impl crate::Readable for AXIMC_COMP_ID_1 {}
#[doc = "AXIMC component ID1 register"]
pub mod aximc_comp_id_1;
#[doc = "AXIMC component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_comp_id_2](aximc_comp_id_2) module"]
pub type AXIMC_COMP_ID_2 = crate::Reg<u32, _AXIMC_COMP_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_COMP_ID_2;
#[doc = "`read()` method returns [aximc_comp_id_2::R](aximc_comp_id_2::R) reader structure"]
impl crate::Readable for AXIMC_COMP_ID_2 {}
#[doc = "AXIMC component ID2 register"]
pub mod aximc_comp_id_2;
#[doc = "AXIMC component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_comp_id_3](aximc_comp_id_3) module"]
pub type AXIMC_COMP_ID_3 = crate::Reg<u32, _AXIMC_COMP_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_COMP_ID_3;
#[doc = "`read()` method returns [aximc_comp_id_3::R](aximc_comp_id_3::R) reader structure"]
impl crate::Readable for AXIMC_COMP_ID_3 {}
#[doc = "AXIMC component ID3 register"]
pub mod aximc_comp_id_3;
#[doc = "AXIMC master 0 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m0_fn_mod_ahb](aximc_m0_fn_mod_ahb) module"]
pub type AXIMC_M0_FN_MOD_AHB = crate::Reg<u32, _AXIMC_M0_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M0_FN_MOD_AHB;
#[doc = "`read()` method returns [aximc_m0_fn_mod_ahb::R](aximc_m0_fn_mod_ahb::R) reader structure"]
impl crate::Readable for AXIMC_M0_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [aximc_m0_fn_mod_ahb::W](aximc_m0_fn_mod_ahb::W) writer structure"]
impl crate::Writable for AXIMC_M0_FN_MOD_AHB {}
#[doc = "AXIMC master 0 AHB conversion override functionality register"]
pub mod aximc_m0_fn_mod_ahb;
#[doc = "AXIMC master 1 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m1_fn_mod_ahb](aximc_m1_fn_mod_ahb) module"]
pub type AXIMC_M1_FN_MOD_AHB = crate::Reg<u32, _AXIMC_M1_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M1_FN_MOD_AHB;
#[doc = "`read()` method returns [aximc_m1_fn_mod_ahb::R](aximc_m1_fn_mod_ahb::R) reader structure"]
impl crate::Readable for AXIMC_M1_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [aximc_m1_fn_mod_ahb::W](aximc_m1_fn_mod_ahb::W) writer structure"]
impl crate::Writable for AXIMC_M1_FN_MOD_AHB {}
#[doc = "AXIMC master 1 AHB conversion override functionality register"]
pub mod aximc_m1_fn_mod_ahb;
#[doc = "AXIMC master 2 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m2_fn_mod_ahb](aximc_m2_fn_mod_ahb) module"]
pub type AXIMC_M2_FN_MOD_AHB = crate::Reg<u32, _AXIMC_M2_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M2_FN_MOD_AHB;
#[doc = "`read()` method returns [aximc_m2_fn_mod_ahb::R](aximc_m2_fn_mod_ahb::R) reader structure"]
impl crate::Readable for AXIMC_M2_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [aximc_m2_fn_mod_ahb::W](aximc_m2_fn_mod_ahb::W) writer structure"]
impl crate::Writable for AXIMC_M2_FN_MOD_AHB {}
#[doc = "AXIMC master 2 AHB conversion override functionality register"]
pub mod aximc_m2_fn_mod_ahb;
#[doc = "AXIMC master 5 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m5_fn_mod_ahb](aximc_m5_fn_mod_ahb) module"]
pub type AXIMC_M5_FN_MOD_AHB = crate::Reg<u32, _AXIMC_M5_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M5_FN_MOD_AHB;
#[doc = "`read()` method returns [aximc_m5_fn_mod_ahb::R](aximc_m5_fn_mod_ahb::R) reader structure"]
impl crate::Readable for AXIMC_M5_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [aximc_m5_fn_mod_ahb::W](aximc_m5_fn_mod_ahb::W) writer structure"]
impl crate::Writable for AXIMC_M5_FN_MOD_AHB {}
#[doc = "AXIMC master 5 AHB conversion override functionality register"]
pub mod aximc_m5_fn_mod_ahb;
#[doc = "AXIMC master 6 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_m6_fn_mod_ahb](aximc_m6_fn_mod_ahb) module"]
pub type AXIMC_M6_FN_MOD_AHB = crate::Reg<u32, _AXIMC_M6_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_M6_FN_MOD_AHB;
#[doc = "`read()` method returns [aximc_m6_fn_mod_ahb::R](aximc_m6_fn_mod_ahb::R) reader structure"]
impl crate::Readable for AXIMC_M6_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [aximc_m6_fn_mod_ahb::W](aximc_m6_fn_mod_ahb::W) writer structure"]
impl crate::Writable for AXIMC_M6_FN_MOD_AHB {}
#[doc = "AXIMC master 6 AHB conversion override functionality register"]
pub mod aximc_m6_fn_mod_ahb;
#[doc = "AXIMC long burst capability inhibition register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_fn_mod_lb](aximc_fn_mod_lb) module"]
pub type AXIMC_FN_MOD_LB = crate::Reg<u32, _AXIMC_FN_MOD_LB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXIMC_FN_MOD_LB;
#[doc = "`read()` method returns [aximc_fn_mod_lb::R](aximc_fn_mod_lb::R) reader structure"]
impl crate::Readable for AXIMC_FN_MOD_LB {}
#[doc = "`write(|w| ..)` method takes [aximc_fn_mod_lb::W](aximc_fn_mod_lb::W) writer structure"]
impl crate::Writable for AXIMC_FN_MOD_LB {}
#[doc = "AXIMC long burst capability inhibition register"]
pub mod aximc_fn_mod_lb;
