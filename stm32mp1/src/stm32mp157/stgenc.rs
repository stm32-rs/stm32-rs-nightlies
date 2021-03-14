#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - STGENC control register"]
    pub stgenc_cntcr: STGENC_CNTCR,
    #[doc = "0x04 - STGENC status register"]
    pub stgenc_cntsr: STGENC_CNTSR,
    #[doc = "0x08 - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
    pub stgenc_cntcvl: STGENC_CNTCVL,
    #[doc = "0x0c - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
    pub stgenc_cntcvu: STGENC_CNTCVU,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    pub stgenc_cntfid0: STGENC_CNTFID0,
    _reserved5: [u8; 4012usize],
    #[doc = "0xfd0 - STGENC peripheral ID4 register"]
    pub stgenc_pidr4: STGENC_PIDR4,
    #[doc = "0xfd4 - STGENC peripheral ID5 register"]
    pub stgenc_pidr5: STGENC_PIDR5,
    #[doc = "0xfd8 - STGENC peripheral ID6 register"]
    pub stgenc_pidr6: STGENC_PIDR6,
    #[doc = "0xfdc - STGENC peripheral ID7 register"]
    pub stgenc_pidr7: STGENC_PIDR7,
    #[doc = "0xfe0 - STGENC peripheral ID0 register"]
    pub stgenc_pidr0: STGENC_PIDR0,
    #[doc = "0xfe4 - STGENC peripheral ID1 register"]
    pub stgenc_pidr1: STGENC_PIDR1,
    #[doc = "0xfe8 - STGENC peripheral ID2 register"]
    pub stgenc_pidr2: STGENC_PIDR2,
    #[doc = "0xfec - STGENC peripheral ID3 register"]
    pub stgenc_pidr3: STGENC_PIDR3,
    #[doc = "0xff0 - STGENC component ID0 register"]
    pub stgenc_cidr0: STGENC_CIDR0,
    #[doc = "0xff4 - STGENC component ID1 register"]
    pub stgenc_cidr1: STGENC_CIDR1,
    #[doc = "0xff8 - STGENC component ID2 register"]
    pub stgenc_cidr2: STGENC_CIDR2,
    #[doc = "0xffc - STGENC component ID3 register"]
    pub stgenc_cidr3: STGENC_CIDR3,
}
#[doc = "STGENC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcr](stgenc_cntcr) module"]
pub type STGENC_CNTCR = crate::Reg<u32, _STGENC_CNTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CNTCR;
#[doc = "`read()` method returns [stgenc_cntcr::R](stgenc_cntcr::R) reader structure"]
impl crate::Readable for STGENC_CNTCR {}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcr::W](stgenc_cntcr::W) writer structure"]
impl crate::Writable for STGENC_CNTCR {}
#[doc = "STGENC control register"]
pub mod stgenc_cntcr;
#[doc = "STGENC status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntsr](stgenc_cntsr) module"]
pub type STGENC_CNTSR = crate::Reg<u32, _STGENC_CNTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CNTSR;
#[doc = "`read()` method returns [stgenc_cntsr::R](stgenc_cntsr::R) reader structure"]
impl crate::Readable for STGENC_CNTSR {}
#[doc = "STGENC status register"]
pub mod stgenc_cntsr;
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcvl](stgenc_cntcvl) module"]
pub type STGENC_CNTCVL = crate::Reg<u32, _STGENC_CNTCVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CNTCVL;
#[doc = "`read()` method returns [stgenc_cntcvl::R](stgenc_cntcvl::R) reader structure"]
impl crate::Readable for STGENC_CNTCVL {}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcvl::W](stgenc_cntcvl::W) writer structure"]
impl crate::Writable for STGENC_CNTCVL {}
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
pub mod stgenc_cntcvl;
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcvu](stgenc_cntcvu) module"]
pub type STGENC_CNTCVU = crate::Reg<u32, _STGENC_CNTCVU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CNTCVU;
#[doc = "`read()` method returns [stgenc_cntcvu::R](stgenc_cntcvu::R) reader structure"]
impl crate::Readable for STGENC_CNTCVU {}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcvu::W](stgenc_cntcvu::W) writer structure"]
impl crate::Writable for STGENC_CNTCVU {}
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register."]
pub mod stgenc_cntcvu;
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntfid0](stgenc_cntfid0) module"]
pub type STGENC_CNTFID0 = crate::Reg<u32, _STGENC_CNTFID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CNTFID0;
#[doc = "`read()` method returns [stgenc_cntfid0::R](stgenc_cntfid0::R) reader structure"]
impl crate::Readable for STGENC_CNTFID0 {}
#[doc = "`write(|w| ..)` method takes [stgenc_cntfid0::W](stgenc_cntfid0::W) writer structure"]
impl crate::Writable for STGENC_CNTFID0 {}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenc_cntfid0;
#[doc = "STGENC peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr4](stgenc_pidr4) module"]
pub type STGENC_PIDR4 = crate::Reg<u32, _STGENC_PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR4;
#[doc = "`read()` method returns [stgenc_pidr4::R](stgenc_pidr4::R) reader structure"]
impl crate::Readable for STGENC_PIDR4 {}
#[doc = "STGENC peripheral ID4 register"]
pub mod stgenc_pidr4;
#[doc = "STGENC peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr5](stgenc_pidr5) module"]
pub type STGENC_PIDR5 = crate::Reg<u32, _STGENC_PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR5;
#[doc = "`read()` method returns [stgenc_pidr5::R](stgenc_pidr5::R) reader structure"]
impl crate::Readable for STGENC_PIDR5 {}
#[doc = "STGENC peripheral ID5 register"]
pub mod stgenc_pidr5;
#[doc = "STGENC peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr6](stgenc_pidr6) module"]
pub type STGENC_PIDR6 = crate::Reg<u32, _STGENC_PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR6;
#[doc = "`read()` method returns [stgenc_pidr6::R](stgenc_pidr6::R) reader structure"]
impl crate::Readable for STGENC_PIDR6 {}
#[doc = "STGENC peripheral ID6 register"]
pub mod stgenc_pidr6;
#[doc = "STGENC peripheral ID7 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr7](stgenc_pidr7) module"]
pub type STGENC_PIDR7 = crate::Reg<u32, _STGENC_PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR7;
#[doc = "`read()` method returns [stgenc_pidr7::R](stgenc_pidr7::R) reader structure"]
impl crate::Readable for STGENC_PIDR7 {}
#[doc = "STGENC peripheral ID7 register"]
pub mod stgenc_pidr7;
#[doc = "STGENC peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr0](stgenc_pidr0) module"]
pub type STGENC_PIDR0 = crate::Reg<u32, _STGENC_PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR0;
#[doc = "`read()` method returns [stgenc_pidr0::R](stgenc_pidr0::R) reader structure"]
impl crate::Readable for STGENC_PIDR0 {}
#[doc = "STGENC peripheral ID0 register"]
pub mod stgenc_pidr0;
#[doc = "STGENC peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr1](stgenc_pidr1) module"]
pub type STGENC_PIDR1 = crate::Reg<u32, _STGENC_PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR1;
#[doc = "`read()` method returns [stgenc_pidr1::R](stgenc_pidr1::R) reader structure"]
impl crate::Readable for STGENC_PIDR1 {}
#[doc = "STGENC peripheral ID1 register"]
pub mod stgenc_pidr1;
#[doc = "STGENC peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr2](stgenc_pidr2) module"]
pub type STGENC_PIDR2 = crate::Reg<u32, _STGENC_PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR2;
#[doc = "`read()` method returns [stgenc_pidr2::R](stgenc_pidr2::R) reader structure"]
impl crate::Readable for STGENC_PIDR2 {}
#[doc = "STGENC peripheral ID2 register"]
pub mod stgenc_pidr2;
#[doc = "STGENC peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr3](stgenc_pidr3) module"]
pub type STGENC_PIDR3 = crate::Reg<u32, _STGENC_PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_PIDR3;
#[doc = "`read()` method returns [stgenc_pidr3::R](stgenc_pidr3::R) reader structure"]
impl crate::Readable for STGENC_PIDR3 {}
#[doc = "STGENC peripheral ID3 register"]
pub mod stgenc_pidr3;
#[doc = "STGENC component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cidr0](stgenc_cidr0) module"]
pub type STGENC_CIDR0 = crate::Reg<u32, _STGENC_CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CIDR0;
#[doc = "`read()` method returns [stgenc_cidr0::R](stgenc_cidr0::R) reader structure"]
impl crate::Readable for STGENC_CIDR0 {}
#[doc = "STGENC component ID0 register"]
pub mod stgenc_cidr0;
#[doc = "STGENC component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cidr1](stgenc_cidr1) module"]
pub type STGENC_CIDR1 = crate::Reg<u32, _STGENC_CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CIDR1;
#[doc = "`read()` method returns [stgenc_cidr1::R](stgenc_cidr1::R) reader structure"]
impl crate::Readable for STGENC_CIDR1 {}
#[doc = "STGENC component ID1 register"]
pub mod stgenc_cidr1;
#[doc = "STGENC component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cidr2](stgenc_cidr2) module"]
pub type STGENC_CIDR2 = crate::Reg<u32, _STGENC_CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CIDR2;
#[doc = "`read()` method returns [stgenc_cidr2::R](stgenc_cidr2::R) reader structure"]
impl crate::Readable for STGENC_CIDR2 {}
#[doc = "STGENC component ID2 register"]
pub mod stgenc_cidr2;
#[doc = "STGENC component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cidr3](stgenc_cidr3) module"]
pub type STGENC_CIDR3 = crate::Reg<u32, _STGENC_CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENC_CIDR3;
#[doc = "`read()` method returns [stgenc_cidr3::R](stgenc_cidr3::R) reader structure"]
impl crate::Readable for STGENC_CIDR3 {}
#[doc = "STGENC component ID3 register"]
pub mod stgenc_cidr3;
