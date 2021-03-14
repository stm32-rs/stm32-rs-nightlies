#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA low interrupt status register"]
    pub dma_lisr: DMA_LISR,
    #[doc = "0x04 - DMA high interrupt status register"]
    pub dma_hisr: DMA_HISR,
    #[doc = "0x08 - DMA low interrupt flag clear register"]
    pub dma_lifcr: DMA_LIFCR,
    #[doc = "0x0c - DMA high interrupt flag clear register"]
    pub dma_hifcr: DMA_HIFCR,
    #[doc = "0x10 - This register is used to configure the concerned stream."]
    pub dma_s0cr: DMA_S0CR,
    #[doc = "0x14 - DMA stream 0 number of data register"]
    pub dma_s0ndtr: DMA_S0NDTR,
    #[doc = "0x18 - DMA stream 0 peripheral address register"]
    pub dma_s0par: DMA_S0PAR,
    #[doc = "0x1c - DMA stream 0 memory 0 address register"]
    pub dma_s0m0ar: DMA_S0M0AR,
    #[doc = "0x20 - DMA stream 0 memory 1 address register"]
    pub dma_s0m1ar: DMA_S0M1AR,
    #[doc = "0x24 - DMA stream 0 FIFO control register"]
    pub dma_s0fcr: DMA_S0FCR,
    #[doc = "0x28 - This register is used to configure the concerned stream."]
    pub dma_s1cr: DMA_S1CR,
    #[doc = "0x2c - DMA stream 1 number of data register"]
    pub dma_s1ndtr: DMA_S1NDTR,
    #[doc = "0x30 - DMA stream 1 peripheral address register"]
    pub dma_s1par: DMA_S1PAR,
    #[doc = "0x34 - DMA stream 1 memory 0 address register"]
    pub dma_s1m0ar: DMA_S1M0AR,
    #[doc = "0x38 - DMA stream 1 memory 1 address register"]
    pub dma_s1m1ar: DMA_S1M1AR,
    #[doc = "0x3c - DMA stream 1 FIFO control register"]
    pub dma_s1fcr: DMA_S1FCR,
    #[doc = "0x40 - This register is used to configure the concerned stream."]
    pub dma_s2cr: DMA_S2CR,
    #[doc = "0x44 - DMA stream 2 number of data register"]
    pub dma_s2ndtr: DMA_S2NDTR,
    #[doc = "0x48 - DMA stream 2 peripheral address register"]
    pub dma_s2par: DMA_S2PAR,
    #[doc = "0x4c - DMA stream 2 memory 0 address register"]
    pub dma_s2m0ar: DMA_S2M0AR,
    #[doc = "0x50 - DMA stream 2 memory 1 address register"]
    pub dma_s2m1ar: DMA_S2M1AR,
    #[doc = "0x54 - DMA stream 2 FIFO control register"]
    pub dma_s2fcr: DMA_S2FCR,
    #[doc = "0x58 - This register is used to configure the concerned stream."]
    pub dma_s3cr: DMA_S3CR,
    #[doc = "0x5c - DMA stream 3 number of data register"]
    pub dma_s3ndtr: DMA_S3NDTR,
    #[doc = "0x60 - DMA stream 3 peripheral address register"]
    pub dma_s3par: DMA_S3PAR,
    #[doc = "0x64 - DMA stream 3 memory 0 address register"]
    pub dma_s3m0ar: DMA_S3M0AR,
    #[doc = "0x68 - DMA stream 3 memory 1 address register"]
    pub dma_s3m1ar: DMA_S3M1AR,
    #[doc = "0x6c - DMA stream 3 FIFO control register"]
    pub dma_s3fcr: DMA_S3FCR,
    #[doc = "0x70 - This register is used to configure the concerned stream."]
    pub dma_s4cr: DMA_S4CR,
    #[doc = "0x74 - DMA stream 4 number of data register"]
    pub dma_s4ndtr: DMA_S4NDTR,
    #[doc = "0x78 - DMA stream 4 peripheral address register"]
    pub dma_s4par: DMA_S4PAR,
    #[doc = "0x7c - DMA stream 4 memory 0 address register"]
    pub dma_s4m0ar: DMA_S4M0AR,
    #[doc = "0x80 - DMA stream 4 memory 1 address register"]
    pub dma_s4m1ar: DMA_S4M1AR,
    #[doc = "0x84 - DMA stream 4 FIFO control register"]
    pub dma_s4fcr: DMA_S4FCR,
    #[doc = "0x88 - This register is used to configure the concerned stream."]
    pub dma_s5cr: DMA_S5CR,
    #[doc = "0x8c - DMA stream 5 number of data register"]
    pub dma_s5ndtr: DMA_S5NDTR,
    #[doc = "0x90 - DMA stream 5 peripheral address register"]
    pub dma_s5par: DMA_S5PAR,
    #[doc = "0x94 - DMA stream 5 memory 0 address register"]
    pub dma_s5m0ar: DMA_S5M0AR,
    #[doc = "0x98 - DMA stream 5 memory 1 address register"]
    pub dma_s5m1ar: DMA_S5M1AR,
    #[doc = "0x9c - DMA stream 5 FIFO control register"]
    pub dma_s5fcr: DMA_S5FCR,
    #[doc = "0xa0 - This register is used to configure the concerned stream."]
    pub dma_s6cr: DMA_S6CR,
    #[doc = "0xa4 - DMA stream 6 number of data register"]
    pub dma_s6ndtr: DMA_S6NDTR,
    #[doc = "0xa8 - DMA stream 6 peripheral address register"]
    pub dma_s6par: DMA_S6PAR,
    #[doc = "0xac - DMA stream 6 memory 0 address register"]
    pub dma_s6m0ar: DMA_S6M0AR,
    #[doc = "0xb0 - DMA stream 6 memory 1 address register"]
    pub dma_s6m1ar: DMA_S6M1AR,
    #[doc = "0xb4 - DMA stream 6 FIFO control register"]
    pub dma_s6fcr: DMA_S6FCR,
    #[doc = "0xb8 - This register is used to configure the concerned stream."]
    pub dma_s7cr: DMA_S7CR,
    #[doc = "0xbc - DMA stream 7 number of data register"]
    pub dma_s7ndtr: DMA_S7NDTR,
    #[doc = "0xc0 - DMA stream 7 peripheral address register"]
    pub dma_s7par: DMA_S7PAR,
    #[doc = "0xc4 - DMA stream 7 memory 0 address register"]
    pub dma_s7m0ar: DMA_S7M0AR,
    #[doc = "0xc8 - DMA stream 7 memory 1 address register"]
    pub dma_s7m1ar: DMA_S7M1AR,
    #[doc = "0xcc - DMA stream 7 FIFO control register"]
    pub dma_s7fcr: DMA_S7FCR,
    _reserved52: [u8; 796usize],
    #[doc = "0x3ec - DMA hardware configuration 2register"]
    pub dma_hwcfgr2: DMA_HWCFGR2,
    #[doc = "0x3f0 - DMA hardware configuration 1 register"]
    pub dma_hwcfgr1: DMA_HWCFGR1,
    #[doc = "0x3f4 - This register identifies the version of the IP."]
    pub dma_verr: DMA_VERR,
    #[doc = "0x3f8 - DMA IP identification register"]
    pub dma_ipdr: DMA_IPDR,
    #[doc = "0x3fc - DMA size identification register"]
    pub dma_sidr: DMA_SIDR,
}
#[doc = "DMA low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_lisr](dma_lisr) module"]
pub type DMA_LISR = crate::Reg<u32, _DMA_LISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_LISR;
#[doc = "`read()` method returns [dma_lisr::R](dma_lisr::R) reader structure"]
impl crate::Readable for DMA_LISR {}
#[doc = "DMA low interrupt status register"]
pub mod dma_lisr;
#[doc = "DMA high interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hisr](dma_hisr) module"]
pub type DMA_HISR = crate::Reg<u32, _DMA_HISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_HISR;
#[doc = "`read()` method returns [dma_hisr::R](dma_hisr::R) reader structure"]
impl crate::Readable for DMA_HISR {}
#[doc = "DMA high interrupt status register"]
pub mod dma_hisr;
#[doc = "DMA low interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_lifcr](dma_lifcr) module"]
pub type DMA_LIFCR = crate::Reg<u32, _DMA_LIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_LIFCR;
#[doc = "`write(|w| ..)` method takes [dma_lifcr::W](dma_lifcr::W) writer structure"]
impl crate::Writable for DMA_LIFCR {}
#[doc = "DMA low interrupt flag clear register"]
pub mod dma_lifcr;
#[doc = "DMA high interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hifcr](dma_hifcr) module"]
pub type DMA_HIFCR = crate::Reg<u32, _DMA_HIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_HIFCR;
#[doc = "`write(|w| ..)` method takes [dma_hifcr::W](dma_hifcr::W) writer structure"]
impl crate::Writable for DMA_HIFCR {}
#[doc = "DMA high interrupt flag clear register"]
pub mod dma_hifcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s0cr](dma_s0cr) module"]
pub type DMA_S0CR = crate::Reg<u32, _DMA_S0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S0CR;
#[doc = "`read()` method returns [dma_s0cr::R](dma_s0cr::R) reader structure"]
impl crate::Readable for DMA_S0CR {}
#[doc = "`write(|w| ..)` method takes [dma_s0cr::W](dma_s0cr::W) writer structure"]
impl crate::Writable for DMA_S0CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s0cr;
#[doc = "DMA stream 0 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s0ndtr](dma_s0ndtr) module"]
pub type DMA_S0NDTR = crate::Reg<u32, _DMA_S0NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S0NDTR;
#[doc = "`read()` method returns [dma_s0ndtr::R](dma_s0ndtr::R) reader structure"]
impl crate::Readable for DMA_S0NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s0ndtr::W](dma_s0ndtr::W) writer structure"]
impl crate::Writable for DMA_S0NDTR {}
#[doc = "DMA stream 0 number of data register"]
pub mod dma_s0ndtr;
#[doc = "DMA stream 0 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s0par](dma_s0par) module"]
pub type DMA_S0PAR = crate::Reg<u32, _DMA_S0PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S0PAR;
#[doc = "`read()` method returns [dma_s0par::R](dma_s0par::R) reader structure"]
impl crate::Readable for DMA_S0PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s0par::W](dma_s0par::W) writer structure"]
impl crate::Writable for DMA_S0PAR {}
#[doc = "DMA stream 0 peripheral address register"]
pub mod dma_s0par;
#[doc = "DMA stream 0 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s0m0ar](dma_s0m0ar) module"]
pub type DMA_S0M0AR = crate::Reg<u32, _DMA_S0M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S0M0AR;
#[doc = "`read()` method returns [dma_s0m0ar::R](dma_s0m0ar::R) reader structure"]
impl crate::Readable for DMA_S0M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s0m0ar::W](dma_s0m0ar::W) writer structure"]
impl crate::Writable for DMA_S0M0AR {}
#[doc = "DMA stream 0 memory 0 address register"]
pub mod dma_s0m0ar;
#[doc = "DMA stream 0 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s0m1ar](dma_s0m1ar) module"]
pub type DMA_S0M1AR = crate::Reg<u32, _DMA_S0M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S0M1AR;
#[doc = "`read()` method returns [dma_s0m1ar::R](dma_s0m1ar::R) reader structure"]
impl crate::Readable for DMA_S0M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s0m1ar::W](dma_s0m1ar::W) writer structure"]
impl crate::Writable for DMA_S0M1AR {}
#[doc = "DMA stream 0 memory 1 address register"]
pub mod dma_s0m1ar;
#[doc = "DMA stream 0 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s0fcr](dma_s0fcr) module"]
pub type DMA_S0FCR = crate::Reg<u32, _DMA_S0FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S0FCR;
#[doc = "`read()` method returns [dma_s0fcr::R](dma_s0fcr::R) reader structure"]
impl crate::Readable for DMA_S0FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s0fcr::W](dma_s0fcr::W) writer structure"]
impl crate::Writable for DMA_S0FCR {}
#[doc = "DMA stream 0 FIFO control register"]
pub mod dma_s0fcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s1cr](dma_s1cr) module"]
pub type DMA_S1CR = crate::Reg<u32, _DMA_S1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S1CR;
#[doc = "`read()` method returns [dma_s1cr::R](dma_s1cr::R) reader structure"]
impl crate::Readable for DMA_S1CR {}
#[doc = "`write(|w| ..)` method takes [dma_s1cr::W](dma_s1cr::W) writer structure"]
impl crate::Writable for DMA_S1CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s1cr;
#[doc = "DMA stream 1 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s1ndtr](dma_s1ndtr) module"]
pub type DMA_S1NDTR = crate::Reg<u32, _DMA_S1NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S1NDTR;
#[doc = "`read()` method returns [dma_s1ndtr::R](dma_s1ndtr::R) reader structure"]
impl crate::Readable for DMA_S1NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s1ndtr::W](dma_s1ndtr::W) writer structure"]
impl crate::Writable for DMA_S1NDTR {}
#[doc = "DMA stream 1 number of data register"]
pub mod dma_s1ndtr;
#[doc = "DMA stream 1 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s1par](dma_s1par) module"]
pub type DMA_S1PAR = crate::Reg<u32, _DMA_S1PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S1PAR;
#[doc = "`read()` method returns [dma_s1par::R](dma_s1par::R) reader structure"]
impl crate::Readable for DMA_S1PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s1par::W](dma_s1par::W) writer structure"]
impl crate::Writable for DMA_S1PAR {}
#[doc = "DMA stream 1 peripheral address register"]
pub mod dma_s1par;
#[doc = "DMA stream 1 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s1m0ar](dma_s1m0ar) module"]
pub type DMA_S1M0AR = crate::Reg<u32, _DMA_S1M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S1M0AR;
#[doc = "`read()` method returns [dma_s1m0ar::R](dma_s1m0ar::R) reader structure"]
impl crate::Readable for DMA_S1M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s1m0ar::W](dma_s1m0ar::W) writer structure"]
impl crate::Writable for DMA_S1M0AR {}
#[doc = "DMA stream 1 memory 0 address register"]
pub mod dma_s1m0ar;
#[doc = "DMA stream 1 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s1m1ar](dma_s1m1ar) module"]
pub type DMA_S1M1AR = crate::Reg<u32, _DMA_S1M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S1M1AR;
#[doc = "`read()` method returns [dma_s1m1ar::R](dma_s1m1ar::R) reader structure"]
impl crate::Readable for DMA_S1M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s1m1ar::W](dma_s1m1ar::W) writer structure"]
impl crate::Writable for DMA_S1M1AR {}
#[doc = "DMA stream 1 memory 1 address register"]
pub mod dma_s1m1ar;
#[doc = "DMA stream 1 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s1fcr](dma_s1fcr) module"]
pub type DMA_S1FCR = crate::Reg<u32, _DMA_S1FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S1FCR;
#[doc = "`read()` method returns [dma_s1fcr::R](dma_s1fcr::R) reader structure"]
impl crate::Readable for DMA_S1FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s1fcr::W](dma_s1fcr::W) writer structure"]
impl crate::Writable for DMA_S1FCR {}
#[doc = "DMA stream 1 FIFO control register"]
pub mod dma_s1fcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2cr](dma_s2cr) module"]
pub type DMA_S2CR = crate::Reg<u32, _DMA_S2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S2CR;
#[doc = "`read()` method returns [dma_s2cr::R](dma_s2cr::R) reader structure"]
impl crate::Readable for DMA_S2CR {}
#[doc = "`write(|w| ..)` method takes [dma_s2cr::W](dma_s2cr::W) writer structure"]
impl crate::Writable for DMA_S2CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s2cr;
#[doc = "DMA stream 2 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2ndtr](dma_s2ndtr) module"]
pub type DMA_S2NDTR = crate::Reg<u32, _DMA_S2NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S2NDTR;
#[doc = "`read()` method returns [dma_s2ndtr::R](dma_s2ndtr::R) reader structure"]
impl crate::Readable for DMA_S2NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s2ndtr::W](dma_s2ndtr::W) writer structure"]
impl crate::Writable for DMA_S2NDTR {}
#[doc = "DMA stream 2 number of data register"]
pub mod dma_s2ndtr;
#[doc = "DMA stream 2 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2par](dma_s2par) module"]
pub type DMA_S2PAR = crate::Reg<u32, _DMA_S2PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S2PAR;
#[doc = "`read()` method returns [dma_s2par::R](dma_s2par::R) reader structure"]
impl crate::Readable for DMA_S2PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s2par::W](dma_s2par::W) writer structure"]
impl crate::Writable for DMA_S2PAR {}
#[doc = "DMA stream 2 peripheral address register"]
pub mod dma_s2par;
#[doc = "DMA stream 2 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2m0ar](dma_s2m0ar) module"]
pub type DMA_S2M0AR = crate::Reg<u32, _DMA_S2M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S2M0AR;
#[doc = "`read()` method returns [dma_s2m0ar::R](dma_s2m0ar::R) reader structure"]
impl crate::Readable for DMA_S2M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s2m0ar::W](dma_s2m0ar::W) writer structure"]
impl crate::Writable for DMA_S2M0AR {}
#[doc = "DMA stream 2 memory 0 address register"]
pub mod dma_s2m0ar;
#[doc = "DMA stream 2 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2m1ar](dma_s2m1ar) module"]
pub type DMA_S2M1AR = crate::Reg<u32, _DMA_S2M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S2M1AR;
#[doc = "`read()` method returns [dma_s2m1ar::R](dma_s2m1ar::R) reader structure"]
impl crate::Readable for DMA_S2M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s2m1ar::W](dma_s2m1ar::W) writer structure"]
impl crate::Writable for DMA_S2M1AR {}
#[doc = "DMA stream 2 memory 1 address register"]
pub mod dma_s2m1ar;
#[doc = "DMA stream 2 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2fcr](dma_s2fcr) module"]
pub type DMA_S2FCR = crate::Reg<u32, _DMA_S2FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S2FCR;
#[doc = "`read()` method returns [dma_s2fcr::R](dma_s2fcr::R) reader structure"]
impl crate::Readable for DMA_S2FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s2fcr::W](dma_s2fcr::W) writer structure"]
impl crate::Writable for DMA_S2FCR {}
#[doc = "DMA stream 2 FIFO control register"]
pub mod dma_s2fcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s3cr](dma_s3cr) module"]
pub type DMA_S3CR = crate::Reg<u32, _DMA_S3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S3CR;
#[doc = "`read()` method returns [dma_s3cr::R](dma_s3cr::R) reader structure"]
impl crate::Readable for DMA_S3CR {}
#[doc = "`write(|w| ..)` method takes [dma_s3cr::W](dma_s3cr::W) writer structure"]
impl crate::Writable for DMA_S3CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s3cr;
#[doc = "DMA stream 3 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s3ndtr](dma_s3ndtr) module"]
pub type DMA_S3NDTR = crate::Reg<u32, _DMA_S3NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S3NDTR;
#[doc = "`read()` method returns [dma_s3ndtr::R](dma_s3ndtr::R) reader structure"]
impl crate::Readable for DMA_S3NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s3ndtr::W](dma_s3ndtr::W) writer structure"]
impl crate::Writable for DMA_S3NDTR {}
#[doc = "DMA stream 3 number of data register"]
pub mod dma_s3ndtr;
#[doc = "DMA stream 3 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s3par](dma_s3par) module"]
pub type DMA_S3PAR = crate::Reg<u32, _DMA_S3PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S3PAR;
#[doc = "`read()` method returns [dma_s3par::R](dma_s3par::R) reader structure"]
impl crate::Readable for DMA_S3PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s3par::W](dma_s3par::W) writer structure"]
impl crate::Writable for DMA_S3PAR {}
#[doc = "DMA stream 3 peripheral address register"]
pub mod dma_s3par;
#[doc = "DMA stream 3 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s3m0ar](dma_s3m0ar) module"]
pub type DMA_S3M0AR = crate::Reg<u32, _DMA_S3M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S3M0AR;
#[doc = "`read()` method returns [dma_s3m0ar::R](dma_s3m0ar::R) reader structure"]
impl crate::Readable for DMA_S3M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s3m0ar::W](dma_s3m0ar::W) writer structure"]
impl crate::Writable for DMA_S3M0AR {}
#[doc = "DMA stream 3 memory 0 address register"]
pub mod dma_s3m0ar;
#[doc = "DMA stream 3 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s3m1ar](dma_s3m1ar) module"]
pub type DMA_S3M1AR = crate::Reg<u32, _DMA_S3M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S3M1AR;
#[doc = "`read()` method returns [dma_s3m1ar::R](dma_s3m1ar::R) reader structure"]
impl crate::Readable for DMA_S3M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s3m1ar::W](dma_s3m1ar::W) writer structure"]
impl crate::Writable for DMA_S3M1AR {}
#[doc = "DMA stream 3 memory 1 address register"]
pub mod dma_s3m1ar;
#[doc = "DMA stream 3 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s3fcr](dma_s3fcr) module"]
pub type DMA_S3FCR = crate::Reg<u32, _DMA_S3FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S3FCR;
#[doc = "`read()` method returns [dma_s3fcr::R](dma_s3fcr::R) reader structure"]
impl crate::Readable for DMA_S3FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s3fcr::W](dma_s3fcr::W) writer structure"]
impl crate::Writable for DMA_S3FCR {}
#[doc = "DMA stream 3 FIFO control register"]
pub mod dma_s3fcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s4cr](dma_s4cr) module"]
pub type DMA_S4CR = crate::Reg<u32, _DMA_S4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S4CR;
#[doc = "`read()` method returns [dma_s4cr::R](dma_s4cr::R) reader structure"]
impl crate::Readable for DMA_S4CR {}
#[doc = "`write(|w| ..)` method takes [dma_s4cr::W](dma_s4cr::W) writer structure"]
impl crate::Writable for DMA_S4CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s4cr;
#[doc = "DMA stream 4 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s4ndtr](dma_s4ndtr) module"]
pub type DMA_S4NDTR = crate::Reg<u32, _DMA_S4NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S4NDTR;
#[doc = "`read()` method returns [dma_s4ndtr::R](dma_s4ndtr::R) reader structure"]
impl crate::Readable for DMA_S4NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s4ndtr::W](dma_s4ndtr::W) writer structure"]
impl crate::Writable for DMA_S4NDTR {}
#[doc = "DMA stream 4 number of data register"]
pub mod dma_s4ndtr;
#[doc = "DMA stream 4 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s4par](dma_s4par) module"]
pub type DMA_S4PAR = crate::Reg<u32, _DMA_S4PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S4PAR;
#[doc = "`read()` method returns [dma_s4par::R](dma_s4par::R) reader structure"]
impl crate::Readable for DMA_S4PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s4par::W](dma_s4par::W) writer structure"]
impl crate::Writable for DMA_S4PAR {}
#[doc = "DMA stream 4 peripheral address register"]
pub mod dma_s4par;
#[doc = "DMA stream 4 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s4m0ar](dma_s4m0ar) module"]
pub type DMA_S4M0AR = crate::Reg<u32, _DMA_S4M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S4M0AR;
#[doc = "`read()` method returns [dma_s4m0ar::R](dma_s4m0ar::R) reader structure"]
impl crate::Readable for DMA_S4M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s4m0ar::W](dma_s4m0ar::W) writer structure"]
impl crate::Writable for DMA_S4M0AR {}
#[doc = "DMA stream 4 memory 0 address register"]
pub mod dma_s4m0ar;
#[doc = "DMA stream 4 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s4m1ar](dma_s4m1ar) module"]
pub type DMA_S4M1AR = crate::Reg<u32, _DMA_S4M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S4M1AR;
#[doc = "`read()` method returns [dma_s4m1ar::R](dma_s4m1ar::R) reader structure"]
impl crate::Readable for DMA_S4M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s4m1ar::W](dma_s4m1ar::W) writer structure"]
impl crate::Writable for DMA_S4M1AR {}
#[doc = "DMA stream 4 memory 1 address register"]
pub mod dma_s4m1ar;
#[doc = "DMA stream 4 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s4fcr](dma_s4fcr) module"]
pub type DMA_S4FCR = crate::Reg<u32, _DMA_S4FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S4FCR;
#[doc = "`read()` method returns [dma_s4fcr::R](dma_s4fcr::R) reader structure"]
impl crate::Readable for DMA_S4FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s4fcr::W](dma_s4fcr::W) writer structure"]
impl crate::Writable for DMA_S4FCR {}
#[doc = "DMA stream 4 FIFO control register"]
pub mod dma_s4fcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s5cr](dma_s5cr) module"]
pub type DMA_S5CR = crate::Reg<u32, _DMA_S5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S5CR;
#[doc = "`read()` method returns [dma_s5cr::R](dma_s5cr::R) reader structure"]
impl crate::Readable for DMA_S5CR {}
#[doc = "`write(|w| ..)` method takes [dma_s5cr::W](dma_s5cr::W) writer structure"]
impl crate::Writable for DMA_S5CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s5cr;
#[doc = "DMA stream 5 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s5ndtr](dma_s5ndtr) module"]
pub type DMA_S5NDTR = crate::Reg<u32, _DMA_S5NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S5NDTR;
#[doc = "`read()` method returns [dma_s5ndtr::R](dma_s5ndtr::R) reader structure"]
impl crate::Readable for DMA_S5NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s5ndtr::W](dma_s5ndtr::W) writer structure"]
impl crate::Writable for DMA_S5NDTR {}
#[doc = "DMA stream 5 number of data register"]
pub mod dma_s5ndtr;
#[doc = "DMA stream 5 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s5par](dma_s5par) module"]
pub type DMA_S5PAR = crate::Reg<u32, _DMA_S5PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S5PAR;
#[doc = "`read()` method returns [dma_s5par::R](dma_s5par::R) reader structure"]
impl crate::Readable for DMA_S5PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s5par::W](dma_s5par::W) writer structure"]
impl crate::Writable for DMA_S5PAR {}
#[doc = "DMA stream 5 peripheral address register"]
pub mod dma_s5par;
#[doc = "DMA stream 5 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s5m0ar](dma_s5m0ar) module"]
pub type DMA_S5M0AR = crate::Reg<u32, _DMA_S5M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S5M0AR;
#[doc = "`read()` method returns [dma_s5m0ar::R](dma_s5m0ar::R) reader structure"]
impl crate::Readable for DMA_S5M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s5m0ar::W](dma_s5m0ar::W) writer structure"]
impl crate::Writable for DMA_S5M0AR {}
#[doc = "DMA stream 5 memory 0 address register"]
pub mod dma_s5m0ar;
#[doc = "DMA stream 5 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s5m1ar](dma_s5m1ar) module"]
pub type DMA_S5M1AR = crate::Reg<u32, _DMA_S5M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S5M1AR;
#[doc = "`read()` method returns [dma_s5m1ar::R](dma_s5m1ar::R) reader structure"]
impl crate::Readable for DMA_S5M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s5m1ar::W](dma_s5m1ar::W) writer structure"]
impl crate::Writable for DMA_S5M1AR {}
#[doc = "DMA stream 5 memory 1 address register"]
pub mod dma_s5m1ar;
#[doc = "DMA stream 5 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s5fcr](dma_s5fcr) module"]
pub type DMA_S5FCR = crate::Reg<u32, _DMA_S5FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S5FCR;
#[doc = "`read()` method returns [dma_s5fcr::R](dma_s5fcr::R) reader structure"]
impl crate::Readable for DMA_S5FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s5fcr::W](dma_s5fcr::W) writer structure"]
impl crate::Writable for DMA_S5FCR {}
#[doc = "DMA stream 5 FIFO control register"]
pub mod dma_s5fcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s6cr](dma_s6cr) module"]
pub type DMA_S6CR = crate::Reg<u32, _DMA_S6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S6CR;
#[doc = "`read()` method returns [dma_s6cr::R](dma_s6cr::R) reader structure"]
impl crate::Readable for DMA_S6CR {}
#[doc = "`write(|w| ..)` method takes [dma_s6cr::W](dma_s6cr::W) writer structure"]
impl crate::Writable for DMA_S6CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s6cr;
#[doc = "DMA stream 6 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s6ndtr](dma_s6ndtr) module"]
pub type DMA_S6NDTR = crate::Reg<u32, _DMA_S6NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S6NDTR;
#[doc = "`read()` method returns [dma_s6ndtr::R](dma_s6ndtr::R) reader structure"]
impl crate::Readable for DMA_S6NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s6ndtr::W](dma_s6ndtr::W) writer structure"]
impl crate::Writable for DMA_S6NDTR {}
#[doc = "DMA stream 6 number of data register"]
pub mod dma_s6ndtr;
#[doc = "DMA stream 6 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s6par](dma_s6par) module"]
pub type DMA_S6PAR = crate::Reg<u32, _DMA_S6PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S6PAR;
#[doc = "`read()` method returns [dma_s6par::R](dma_s6par::R) reader structure"]
impl crate::Readable for DMA_S6PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s6par::W](dma_s6par::W) writer structure"]
impl crate::Writable for DMA_S6PAR {}
#[doc = "DMA stream 6 peripheral address register"]
pub mod dma_s6par;
#[doc = "DMA stream 6 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s6m0ar](dma_s6m0ar) module"]
pub type DMA_S6M0AR = crate::Reg<u32, _DMA_S6M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S6M0AR;
#[doc = "`read()` method returns [dma_s6m0ar::R](dma_s6m0ar::R) reader structure"]
impl crate::Readable for DMA_S6M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s6m0ar::W](dma_s6m0ar::W) writer structure"]
impl crate::Writable for DMA_S6M0AR {}
#[doc = "DMA stream 6 memory 0 address register"]
pub mod dma_s6m0ar;
#[doc = "DMA stream 6 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s6m1ar](dma_s6m1ar) module"]
pub type DMA_S6M1AR = crate::Reg<u32, _DMA_S6M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S6M1AR;
#[doc = "`read()` method returns [dma_s6m1ar::R](dma_s6m1ar::R) reader structure"]
impl crate::Readable for DMA_S6M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s6m1ar::W](dma_s6m1ar::W) writer structure"]
impl crate::Writable for DMA_S6M1AR {}
#[doc = "DMA stream 6 memory 1 address register"]
pub mod dma_s6m1ar;
#[doc = "DMA stream 6 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s6fcr](dma_s6fcr) module"]
pub type DMA_S6FCR = crate::Reg<u32, _DMA_S6FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S6FCR;
#[doc = "`read()` method returns [dma_s6fcr::R](dma_s6fcr::R) reader structure"]
impl crate::Readable for DMA_S6FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s6fcr::W](dma_s6fcr::W) writer structure"]
impl crate::Writable for DMA_S6FCR {}
#[doc = "DMA stream 6 FIFO control register"]
pub mod dma_s6fcr;
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s7cr](dma_s7cr) module"]
pub type DMA_S7CR = crate::Reg<u32, _DMA_S7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S7CR;
#[doc = "`read()` method returns [dma_s7cr::R](dma_s7cr::R) reader structure"]
impl crate::Readable for DMA_S7CR {}
#[doc = "`write(|w| ..)` method takes [dma_s7cr::W](dma_s7cr::W) writer structure"]
impl crate::Writable for DMA_S7CR {}
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s7cr;
#[doc = "DMA stream 7 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s7ndtr](dma_s7ndtr) module"]
pub type DMA_S7NDTR = crate::Reg<u32, _DMA_S7NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S7NDTR;
#[doc = "`read()` method returns [dma_s7ndtr::R](dma_s7ndtr::R) reader structure"]
impl crate::Readable for DMA_S7NDTR {}
#[doc = "`write(|w| ..)` method takes [dma_s7ndtr::W](dma_s7ndtr::W) writer structure"]
impl crate::Writable for DMA_S7NDTR {}
#[doc = "DMA stream 7 number of data register"]
pub mod dma_s7ndtr;
#[doc = "DMA stream 7 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s7par](dma_s7par) module"]
pub type DMA_S7PAR = crate::Reg<u32, _DMA_S7PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S7PAR;
#[doc = "`read()` method returns [dma_s7par::R](dma_s7par::R) reader structure"]
impl crate::Readable for DMA_S7PAR {}
#[doc = "`write(|w| ..)` method takes [dma_s7par::W](dma_s7par::W) writer structure"]
impl crate::Writable for DMA_S7PAR {}
#[doc = "DMA stream 7 peripheral address register"]
pub mod dma_s7par;
#[doc = "DMA stream 7 memory 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s7m0ar](dma_s7m0ar) module"]
pub type DMA_S7M0AR = crate::Reg<u32, _DMA_S7M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S7M0AR;
#[doc = "`read()` method returns [dma_s7m0ar::R](dma_s7m0ar::R) reader structure"]
impl crate::Readable for DMA_S7M0AR {}
#[doc = "`write(|w| ..)` method takes [dma_s7m0ar::W](dma_s7m0ar::W) writer structure"]
impl crate::Writable for DMA_S7M0AR {}
#[doc = "DMA stream 7 memory 0 address register"]
pub mod dma_s7m0ar;
#[doc = "DMA stream 7 memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s7m1ar](dma_s7m1ar) module"]
pub type DMA_S7M1AR = crate::Reg<u32, _DMA_S7M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S7M1AR;
#[doc = "`read()` method returns [dma_s7m1ar::R](dma_s7m1ar::R) reader structure"]
impl crate::Readable for DMA_S7M1AR {}
#[doc = "`write(|w| ..)` method takes [dma_s7m1ar::W](dma_s7m1ar::W) writer structure"]
impl crate::Writable for DMA_S7M1AR {}
#[doc = "DMA stream 7 memory 1 address register"]
pub mod dma_s7m1ar;
#[doc = "DMA stream 7 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s7fcr](dma_s7fcr) module"]
pub type DMA_S7FCR = crate::Reg<u32, _DMA_S7FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_S7FCR;
#[doc = "`read()` method returns [dma_s7fcr::R](dma_s7fcr::R) reader structure"]
impl crate::Readable for DMA_S7FCR {}
#[doc = "`write(|w| ..)` method takes [dma_s7fcr::W](dma_s7fcr::W) writer structure"]
impl crate::Writable for DMA_S7FCR {}
#[doc = "DMA stream 7 FIFO control register"]
pub mod dma_s7fcr;
#[doc = "DMA hardware configuration 2register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hwcfgr2](dma_hwcfgr2) module"]
pub type DMA_HWCFGR2 = crate::Reg<u32, _DMA_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_HWCFGR2;
#[doc = "`read()` method returns [dma_hwcfgr2::R](dma_hwcfgr2::R) reader structure"]
impl crate::Readable for DMA_HWCFGR2 {}
#[doc = "DMA hardware configuration 2register"]
pub mod dma_hwcfgr2;
#[doc = "DMA hardware configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hwcfgr1](dma_hwcfgr1) module"]
pub type DMA_HWCFGR1 = crate::Reg<u32, _DMA_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_HWCFGR1;
#[doc = "`read()` method returns [dma_hwcfgr1::R](dma_hwcfgr1::R) reader structure"]
impl crate::Readable for DMA_HWCFGR1 {}
#[doc = "DMA hardware configuration 1 register"]
pub mod dma_hwcfgr1;
#[doc = "This register identifies the version of the IP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_verr](dma_verr) module"]
pub type DMA_VERR = crate::Reg<u32, _DMA_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_VERR;
#[doc = "`read()` method returns [dma_verr::R](dma_verr::R) reader structure"]
impl crate::Readable for DMA_VERR {}
#[doc = "This register identifies the version of the IP."]
pub mod dma_verr;
#[doc = "DMA IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ipdr](dma_ipdr) module"]
pub type DMA_IPDR = crate::Reg<u32, _DMA_IPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_IPDR;
#[doc = "`read()` method returns [dma_ipdr::R](dma_ipdr::R) reader structure"]
impl crate::Readable for DMA_IPDR {}
#[doc = "DMA IP identification register"]
pub mod dma_ipdr;
#[doc = "DMA size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sidr](dma_sidr) module"]
pub type DMA_SIDR = crate::Reg<u32, _DMA_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SIDR;
#[doc = "`read()` method returns [dma_sidr::R](dma_sidr::R) reader structure"]
impl crate::Readable for DMA_SIDR {}
#[doc = "DMA size identification register"]
pub mod dma_sidr;
