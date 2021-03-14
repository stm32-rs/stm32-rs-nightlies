#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    pub stgenr_cntcvl: STGENR_CNTCVL,
    #[doc = "0x04 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    pub stgenr_cntcvu: STGENR_CNTCVU,
    _reserved2: [u8; 4040usize],
    #[doc = "0xfd0 - STGENR peripheral ID4 register"]
    pub stgenr_pidr4: STGENR_PIDR4,
    #[doc = "0xfd4 - STGENR peripheral ID5 register"]
    pub stgenr_pidr5: STGENR_PIDR5,
    #[doc = "0xfd8 - STGENR peripheral ID6 register"]
    pub stgenr_pidr6: STGENR_PIDR6,
    #[doc = "0xfdc - STGENR peripheral ID7 register"]
    pub stgenr_pidr7: STGENR_PIDR7,
    #[doc = "0xfe0 - STGENR peripheral ID0 register"]
    pub stgenr_pidr0: STGENR_PIDR0,
    #[doc = "0xfe4 - STGENR peripheral ID1 register"]
    pub stgenr_pidr1: STGENR_PIDR1,
    #[doc = "0xfe8 - STGENR peripheral ID2 register"]
    pub stgenr_pidr2: STGENR_PIDR2,
    #[doc = "0xfec - STGENR peripheral ID3 register"]
    pub stgenr_pidr3: STGENR_PIDR3,
    #[doc = "0xff0 - STGENR component ID0 register"]
    pub stgenr_cidr0: STGENR_CIDR0,
    #[doc = "0xff4 - STGENR component ID1 register"]
    pub stgenr_cidr1: STGENR_CIDR1,
    #[doc = "0xff8 - STGENR component ID2 register"]
    pub stgenr_cidr2: STGENR_CIDR2,
    #[doc = "0xffc - STGENR component ID3 register"]
    pub stgenr_cidr3: STGENR_CIDR3,
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cntcvl](stgenr_cntcvl) module"]
pub type STGENR_CNTCVL = crate::Reg<u32, _STGENR_CNTCVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_CNTCVL;
#[doc = "`read()` method returns [stgenr_cntcvl::R](stgenr_cntcvl::R) reader structure"]
impl crate::Readable for STGENR_CNTCVL {}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenr_cntcvl;
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cntcvu](stgenr_cntcvu) module"]
pub type STGENR_CNTCVU = crate::Reg<u32, _STGENR_CNTCVU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_CNTCVU;
#[doc = "`read()` method returns [stgenr_cntcvu::R](stgenr_cntcvu::R) reader structure"]
impl crate::Readable for STGENR_CNTCVU {}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenr_cntcvu;
#[doc = "STGENR peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr4](stgenr_pidr4) module"]
pub type STGENR_PIDR4 = crate::Reg<u32, _STGENR_PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR4;
#[doc = "`read()` method returns [stgenr_pidr4::R](stgenr_pidr4::R) reader structure"]
impl crate::Readable for STGENR_PIDR4 {}
#[doc = "STGENR peripheral ID4 register"]
pub mod stgenr_pidr4;
#[doc = "STGENR peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr5](stgenr_pidr5) module"]
pub type STGENR_PIDR5 = crate::Reg<u32, _STGENR_PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR5;
#[doc = "`read()` method returns [stgenr_pidr5::R](stgenr_pidr5::R) reader structure"]
impl crate::Readable for STGENR_PIDR5 {}
#[doc = "STGENR peripheral ID5 register"]
pub mod stgenr_pidr5;
#[doc = "STGENR peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr6](stgenr_pidr6) module"]
pub type STGENR_PIDR6 = crate::Reg<u32, _STGENR_PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR6;
#[doc = "`read()` method returns [stgenr_pidr6::R](stgenr_pidr6::R) reader structure"]
impl crate::Readable for STGENR_PIDR6 {}
#[doc = "STGENR peripheral ID6 register"]
pub mod stgenr_pidr6;
#[doc = "STGENR peripheral ID7 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr7](stgenr_pidr7) module"]
pub type STGENR_PIDR7 = crate::Reg<u32, _STGENR_PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR7;
#[doc = "`read()` method returns [stgenr_pidr7::R](stgenr_pidr7::R) reader structure"]
impl crate::Readable for STGENR_PIDR7 {}
#[doc = "STGENR peripheral ID7 register"]
pub mod stgenr_pidr7;
#[doc = "STGENR peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr0](stgenr_pidr0) module"]
pub type STGENR_PIDR0 = crate::Reg<u32, _STGENR_PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR0;
#[doc = "`read()` method returns [stgenr_pidr0::R](stgenr_pidr0::R) reader structure"]
impl crate::Readable for STGENR_PIDR0 {}
#[doc = "STGENR peripheral ID0 register"]
pub mod stgenr_pidr0;
#[doc = "STGENR peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr1](stgenr_pidr1) module"]
pub type STGENR_PIDR1 = crate::Reg<u32, _STGENR_PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR1;
#[doc = "`read()` method returns [stgenr_pidr1::R](stgenr_pidr1::R) reader structure"]
impl crate::Readable for STGENR_PIDR1 {}
#[doc = "STGENR peripheral ID1 register"]
pub mod stgenr_pidr1;
#[doc = "STGENR peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr2](stgenr_pidr2) module"]
pub type STGENR_PIDR2 = crate::Reg<u32, _STGENR_PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR2;
#[doc = "`read()` method returns [stgenr_pidr2::R](stgenr_pidr2::R) reader structure"]
impl crate::Readable for STGENR_PIDR2 {}
#[doc = "STGENR peripheral ID2 register"]
pub mod stgenr_pidr2;
#[doc = "STGENR peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr3](stgenr_pidr3) module"]
pub type STGENR_PIDR3 = crate::Reg<u32, _STGENR_PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_PIDR3;
#[doc = "`read()` method returns [stgenr_pidr3::R](stgenr_pidr3::R) reader structure"]
impl crate::Readable for STGENR_PIDR3 {}
#[doc = "STGENR peripheral ID3 register"]
pub mod stgenr_pidr3;
#[doc = "STGENR component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cidr0](stgenr_cidr0) module"]
pub type STGENR_CIDR0 = crate::Reg<u32, _STGENR_CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_CIDR0;
#[doc = "`read()` method returns [stgenr_cidr0::R](stgenr_cidr0::R) reader structure"]
impl crate::Readable for STGENR_CIDR0 {}
#[doc = "STGENR component ID0 register"]
pub mod stgenr_cidr0;
#[doc = "STGENR component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cidr1](stgenr_cidr1) module"]
pub type STGENR_CIDR1 = crate::Reg<u32, _STGENR_CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_CIDR1;
#[doc = "`read()` method returns [stgenr_cidr1::R](stgenr_cidr1::R) reader structure"]
impl crate::Readable for STGENR_CIDR1 {}
#[doc = "STGENR component ID1 register"]
pub mod stgenr_cidr1;
#[doc = "STGENR component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cidr2](stgenr_cidr2) module"]
pub type STGENR_CIDR2 = crate::Reg<u32, _STGENR_CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_CIDR2;
#[doc = "`read()` method returns [stgenr_cidr2::R](stgenr_cidr2::R) reader structure"]
impl crate::Readable for STGENR_CIDR2 {}
#[doc = "STGENR component ID2 register"]
pub mod stgenr_cidr2;
#[doc = "STGENR component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cidr3](stgenr_cidr3) module"]
pub type STGENR_CIDR3 = crate::Reg<u32, _STGENR_CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STGENR_CIDR3;
#[doc = "`read()` method returns [stgenr_cidr3::R](stgenr_cidr3::R) reader structure"]
impl crate::Readable for STGENR_CIDR3 {}
#[doc = "STGENR component ID3 register"]
pub mod stgenr_cidr3;
