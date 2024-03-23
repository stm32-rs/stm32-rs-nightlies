#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_lisr: DMA_LISR,
    dma_hisr: DMA_HISR,
    dma_lifcr: DMA_LIFCR,
    dma_hifcr: DMA_HIFCR,
    dma_s0cr: DMA_S0CR,
    dma_s0ndtr: DMA_S0NDTR,
    dma_s0par: DMA_S0PAR,
    dma_s0m0ar: DMA_S0M0AR,
    dma_s0m1ar: DMA_S0M1AR,
    dma_s0fcr: DMA_S0FCR,
    dma_s1cr: DMA_S1CR,
    dma_s1ndtr: DMA_S1NDTR,
    dma_s1par: DMA_S1PAR,
    dma_s1m0ar: DMA_S1M0AR,
    dma_s1m1ar: DMA_S1M1AR,
    dma_s1fcr: DMA_S1FCR,
    dma_s2cr: DMA_S2CR,
    dma_s2ndtr: DMA_S2NDTR,
    dma_s2par: DMA_S2PAR,
    dma_s2m0ar: DMA_S2M0AR,
    dma_s2m1ar: DMA_S2M1AR,
    dma_s2fcr: DMA_S2FCR,
    dma_s3cr: DMA_S3CR,
    dma_s3ndtr: DMA_S3NDTR,
    dma_s3par: DMA_S3PAR,
    dma_s3m0ar: DMA_S3M0AR,
    dma_s3m1ar: DMA_S3M1AR,
    dma_s3fcr: DMA_S3FCR,
    dma_s4cr: DMA_S4CR,
    dma_s4ndtr: DMA_S4NDTR,
    dma_s4par: DMA_S4PAR,
    dma_s4m0ar: DMA_S4M0AR,
    dma_s4m1ar: DMA_S4M1AR,
    dma_s4fcr: DMA_S4FCR,
    dma_s5cr: DMA_S5CR,
    dma_s5ndtr: DMA_S5NDTR,
    dma_s5par: DMA_S5PAR,
    dma_s5m0ar: DMA_S5M0AR,
    dma_s5m1ar: DMA_S5M1AR,
    dma_s5fcr: DMA_S5FCR,
    dma_s6cr: DMA_S6CR,
    dma_s6ndtr: DMA_S6NDTR,
    dma_s6par: DMA_S6PAR,
    dma_s6m0ar: DMA_S6M0AR,
    dma_s6m1ar: DMA_S6M1AR,
    dma_s6fcr: DMA_S6FCR,
    dma_s7cr: DMA_S7CR,
    dma_s7ndtr: DMA_S7NDTR,
    dma_s7par: DMA_S7PAR,
    dma_s7m0ar: DMA_S7M0AR,
    dma_s7m1ar: DMA_S7M1AR,
    dma_s7fcr: DMA_S7FCR,
    _reserved52: [u8; 0x031c],
    dma_hwcfgr2: DMA_HWCFGR2,
    dma_hwcfgr1: DMA_HWCFGR1,
    dma_verr: DMA_VERR,
    dma_ipdr: DMA_IPDR,
    dma_sidr: DMA_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA low interrupt status register"]
    #[inline(always)]
    pub const fn dma_lisr(&self) -> &DMA_LISR {
        &self.dma_lisr
    }
    #[doc = "0x04 - DMA high interrupt status register"]
    #[inline(always)]
    pub const fn dma_hisr(&self) -> &DMA_HISR {
        &self.dma_hisr
    }
    #[doc = "0x08 - DMA low interrupt flag clear register"]
    #[inline(always)]
    pub const fn dma_lifcr(&self) -> &DMA_LIFCR {
        &self.dma_lifcr
    }
    #[doc = "0x0c - DMA high interrupt flag clear register"]
    #[inline(always)]
    pub const fn dma_hifcr(&self) -> &DMA_HIFCR {
        &self.dma_hifcr
    }
    #[doc = "0x10 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s0cr(&self) -> &DMA_S0CR {
        &self.dma_s0cr
    }
    #[doc = "0x14 - DMA stream 0 number of data register"]
    #[inline(always)]
    pub const fn dma_s0ndtr(&self) -> &DMA_S0NDTR {
        &self.dma_s0ndtr
    }
    #[doc = "0x18 - DMA stream 0 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s0par(&self) -> &DMA_S0PAR {
        &self.dma_s0par
    }
    #[doc = "0x1c - DMA stream 0 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s0m0ar(&self) -> &DMA_S0M0AR {
        &self.dma_s0m0ar
    }
    #[doc = "0x20 - DMA stream 0 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s0m1ar(&self) -> &DMA_S0M1AR {
        &self.dma_s0m1ar
    }
    #[doc = "0x24 - DMA stream 0 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s0fcr(&self) -> &DMA_S0FCR {
        &self.dma_s0fcr
    }
    #[doc = "0x28 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s1cr(&self) -> &DMA_S1CR {
        &self.dma_s1cr
    }
    #[doc = "0x2c - DMA stream 1 number of data register"]
    #[inline(always)]
    pub const fn dma_s1ndtr(&self) -> &DMA_S1NDTR {
        &self.dma_s1ndtr
    }
    #[doc = "0x30 - DMA stream 1 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s1par(&self) -> &DMA_S1PAR {
        &self.dma_s1par
    }
    #[doc = "0x34 - DMA stream 1 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s1m0ar(&self) -> &DMA_S1M0AR {
        &self.dma_s1m0ar
    }
    #[doc = "0x38 - DMA stream 1 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s1m1ar(&self) -> &DMA_S1M1AR {
        &self.dma_s1m1ar
    }
    #[doc = "0x3c - DMA stream 1 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s1fcr(&self) -> &DMA_S1FCR {
        &self.dma_s1fcr
    }
    #[doc = "0x40 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s2cr(&self) -> &DMA_S2CR {
        &self.dma_s2cr
    }
    #[doc = "0x44 - DMA stream 2 number of data register"]
    #[inline(always)]
    pub const fn dma_s2ndtr(&self) -> &DMA_S2NDTR {
        &self.dma_s2ndtr
    }
    #[doc = "0x48 - DMA stream 2 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s2par(&self) -> &DMA_S2PAR {
        &self.dma_s2par
    }
    #[doc = "0x4c - DMA stream 2 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s2m0ar(&self) -> &DMA_S2M0AR {
        &self.dma_s2m0ar
    }
    #[doc = "0x50 - DMA stream 2 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s2m1ar(&self) -> &DMA_S2M1AR {
        &self.dma_s2m1ar
    }
    #[doc = "0x54 - DMA stream 2 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s2fcr(&self) -> &DMA_S2FCR {
        &self.dma_s2fcr
    }
    #[doc = "0x58 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s3cr(&self) -> &DMA_S3CR {
        &self.dma_s3cr
    }
    #[doc = "0x5c - DMA stream 3 number of data register"]
    #[inline(always)]
    pub const fn dma_s3ndtr(&self) -> &DMA_S3NDTR {
        &self.dma_s3ndtr
    }
    #[doc = "0x60 - DMA stream 3 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s3par(&self) -> &DMA_S3PAR {
        &self.dma_s3par
    }
    #[doc = "0x64 - DMA stream 3 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s3m0ar(&self) -> &DMA_S3M0AR {
        &self.dma_s3m0ar
    }
    #[doc = "0x68 - DMA stream 3 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s3m1ar(&self) -> &DMA_S3M1AR {
        &self.dma_s3m1ar
    }
    #[doc = "0x6c - DMA stream 3 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s3fcr(&self) -> &DMA_S3FCR {
        &self.dma_s3fcr
    }
    #[doc = "0x70 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s4cr(&self) -> &DMA_S4CR {
        &self.dma_s4cr
    }
    #[doc = "0x74 - DMA stream 4 number of data register"]
    #[inline(always)]
    pub const fn dma_s4ndtr(&self) -> &DMA_S4NDTR {
        &self.dma_s4ndtr
    }
    #[doc = "0x78 - DMA stream 4 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s4par(&self) -> &DMA_S4PAR {
        &self.dma_s4par
    }
    #[doc = "0x7c - DMA stream 4 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s4m0ar(&self) -> &DMA_S4M0AR {
        &self.dma_s4m0ar
    }
    #[doc = "0x80 - DMA stream 4 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s4m1ar(&self) -> &DMA_S4M1AR {
        &self.dma_s4m1ar
    }
    #[doc = "0x84 - DMA stream 4 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s4fcr(&self) -> &DMA_S4FCR {
        &self.dma_s4fcr
    }
    #[doc = "0x88 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s5cr(&self) -> &DMA_S5CR {
        &self.dma_s5cr
    }
    #[doc = "0x8c - DMA stream 5 number of data register"]
    #[inline(always)]
    pub const fn dma_s5ndtr(&self) -> &DMA_S5NDTR {
        &self.dma_s5ndtr
    }
    #[doc = "0x90 - DMA stream 5 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s5par(&self) -> &DMA_S5PAR {
        &self.dma_s5par
    }
    #[doc = "0x94 - DMA stream 5 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s5m0ar(&self) -> &DMA_S5M0AR {
        &self.dma_s5m0ar
    }
    #[doc = "0x98 - DMA stream 5 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s5m1ar(&self) -> &DMA_S5M1AR {
        &self.dma_s5m1ar
    }
    #[doc = "0x9c - DMA stream 5 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s5fcr(&self) -> &DMA_S5FCR {
        &self.dma_s5fcr
    }
    #[doc = "0xa0 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s6cr(&self) -> &DMA_S6CR {
        &self.dma_s6cr
    }
    #[doc = "0xa4 - DMA stream 6 number of data register"]
    #[inline(always)]
    pub const fn dma_s6ndtr(&self) -> &DMA_S6NDTR {
        &self.dma_s6ndtr
    }
    #[doc = "0xa8 - DMA stream 6 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s6par(&self) -> &DMA_S6PAR {
        &self.dma_s6par
    }
    #[doc = "0xac - DMA stream 6 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s6m0ar(&self) -> &DMA_S6M0AR {
        &self.dma_s6m0ar
    }
    #[doc = "0xb0 - DMA stream 6 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s6m1ar(&self) -> &DMA_S6M1AR {
        &self.dma_s6m1ar
    }
    #[doc = "0xb4 - DMA stream 6 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s6fcr(&self) -> &DMA_S6FCR {
        &self.dma_s6fcr
    }
    #[doc = "0xb8 - This register is used to configure the concerned stream."]
    #[inline(always)]
    pub const fn dma_s7cr(&self) -> &DMA_S7CR {
        &self.dma_s7cr
    }
    #[doc = "0xbc - DMA stream 7 number of data register"]
    #[inline(always)]
    pub const fn dma_s7ndtr(&self) -> &DMA_S7NDTR {
        &self.dma_s7ndtr
    }
    #[doc = "0xc0 - DMA stream 7 peripheral address register"]
    #[inline(always)]
    pub const fn dma_s7par(&self) -> &DMA_S7PAR {
        &self.dma_s7par
    }
    #[doc = "0xc4 - DMA stream 7 memory 0 address register"]
    #[inline(always)]
    pub const fn dma_s7m0ar(&self) -> &DMA_S7M0AR {
        &self.dma_s7m0ar
    }
    #[doc = "0xc8 - DMA stream 7 memory 1 address register"]
    #[inline(always)]
    pub const fn dma_s7m1ar(&self) -> &DMA_S7M1AR {
        &self.dma_s7m1ar
    }
    #[doc = "0xcc - DMA stream 7 FIFO control register"]
    #[inline(always)]
    pub const fn dma_s7fcr(&self) -> &DMA_S7FCR {
        &self.dma_s7fcr
    }
    #[doc = "0x3ec - DMA hardware configuration 2register"]
    #[inline(always)]
    pub const fn dma_hwcfgr2(&self) -> &DMA_HWCFGR2 {
        &self.dma_hwcfgr2
    }
    #[doc = "0x3f0 - DMA hardware configuration 1 register"]
    #[inline(always)]
    pub const fn dma_hwcfgr1(&self) -> &DMA_HWCFGR1 {
        &self.dma_hwcfgr1
    }
    #[doc = "0x3f4 - This register identifies the version of the IP."]
    #[inline(always)]
    pub const fn dma_verr(&self) -> &DMA_VERR {
        &self.dma_verr
    }
    #[doc = "0x3f8 - DMA IP identification register"]
    #[inline(always)]
    pub const fn dma_ipdr(&self) -> &DMA_IPDR {
        &self.dma_ipdr
    }
    #[doc = "0x3fc - DMA size identification register"]
    #[inline(always)]
    pub const fn dma_sidr(&self) -> &DMA_SIDR {
        &self.dma_sidr
    }
}
#[doc = "DMA_LISR (r) register accessor: DMA low interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_lisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_lisr`]
module"]
pub type DMA_LISR = crate::Reg<dma_lisr::DMA_LISRrs>;
#[doc = "DMA low interrupt status register"]
pub mod dma_lisr;
#[doc = "DMA_HISR (r) register accessor: DMA high interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_hisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_hisr`]
module"]
pub type DMA_HISR = crate::Reg<dma_hisr::DMA_HISRrs>;
#[doc = "DMA high interrupt status register"]
pub mod dma_hisr;
#[doc = "DMA_LIFCR (w) register accessor: DMA low interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_lifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_lifcr`]
module"]
pub type DMA_LIFCR = crate::Reg<dma_lifcr::DMA_LIFCRrs>;
#[doc = "DMA low interrupt flag clear register"]
pub mod dma_lifcr;
#[doc = "DMA_HIFCR (w) register accessor: DMA high interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_hifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_hifcr`]
module"]
pub type DMA_HIFCR = crate::Reg<dma_hifcr::DMA_HIFCRrs>;
#[doc = "DMA high interrupt flag clear register"]
pub mod dma_hifcr;
#[doc = "DMA_S0CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s0cr`]
module"]
pub type DMA_S0CR = crate::Reg<dma_s0cr::DMA_S0CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s0cr;
#[doc = "DMA_S0NDTR (rw) register accessor: DMA stream 0 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s0ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s0ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s0ndtr`]
module"]
pub type DMA_S0NDTR = crate::Reg<dma_s0ndtr::DMA_S0NDTRrs>;
#[doc = "DMA stream 0 number of data register"]
pub mod dma_s0ndtr;
#[doc = "DMA_S0PAR (rw) register accessor: DMA stream 0 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s0par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s0par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s0par`]
module"]
pub type DMA_S0PAR = crate::Reg<dma_s0par::DMA_S0PARrs>;
#[doc = "DMA stream 0 peripheral address register"]
pub mod dma_s0par;
#[doc = "DMA_S0M0AR (rw) register accessor: DMA stream 0 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s0m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s0m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s0m0ar`]
module"]
pub type DMA_S0M0AR = crate::Reg<dma_s0m0ar::DMA_S0M0ARrs>;
#[doc = "DMA stream 0 memory 0 address register"]
pub mod dma_s0m0ar;
#[doc = "DMA_S0M1AR (rw) register accessor: DMA stream 0 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s0m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s0m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s0m1ar`]
module"]
pub type DMA_S0M1AR = crate::Reg<dma_s0m1ar::DMA_S0M1ARrs>;
#[doc = "DMA stream 0 memory 1 address register"]
pub mod dma_s0m1ar;
#[doc = "DMA_S0FCR (rw) register accessor: DMA stream 0 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s0fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s0fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s0fcr`]
module"]
pub type DMA_S0FCR = crate::Reg<dma_s0fcr::DMA_S0FCRrs>;
#[doc = "DMA stream 0 FIFO control register"]
pub mod dma_s0fcr;
#[doc = "DMA_S1CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s1cr`]
module"]
pub type DMA_S1CR = crate::Reg<dma_s1cr::DMA_S1CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s1cr;
#[doc = "DMA_S1NDTR (rw) register accessor: DMA stream 1 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s1ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s1ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s1ndtr`]
module"]
pub type DMA_S1NDTR = crate::Reg<dma_s1ndtr::DMA_S1NDTRrs>;
#[doc = "DMA stream 1 number of data register"]
pub mod dma_s1ndtr;
#[doc = "DMA_S1PAR (rw) register accessor: DMA stream 1 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s1par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s1par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s1par`]
module"]
pub type DMA_S1PAR = crate::Reg<dma_s1par::DMA_S1PARrs>;
#[doc = "DMA stream 1 peripheral address register"]
pub mod dma_s1par;
#[doc = "DMA_S1M0AR (rw) register accessor: DMA stream 1 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s1m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s1m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s1m0ar`]
module"]
pub type DMA_S1M0AR = crate::Reg<dma_s1m0ar::DMA_S1M0ARrs>;
#[doc = "DMA stream 1 memory 0 address register"]
pub mod dma_s1m0ar;
#[doc = "DMA_S1M1AR (rw) register accessor: DMA stream 1 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s1m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s1m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s1m1ar`]
module"]
pub type DMA_S1M1AR = crate::Reg<dma_s1m1ar::DMA_S1M1ARrs>;
#[doc = "DMA stream 1 memory 1 address register"]
pub mod dma_s1m1ar;
#[doc = "DMA_S1FCR (rw) register accessor: DMA stream 1 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s1fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s1fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s1fcr`]
module"]
pub type DMA_S1FCR = crate::Reg<dma_s1fcr::DMA_S1FCRrs>;
#[doc = "DMA stream 1 FIFO control register"]
pub mod dma_s1fcr;
#[doc = "DMA_S2CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s2cr`]
module"]
pub type DMA_S2CR = crate::Reg<dma_s2cr::DMA_S2CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s2cr;
#[doc = "DMA_S2NDTR (rw) register accessor: DMA stream 2 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s2ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s2ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s2ndtr`]
module"]
pub type DMA_S2NDTR = crate::Reg<dma_s2ndtr::DMA_S2NDTRrs>;
#[doc = "DMA stream 2 number of data register"]
pub mod dma_s2ndtr;
#[doc = "DMA_S2PAR (rw) register accessor: DMA stream 2 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s2par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s2par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s2par`]
module"]
pub type DMA_S2PAR = crate::Reg<dma_s2par::DMA_S2PARrs>;
#[doc = "DMA stream 2 peripheral address register"]
pub mod dma_s2par;
#[doc = "DMA_S2M0AR (rw) register accessor: DMA stream 2 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s2m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s2m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s2m0ar`]
module"]
pub type DMA_S2M0AR = crate::Reg<dma_s2m0ar::DMA_S2M0ARrs>;
#[doc = "DMA stream 2 memory 0 address register"]
pub mod dma_s2m0ar;
#[doc = "DMA_S2M1AR (rw) register accessor: DMA stream 2 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s2m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s2m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s2m1ar`]
module"]
pub type DMA_S2M1AR = crate::Reg<dma_s2m1ar::DMA_S2M1ARrs>;
#[doc = "DMA stream 2 memory 1 address register"]
pub mod dma_s2m1ar;
#[doc = "DMA_S2FCR (rw) register accessor: DMA stream 2 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s2fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s2fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s2fcr`]
module"]
pub type DMA_S2FCR = crate::Reg<dma_s2fcr::DMA_S2FCRrs>;
#[doc = "DMA stream 2 FIFO control register"]
pub mod dma_s2fcr;
#[doc = "DMA_S3CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s3cr`]
module"]
pub type DMA_S3CR = crate::Reg<dma_s3cr::DMA_S3CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s3cr;
#[doc = "DMA_S3NDTR (rw) register accessor: DMA stream 3 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s3ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s3ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s3ndtr`]
module"]
pub type DMA_S3NDTR = crate::Reg<dma_s3ndtr::DMA_S3NDTRrs>;
#[doc = "DMA stream 3 number of data register"]
pub mod dma_s3ndtr;
#[doc = "DMA_S3PAR (rw) register accessor: DMA stream 3 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s3par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s3par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s3par`]
module"]
pub type DMA_S3PAR = crate::Reg<dma_s3par::DMA_S3PARrs>;
#[doc = "DMA stream 3 peripheral address register"]
pub mod dma_s3par;
#[doc = "DMA_S3M0AR (rw) register accessor: DMA stream 3 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s3m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s3m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s3m0ar`]
module"]
pub type DMA_S3M0AR = crate::Reg<dma_s3m0ar::DMA_S3M0ARrs>;
#[doc = "DMA stream 3 memory 0 address register"]
pub mod dma_s3m0ar;
#[doc = "DMA_S3M1AR (rw) register accessor: DMA stream 3 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s3m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s3m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s3m1ar`]
module"]
pub type DMA_S3M1AR = crate::Reg<dma_s3m1ar::DMA_S3M1ARrs>;
#[doc = "DMA stream 3 memory 1 address register"]
pub mod dma_s3m1ar;
#[doc = "DMA_S3FCR (rw) register accessor: DMA stream 3 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s3fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s3fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s3fcr`]
module"]
pub type DMA_S3FCR = crate::Reg<dma_s3fcr::DMA_S3FCRrs>;
#[doc = "DMA stream 3 FIFO control register"]
pub mod dma_s3fcr;
#[doc = "DMA_S4CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s4cr`]
module"]
pub type DMA_S4CR = crate::Reg<dma_s4cr::DMA_S4CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s4cr;
#[doc = "DMA_S4NDTR (rw) register accessor: DMA stream 4 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s4ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s4ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s4ndtr`]
module"]
pub type DMA_S4NDTR = crate::Reg<dma_s4ndtr::DMA_S4NDTRrs>;
#[doc = "DMA stream 4 number of data register"]
pub mod dma_s4ndtr;
#[doc = "DMA_S4PAR (rw) register accessor: DMA stream 4 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s4par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s4par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s4par`]
module"]
pub type DMA_S4PAR = crate::Reg<dma_s4par::DMA_S4PARrs>;
#[doc = "DMA stream 4 peripheral address register"]
pub mod dma_s4par;
#[doc = "DMA_S4M0AR (rw) register accessor: DMA stream 4 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s4m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s4m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s4m0ar`]
module"]
pub type DMA_S4M0AR = crate::Reg<dma_s4m0ar::DMA_S4M0ARrs>;
#[doc = "DMA stream 4 memory 0 address register"]
pub mod dma_s4m0ar;
#[doc = "DMA_S4M1AR (rw) register accessor: DMA stream 4 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s4m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s4m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s4m1ar`]
module"]
pub type DMA_S4M1AR = crate::Reg<dma_s4m1ar::DMA_S4M1ARrs>;
#[doc = "DMA stream 4 memory 1 address register"]
pub mod dma_s4m1ar;
#[doc = "DMA_S4FCR (rw) register accessor: DMA stream 4 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s4fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s4fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s4fcr`]
module"]
pub type DMA_S4FCR = crate::Reg<dma_s4fcr::DMA_S4FCRrs>;
#[doc = "DMA stream 4 FIFO control register"]
pub mod dma_s4fcr;
#[doc = "DMA_S5CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s5cr`]
module"]
pub type DMA_S5CR = crate::Reg<dma_s5cr::DMA_S5CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s5cr;
#[doc = "DMA_S5NDTR (rw) register accessor: DMA stream 5 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s5ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s5ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s5ndtr`]
module"]
pub type DMA_S5NDTR = crate::Reg<dma_s5ndtr::DMA_S5NDTRrs>;
#[doc = "DMA stream 5 number of data register"]
pub mod dma_s5ndtr;
#[doc = "DMA_S5PAR (rw) register accessor: DMA stream 5 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s5par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s5par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s5par`]
module"]
pub type DMA_S5PAR = crate::Reg<dma_s5par::DMA_S5PARrs>;
#[doc = "DMA stream 5 peripheral address register"]
pub mod dma_s5par;
#[doc = "DMA_S5M0AR (rw) register accessor: DMA stream 5 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s5m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s5m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s5m0ar`]
module"]
pub type DMA_S5M0AR = crate::Reg<dma_s5m0ar::DMA_S5M0ARrs>;
#[doc = "DMA stream 5 memory 0 address register"]
pub mod dma_s5m0ar;
#[doc = "DMA_S5M1AR (rw) register accessor: DMA stream 5 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s5m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s5m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s5m1ar`]
module"]
pub type DMA_S5M1AR = crate::Reg<dma_s5m1ar::DMA_S5M1ARrs>;
#[doc = "DMA stream 5 memory 1 address register"]
pub mod dma_s5m1ar;
#[doc = "DMA_S5FCR (rw) register accessor: DMA stream 5 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s5fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s5fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s5fcr`]
module"]
pub type DMA_S5FCR = crate::Reg<dma_s5fcr::DMA_S5FCRrs>;
#[doc = "DMA stream 5 FIFO control register"]
pub mod dma_s5fcr;
#[doc = "DMA_S6CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s6cr`]
module"]
pub type DMA_S6CR = crate::Reg<dma_s6cr::DMA_S6CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s6cr;
#[doc = "DMA_S6NDTR (rw) register accessor: DMA stream 6 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s6ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s6ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s6ndtr`]
module"]
pub type DMA_S6NDTR = crate::Reg<dma_s6ndtr::DMA_S6NDTRrs>;
#[doc = "DMA stream 6 number of data register"]
pub mod dma_s6ndtr;
#[doc = "DMA_S6PAR (rw) register accessor: DMA stream 6 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s6par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s6par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s6par`]
module"]
pub type DMA_S6PAR = crate::Reg<dma_s6par::DMA_S6PARrs>;
#[doc = "DMA stream 6 peripheral address register"]
pub mod dma_s6par;
#[doc = "DMA_S6M0AR (rw) register accessor: DMA stream 6 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s6m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s6m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s6m0ar`]
module"]
pub type DMA_S6M0AR = crate::Reg<dma_s6m0ar::DMA_S6M0ARrs>;
#[doc = "DMA stream 6 memory 0 address register"]
pub mod dma_s6m0ar;
#[doc = "DMA_S6M1AR (rw) register accessor: DMA stream 6 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s6m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s6m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s6m1ar`]
module"]
pub type DMA_S6M1AR = crate::Reg<dma_s6m1ar::DMA_S6M1ARrs>;
#[doc = "DMA stream 6 memory 1 address register"]
pub mod dma_s6m1ar;
#[doc = "DMA_S6FCR (rw) register accessor: DMA stream 6 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s6fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s6fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s6fcr`]
module"]
pub type DMA_S6FCR = crate::Reg<dma_s6fcr::DMA_S6FCRrs>;
#[doc = "DMA stream 6 FIFO control register"]
pub mod dma_s6fcr;
#[doc = "DMA_S7CR (rw) register accessor: This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s7cr`]
module"]
pub type DMA_S7CR = crate::Reg<dma_s7cr::DMA_S7CRrs>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s7cr;
#[doc = "DMA_S7NDTR (rw) register accessor: DMA stream 7 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s7ndtr`]
module"]
pub type DMA_S7NDTR = crate::Reg<dma_s7ndtr::DMA_S7NDTRrs>;
#[doc = "DMA stream 7 number of data register"]
pub mod dma_s7ndtr;
#[doc = "DMA_S7PAR (rw) register accessor: DMA stream 7 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s7par`]
module"]
pub type DMA_S7PAR = crate::Reg<dma_s7par::DMA_S7PARrs>;
#[doc = "DMA stream 7 peripheral address register"]
pub mod dma_s7par;
#[doc = "DMA_S7M0AR (rw) register accessor: DMA stream 7 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s7m0ar`]
module"]
pub type DMA_S7M0AR = crate::Reg<dma_s7m0ar::DMA_S7M0ARrs>;
#[doc = "DMA stream 7 memory 0 address register"]
pub mod dma_s7m0ar;
#[doc = "DMA_S7M1AR (rw) register accessor: DMA stream 7 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s7m1ar`]
module"]
pub type DMA_S7M1AR = crate::Reg<dma_s7m1ar::DMA_S7M1ARrs>;
#[doc = "DMA stream 7 memory 1 address register"]
pub mod dma_s7m1ar;
#[doc = "DMA_S7FCR (rw) register accessor: DMA stream 7 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_s7fcr`]
module"]
pub type DMA_S7FCR = crate::Reg<dma_s7fcr::DMA_S7FCRrs>;
#[doc = "DMA stream 7 FIFO control register"]
pub mod dma_s7fcr;
#[doc = "DMA_HWCFGR2 (r) register accessor: DMA hardware configuration 2register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_hwcfgr2`]
module"]
pub type DMA_HWCFGR2 = crate::Reg<dma_hwcfgr2::DMA_HWCFGR2rs>;
#[doc = "DMA hardware configuration 2register"]
pub mod dma_hwcfgr2;
#[doc = "DMA_HWCFGR1 (r) register accessor: DMA hardware configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_hwcfgr1`]
module"]
pub type DMA_HWCFGR1 = crate::Reg<dma_hwcfgr1::DMA_HWCFGR1rs>;
#[doc = "DMA hardware configuration 1 register"]
pub mod dma_hwcfgr1;
#[doc = "DMA_VERR (r) register accessor: This register identifies the version of the IP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_verr`]
module"]
pub type DMA_VERR = crate::Reg<dma_verr::DMA_VERRrs>;
#[doc = "This register identifies the version of the IP."]
pub mod dma_verr;
#[doc = "DMA_IPDR (r) register accessor: DMA IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ipdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ipdr`]
module"]
pub type DMA_IPDR = crate::Reg<dma_ipdr::DMA_IPDRrs>;
#[doc = "DMA IP identification register"]
pub mod dma_ipdr;
#[doc = "DMA_SIDR (r) register accessor: DMA size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sidr`]
module"]
pub type DMA_SIDR = crate::Reg<dma_sidr::DMA_SIDRrs>;
#[doc = "DMA size identification register"]
pub mod dma_sidr;
