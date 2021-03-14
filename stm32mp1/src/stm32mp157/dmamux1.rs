#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel 0 configuration register"]
    pub dmamux_c0cr: DMAMUX_C0CR,
    #[doc = "0x04 - DMAMUX request line multiplexer channel 1 configuration register"]
    pub dmamux_c1cr: DMAMUX_C1CR,
    #[doc = "0x08 - DMAMUX request line multiplexer channel 2 configuration register"]
    pub dmamux_c2cr: DMAMUX_C2CR,
    #[doc = "0x0c - DMAMUX request line multiplexer channel 3 configuration register"]
    pub dmamux_c3cr: DMAMUX_C3CR,
    #[doc = "0x10 - DMAMUX request line multiplexer channel 4 configuration register"]
    pub dmamux_c4cr: DMAMUX_C4CR,
    #[doc = "0x14 - DMAMUX request line multiplexer channel 5 configuration register"]
    pub dmamux_c5cr: DMAMUX_C5CR,
    #[doc = "0x18 - DMAMUX request line multiplexer channel 6 configuration register"]
    pub dmamux_c6cr: DMAMUX_C6CR,
    #[doc = "0x1c - DMAMUX request line multiplexer channel 7 configuration register"]
    pub dmamux_c7cr: DMAMUX_C7CR,
    #[doc = "0x20 - DMAMUX request line multiplexer channel 8 configuration register"]
    pub dmamux_c8cr: DMAMUX_C8CR,
    #[doc = "0x24 - DMAMUX request line multiplexer channel 9 configuration register"]
    pub dmamux_c9cr: DMAMUX_C9CR,
    #[doc = "0x28 - DMAMUX request line multiplexer channel 10 configuration register"]
    pub dmamux_c10cr: DMAMUX_C10CR,
    #[doc = "0x2c - DMAMUX request line multiplexer channel 11 configuration register"]
    pub dmamux_c11cr: DMAMUX_C11CR,
    #[doc = "0x30 - DMAMUX request line multiplexer channel 12 configuration register"]
    pub dmamux_c12cr: DMAMUX_C12CR,
    #[doc = "0x34 - DMAMUX request line multiplexer channel 13 configuration register"]
    pub dmamux_c13cr: DMAMUX_C13CR,
    #[doc = "0x38 - DMAMUX request line multiplexer channel 14 configuration register"]
    pub dmamux_c14cr: DMAMUX_C14CR,
    #[doc = "0x3c - DMAMUX request line multiplexer channel 15 configuration register"]
    pub dmamux_c15cr: DMAMUX_C15CR,
    _reserved16: [u8; 64usize],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub dmamux_csr: DMAMUX_CSR,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub dmamux_cfr: DMAMUX_CFR,
    _reserved18: [u8; 120usize],
    #[doc = "0x100 - DMAMUX request generator channel 0 configuration register"]
    pub dmamux_rg0cr: DMAMUX_RG0CR,
    #[doc = "0x104 - DMAMUX request generator channel 1 configuration register"]
    pub dmamux_rg1cr: DMAMUX_RG1CR,
    #[doc = "0x108 - DMAMUX request generator channel 2 configuration register"]
    pub dmamux_rg2cr: DMAMUX_RG2CR,
    #[doc = "0x10c - DMAMUX request generator channel 3 configuration register"]
    pub dmamux_rg3cr: DMAMUX_RG3CR,
    #[doc = "0x110 - DMAMUX request generator channel 4 configuration register"]
    pub dmamux_rg4cr: DMAMUX_RG4CR,
    #[doc = "0x114 - DMAMUX request generator channel 5 configuration register"]
    pub dmamux_rg5cr: DMAMUX_RG5CR,
    #[doc = "0x118 - DMAMUX request generator channel 6 configuration register"]
    pub dmamux_rg6cr: DMAMUX_RG6CR,
    #[doc = "0x11c - DMAMUX request generator channel 7 configuration register"]
    pub dmamux_rg7cr: DMAMUX_RG7CR,
    _reserved26: [u8; 32usize],
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    pub dmamux_rgsr: DMAMUX_RGSR,
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    pub dmamux_rgcfr: DMAMUX_RGCFR,
    _reserved28: [u8; 676usize],
    #[doc = "0x3ec - DMAMUX hardware configuration 2 register"]
    pub dmamux_hwcfgr2: DMAMUX_HWCFGR2,
    #[doc = "0x3f0 - DMAMUX hardware configuration 1 register"]
    pub dmamux_hwcfgr1: DMAMUX_HWCFGR1,
    #[doc = "0x3f4 - This register identifies the IP version."]
    pub dmamux_verr: DMAMUX_VERR,
    #[doc = "0x3f8 - This register identifies the IP."]
    pub dmamux_ipidr: DMAMUX_IPIDR,
    #[doc = "0x3fc - DMAMUX size identification register"]
    pub dmamux_sidr: DMAMUX_SIDR,
}
#[doc = "DMAMUX request line multiplexer channel 0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c0cr](dmamux_c0cr) module"]
pub type DMAMUX_C0CR = crate::Reg<u32, _DMAMUX_C0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C0CR;
#[doc = "`read()` method returns [dmamux_c0cr::R](dmamux_c0cr::R) reader structure"]
impl crate::Readable for DMAMUX_C0CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c0cr::W](dmamux_c0cr::W) writer structure"]
impl crate::Writable for DMAMUX_C0CR {}
#[doc = "DMAMUX request line multiplexer channel 0 configuration register"]
pub mod dmamux_c0cr;
#[doc = "DMAMUX request line multiplexer channel 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c1cr](dmamux_c1cr) module"]
pub type DMAMUX_C1CR = crate::Reg<u32, _DMAMUX_C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C1CR;
#[doc = "`read()` method returns [dmamux_c1cr::R](dmamux_c1cr::R) reader structure"]
impl crate::Readable for DMAMUX_C1CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c1cr::W](dmamux_c1cr::W) writer structure"]
impl crate::Writable for DMAMUX_C1CR {}
#[doc = "DMAMUX request line multiplexer channel 1 configuration register"]
pub mod dmamux_c1cr;
#[doc = "DMAMUX request line multiplexer channel 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c2cr](dmamux_c2cr) module"]
pub type DMAMUX_C2CR = crate::Reg<u32, _DMAMUX_C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C2CR;
#[doc = "`read()` method returns [dmamux_c2cr::R](dmamux_c2cr::R) reader structure"]
impl crate::Readable for DMAMUX_C2CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c2cr::W](dmamux_c2cr::W) writer structure"]
impl crate::Writable for DMAMUX_C2CR {}
#[doc = "DMAMUX request line multiplexer channel 2 configuration register"]
pub mod dmamux_c2cr;
#[doc = "DMAMUX request line multiplexer channel 3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c3cr](dmamux_c3cr) module"]
pub type DMAMUX_C3CR = crate::Reg<u32, _DMAMUX_C3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C3CR;
#[doc = "`read()` method returns [dmamux_c3cr::R](dmamux_c3cr::R) reader structure"]
impl crate::Readable for DMAMUX_C3CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c3cr::W](dmamux_c3cr::W) writer structure"]
impl crate::Writable for DMAMUX_C3CR {}
#[doc = "DMAMUX request line multiplexer channel 3 configuration register"]
pub mod dmamux_c3cr;
#[doc = "DMAMUX request line multiplexer channel 4 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c4cr](dmamux_c4cr) module"]
pub type DMAMUX_C4CR = crate::Reg<u32, _DMAMUX_C4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C4CR;
#[doc = "`read()` method returns [dmamux_c4cr::R](dmamux_c4cr::R) reader structure"]
impl crate::Readable for DMAMUX_C4CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c4cr::W](dmamux_c4cr::W) writer structure"]
impl crate::Writable for DMAMUX_C4CR {}
#[doc = "DMAMUX request line multiplexer channel 4 configuration register"]
pub mod dmamux_c4cr;
#[doc = "DMAMUX request line multiplexer channel 5 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c5cr](dmamux_c5cr) module"]
pub type DMAMUX_C5CR = crate::Reg<u32, _DMAMUX_C5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C5CR;
#[doc = "`read()` method returns [dmamux_c5cr::R](dmamux_c5cr::R) reader structure"]
impl crate::Readable for DMAMUX_C5CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c5cr::W](dmamux_c5cr::W) writer structure"]
impl crate::Writable for DMAMUX_C5CR {}
#[doc = "DMAMUX request line multiplexer channel 5 configuration register"]
pub mod dmamux_c5cr;
#[doc = "DMAMUX request line multiplexer channel 6 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c6cr](dmamux_c6cr) module"]
pub type DMAMUX_C6CR = crate::Reg<u32, _DMAMUX_C6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C6CR;
#[doc = "`read()` method returns [dmamux_c6cr::R](dmamux_c6cr::R) reader structure"]
impl crate::Readable for DMAMUX_C6CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c6cr::W](dmamux_c6cr::W) writer structure"]
impl crate::Writable for DMAMUX_C6CR {}
#[doc = "DMAMUX request line multiplexer channel 6 configuration register"]
pub mod dmamux_c6cr;
#[doc = "DMAMUX request line multiplexer channel 7 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c7cr](dmamux_c7cr) module"]
pub type DMAMUX_C7CR = crate::Reg<u32, _DMAMUX_C7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C7CR;
#[doc = "`read()` method returns [dmamux_c7cr::R](dmamux_c7cr::R) reader structure"]
impl crate::Readable for DMAMUX_C7CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c7cr::W](dmamux_c7cr::W) writer structure"]
impl crate::Writable for DMAMUX_C7CR {}
#[doc = "DMAMUX request line multiplexer channel 7 configuration register"]
pub mod dmamux_c7cr;
#[doc = "DMAMUX request line multiplexer channel 8 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c8cr](dmamux_c8cr) module"]
pub type DMAMUX_C8CR = crate::Reg<u32, _DMAMUX_C8CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C8CR;
#[doc = "`read()` method returns [dmamux_c8cr::R](dmamux_c8cr::R) reader structure"]
impl crate::Readable for DMAMUX_C8CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c8cr::W](dmamux_c8cr::W) writer structure"]
impl crate::Writable for DMAMUX_C8CR {}
#[doc = "DMAMUX request line multiplexer channel 8 configuration register"]
pub mod dmamux_c8cr;
#[doc = "DMAMUX request line multiplexer channel 9 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c9cr](dmamux_c9cr) module"]
pub type DMAMUX_C9CR = crate::Reg<u32, _DMAMUX_C9CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C9CR;
#[doc = "`read()` method returns [dmamux_c9cr::R](dmamux_c9cr::R) reader structure"]
impl crate::Readable for DMAMUX_C9CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c9cr::W](dmamux_c9cr::W) writer structure"]
impl crate::Writable for DMAMUX_C9CR {}
#[doc = "DMAMUX request line multiplexer channel 9 configuration register"]
pub mod dmamux_c9cr;
#[doc = "DMAMUX request line multiplexer channel 10 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c10cr](dmamux_c10cr) module"]
pub type DMAMUX_C10CR = crate::Reg<u32, _DMAMUX_C10CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C10CR;
#[doc = "`read()` method returns [dmamux_c10cr::R](dmamux_c10cr::R) reader structure"]
impl crate::Readable for DMAMUX_C10CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c10cr::W](dmamux_c10cr::W) writer structure"]
impl crate::Writable for DMAMUX_C10CR {}
#[doc = "DMAMUX request line multiplexer channel 10 configuration register"]
pub mod dmamux_c10cr;
#[doc = "DMAMUX request line multiplexer channel 11 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c11cr](dmamux_c11cr) module"]
pub type DMAMUX_C11CR = crate::Reg<u32, _DMAMUX_C11CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C11CR;
#[doc = "`read()` method returns [dmamux_c11cr::R](dmamux_c11cr::R) reader structure"]
impl crate::Readable for DMAMUX_C11CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c11cr::W](dmamux_c11cr::W) writer structure"]
impl crate::Writable for DMAMUX_C11CR {}
#[doc = "DMAMUX request line multiplexer channel 11 configuration register"]
pub mod dmamux_c11cr;
#[doc = "DMAMUX request line multiplexer channel 12 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c12cr](dmamux_c12cr) module"]
pub type DMAMUX_C12CR = crate::Reg<u32, _DMAMUX_C12CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C12CR;
#[doc = "`read()` method returns [dmamux_c12cr::R](dmamux_c12cr::R) reader structure"]
impl crate::Readable for DMAMUX_C12CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c12cr::W](dmamux_c12cr::W) writer structure"]
impl crate::Writable for DMAMUX_C12CR {}
#[doc = "DMAMUX request line multiplexer channel 12 configuration register"]
pub mod dmamux_c12cr;
#[doc = "DMAMUX request line multiplexer channel 13 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c13cr](dmamux_c13cr) module"]
pub type DMAMUX_C13CR = crate::Reg<u32, _DMAMUX_C13CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C13CR;
#[doc = "`read()` method returns [dmamux_c13cr::R](dmamux_c13cr::R) reader structure"]
impl crate::Readable for DMAMUX_C13CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c13cr::W](dmamux_c13cr::W) writer structure"]
impl crate::Writable for DMAMUX_C13CR {}
#[doc = "DMAMUX request line multiplexer channel 13 configuration register"]
pub mod dmamux_c13cr;
#[doc = "DMAMUX request line multiplexer channel 14 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c14cr](dmamux_c14cr) module"]
pub type DMAMUX_C14CR = crate::Reg<u32, _DMAMUX_C14CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C14CR;
#[doc = "`read()` method returns [dmamux_c14cr::R](dmamux_c14cr::R) reader structure"]
impl crate::Readable for DMAMUX_C14CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c14cr::W](dmamux_c14cr::W) writer structure"]
impl crate::Writable for DMAMUX_C14CR {}
#[doc = "DMAMUX request line multiplexer channel 14 configuration register"]
pub mod dmamux_c14cr;
#[doc = "DMAMUX request line multiplexer channel 15 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_c15cr](dmamux_c15cr) module"]
pub type DMAMUX_C15CR = crate::Reg<u32, _DMAMUX_C15CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_C15CR;
#[doc = "`read()` method returns [dmamux_c15cr::R](dmamux_c15cr::R) reader structure"]
impl crate::Readable for DMAMUX_C15CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_c15cr::W](dmamux_c15cr::W) writer structure"]
impl crate::Writable for DMAMUX_C15CR {}
#[doc = "DMAMUX request line multiplexer channel 15 configuration register"]
pub mod dmamux_c15cr;
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_csr](dmamux_csr) module"]
pub type DMAMUX_CSR = crate::Reg<u32, _DMAMUX_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_CSR;
#[doc = "`read()` method returns [dmamux_csr::R](dmamux_csr::R) reader structure"]
impl crate::Readable for DMAMUX_CSR {}
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod dmamux_csr;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_cfr](dmamux_cfr) module"]
pub type DMAMUX_CFR = crate::Reg<u32, _DMAMUX_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_CFR;
#[doc = "`write(|w| ..)` method takes [dmamux_cfr::W](dmamux_cfr::W) writer structure"]
impl crate::Writable for DMAMUX_CFR {}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod dmamux_cfr;
#[doc = "DMAMUX request generator channel 0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg0cr](dmamux_rg0cr) module"]
pub type DMAMUX_RG0CR = crate::Reg<u32, _DMAMUX_RG0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG0CR;
#[doc = "`read()` method returns [dmamux_rg0cr::R](dmamux_rg0cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG0CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg0cr::W](dmamux_rg0cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG0CR {}
#[doc = "DMAMUX request generator channel 0 configuration register"]
pub mod dmamux_rg0cr;
#[doc = "DMAMUX request generator channel 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg1cr](dmamux_rg1cr) module"]
pub type DMAMUX_RG1CR = crate::Reg<u32, _DMAMUX_RG1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG1CR;
#[doc = "`read()` method returns [dmamux_rg1cr::R](dmamux_rg1cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG1CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg1cr::W](dmamux_rg1cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG1CR {}
#[doc = "DMAMUX request generator channel 1 configuration register"]
pub mod dmamux_rg1cr;
#[doc = "DMAMUX request generator channel 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg2cr](dmamux_rg2cr) module"]
pub type DMAMUX_RG2CR = crate::Reg<u32, _DMAMUX_RG2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG2CR;
#[doc = "`read()` method returns [dmamux_rg2cr::R](dmamux_rg2cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG2CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg2cr::W](dmamux_rg2cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG2CR {}
#[doc = "DMAMUX request generator channel 2 configuration register"]
pub mod dmamux_rg2cr;
#[doc = "DMAMUX request generator channel 3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg3cr](dmamux_rg3cr) module"]
pub type DMAMUX_RG3CR = crate::Reg<u32, _DMAMUX_RG3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG3CR;
#[doc = "`read()` method returns [dmamux_rg3cr::R](dmamux_rg3cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG3CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg3cr::W](dmamux_rg3cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG3CR {}
#[doc = "DMAMUX request generator channel 3 configuration register"]
pub mod dmamux_rg3cr;
#[doc = "DMAMUX request generator channel 4 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg4cr](dmamux_rg4cr) module"]
pub type DMAMUX_RG4CR = crate::Reg<u32, _DMAMUX_RG4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG4CR;
#[doc = "`read()` method returns [dmamux_rg4cr::R](dmamux_rg4cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG4CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg4cr::W](dmamux_rg4cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG4CR {}
#[doc = "DMAMUX request generator channel 4 configuration register"]
pub mod dmamux_rg4cr;
#[doc = "DMAMUX request generator channel 5 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg5cr](dmamux_rg5cr) module"]
pub type DMAMUX_RG5CR = crate::Reg<u32, _DMAMUX_RG5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG5CR;
#[doc = "`read()` method returns [dmamux_rg5cr::R](dmamux_rg5cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG5CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg5cr::W](dmamux_rg5cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG5CR {}
#[doc = "DMAMUX request generator channel 5 configuration register"]
pub mod dmamux_rg5cr;
#[doc = "DMAMUX request generator channel 6 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg6cr](dmamux_rg6cr) module"]
pub type DMAMUX_RG6CR = crate::Reg<u32, _DMAMUX_RG6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG6CR;
#[doc = "`read()` method returns [dmamux_rg6cr::R](dmamux_rg6cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG6CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg6cr::W](dmamux_rg6cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG6CR {}
#[doc = "DMAMUX request generator channel 6 configuration register"]
pub mod dmamux_rg6cr;
#[doc = "DMAMUX request generator channel 7 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg7cr](dmamux_rg7cr) module"]
pub type DMAMUX_RG7CR = crate::Reg<u32, _DMAMUX_RG7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RG7CR;
#[doc = "`read()` method returns [dmamux_rg7cr::R](dmamux_rg7cr::R) reader structure"]
impl crate::Readable for DMAMUX_RG7CR {}
#[doc = "`write(|w| ..)` method takes [dmamux_rg7cr::W](dmamux_rg7cr::W) writer structure"]
impl crate::Writable for DMAMUX_RG7CR {}
#[doc = "DMAMUX request generator channel 7 configuration register"]
pub mod dmamux_rg7cr;
#[doc = "DMAMUX request generator interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rgsr](dmamux_rgsr) module"]
pub type DMAMUX_RGSR = crate::Reg<u32, _DMAMUX_RGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RGSR;
#[doc = "`read()` method returns [dmamux_rgsr::R](dmamux_rgsr::R) reader structure"]
impl crate::Readable for DMAMUX_RGSR {}
#[doc = "DMAMUX request generator interrupt status register"]
pub mod dmamux_rgsr;
#[doc = "DMAMUX request generator interrupt clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rgcfr](dmamux_rgcfr) module"]
pub type DMAMUX_RGCFR = crate::Reg<u32, _DMAMUX_RGCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_RGCFR;
#[doc = "`write(|w| ..)` method takes [dmamux_rgcfr::W](dmamux_rgcfr::W) writer structure"]
impl crate::Writable for DMAMUX_RGCFR {}
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod dmamux_rgcfr;
#[doc = "DMAMUX hardware configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_hwcfgr2](dmamux_hwcfgr2) module"]
pub type DMAMUX_HWCFGR2 = crate::Reg<u32, _DMAMUX_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_HWCFGR2;
#[doc = "`read()` method returns [dmamux_hwcfgr2::R](dmamux_hwcfgr2::R) reader structure"]
impl crate::Readable for DMAMUX_HWCFGR2 {}
#[doc = "DMAMUX hardware configuration 2 register"]
pub mod dmamux_hwcfgr2;
#[doc = "DMAMUX hardware configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_hwcfgr1](dmamux_hwcfgr1) module"]
pub type DMAMUX_HWCFGR1 = crate::Reg<u32, _DMAMUX_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_HWCFGR1;
#[doc = "`read()` method returns [dmamux_hwcfgr1::R](dmamux_hwcfgr1::R) reader structure"]
impl crate::Readable for DMAMUX_HWCFGR1 {}
#[doc = "DMAMUX hardware configuration 1 register"]
pub mod dmamux_hwcfgr1;
#[doc = "This register identifies the IP version.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_verr](dmamux_verr) module"]
pub type DMAMUX_VERR = crate::Reg<u32, _DMAMUX_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_VERR;
#[doc = "`read()` method returns [dmamux_verr::R](dmamux_verr::R) reader structure"]
impl crate::Readable for DMAMUX_VERR {}
#[doc = "This register identifies the IP version."]
pub mod dmamux_verr;
#[doc = "This register identifies the IP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_ipidr](dmamux_ipidr) module"]
pub type DMAMUX_IPIDR = crate::Reg<u32, _DMAMUX_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_IPIDR;
#[doc = "`read()` method returns [dmamux_ipidr::R](dmamux_ipidr::R) reader structure"]
impl crate::Readable for DMAMUX_IPIDR {}
#[doc = "This register identifies the IP."]
pub mod dmamux_ipidr;
#[doc = "DMAMUX size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_sidr](dmamux_sidr) module"]
pub type DMAMUX_SIDR = crate::Reg<u32, _DMAMUX_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX_SIDR;
#[doc = "`read()` method returns [dmamux_sidr::R](dmamux_sidr::R) reader structure"]
impl crate::Readable for DMAMUX_SIDR {}
#[doc = "DMAMUX size identification register"]
pub mod dmamux_sidr;
