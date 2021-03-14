#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write-only register. A read request returns all zeros."]
    pub ddrperfm_ctl: DDRPERFM_CTL,
    #[doc = "0x04 - DDRPERFM configurationl register"]
    pub ddrperfm_cfg: DDRPERFM_CFG,
    #[doc = "0x08 - DDRPERFM status register"]
    pub ddrperfm_status: DDRPERFM_STATUS,
    #[doc = "0x0c - Write-only register. A read request returns all zeros"]
    pub ddrperfm_ccr: DDRPERFM_CCR,
    #[doc = "0x10 - DDRPERFM interrupt enable register"]
    pub ddrperfm_ier: DDRPERFM_IER,
    #[doc = "0x14 - DDRPERFM interrupt status register"]
    pub ddrperfm_isr: DDRPERFM_ISR,
    #[doc = "0x18 - Write-only register. A read request returns all zeros"]
    pub ddrperfm_icr: DDRPERFM_ICR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - DDRPERFM time counter register"]
    pub ddrperfm_tcnt: DDRPERFM_TCNT,
    _reserved8: [u8; 60usize],
    #[doc = "0x60 - DDRPERFM event counter 0 register"]
    pub ddrperfm_cnt0: DDRPERFM_CNT0,
    _reserved9: [u8; 4usize],
    #[doc = "0x68 - DDRPERFM event counter 1 register"]
    pub ddrperfm_cnt1: DDRPERFM_CNT1,
    _reserved10: [u8; 4usize],
    #[doc = "0x70 - DDRPERFM event counter 2 register"]
    pub ddrperfm_cnt2: DDRPERFM_CNT2,
    _reserved11: [u8; 4usize],
    #[doc = "0x78 - DDRPERFM event counter 3 register"]
    pub ddrperfm_cnt3: DDRPERFM_CNT3,
    _reserved12: [u8; 884usize],
    #[doc = "0x3f0 - DDRPERFM hardware configuration register"]
    pub ddrperfm_hwcfg: DDRPERFM_HWCFG,
    #[doc = "0x3f4 - DDRPERFM version register"]
    pub ddrperfm_ver: DDRPERFM_VER,
    #[doc = "0x3f8 - DDRPERFM ID register"]
    pub ddrperfm_id: DDRPERFM_ID,
    #[doc = "0x3fc - DDRPERFM magic ID register"]
    pub ddrperfm_sid: DDRPERFM_SID,
}
#[doc = "Write-only register. A read request returns all zeros.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_ctl](ddrperfm_ctl) module"]
pub type DDRPERFM_CTL = crate::Reg<u32, _DDRPERFM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_CTL;
#[doc = "`write(|w| ..)` method takes [ddrperfm_ctl::W](ddrperfm_ctl::W) writer structure"]
impl crate::Writable for DDRPERFM_CTL {}
#[doc = "Write-only register. A read request returns all zeros."]
pub mod ddrperfm_ctl;
#[doc = "DDRPERFM configurationl register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cfg](ddrperfm_cfg) module"]
pub type DDRPERFM_CFG = crate::Reg<u32, _DDRPERFM_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_CFG;
#[doc = "`read()` method returns [ddrperfm_cfg::R](ddrperfm_cfg::R) reader structure"]
impl crate::Readable for DDRPERFM_CFG {}
#[doc = "`write(|w| ..)` method takes [ddrperfm_cfg::W](ddrperfm_cfg::W) writer structure"]
impl crate::Writable for DDRPERFM_CFG {}
#[doc = "DDRPERFM configurationl register"]
pub mod ddrperfm_cfg;
#[doc = "DDRPERFM status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_status](ddrperfm_status) module"]
pub type DDRPERFM_STATUS = crate::Reg<u32, _DDRPERFM_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_STATUS;
#[doc = "`read()` method returns [ddrperfm_status::R](ddrperfm_status::R) reader structure"]
impl crate::Readable for DDRPERFM_STATUS {}
#[doc = "DDRPERFM status register"]
pub mod ddrperfm_status;
#[doc = "Write-only register. A read request returns all zeros\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_ccr](ddrperfm_ccr) module"]
pub type DDRPERFM_CCR = crate::Reg<u32, _DDRPERFM_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_CCR;
#[doc = "`write(|w| ..)` method takes [ddrperfm_ccr::W](ddrperfm_ccr::W) writer structure"]
impl crate::Writable for DDRPERFM_CCR {}
#[doc = "Write-only register. A read request returns all zeros"]
pub mod ddrperfm_ccr;
#[doc = "DDRPERFM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_ier](ddrperfm_ier) module"]
pub type DDRPERFM_IER = crate::Reg<u32, _DDRPERFM_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_IER;
#[doc = "`read()` method returns [ddrperfm_ier::R](ddrperfm_ier::R) reader structure"]
impl crate::Readable for DDRPERFM_IER {}
#[doc = "`write(|w| ..)` method takes [ddrperfm_ier::W](ddrperfm_ier::W) writer structure"]
impl crate::Writable for DDRPERFM_IER {}
#[doc = "DDRPERFM interrupt enable register"]
pub mod ddrperfm_ier;
#[doc = "DDRPERFM interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_isr](ddrperfm_isr) module"]
pub type DDRPERFM_ISR = crate::Reg<u32, _DDRPERFM_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_ISR;
#[doc = "`read()` method returns [ddrperfm_isr::R](ddrperfm_isr::R) reader structure"]
impl crate::Readable for DDRPERFM_ISR {}
#[doc = "DDRPERFM interrupt status register"]
pub mod ddrperfm_isr;
#[doc = "Write-only register. A read request returns all zeros\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_icr](ddrperfm_icr) module"]
pub type DDRPERFM_ICR = crate::Reg<u32, _DDRPERFM_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_ICR;
#[doc = "`write(|w| ..)` method takes [ddrperfm_icr::W](ddrperfm_icr::W) writer structure"]
impl crate::Writable for DDRPERFM_ICR {}
#[doc = "Write-only register. A read request returns all zeros"]
pub mod ddrperfm_icr;
#[doc = "DDRPERFM time counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_tcnt](ddrperfm_tcnt) module"]
pub type DDRPERFM_TCNT = crate::Reg<u32, _DDRPERFM_TCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_TCNT;
#[doc = "`read()` method returns [ddrperfm_tcnt::R](ddrperfm_tcnt::R) reader structure"]
impl crate::Readable for DDRPERFM_TCNT {}
#[doc = "DDRPERFM time counter register"]
pub mod ddrperfm_tcnt;
#[doc = "DDRPERFM event counter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cnt0](ddrperfm_cnt0) module"]
pub type DDRPERFM_CNT0 = crate::Reg<u32, _DDRPERFM_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_CNT0;
#[doc = "`read()` method returns [ddrperfm_cnt0::R](ddrperfm_cnt0::R) reader structure"]
impl crate::Readable for DDRPERFM_CNT0 {}
#[doc = "DDRPERFM event counter 0 register"]
pub mod ddrperfm_cnt0;
#[doc = "DDRPERFM event counter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cnt1](ddrperfm_cnt1) module"]
pub type DDRPERFM_CNT1 = crate::Reg<u32, _DDRPERFM_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_CNT1;
#[doc = "`read()` method returns [ddrperfm_cnt1::R](ddrperfm_cnt1::R) reader structure"]
impl crate::Readable for DDRPERFM_CNT1 {}
#[doc = "DDRPERFM event counter 1 register"]
pub mod ddrperfm_cnt1;
#[doc = "DDRPERFM event counter 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cnt2](ddrperfm_cnt2) module"]
pub type DDRPERFM_CNT2 = crate::Reg<u32, _DDRPERFM_CNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_CNT2;
#[doc = "`read()` method returns [ddrperfm_cnt2::R](ddrperfm_cnt2::R) reader structure"]
impl crate::Readable for DDRPERFM_CNT2 {}
#[doc = "DDRPERFM event counter 2 register"]
pub mod ddrperfm_cnt2;
#[doc = "DDRPERFM event counter 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cnt3](ddrperfm_cnt3) module"]
pub type DDRPERFM_CNT3 = crate::Reg<u32, _DDRPERFM_CNT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_CNT3;
#[doc = "`read()` method returns [ddrperfm_cnt3::R](ddrperfm_cnt3::R) reader structure"]
impl crate::Readable for DDRPERFM_CNT3 {}
#[doc = "DDRPERFM event counter 3 register"]
pub mod ddrperfm_cnt3;
#[doc = "DDRPERFM hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_hwcfg](ddrperfm_hwcfg) module"]
pub type DDRPERFM_HWCFG = crate::Reg<u32, _DDRPERFM_HWCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_HWCFG;
#[doc = "`read()` method returns [ddrperfm_hwcfg::R](ddrperfm_hwcfg::R) reader structure"]
impl crate::Readable for DDRPERFM_HWCFG {}
#[doc = "DDRPERFM hardware configuration register"]
pub mod ddrperfm_hwcfg;
#[doc = "DDRPERFM version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_ver](ddrperfm_ver) module"]
pub type DDRPERFM_VER = crate::Reg<u32, _DDRPERFM_VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_VER;
#[doc = "`read()` method returns [ddrperfm_ver::R](ddrperfm_ver::R) reader structure"]
impl crate::Readable for DDRPERFM_VER {}
#[doc = "DDRPERFM version register"]
pub mod ddrperfm_ver;
#[doc = "DDRPERFM ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_id](ddrperfm_id) module"]
pub type DDRPERFM_ID = crate::Reg<u32, _DDRPERFM_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_ID;
#[doc = "`read()` method returns [ddrperfm_id::R](ddrperfm_id::R) reader structure"]
impl crate::Readable for DDRPERFM_ID {}
#[doc = "DDRPERFM ID register"]
pub mod ddrperfm_id;
#[doc = "DDRPERFM magic ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_sid](ddrperfm_sid) module"]
pub type DDRPERFM_SID = crate::Reg<u32, _DDRPERFM_SID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPERFM_SID;
#[doc = "`read()` method returns [ddrperfm_sid::R](ddrperfm_sid::R) reader structure"]
impl crate::Readable for DDRPERFM_SID {}
#[doc = "DDRPERFM magic ID register"]
pub mod ddrperfm_sid;
