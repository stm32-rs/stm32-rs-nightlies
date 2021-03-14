#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub lisr: LISR,
    #[doc = "0x04 - high interrupt status register"]
    pub hisr: HISR,
    #[doc = "0x08 - low interrupt flag clear register"]
    pub lifcr: LIFCR,
    #[doc = "0x0c - high interrupt flag clear register"]
    pub hifcr: HIFCR,
    #[doc = "0x10 - stream x configuration register"]
    pub s0cr: S0CR,
    #[doc = "0x14 - stream x number of data register"]
    pub s0ndtr: S0NDTR,
    #[doc = "0x18 - stream x peripheral address register"]
    pub s0par: S0PAR,
    #[doc = "0x1c - stream x memory 0 address register"]
    pub s0m0ar: S0M0AR,
    #[doc = "0x20 - stream x memory 1 address register"]
    pub s0m1ar: S0M1AR,
    #[doc = "0x24 - stream x FIFO control register"]
    pub s0fcr: S0FCR,
    #[doc = "0x28 - stream x configuration register"]
    pub s1cr: S1CR,
    #[doc = "0x2c - stream x number of data register"]
    pub s1ndtr: S1NDTR,
    #[doc = "0x30 - stream x peripheral address register"]
    pub s1par: S1PAR,
    #[doc = "0x34 - stream x memory 0 address register"]
    pub s1m0ar: S1M0AR,
    #[doc = "0x38 - stream x memory 1 address register"]
    pub s1m1ar: S1M1AR,
    #[doc = "0x3c - stream x FIFO control register"]
    pub s1fcr: S1FCR,
    #[doc = "0x40 - stream x configuration register"]
    pub s2cr: S2CR,
    #[doc = "0x44 - stream x number of data register"]
    pub s2ndtr: S2NDTR,
    #[doc = "0x48 - stream x peripheral address register"]
    pub s2par: S2PAR,
    #[doc = "0x4c - stream x memory 0 address register"]
    pub s2m0ar: S2M0AR,
    #[doc = "0x50 - stream x memory 1 address register"]
    pub s2m1ar: S2M1AR,
    #[doc = "0x54 - stream x FIFO control register"]
    pub s2fcr: S2FCR,
    #[doc = "0x58 - stream x configuration register"]
    pub s3cr: S3CR,
    #[doc = "0x5c - stream x number of data register"]
    pub s3ndtr: S3NDTR,
    #[doc = "0x60 - stream x peripheral address register"]
    pub s3par: S3PAR,
    #[doc = "0x64 - stream x memory 0 address register"]
    pub s3m0ar: S3M0AR,
    #[doc = "0x68 - stream x memory 1 address register"]
    pub s3m1ar: S3M1AR,
    #[doc = "0x6c - stream x FIFO control register"]
    pub s3fcr: S3FCR,
    #[doc = "0x70 - stream x configuration register"]
    pub s4cr: S4CR,
    #[doc = "0x74 - stream x number of data register"]
    pub s4ndtr: S4NDTR,
    #[doc = "0x78 - stream x peripheral address register"]
    pub s4par: S4PAR,
    #[doc = "0x7c - stream x memory 0 address register"]
    pub s4m0ar: S4M0AR,
    #[doc = "0x80 - stream x memory 1 address register"]
    pub s4m1ar: S4M1AR,
    #[doc = "0x84 - stream x FIFO control register"]
    pub s4fcr: S4FCR,
    #[doc = "0x88 - stream x configuration register"]
    pub s5cr: S5CR,
    #[doc = "0x8c - stream x number of data register"]
    pub s5ndtr: S5NDTR,
    #[doc = "0x90 - stream x peripheral address register"]
    pub s5par: S5PAR,
    #[doc = "0x94 - stream x memory 0 address register"]
    pub s5m0ar: S5M0AR,
    #[doc = "0x98 - stream x memory 1 address register"]
    pub s5m1ar: S5M1AR,
    #[doc = "0x9c - stream x FIFO control register"]
    pub s5fcr: S5FCR,
    #[doc = "0xa0 - stream x configuration register"]
    pub s6cr: S6CR,
    #[doc = "0xa4 - stream x number of data register"]
    pub s6ndtr: S6NDTR,
    #[doc = "0xa8 - stream x peripheral address register"]
    pub s6par: S6PAR,
    #[doc = "0xac - stream x memory 0 address register"]
    pub s6m0ar: S6M0AR,
    #[doc = "0xb0 - stream x memory 1 address register"]
    pub s6m1ar: S6M1AR,
    #[doc = "0xb4 - stream x FIFO control register"]
    pub s6fcr: S6FCR,
    #[doc = "0xb8 - stream x configuration register"]
    pub s7cr: S7CR,
    #[doc = "0xbc - stream x number of data register"]
    pub s7ndtr: S7NDTR,
    #[doc = "0xc0 - stream x peripheral address register"]
    pub s7par: S7PAR,
    #[doc = "0xc4 - stream x memory 0 address register"]
    pub s7m0ar: S7M0AR,
    #[doc = "0xc8 - stream x memory 1 address register"]
    pub s7m1ar: S7M1AR,
    #[doc = "0xcc - stream x FIFO control register"]
    pub s7fcr: S7FCR,
}
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lisr](lisr) module"]
pub type LISR = crate::Reg<u32, _LISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LISR;
#[doc = "`read()` method returns [lisr::R](lisr::R) reader structure"]
impl crate::Readable for LISR {}
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "high interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hisr](hisr) module"]
pub type HISR = crate::Reg<u32, _HISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HISR;
#[doc = "`read()` method returns [hisr::R](hisr::R) reader structure"]
impl crate::Readable for HISR {}
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "low interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lifcr](lifcr) module"]
pub type LIFCR = crate::Reg<u32, _LIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIFCR;
#[doc = "`write(|w| ..)` method takes [lifcr::W](lifcr::W) writer structure"]
impl crate::Writable for LIFCR {}
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "high interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hifcr](hifcr) module"]
pub type HIFCR = crate::Reg<u32, _HIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIFCR;
#[doc = "`write(|w| ..)` method takes [hifcr::W](hifcr::W) writer structure"]
impl crate::Writable for HIFCR {}
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0cr](s0cr) module"]
pub type S0CR = crate::Reg<u32, _S0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S0CR;
#[doc = "`read()` method returns [s0cr::R](s0cr::R) reader structure"]
impl crate::Readable for S0CR {}
#[doc = "`write(|w| ..)` method takes [s0cr::W](s0cr::W) writer structure"]
impl crate::Writable for S0CR {}
#[doc = "stream x configuration register"]
pub mod s0cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0ndtr](s0ndtr) module"]
pub type S0NDTR = crate::Reg<u32, _S0NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S0NDTR;
#[doc = "`read()` method returns [s0ndtr::R](s0ndtr::R) reader structure"]
impl crate::Readable for S0NDTR {}
#[doc = "`write(|w| ..)` method takes [s0ndtr::W](s0ndtr::W) writer structure"]
impl crate::Writable for S0NDTR {}
#[doc = "stream x number of data register"]
pub mod s0ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0par](s0par) module"]
pub type S0PAR = crate::Reg<u32, _S0PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S0PAR;
#[doc = "`read()` method returns [s0par::R](s0par::R) reader structure"]
impl crate::Readable for S0PAR {}
#[doc = "`write(|w| ..)` method takes [s0par::W](s0par::W) writer structure"]
impl crate::Writable for S0PAR {}
#[doc = "stream x peripheral address register"]
pub mod s0par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0m0ar](s0m0ar) module"]
pub type S0M0AR = crate::Reg<u32, _S0M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S0M0AR;
#[doc = "`read()` method returns [s0m0ar::R](s0m0ar::R) reader structure"]
impl crate::Readable for S0M0AR {}
#[doc = "`write(|w| ..)` method takes [s0m0ar::W](s0m0ar::W) writer structure"]
impl crate::Writable for S0M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s0m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0m1ar](s0m1ar) module"]
pub type S0M1AR = crate::Reg<u32, _S0M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S0M1AR;
#[doc = "`read()` method returns [s0m1ar::R](s0m1ar::R) reader structure"]
impl crate::Readable for S0M1AR {}
#[doc = "`write(|w| ..)` method takes [s0m1ar::W](s0m1ar::W) writer structure"]
impl crate::Writable for S0M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s0m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0fcr](s0fcr) module"]
pub type S0FCR = crate::Reg<u32, _S0FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S0FCR;
#[doc = "`read()` method returns [s0fcr::R](s0fcr::R) reader structure"]
impl crate::Readable for S0FCR {}
#[doc = "`write(|w| ..)` method takes [s0fcr::W](s0fcr::W) writer structure"]
impl crate::Writable for S0FCR {}
#[doc = "stream x FIFO control register"]
pub mod s0fcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1cr](s1cr) module"]
pub type S1CR = crate::Reg<u32, _S1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1CR;
#[doc = "`read()` method returns [s1cr::R](s1cr::R) reader structure"]
impl crate::Readable for S1CR {}
#[doc = "`write(|w| ..)` method takes [s1cr::W](s1cr::W) writer structure"]
impl crate::Writable for S1CR {}
#[doc = "stream x configuration register"]
pub mod s1cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1ndtr](s1ndtr) module"]
pub type S1NDTR = crate::Reg<u32, _S1NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1NDTR;
#[doc = "`read()` method returns [s1ndtr::R](s1ndtr::R) reader structure"]
impl crate::Readable for S1NDTR {}
#[doc = "`write(|w| ..)` method takes [s1ndtr::W](s1ndtr::W) writer structure"]
impl crate::Writable for S1NDTR {}
#[doc = "stream x number of data register"]
pub mod s1ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1par](s1par) module"]
pub type S1PAR = crate::Reg<u32, _S1PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1PAR;
#[doc = "`read()` method returns [s1par::R](s1par::R) reader structure"]
impl crate::Readable for S1PAR {}
#[doc = "`write(|w| ..)` method takes [s1par::W](s1par::W) writer structure"]
impl crate::Writable for S1PAR {}
#[doc = "stream x peripheral address register"]
pub mod s1par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1m0ar](s1m0ar) module"]
pub type S1M0AR = crate::Reg<u32, _S1M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1M0AR;
#[doc = "`read()` method returns [s1m0ar::R](s1m0ar::R) reader structure"]
impl crate::Readable for S1M0AR {}
#[doc = "`write(|w| ..)` method takes [s1m0ar::W](s1m0ar::W) writer structure"]
impl crate::Writable for S1M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s1m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1m1ar](s1m1ar) module"]
pub type S1M1AR = crate::Reg<u32, _S1M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1M1AR;
#[doc = "`read()` method returns [s1m1ar::R](s1m1ar::R) reader structure"]
impl crate::Readable for S1M1AR {}
#[doc = "`write(|w| ..)` method takes [s1m1ar::W](s1m1ar::W) writer structure"]
impl crate::Writable for S1M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s1m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1fcr](s1fcr) module"]
pub type S1FCR = crate::Reg<u32, _S1FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1FCR;
#[doc = "`read()` method returns [s1fcr::R](s1fcr::R) reader structure"]
impl crate::Readable for S1FCR {}
#[doc = "`write(|w| ..)` method takes [s1fcr::W](s1fcr::W) writer structure"]
impl crate::Writable for S1FCR {}
#[doc = "stream x FIFO control register"]
pub mod s1fcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2cr](s2cr) module"]
pub type S2CR = crate::Reg<u32, _S2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2CR;
#[doc = "`read()` method returns [s2cr::R](s2cr::R) reader structure"]
impl crate::Readable for S2CR {}
#[doc = "`write(|w| ..)` method takes [s2cr::W](s2cr::W) writer structure"]
impl crate::Writable for S2CR {}
#[doc = "stream x configuration register"]
pub mod s2cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2ndtr](s2ndtr) module"]
pub type S2NDTR = crate::Reg<u32, _S2NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2NDTR;
#[doc = "`read()` method returns [s2ndtr::R](s2ndtr::R) reader structure"]
impl crate::Readable for S2NDTR {}
#[doc = "`write(|w| ..)` method takes [s2ndtr::W](s2ndtr::W) writer structure"]
impl crate::Writable for S2NDTR {}
#[doc = "stream x number of data register"]
pub mod s2ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2par](s2par) module"]
pub type S2PAR = crate::Reg<u32, _S2PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2PAR;
#[doc = "`read()` method returns [s2par::R](s2par::R) reader structure"]
impl crate::Readable for S2PAR {}
#[doc = "`write(|w| ..)` method takes [s2par::W](s2par::W) writer structure"]
impl crate::Writable for S2PAR {}
#[doc = "stream x peripheral address register"]
pub mod s2par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2m0ar](s2m0ar) module"]
pub type S2M0AR = crate::Reg<u32, _S2M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2M0AR;
#[doc = "`read()` method returns [s2m0ar::R](s2m0ar::R) reader structure"]
impl crate::Readable for S2M0AR {}
#[doc = "`write(|w| ..)` method takes [s2m0ar::W](s2m0ar::W) writer structure"]
impl crate::Writable for S2M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s2m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2m1ar](s2m1ar) module"]
pub type S2M1AR = crate::Reg<u32, _S2M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2M1AR;
#[doc = "`read()` method returns [s2m1ar::R](s2m1ar::R) reader structure"]
impl crate::Readable for S2M1AR {}
#[doc = "`write(|w| ..)` method takes [s2m1ar::W](s2m1ar::W) writer structure"]
impl crate::Writable for S2M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s2m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2fcr](s2fcr) module"]
pub type S2FCR = crate::Reg<u32, _S2FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2FCR;
#[doc = "`read()` method returns [s2fcr::R](s2fcr::R) reader structure"]
impl crate::Readable for S2FCR {}
#[doc = "`write(|w| ..)` method takes [s2fcr::W](s2fcr::W) writer structure"]
impl crate::Writable for S2FCR {}
#[doc = "stream x FIFO control register"]
pub mod s2fcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s3cr](s3cr) module"]
pub type S3CR = crate::Reg<u32, _S3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S3CR;
#[doc = "`read()` method returns [s3cr::R](s3cr::R) reader structure"]
impl crate::Readable for S3CR {}
#[doc = "`write(|w| ..)` method takes [s3cr::W](s3cr::W) writer structure"]
impl crate::Writable for S3CR {}
#[doc = "stream x configuration register"]
pub mod s3cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s3ndtr](s3ndtr) module"]
pub type S3NDTR = crate::Reg<u32, _S3NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S3NDTR;
#[doc = "`read()` method returns [s3ndtr::R](s3ndtr::R) reader structure"]
impl crate::Readable for S3NDTR {}
#[doc = "`write(|w| ..)` method takes [s3ndtr::W](s3ndtr::W) writer structure"]
impl crate::Writable for S3NDTR {}
#[doc = "stream x number of data register"]
pub mod s3ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s3par](s3par) module"]
pub type S3PAR = crate::Reg<u32, _S3PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S3PAR;
#[doc = "`read()` method returns [s3par::R](s3par::R) reader structure"]
impl crate::Readable for S3PAR {}
#[doc = "`write(|w| ..)` method takes [s3par::W](s3par::W) writer structure"]
impl crate::Writable for S3PAR {}
#[doc = "stream x peripheral address register"]
pub mod s3par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s3m0ar](s3m0ar) module"]
pub type S3M0AR = crate::Reg<u32, _S3M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S3M0AR;
#[doc = "`read()` method returns [s3m0ar::R](s3m0ar::R) reader structure"]
impl crate::Readable for S3M0AR {}
#[doc = "`write(|w| ..)` method takes [s3m0ar::W](s3m0ar::W) writer structure"]
impl crate::Writable for S3M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s3m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s3m1ar](s3m1ar) module"]
pub type S3M1AR = crate::Reg<u32, _S3M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S3M1AR;
#[doc = "`read()` method returns [s3m1ar::R](s3m1ar::R) reader structure"]
impl crate::Readable for S3M1AR {}
#[doc = "`write(|w| ..)` method takes [s3m1ar::W](s3m1ar::W) writer structure"]
impl crate::Writable for S3M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s3m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s3fcr](s3fcr) module"]
pub type S3FCR = crate::Reg<u32, _S3FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S3FCR;
#[doc = "`read()` method returns [s3fcr::R](s3fcr::R) reader structure"]
impl crate::Readable for S3FCR {}
#[doc = "`write(|w| ..)` method takes [s3fcr::W](s3fcr::W) writer structure"]
impl crate::Writable for S3FCR {}
#[doc = "stream x FIFO control register"]
pub mod s3fcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s4cr](s4cr) module"]
pub type S4CR = crate::Reg<u32, _S4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S4CR;
#[doc = "`read()` method returns [s4cr::R](s4cr::R) reader structure"]
impl crate::Readable for S4CR {}
#[doc = "`write(|w| ..)` method takes [s4cr::W](s4cr::W) writer structure"]
impl crate::Writable for S4CR {}
#[doc = "stream x configuration register"]
pub mod s4cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s4ndtr](s4ndtr) module"]
pub type S4NDTR = crate::Reg<u32, _S4NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S4NDTR;
#[doc = "`read()` method returns [s4ndtr::R](s4ndtr::R) reader structure"]
impl crate::Readable for S4NDTR {}
#[doc = "`write(|w| ..)` method takes [s4ndtr::W](s4ndtr::W) writer structure"]
impl crate::Writable for S4NDTR {}
#[doc = "stream x number of data register"]
pub mod s4ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s4par](s4par) module"]
pub type S4PAR = crate::Reg<u32, _S4PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S4PAR;
#[doc = "`read()` method returns [s4par::R](s4par::R) reader structure"]
impl crate::Readable for S4PAR {}
#[doc = "`write(|w| ..)` method takes [s4par::W](s4par::W) writer structure"]
impl crate::Writable for S4PAR {}
#[doc = "stream x peripheral address register"]
pub mod s4par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s4m0ar](s4m0ar) module"]
pub type S4M0AR = crate::Reg<u32, _S4M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S4M0AR;
#[doc = "`read()` method returns [s4m0ar::R](s4m0ar::R) reader structure"]
impl crate::Readable for S4M0AR {}
#[doc = "`write(|w| ..)` method takes [s4m0ar::W](s4m0ar::W) writer structure"]
impl crate::Writable for S4M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s4m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s4m1ar](s4m1ar) module"]
pub type S4M1AR = crate::Reg<u32, _S4M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S4M1AR;
#[doc = "`read()` method returns [s4m1ar::R](s4m1ar::R) reader structure"]
impl crate::Readable for S4M1AR {}
#[doc = "`write(|w| ..)` method takes [s4m1ar::W](s4m1ar::W) writer structure"]
impl crate::Writable for S4M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s4m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s4fcr](s4fcr) module"]
pub type S4FCR = crate::Reg<u32, _S4FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S4FCR;
#[doc = "`read()` method returns [s4fcr::R](s4fcr::R) reader structure"]
impl crate::Readable for S4FCR {}
#[doc = "`write(|w| ..)` method takes [s4fcr::W](s4fcr::W) writer structure"]
impl crate::Writable for S4FCR {}
#[doc = "stream x FIFO control register"]
pub mod s4fcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s5cr](s5cr) module"]
pub type S5CR = crate::Reg<u32, _S5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S5CR;
#[doc = "`read()` method returns [s5cr::R](s5cr::R) reader structure"]
impl crate::Readable for S5CR {}
#[doc = "`write(|w| ..)` method takes [s5cr::W](s5cr::W) writer structure"]
impl crate::Writable for S5CR {}
#[doc = "stream x configuration register"]
pub mod s5cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s5ndtr](s5ndtr) module"]
pub type S5NDTR = crate::Reg<u32, _S5NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S5NDTR;
#[doc = "`read()` method returns [s5ndtr::R](s5ndtr::R) reader structure"]
impl crate::Readable for S5NDTR {}
#[doc = "`write(|w| ..)` method takes [s5ndtr::W](s5ndtr::W) writer structure"]
impl crate::Writable for S5NDTR {}
#[doc = "stream x number of data register"]
pub mod s5ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s5par](s5par) module"]
pub type S5PAR = crate::Reg<u32, _S5PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S5PAR;
#[doc = "`read()` method returns [s5par::R](s5par::R) reader structure"]
impl crate::Readable for S5PAR {}
#[doc = "`write(|w| ..)` method takes [s5par::W](s5par::W) writer structure"]
impl crate::Writable for S5PAR {}
#[doc = "stream x peripheral address register"]
pub mod s5par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s5m0ar](s5m0ar) module"]
pub type S5M0AR = crate::Reg<u32, _S5M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S5M0AR;
#[doc = "`read()` method returns [s5m0ar::R](s5m0ar::R) reader structure"]
impl crate::Readable for S5M0AR {}
#[doc = "`write(|w| ..)` method takes [s5m0ar::W](s5m0ar::W) writer structure"]
impl crate::Writable for S5M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s5m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s5m1ar](s5m1ar) module"]
pub type S5M1AR = crate::Reg<u32, _S5M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S5M1AR;
#[doc = "`read()` method returns [s5m1ar::R](s5m1ar::R) reader structure"]
impl crate::Readable for S5M1AR {}
#[doc = "`write(|w| ..)` method takes [s5m1ar::W](s5m1ar::W) writer structure"]
impl crate::Writable for S5M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s5m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s5fcr](s5fcr) module"]
pub type S5FCR = crate::Reg<u32, _S5FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S5FCR;
#[doc = "`read()` method returns [s5fcr::R](s5fcr::R) reader structure"]
impl crate::Readable for S5FCR {}
#[doc = "`write(|w| ..)` method takes [s5fcr::W](s5fcr::W) writer structure"]
impl crate::Writable for S5FCR {}
#[doc = "stream x FIFO control register"]
pub mod s5fcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s6cr](s6cr) module"]
pub type S6CR = crate::Reg<u32, _S6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S6CR;
#[doc = "`read()` method returns [s6cr::R](s6cr::R) reader structure"]
impl crate::Readable for S6CR {}
#[doc = "`write(|w| ..)` method takes [s6cr::W](s6cr::W) writer structure"]
impl crate::Writable for S6CR {}
#[doc = "stream x configuration register"]
pub mod s6cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s6ndtr](s6ndtr) module"]
pub type S6NDTR = crate::Reg<u32, _S6NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S6NDTR;
#[doc = "`read()` method returns [s6ndtr::R](s6ndtr::R) reader structure"]
impl crate::Readable for S6NDTR {}
#[doc = "`write(|w| ..)` method takes [s6ndtr::W](s6ndtr::W) writer structure"]
impl crate::Writable for S6NDTR {}
#[doc = "stream x number of data register"]
pub mod s6ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s6par](s6par) module"]
pub type S6PAR = crate::Reg<u32, _S6PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S6PAR;
#[doc = "`read()` method returns [s6par::R](s6par::R) reader structure"]
impl crate::Readable for S6PAR {}
#[doc = "`write(|w| ..)` method takes [s6par::W](s6par::W) writer structure"]
impl crate::Writable for S6PAR {}
#[doc = "stream x peripheral address register"]
pub mod s6par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s6m0ar](s6m0ar) module"]
pub type S6M0AR = crate::Reg<u32, _S6M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S6M0AR;
#[doc = "`read()` method returns [s6m0ar::R](s6m0ar::R) reader structure"]
impl crate::Readable for S6M0AR {}
#[doc = "`write(|w| ..)` method takes [s6m0ar::W](s6m0ar::W) writer structure"]
impl crate::Writable for S6M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s6m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s6m1ar](s6m1ar) module"]
pub type S6M1AR = crate::Reg<u32, _S6M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S6M1AR;
#[doc = "`read()` method returns [s6m1ar::R](s6m1ar::R) reader structure"]
impl crate::Readable for S6M1AR {}
#[doc = "`write(|w| ..)` method takes [s6m1ar::W](s6m1ar::W) writer structure"]
impl crate::Writable for S6M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s6m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s6fcr](s6fcr) module"]
pub type S6FCR = crate::Reg<u32, _S6FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S6FCR;
#[doc = "`read()` method returns [s6fcr::R](s6fcr::R) reader structure"]
impl crate::Readable for S6FCR {}
#[doc = "`write(|w| ..)` method takes [s6fcr::W](s6fcr::W) writer structure"]
impl crate::Writable for S6FCR {}
#[doc = "stream x FIFO control register"]
pub mod s6fcr;
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s7cr](s7cr) module"]
pub type S7CR = crate::Reg<u32, _S7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S7CR;
#[doc = "`read()` method returns [s7cr::R](s7cr::R) reader structure"]
impl crate::Readable for S7CR {}
#[doc = "`write(|w| ..)` method takes [s7cr::W](s7cr::W) writer structure"]
impl crate::Writable for S7CR {}
#[doc = "stream x configuration register"]
pub mod s7cr;
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s7ndtr](s7ndtr) module"]
pub type S7NDTR = crate::Reg<u32, _S7NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S7NDTR;
#[doc = "`read()` method returns [s7ndtr::R](s7ndtr::R) reader structure"]
impl crate::Readable for S7NDTR {}
#[doc = "`write(|w| ..)` method takes [s7ndtr::W](s7ndtr::W) writer structure"]
impl crate::Writable for S7NDTR {}
#[doc = "stream x number of data register"]
pub mod s7ndtr;
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s7par](s7par) module"]
pub type S7PAR = crate::Reg<u32, _S7PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S7PAR;
#[doc = "`read()` method returns [s7par::R](s7par::R) reader structure"]
impl crate::Readable for S7PAR {}
#[doc = "`write(|w| ..)` method takes [s7par::W](s7par::W) writer structure"]
impl crate::Writable for S7PAR {}
#[doc = "stream x peripheral address register"]
pub mod s7par;
#[doc = "stream x memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s7m0ar](s7m0ar) module"]
pub type S7M0AR = crate::Reg<u32, _S7M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S7M0AR;
#[doc = "`read()` method returns [s7m0ar::R](s7m0ar::R) reader structure"]
impl crate::Readable for S7M0AR {}
#[doc = "`write(|w| ..)` method takes [s7m0ar::W](s7m0ar::W) writer structure"]
impl crate::Writable for S7M0AR {}
#[doc = "stream x memory 0 address register"]
pub mod s7m0ar;
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s7m1ar](s7m1ar) module"]
pub type S7M1AR = crate::Reg<u32, _S7M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S7M1AR;
#[doc = "`read()` method returns [s7m1ar::R](s7m1ar::R) reader structure"]
impl crate::Readable for S7M1AR {}
#[doc = "`write(|w| ..)` method takes [s7m1ar::W](s7m1ar::W) writer structure"]
impl crate::Writable for S7M1AR {}
#[doc = "stream x memory 1 address register"]
pub mod s7m1ar;
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s7fcr](s7fcr) module"]
pub type S7FCR = crate::Reg<u32, _S7FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S7FCR;
#[doc = "`read()` method returns [s7fcr::R](s7fcr::R) reader structure"]
impl crate::Readable for S7FCR {}
#[doc = "`write(|w| ..)` method takes [s7fcr::W](s7fcr::W) writer structure"]
impl crate::Writable for S7FCR {}
#[doc = "stream x FIFO control register"]
pub mod s7fcr;
