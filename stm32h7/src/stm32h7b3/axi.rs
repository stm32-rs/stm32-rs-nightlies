#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8144usize],
    #[doc = "0x1fd0 - AXI interconnect - peripheral ID4 register"]
    pub periph_id_4: PERIPH_ID_4,
    _reserved1: [u8; 12usize],
    #[doc = "0x1fe0 - AXI interconnect - peripheral ID0 register"]
    pub periph_id_0: PERIPH_ID_0,
    #[doc = "0x1fe4 - AXI interconnect - peripheral ID1 register"]
    pub periph_id_1: PERIPH_ID_1,
    #[doc = "0x1fe8 - AXI interconnect - peripheral ID2 register"]
    pub periph_id_2: PERIPH_ID_2,
    #[doc = "0x1fec - AXI interconnect - peripheral ID3 register"]
    pub periph_id_3: PERIPH_ID_3,
    #[doc = "0x1ff0 - AXI interconnect - component ID0 register"]
    pub comp_id_0: COMP_ID_0,
    #[doc = "0x1ff4 - AXI interconnect - component ID1 register"]
    pub comp_id_1: COMP_ID_1,
    #[doc = "0x1ff8 - AXI interconnect - component ID2 register"]
    pub comp_id_2: COMP_ID_2,
    #[doc = "0x1ffc - AXI interconnect - component ID3 register"]
    pub comp_id_3: COMP_ID_3,
    _reserved9: [u8; 8usize],
    #[doc = "0x2008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ1_fn_mod_iss_bm: TARG1_FN_MOD_ISS_BM,
    _reserved10: [u8; 24usize],
    #[doc = "0x2024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub targ1_fn_mod2: TARG1_FN_MOD2,
    _reserved11: [u8; 4usize],
    #[doc = "0x202c - AXI interconnect - TARG x long burst functionality modification"]
    pub targ1_fn_mod_lb: TARG1_FN_MOD_LB,
    _reserved12: [u8; 216usize],
    #[doc = "0x2108 - AXI interconnect - TARG x long burst functionality modification"]
    pub targ1_fn_mod: TARG1_FN_MOD,
    _reserved13: [u8; 3836usize],
    #[doc = "0x3008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ2_fn_mod_iss_bm: TARG2_FN_MOD_ISS_BM,
    _reserved14: [u8; 24usize],
    #[doc = "0x3024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub targ2_fn_mod2: TARG2_FN_MOD2,
    _reserved15: [u8; 4usize],
    #[doc = "0x302c - AXI interconnect - TARG x long burst functionality modification"]
    pub targ2_fn_mod_lb: TARG2_FN_MOD_LB,
    _reserved16: [u8; 216usize],
    #[doc = "0x3108 - AXI interconnect - TARG x long burst functionality modification"]
    pub targ2_fn_mod: TARG2_FN_MOD,
    _reserved17: [u8; 3836usize],
    #[doc = "0x4008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ3_fn_mod_iss_bm: TARG3_FN_MOD_ISS_BM,
    _reserved18: [u8; 4092usize],
    #[doc = "0x5008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ4_fn_mod_iss_bm: TARG4_FN_MOD_ISS_BM,
    _reserved19: [u8; 4092usize],
    #[doc = "0x6008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ5_fn_mod_iss_bm: TARG5_FN_MOD_ISS_BM,
    _reserved20: [u8; 4092usize],
    #[doc = "0x7008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ6_fn_mod_iss_bm: TARG6_FN_MOD_ISS_BM,
    _reserved21: [u8; 4096usize],
    #[doc = "0x800c - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ7_fn_mod_iss_bm: TARG7_FN_MOD_ISS_BM,
    _reserved22: [u8; 20usize],
    #[doc = "0x8024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub targ7_fn_mod2: TARG7_FN_MOD2,
    _reserved23: [u8; 224usize],
    #[doc = "0x8108 - AXI interconnect - TARG x long burst functionality modification"]
    pub targ7_fn_mod: TARG7_FN_MOD,
    _reserved24: [u8; 237336usize],
    #[doc = "0x42024 - AXI interconnect - INI x functionality modification 2 register"]
    pub ini1_fn_mod2: INI1_FN_MOD2,
    #[doc = "0x42028 - AXI interconnect - INI x AHB functionality modification register"]
    pub ini1_fn_mod_ahb: INI1_FN_MOD_AHB,
    _reserved26: [u8; 212usize],
    #[doc = "0x42100 - AXI interconnect - INI x read QoS register"]
    pub ini1_read_qos: INI1_READ_QOS,
    #[doc = "0x42104 - AXI interconnect - INI x write QoS register"]
    pub ini1_write_qos: INI1_WRITE_QOS,
    #[doc = "0x42108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini1_fn_mod: INI1_FN_MOD,
    _reserved29: [u8; 4084usize],
    #[doc = "0x43100 - AXI interconnect - INI x read QoS register"]
    pub ini2_read_qos: INI2_READ_QOS,
    #[doc = "0x43104 - AXI interconnect - INI x write QoS register"]
    pub ini2_write_qos: INI2_WRITE_QOS,
    #[doc = "0x43108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini2_fn_mod: INI2_FN_MOD,
    _reserved32: [u8; 3864usize],
    #[doc = "0x44024 - AXI interconnect - INI x functionality modification 2 register"]
    pub ini3_fn_mod2: INI3_FN_MOD2,
    #[doc = "0x44028 - AXI interconnect - INI x AHB functionality modification register"]
    pub ini3_fn_mod_ahb: INI3_FN_MOD_AHB,
    _reserved34: [u8; 212usize],
    #[doc = "0x44100 - AXI interconnect - INI x read QoS register"]
    pub ini3_read_qos: INI3_READ_QOS,
    #[doc = "0x44104 - AXI interconnect - INI x write QoS register"]
    pub ini3_write_qos: INI3_WRITE_QOS,
    #[doc = "0x44108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini3_fn_mod: INI3_FN_MOD,
    _reserved37: [u8; 4084usize],
    #[doc = "0x45100 - AXI interconnect - INI x read QoS register"]
    pub ini4_read_qos: INI4_READ_QOS,
    #[doc = "0x45104 - AXI interconnect - INI x write QoS register"]
    pub ini4_write_qos: INI4_WRITE_QOS,
    #[doc = "0x45108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini4_fn_mod: INI4_FN_MOD,
    _reserved40: [u8; 4084usize],
    #[doc = "0x46100 - AXI interconnect - INI x read QoS register"]
    pub ini5_read_qos: INI5_READ_QOS,
    #[doc = "0x46104 - AXI interconnect - INI x write QoS register"]
    pub ini5_write_qos: INI5_WRITE_QOS,
    #[doc = "0x46108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini5_fn_mod: INI5_FN_MOD,
    _reserved43: [u8; 4084usize],
    #[doc = "0x47100 - AXI interconnect - INI x read QoS register"]
    pub ini6_read_qos: INI6_READ_QOS,
    #[doc = "0x47104 - AXI interconnect - INI x write QoS register"]
    pub ini6_write_qos: INI6_WRITE_QOS,
    #[doc = "0x47108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini6_fn_mod: INI6_FN_MOD,
}
#[doc = "AXI interconnect - peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_4](periph_id_4) module"]
pub type PERIPH_ID_4 = crate::Reg<u32, _PERIPH_ID_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_4;
#[doc = "`read()` method returns [periph_id_4::R](periph_id_4::R) reader structure"]
impl crate::Readable for PERIPH_ID_4 {}
#[doc = "AXI interconnect - peripheral ID4 register"]
pub mod periph_id_4;
#[doc = "AXI interconnect - peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_0](periph_id_0) module"]
pub type PERIPH_ID_0 = crate::Reg<u32, _PERIPH_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_0;
#[doc = "`read()` method returns [periph_id_0::R](periph_id_0::R) reader structure"]
impl crate::Readable for PERIPH_ID_0 {}
#[doc = "AXI interconnect - peripheral ID0 register"]
pub mod periph_id_0;
#[doc = "AXI interconnect - peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_1](periph_id_1) module"]
pub type PERIPH_ID_1 = crate::Reg<u32, _PERIPH_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_1;
#[doc = "`read()` method returns [periph_id_1::R](periph_id_1::R) reader structure"]
impl crate::Readable for PERIPH_ID_1 {}
#[doc = "AXI interconnect - peripheral ID1 register"]
pub mod periph_id_1;
#[doc = "AXI interconnect - peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_2](periph_id_2) module"]
pub type PERIPH_ID_2 = crate::Reg<u32, _PERIPH_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_2;
#[doc = "`read()` method returns [periph_id_2::R](periph_id_2::R) reader structure"]
impl crate::Readable for PERIPH_ID_2 {}
#[doc = "AXI interconnect - peripheral ID2 register"]
pub mod periph_id_2;
#[doc = "AXI interconnect - peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_3](periph_id_3) module"]
pub type PERIPH_ID_3 = crate::Reg<u32, _PERIPH_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_3;
#[doc = "`read()` method returns [periph_id_3::R](periph_id_3::R) reader structure"]
impl crate::Readable for PERIPH_ID_3 {}
#[doc = "AXI interconnect - peripheral ID3 register"]
pub mod periph_id_3;
#[doc = "AXI interconnect - component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_0](comp_id_0) module"]
pub type COMP_ID_0 = crate::Reg<u32, _COMP_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_0;
#[doc = "`read()` method returns [comp_id_0::R](comp_id_0::R) reader structure"]
impl crate::Readable for COMP_ID_0 {}
#[doc = "AXI interconnect - component ID0 register"]
pub mod comp_id_0;
#[doc = "AXI interconnect - component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_1](comp_id_1) module"]
pub type COMP_ID_1 = crate::Reg<u32, _COMP_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_1;
#[doc = "`read()` method returns [comp_id_1::R](comp_id_1::R) reader structure"]
impl crate::Readable for COMP_ID_1 {}
#[doc = "AXI interconnect - component ID1 register"]
pub mod comp_id_1;
#[doc = "AXI interconnect - component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_2](comp_id_2) module"]
pub type COMP_ID_2 = crate::Reg<u32, _COMP_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_2;
#[doc = "`read()` method returns [comp_id_2::R](comp_id_2::R) reader structure"]
impl crate::Readable for COMP_ID_2 {}
#[doc = "AXI interconnect - component ID2 register"]
pub mod comp_id_2;
#[doc = "AXI interconnect - component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_3](comp_id_3) module"]
pub type COMP_ID_3 = crate::Reg<u32, _COMP_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_3;
#[doc = "`read()` method returns [comp_id_3::R](comp_id_3::R) reader structure"]
impl crate::Readable for COMP_ID_3 {}
#[doc = "AXI interconnect - component ID3 register"]
pub mod comp_id_3;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ1_fn_mod_iss_bm](targ1_fn_mod_iss_bm) module"]
pub type TARG1_FN_MOD_ISS_BM = crate::Reg<u32, _TARG1_FN_MOD_ISS_BM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG1_FN_MOD_ISS_BM;
#[doc = "`read()` method returns [targ1_fn_mod_iss_bm::R](targ1_fn_mod_iss_bm::R) reader structure"]
impl crate::Readable for TARG1_FN_MOD_ISS_BM {}
#[doc = "`write(|w| ..)` method takes [targ1_fn_mod_iss_bm::W](targ1_fn_mod_iss_bm::W) writer structure"]
impl crate::Writable for TARG1_FN_MOD_ISS_BM {}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ1_fn_mod_iss_bm;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ2_fn_mod_iss_bm](targ2_fn_mod_iss_bm) module"]
pub type TARG2_FN_MOD_ISS_BM = crate::Reg<u32, _TARG2_FN_MOD_ISS_BM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG2_FN_MOD_ISS_BM;
#[doc = "`read()` method returns [targ2_fn_mod_iss_bm::R](targ2_fn_mod_iss_bm::R) reader structure"]
impl crate::Readable for TARG2_FN_MOD_ISS_BM {}
#[doc = "`write(|w| ..)` method takes [targ2_fn_mod_iss_bm::W](targ2_fn_mod_iss_bm::W) writer structure"]
impl crate::Writable for TARG2_FN_MOD_ISS_BM {}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ2_fn_mod_iss_bm;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ3_fn_mod_iss_bm](targ3_fn_mod_iss_bm) module"]
pub type TARG3_FN_MOD_ISS_BM = crate::Reg<u32, _TARG3_FN_MOD_ISS_BM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG3_FN_MOD_ISS_BM;
#[doc = "`read()` method returns [targ3_fn_mod_iss_bm::R](targ3_fn_mod_iss_bm::R) reader structure"]
impl crate::Readable for TARG3_FN_MOD_ISS_BM {}
#[doc = "`write(|w| ..)` method takes [targ3_fn_mod_iss_bm::W](targ3_fn_mod_iss_bm::W) writer structure"]
impl crate::Writable for TARG3_FN_MOD_ISS_BM {}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ3_fn_mod_iss_bm;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ4_fn_mod_iss_bm](targ4_fn_mod_iss_bm) module"]
pub type TARG4_FN_MOD_ISS_BM = crate::Reg<u32, _TARG4_FN_MOD_ISS_BM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG4_FN_MOD_ISS_BM;
#[doc = "`read()` method returns [targ4_fn_mod_iss_bm::R](targ4_fn_mod_iss_bm::R) reader structure"]
impl crate::Readable for TARG4_FN_MOD_ISS_BM {}
#[doc = "`write(|w| ..)` method takes [targ4_fn_mod_iss_bm::W](targ4_fn_mod_iss_bm::W) writer structure"]
impl crate::Writable for TARG4_FN_MOD_ISS_BM {}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ4_fn_mod_iss_bm;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ5_fn_mod_iss_bm](targ5_fn_mod_iss_bm) module"]
pub type TARG5_FN_MOD_ISS_BM = crate::Reg<u32, _TARG5_FN_MOD_ISS_BM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG5_FN_MOD_ISS_BM;
#[doc = "`read()` method returns [targ5_fn_mod_iss_bm::R](targ5_fn_mod_iss_bm::R) reader structure"]
impl crate::Readable for TARG5_FN_MOD_ISS_BM {}
#[doc = "`write(|w| ..)` method takes [targ5_fn_mod_iss_bm::W](targ5_fn_mod_iss_bm::W) writer structure"]
impl crate::Writable for TARG5_FN_MOD_ISS_BM {}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ5_fn_mod_iss_bm;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ6_fn_mod_iss_bm](targ6_fn_mod_iss_bm) module"]
pub type TARG6_FN_MOD_ISS_BM = crate::Reg<u32, _TARG6_FN_MOD_ISS_BM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG6_FN_MOD_ISS_BM;
#[doc = "`read()` method returns [targ6_fn_mod_iss_bm::R](targ6_fn_mod_iss_bm::R) reader structure"]
impl crate::Readable for TARG6_FN_MOD_ISS_BM {}
#[doc = "`write(|w| ..)` method takes [targ6_fn_mod_iss_bm::W](targ6_fn_mod_iss_bm::W) writer structure"]
impl crate::Writable for TARG6_FN_MOD_ISS_BM {}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ6_fn_mod_iss_bm;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ7_fn_mod_iss_bm](targ7_fn_mod_iss_bm) module"]
pub type TARG7_FN_MOD_ISS_BM = crate::Reg<u32, _TARG7_FN_MOD_ISS_BM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG7_FN_MOD_ISS_BM;
#[doc = "`read()` method returns [targ7_fn_mod_iss_bm::R](targ7_fn_mod_iss_bm::R) reader structure"]
impl crate::Readable for TARG7_FN_MOD_ISS_BM {}
#[doc = "`write(|w| ..)` method takes [targ7_fn_mod_iss_bm::W](targ7_fn_mod_iss_bm::W) writer structure"]
impl crate::Writable for TARG7_FN_MOD_ISS_BM {}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ7_fn_mod_iss_bm;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ1_fn_mod2](targ1_fn_mod2) module"]
pub type TARG1_FN_MOD2 = crate::Reg<u32, _TARG1_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG1_FN_MOD2;
#[doc = "`read()` method returns [targ1_fn_mod2::R](targ1_fn_mod2::R) reader structure"]
impl crate::Readable for TARG1_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [targ1_fn_mod2::W](targ1_fn_mod2::W) writer structure"]
impl crate::Writable for TARG1_FN_MOD2 {}
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ1_fn_mod2;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ2_fn_mod2](targ2_fn_mod2) module"]
pub type TARG2_FN_MOD2 = crate::Reg<u32, _TARG2_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG2_FN_MOD2;
#[doc = "`read()` method returns [targ2_fn_mod2::R](targ2_fn_mod2::R) reader structure"]
impl crate::Readable for TARG2_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [targ2_fn_mod2::W](targ2_fn_mod2::W) writer structure"]
impl crate::Writable for TARG2_FN_MOD2 {}
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ2_fn_mod2;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ7_fn_mod2](targ7_fn_mod2) module"]
pub type TARG7_FN_MOD2 = crate::Reg<u32, _TARG7_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG7_FN_MOD2;
#[doc = "`read()` method returns [targ7_fn_mod2::R](targ7_fn_mod2::R) reader structure"]
impl crate::Readable for TARG7_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [targ7_fn_mod2::W](targ7_fn_mod2::W) writer structure"]
impl crate::Writable for TARG7_FN_MOD2 {}
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ7_fn_mod2;
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ1_fn_mod_lb](targ1_fn_mod_lb) module"]
pub type TARG1_FN_MOD_LB = crate::Reg<u32, _TARG1_FN_MOD_LB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG1_FN_MOD_LB;
#[doc = "`read()` method returns [targ1_fn_mod_lb::R](targ1_fn_mod_lb::R) reader structure"]
impl crate::Readable for TARG1_FN_MOD_LB {}
#[doc = "`write(|w| ..)` method takes [targ1_fn_mod_lb::W](targ1_fn_mod_lb::W) writer structure"]
impl crate::Writable for TARG1_FN_MOD_LB {}
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ1_fn_mod_lb;
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ2_fn_mod_lb](targ2_fn_mod_lb) module"]
pub type TARG2_FN_MOD_LB = crate::Reg<u32, _TARG2_FN_MOD_LB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG2_FN_MOD_LB;
#[doc = "`read()` method returns [targ2_fn_mod_lb::R](targ2_fn_mod_lb::R) reader structure"]
impl crate::Readable for TARG2_FN_MOD_LB {}
#[doc = "`write(|w| ..)` method takes [targ2_fn_mod_lb::W](targ2_fn_mod_lb::W) writer structure"]
impl crate::Writable for TARG2_FN_MOD_LB {}
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ2_fn_mod_lb;
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ1_fn_mod](targ1_fn_mod) module"]
pub type TARG1_FN_MOD = crate::Reg<u32, _TARG1_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG1_FN_MOD;
#[doc = "`read()` method returns [targ1_fn_mod::R](targ1_fn_mod::R) reader structure"]
impl crate::Readable for TARG1_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [targ1_fn_mod::W](targ1_fn_mod::W) writer structure"]
impl crate::Writable for TARG1_FN_MOD {}
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ1_fn_mod;
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ2_fn_mod](targ2_fn_mod) module"]
pub type TARG2_FN_MOD = crate::Reg<u32, _TARG2_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG2_FN_MOD;
#[doc = "`read()` method returns [targ2_fn_mod::R](targ2_fn_mod::R) reader structure"]
impl crate::Readable for TARG2_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [targ2_fn_mod::W](targ2_fn_mod::W) writer structure"]
impl crate::Writable for TARG2_FN_MOD {}
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ2_fn_mod;
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ7_fn_mod](targ7_fn_mod) module"]
pub type TARG7_FN_MOD = crate::Reg<u32, _TARG7_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARG7_FN_MOD;
#[doc = "`read()` method returns [targ7_fn_mod::R](targ7_fn_mod::R) reader structure"]
impl crate::Readable for TARG7_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [targ7_fn_mod::W](targ7_fn_mod::W) writer structure"]
impl crate::Writable for TARG7_FN_MOD {}
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ7_fn_mod;
#[doc = "AXI interconnect - INI x functionality modification 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini1_fn_mod2](ini1_fn_mod2) module"]
pub type INI1_FN_MOD2 = crate::Reg<u32, _INI1_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI1_FN_MOD2;
#[doc = "`read()` method returns [ini1_fn_mod2::R](ini1_fn_mod2::R) reader structure"]
impl crate::Readable for INI1_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [ini1_fn_mod2::W](ini1_fn_mod2::W) writer structure"]
impl crate::Writable for INI1_FN_MOD2 {}
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod ini1_fn_mod2;
#[doc = "AXI interconnect - INI x functionality modification 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini3_fn_mod2](ini3_fn_mod2) module"]
pub type INI3_FN_MOD2 = crate::Reg<u32, _INI3_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI3_FN_MOD2;
#[doc = "`read()` method returns [ini3_fn_mod2::R](ini3_fn_mod2::R) reader structure"]
impl crate::Readable for INI3_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [ini3_fn_mod2::W](ini3_fn_mod2::W) writer structure"]
impl crate::Writable for INI3_FN_MOD2 {}
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod ini3_fn_mod2;
#[doc = "AXI interconnect - INI x AHB functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini1_fn_mod_ahb](ini1_fn_mod_ahb) module"]
pub type INI1_FN_MOD_AHB = crate::Reg<u32, _INI1_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI1_FN_MOD_AHB;
#[doc = "`read()` method returns [ini1_fn_mod_ahb::R](ini1_fn_mod_ahb::R) reader structure"]
impl crate::Readable for INI1_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [ini1_fn_mod_ahb::W](ini1_fn_mod_ahb::W) writer structure"]
impl crate::Writable for INI1_FN_MOD_AHB {}
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod ini1_fn_mod_ahb;
#[doc = "AXI interconnect - INI x AHB functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini3_fn_mod_ahb](ini3_fn_mod_ahb) module"]
pub type INI3_FN_MOD_AHB = crate::Reg<u32, _INI3_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI3_FN_MOD_AHB;
#[doc = "`read()` method returns [ini3_fn_mod_ahb::R](ini3_fn_mod_ahb::R) reader structure"]
impl crate::Readable for INI3_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [ini3_fn_mod_ahb::W](ini3_fn_mod_ahb::W) writer structure"]
impl crate::Writable for INI3_FN_MOD_AHB {}
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod ini3_fn_mod_ahb;
#[doc = "AXI interconnect - INI x read QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini1_read_qos](ini1_read_qos) module"]
pub type INI1_READ_QOS = crate::Reg<u32, _INI1_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI1_READ_QOS;
#[doc = "`read()` method returns [ini1_read_qos::R](ini1_read_qos::R) reader structure"]
impl crate::Readable for INI1_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [ini1_read_qos::W](ini1_read_qos::W) writer structure"]
impl crate::Writable for INI1_READ_QOS {}
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini1_read_qos;
#[doc = "AXI interconnect - INI x read QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini2_read_qos](ini2_read_qos) module"]
pub type INI2_READ_QOS = crate::Reg<u32, _INI2_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI2_READ_QOS;
#[doc = "`read()` method returns [ini2_read_qos::R](ini2_read_qos::R) reader structure"]
impl crate::Readable for INI2_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [ini2_read_qos::W](ini2_read_qos::W) writer structure"]
impl crate::Writable for INI2_READ_QOS {}
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini2_read_qos;
#[doc = "AXI interconnect - INI x read QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini3_read_qos](ini3_read_qos) module"]
pub type INI3_READ_QOS = crate::Reg<u32, _INI3_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI3_READ_QOS;
#[doc = "`read()` method returns [ini3_read_qos::R](ini3_read_qos::R) reader structure"]
impl crate::Readable for INI3_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [ini3_read_qos::W](ini3_read_qos::W) writer structure"]
impl crate::Writable for INI3_READ_QOS {}
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini3_read_qos;
#[doc = "AXI interconnect - INI x read QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini4_read_qos](ini4_read_qos) module"]
pub type INI4_READ_QOS = crate::Reg<u32, _INI4_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI4_READ_QOS;
#[doc = "`read()` method returns [ini4_read_qos::R](ini4_read_qos::R) reader structure"]
impl crate::Readable for INI4_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [ini4_read_qos::W](ini4_read_qos::W) writer structure"]
impl crate::Writable for INI4_READ_QOS {}
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini4_read_qos;
#[doc = "AXI interconnect - INI x read QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini5_read_qos](ini5_read_qos) module"]
pub type INI5_READ_QOS = crate::Reg<u32, _INI5_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI5_READ_QOS;
#[doc = "`read()` method returns [ini5_read_qos::R](ini5_read_qos::R) reader structure"]
impl crate::Readable for INI5_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [ini5_read_qos::W](ini5_read_qos::W) writer structure"]
impl crate::Writable for INI5_READ_QOS {}
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini5_read_qos;
#[doc = "AXI interconnect - INI x read QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini6_read_qos](ini6_read_qos) module"]
pub type INI6_READ_QOS = crate::Reg<u32, _INI6_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI6_READ_QOS;
#[doc = "`read()` method returns [ini6_read_qos::R](ini6_read_qos::R) reader structure"]
impl crate::Readable for INI6_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [ini6_read_qos::W](ini6_read_qos::W) writer structure"]
impl crate::Writable for INI6_READ_QOS {}
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini6_read_qos;
#[doc = "AXI interconnect - INI x write QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini1_write_qos](ini1_write_qos) module"]
pub type INI1_WRITE_QOS = crate::Reg<u32, _INI1_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI1_WRITE_QOS;
#[doc = "`read()` method returns [ini1_write_qos::R](ini1_write_qos::R) reader structure"]
impl crate::Readable for INI1_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [ini1_write_qos::W](ini1_write_qos::W) writer structure"]
impl crate::Writable for INI1_WRITE_QOS {}
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini1_write_qos;
#[doc = "AXI interconnect - INI x write QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini2_write_qos](ini2_write_qos) module"]
pub type INI2_WRITE_QOS = crate::Reg<u32, _INI2_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI2_WRITE_QOS;
#[doc = "`read()` method returns [ini2_write_qos::R](ini2_write_qos::R) reader structure"]
impl crate::Readable for INI2_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [ini2_write_qos::W](ini2_write_qos::W) writer structure"]
impl crate::Writable for INI2_WRITE_QOS {}
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini2_write_qos;
#[doc = "AXI interconnect - INI x write QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini3_write_qos](ini3_write_qos) module"]
pub type INI3_WRITE_QOS = crate::Reg<u32, _INI3_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI3_WRITE_QOS;
#[doc = "`read()` method returns [ini3_write_qos::R](ini3_write_qos::R) reader structure"]
impl crate::Readable for INI3_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [ini3_write_qos::W](ini3_write_qos::W) writer structure"]
impl crate::Writable for INI3_WRITE_QOS {}
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini3_write_qos;
#[doc = "AXI interconnect - INI x write QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini4_write_qos](ini4_write_qos) module"]
pub type INI4_WRITE_QOS = crate::Reg<u32, _INI4_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI4_WRITE_QOS;
#[doc = "`read()` method returns [ini4_write_qos::R](ini4_write_qos::R) reader structure"]
impl crate::Readable for INI4_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [ini4_write_qos::W](ini4_write_qos::W) writer structure"]
impl crate::Writable for INI4_WRITE_QOS {}
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini4_write_qos;
#[doc = "AXI interconnect - INI x write QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini5_write_qos](ini5_write_qos) module"]
pub type INI5_WRITE_QOS = crate::Reg<u32, _INI5_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI5_WRITE_QOS;
#[doc = "`read()` method returns [ini5_write_qos::R](ini5_write_qos::R) reader structure"]
impl crate::Readable for INI5_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [ini5_write_qos::W](ini5_write_qos::W) writer structure"]
impl crate::Writable for INI5_WRITE_QOS {}
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini5_write_qos;
#[doc = "AXI interconnect - INI x write QoS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini6_write_qos](ini6_write_qos) module"]
pub type INI6_WRITE_QOS = crate::Reg<u32, _INI6_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI6_WRITE_QOS;
#[doc = "`read()` method returns [ini6_write_qos::R](ini6_write_qos::R) reader structure"]
impl crate::Readable for INI6_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [ini6_write_qos::W](ini6_write_qos::W) writer structure"]
impl crate::Writable for INI6_WRITE_QOS {}
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini6_write_qos;
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini1_fn_mod](ini1_fn_mod) module"]
pub type INI1_FN_MOD = crate::Reg<u32, _INI1_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI1_FN_MOD;
#[doc = "`read()` method returns [ini1_fn_mod::R](ini1_fn_mod::R) reader structure"]
impl crate::Readable for INI1_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [ini1_fn_mod::W](ini1_fn_mod::W) writer structure"]
impl crate::Writable for INI1_FN_MOD {}
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini1_fn_mod;
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini2_fn_mod](ini2_fn_mod) module"]
pub type INI2_FN_MOD = crate::Reg<u32, _INI2_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI2_FN_MOD;
#[doc = "`read()` method returns [ini2_fn_mod::R](ini2_fn_mod::R) reader structure"]
impl crate::Readable for INI2_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [ini2_fn_mod::W](ini2_fn_mod::W) writer structure"]
impl crate::Writable for INI2_FN_MOD {}
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini2_fn_mod;
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini3_fn_mod](ini3_fn_mod) module"]
pub type INI3_FN_MOD = crate::Reg<u32, _INI3_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI3_FN_MOD;
#[doc = "`read()` method returns [ini3_fn_mod::R](ini3_fn_mod::R) reader structure"]
impl crate::Readable for INI3_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [ini3_fn_mod::W](ini3_fn_mod::W) writer structure"]
impl crate::Writable for INI3_FN_MOD {}
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini3_fn_mod;
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini4_fn_mod](ini4_fn_mod) module"]
pub type INI4_FN_MOD = crate::Reg<u32, _INI4_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI4_FN_MOD;
#[doc = "`read()` method returns [ini4_fn_mod::R](ini4_fn_mod::R) reader structure"]
impl crate::Readable for INI4_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [ini4_fn_mod::W](ini4_fn_mod::W) writer structure"]
impl crate::Writable for INI4_FN_MOD {}
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini4_fn_mod;
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini5_fn_mod](ini5_fn_mod) module"]
pub type INI5_FN_MOD = crate::Reg<u32, _INI5_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI5_FN_MOD;
#[doc = "`read()` method returns [ini5_fn_mod::R](ini5_fn_mod::R) reader structure"]
impl crate::Readable for INI5_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [ini5_fn_mod::W](ini5_fn_mod::W) writer structure"]
impl crate::Writable for INI5_FN_MOD {}
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini5_fn_mod;
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini6_fn_mod](ini6_fn_mod) module"]
pub type INI6_FN_MOD = crate::Reg<u32, _INI6_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INI6_FN_MOD;
#[doc = "`read()` method returns [ini6_fn_mod::R](ini6_fn_mod::R) reader structure"]
impl crate::Readable for INI6_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [ini6_fn_mod::W](ini6_fn_mod::W) writer structure"]
impl crate::Writable for INI6_FN_MOD {}
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini6_fn_mod;
