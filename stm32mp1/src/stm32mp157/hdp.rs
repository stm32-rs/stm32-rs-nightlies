#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HDP Control"]
    pub hdp_ctrl: HDP_CTRL,
    #[doc = "0x04 - HDP multiplexing"]
    pub hdp_mux: HDP_MUX,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - HDP value"]
    pub hdp_val: HDP_VAL,
    #[doc = "0x14 - HDP GPO set"]
    pub hdp_gposet: HDP_GPOSET,
    #[doc = "0x18 - HDP GPO clear"]
    pub hdp_gpoclr: HDP_GPOCLR,
    #[doc = "0x1c - HDP GPO value"]
    pub hdp_gpoval: HDP_GPOVAL,
    _reserved6: [u8; 980usize],
    #[doc = "0x3f4 - HDP version register"]
    pub hdp_verr: HDP_VERR,
    #[doc = "0x3f8 - HDP IP identification register"]
    pub hdp_ipidr: HDP_IPIDR,
    #[doc = "0x3fc - HDP size identification register"]
    pub hdp_sidr: HDP_SIDR,
}
#[doc = "HDP Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_ctrl](hdp_ctrl) module"]
pub type HDP_CTRL = crate::Reg<u32, _HDP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_CTRL;
#[doc = "`read()` method returns [hdp_ctrl::R](hdp_ctrl::R) reader structure"]
impl crate::Readable for HDP_CTRL {}
#[doc = "`write(|w| ..)` method takes [hdp_ctrl::W](hdp_ctrl::W) writer structure"]
impl crate::Writable for HDP_CTRL {}
#[doc = "HDP Control"]
pub mod hdp_ctrl;
#[doc = "HDP multiplexing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_mux](hdp_mux) module"]
pub type HDP_MUX = crate::Reg<u32, _HDP_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_MUX;
#[doc = "`read()` method returns [hdp_mux::R](hdp_mux::R) reader structure"]
impl crate::Readable for HDP_MUX {}
#[doc = "`write(|w| ..)` method takes [hdp_mux::W](hdp_mux::W) writer structure"]
impl crate::Writable for HDP_MUX {}
#[doc = "HDP multiplexing"]
pub mod hdp_mux;
#[doc = "HDP value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_val](hdp_val) module"]
pub type HDP_VAL = crate::Reg<u32, _HDP_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_VAL;
#[doc = "`read()` method returns [hdp_val::R](hdp_val::R) reader structure"]
impl crate::Readable for HDP_VAL {}
#[doc = "HDP value"]
pub mod hdp_val;
#[doc = "HDP GPO set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_gposet](hdp_gposet) module"]
pub type HDP_GPOSET = crate::Reg<u32, _HDP_GPOSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_GPOSET;
#[doc = "`write(|w| ..)` method takes [hdp_gposet::W](hdp_gposet::W) writer structure"]
impl crate::Writable for HDP_GPOSET {}
#[doc = "HDP GPO set"]
pub mod hdp_gposet;
#[doc = "HDP GPO clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_gpoclr](hdp_gpoclr) module"]
pub type HDP_GPOCLR = crate::Reg<u32, _HDP_GPOCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_GPOCLR;
#[doc = "`write(|w| ..)` method takes [hdp_gpoclr::W](hdp_gpoclr::W) writer structure"]
impl crate::Writable for HDP_GPOCLR {}
#[doc = "HDP GPO clear"]
pub mod hdp_gpoclr;
#[doc = "HDP GPO value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_gpoval](hdp_gpoval) module"]
pub type HDP_GPOVAL = crate::Reg<u32, _HDP_GPOVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_GPOVAL;
#[doc = "`read()` method returns [hdp_gpoval::R](hdp_gpoval::R) reader structure"]
impl crate::Readable for HDP_GPOVAL {}
#[doc = "`write(|w| ..)` method takes [hdp_gpoval::W](hdp_gpoval::W) writer structure"]
impl crate::Writable for HDP_GPOVAL {}
#[doc = "HDP GPO value"]
pub mod hdp_gpoval;
#[doc = "HDP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_verr](hdp_verr) module"]
pub type HDP_VERR = crate::Reg<u32, _HDP_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_VERR;
#[doc = "`read()` method returns [hdp_verr::R](hdp_verr::R) reader structure"]
impl crate::Readable for HDP_VERR {}
#[doc = "HDP version register"]
pub mod hdp_verr;
#[doc = "HDP IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_ipidr](hdp_ipidr) module"]
pub type HDP_IPIDR = crate::Reg<u32, _HDP_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_IPIDR;
#[doc = "`read()` method returns [hdp_ipidr::R](hdp_ipidr::R) reader structure"]
impl crate::Readable for HDP_IPIDR {}
#[doc = "HDP IP identification register"]
pub mod hdp_ipidr;
#[doc = "HDP size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_sidr](hdp_sidr) module"]
pub type HDP_SIDR = crate::Reg<u32, _HDP_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDP_SIDR;
#[doc = "`read()` method returns [hdp_sidr::R](hdp_sidr::R) reader structure"]
impl crate::Readable for HDP_SIDR {}
#[doc = "HDP size identification register"]
pub mod hdp_sidr;
