#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lisr: LISR,
    hisr: HISR,
    lifcr: LIFCR,
    hifcr: HIFCR,
    s0cr: S0CR,
    s0ndtr: S0NDTR,
    s0par: S0PAR,
    s0m0ar: S0M0AR,
    s0m1ar: S0M1AR,
    s0fcr: S0FCR,
    s1cr: S1CR,
    s1ndtr: S1NDTR,
    s1par: S1PAR,
    s1m0ar: S1M0AR,
    s1m1ar: S1M1AR,
    s1fcr: S1FCR,
    s2cr: S2CR,
    s2ndtr: S2NDTR,
    s2par: S2PAR,
    s2m0ar: S2M0AR,
    s2m1ar: S2M1AR,
    s2fcr: S2FCR,
    s3cr: S3CR,
    s3ndtr: S3NDTR,
    s3par: S3PAR,
    s3m0ar: S3M0AR,
    s3m1ar: S3M1AR,
    s3fcr: S3FCR,
    s4cr: S4CR,
    s4ndtr: S4NDTR,
    s4par: S4PAR,
    s4m0ar: S4M0AR,
    s4m1ar: S4M1AR,
    s4fcr: S4FCR,
    s5cr: S5CR,
    s5ndtr: S5NDTR,
    s5par: S5PAR,
    s5m0ar: S5M0AR,
    s5m1ar: S5M1AR,
    s5fcr: S5FCR,
    s6cr: S6CR,
    s6ndtr: S6NDTR,
    s6par: S6PAR,
    s6m0ar: S6M0AR,
    s6m1ar: S6M1AR,
    s6fcr: S6FCR,
    s7cr: S7CR,
    s7ndtr: S7NDTR,
    s7par: S7PAR,
    s7m0ar: S7M0AR,
    s7m1ar: S7M1AR,
    s7fcr: S7FCR,
}
impl RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    #[inline(always)]
    pub const fn lisr(&self) -> &LISR {
        &self.lisr
    }
    #[doc = "0x04 - high interrupt status register"]
    #[inline(always)]
    pub const fn hisr(&self) -> &HISR {
        &self.hisr
    }
    #[doc = "0x08 - low interrupt flag clear register"]
    #[inline(always)]
    pub const fn lifcr(&self) -> &LIFCR {
        &self.lifcr
    }
    #[doc = "0x0c - high interrupt flag clear register"]
    #[inline(always)]
    pub const fn hifcr(&self) -> &HIFCR {
        &self.hifcr
    }
    #[doc = "0x10 - stream x configuration register"]
    #[inline(always)]
    pub const fn s0cr(&self) -> &S0CR {
        &self.s0cr
    }
    #[doc = "0x14 - stream x number of data register"]
    #[inline(always)]
    pub const fn s0ndtr(&self) -> &S0NDTR {
        &self.s0ndtr
    }
    #[doc = "0x18 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s0par(&self) -> &S0PAR {
        &self.s0par
    }
    #[doc = "0x1c - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s0m0ar(&self) -> &S0M0AR {
        &self.s0m0ar
    }
    #[doc = "0x20 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s0m1ar(&self) -> &S0M1AR {
        &self.s0m1ar
    }
    #[doc = "0x24 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s0fcr(&self) -> &S0FCR {
        &self.s0fcr
    }
    #[doc = "0x28 - stream x configuration register"]
    #[inline(always)]
    pub const fn s1cr(&self) -> &S1CR {
        &self.s1cr
    }
    #[doc = "0x2c - stream x number of data register"]
    #[inline(always)]
    pub const fn s1ndtr(&self) -> &S1NDTR {
        &self.s1ndtr
    }
    #[doc = "0x30 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s1par(&self) -> &S1PAR {
        &self.s1par
    }
    #[doc = "0x34 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s1m0ar(&self) -> &S1M0AR {
        &self.s1m0ar
    }
    #[doc = "0x38 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s1m1ar(&self) -> &S1M1AR {
        &self.s1m1ar
    }
    #[doc = "0x3c - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s1fcr(&self) -> &S1FCR {
        &self.s1fcr
    }
    #[doc = "0x40 - stream x configuration register"]
    #[inline(always)]
    pub const fn s2cr(&self) -> &S2CR {
        &self.s2cr
    }
    #[doc = "0x44 - stream x number of data register"]
    #[inline(always)]
    pub const fn s2ndtr(&self) -> &S2NDTR {
        &self.s2ndtr
    }
    #[doc = "0x48 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s2par(&self) -> &S2PAR {
        &self.s2par
    }
    #[doc = "0x4c - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s2m0ar(&self) -> &S2M0AR {
        &self.s2m0ar
    }
    #[doc = "0x50 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s2m1ar(&self) -> &S2M1AR {
        &self.s2m1ar
    }
    #[doc = "0x54 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s2fcr(&self) -> &S2FCR {
        &self.s2fcr
    }
    #[doc = "0x58 - stream x configuration register"]
    #[inline(always)]
    pub const fn s3cr(&self) -> &S3CR {
        &self.s3cr
    }
    #[doc = "0x5c - stream x number of data register"]
    #[inline(always)]
    pub const fn s3ndtr(&self) -> &S3NDTR {
        &self.s3ndtr
    }
    #[doc = "0x60 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s3par(&self) -> &S3PAR {
        &self.s3par
    }
    #[doc = "0x64 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s3m0ar(&self) -> &S3M0AR {
        &self.s3m0ar
    }
    #[doc = "0x68 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s3m1ar(&self) -> &S3M1AR {
        &self.s3m1ar
    }
    #[doc = "0x6c - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s3fcr(&self) -> &S3FCR {
        &self.s3fcr
    }
    #[doc = "0x70 - stream x configuration register"]
    #[inline(always)]
    pub const fn s4cr(&self) -> &S4CR {
        &self.s4cr
    }
    #[doc = "0x74 - stream x number of data register"]
    #[inline(always)]
    pub const fn s4ndtr(&self) -> &S4NDTR {
        &self.s4ndtr
    }
    #[doc = "0x78 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s4par(&self) -> &S4PAR {
        &self.s4par
    }
    #[doc = "0x7c - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s4m0ar(&self) -> &S4M0AR {
        &self.s4m0ar
    }
    #[doc = "0x80 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s4m1ar(&self) -> &S4M1AR {
        &self.s4m1ar
    }
    #[doc = "0x84 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s4fcr(&self) -> &S4FCR {
        &self.s4fcr
    }
    #[doc = "0x88 - stream x configuration register"]
    #[inline(always)]
    pub const fn s5cr(&self) -> &S5CR {
        &self.s5cr
    }
    #[doc = "0x8c - stream x number of data register"]
    #[inline(always)]
    pub const fn s5ndtr(&self) -> &S5NDTR {
        &self.s5ndtr
    }
    #[doc = "0x90 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s5par(&self) -> &S5PAR {
        &self.s5par
    }
    #[doc = "0x94 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s5m0ar(&self) -> &S5M0AR {
        &self.s5m0ar
    }
    #[doc = "0x98 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s5m1ar(&self) -> &S5M1AR {
        &self.s5m1ar
    }
    #[doc = "0x9c - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s5fcr(&self) -> &S5FCR {
        &self.s5fcr
    }
    #[doc = "0xa0 - stream x configuration register"]
    #[inline(always)]
    pub const fn s6cr(&self) -> &S6CR {
        &self.s6cr
    }
    #[doc = "0xa4 - stream x number of data register"]
    #[inline(always)]
    pub const fn s6ndtr(&self) -> &S6NDTR {
        &self.s6ndtr
    }
    #[doc = "0xa8 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s6par(&self) -> &S6PAR {
        &self.s6par
    }
    #[doc = "0xac - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s6m0ar(&self) -> &S6M0AR {
        &self.s6m0ar
    }
    #[doc = "0xb0 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s6m1ar(&self) -> &S6M1AR {
        &self.s6m1ar
    }
    #[doc = "0xb4 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s6fcr(&self) -> &S6FCR {
        &self.s6fcr
    }
    #[doc = "0xb8 - stream x configuration register"]
    #[inline(always)]
    pub const fn s7cr(&self) -> &S7CR {
        &self.s7cr
    }
    #[doc = "0xbc - stream x number of data register"]
    #[inline(always)]
    pub const fn s7ndtr(&self) -> &S7NDTR {
        &self.s7ndtr
    }
    #[doc = "0xc0 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn s7par(&self) -> &S7PAR {
        &self.s7par
    }
    #[doc = "0xc4 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn s7m0ar(&self) -> &S7M0AR {
        &self.s7m0ar
    }
    #[doc = "0xc8 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s7m1ar(&self) -> &S7M1AR {
        &self.s7m1ar
    }
    #[doc = "0xcc - stream x FIFO control register"]
    #[inline(always)]
    pub const fn s7fcr(&self) -> &S7FCR {
        &self.s7fcr
    }
}
#[doc = "LISR (r) register accessor: low interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lisr`]
module"]
pub type LISR = crate::Reg<lisr::LISRrs>;
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "HISR (r) register accessor: high interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hisr`]
module"]
pub type HISR = crate::Reg<hisr::HISRrs>;
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "LIFCR (w) register accessor: low interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lifcr`]
module"]
pub type LIFCR = crate::Reg<lifcr::LIFCRrs>;
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "HIFCR (w) register accessor: high interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hifcr`]
module"]
pub type HIFCR = crate::Reg<hifcr::HIFCRrs>;
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
#[doc = "S0CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0cr`]
module"]
pub type S0CR = crate::Reg<s0cr::S0CRrs>;
#[doc = "stream x configuration register"]
pub mod s0cr;
#[doc = "S0NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0ndtr`]
module"]
pub type S0NDTR = crate::Reg<s0ndtr::S0NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s0ndtr;
#[doc = "S0PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0par`]
module"]
pub type S0PAR = crate::Reg<s0par::S0PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s0par;
#[doc = "S0M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0m0ar`]
module"]
pub type S0M0AR = crate::Reg<s0m0ar::S0M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s0m0ar;
#[doc = "S0M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0m1ar`]
module"]
pub type S0M1AR = crate::Reg<s0m1ar::S0M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s0m1ar;
#[doc = "S0FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0fcr`]
module"]
pub type S0FCR = crate::Reg<s0fcr::S0FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s0fcr;
#[doc = "S1CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1cr`]
module"]
pub type S1CR = crate::Reg<s1cr::S1CRrs>;
#[doc = "stream x configuration register"]
pub mod s1cr;
#[doc = "S1NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1ndtr`]
module"]
pub type S1NDTR = crate::Reg<s1ndtr::S1NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s1ndtr;
#[doc = "S1PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1par`]
module"]
pub type S1PAR = crate::Reg<s1par::S1PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s1par;
#[doc = "S1M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1m0ar`]
module"]
pub type S1M0AR = crate::Reg<s1m0ar::S1M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s1m0ar;
#[doc = "S1M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1m1ar`]
module"]
pub type S1M1AR = crate::Reg<s1m1ar::S1M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s1m1ar;
#[doc = "S1FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1fcr`]
module"]
pub type S1FCR = crate::Reg<s1fcr::S1FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s1fcr;
#[doc = "S2CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2cr`]
module"]
pub type S2CR = crate::Reg<s2cr::S2CRrs>;
#[doc = "stream x configuration register"]
pub mod s2cr;
#[doc = "S2NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2ndtr`]
module"]
pub type S2NDTR = crate::Reg<s2ndtr::S2NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s2ndtr;
#[doc = "S2PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2par`]
module"]
pub type S2PAR = crate::Reg<s2par::S2PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s2par;
#[doc = "S2M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2m0ar`]
module"]
pub type S2M0AR = crate::Reg<s2m0ar::S2M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s2m0ar;
#[doc = "S2M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2m1ar`]
module"]
pub type S2M1AR = crate::Reg<s2m1ar::S2M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s2m1ar;
#[doc = "S2FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2fcr`]
module"]
pub type S2FCR = crate::Reg<s2fcr::S2FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s2fcr;
#[doc = "S3CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3cr`]
module"]
pub type S3CR = crate::Reg<s3cr::S3CRrs>;
#[doc = "stream x configuration register"]
pub mod s3cr;
#[doc = "S3NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3ndtr`]
module"]
pub type S3NDTR = crate::Reg<s3ndtr::S3NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s3ndtr;
#[doc = "S3PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3par`]
module"]
pub type S3PAR = crate::Reg<s3par::S3PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s3par;
#[doc = "S3M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3m0ar`]
module"]
pub type S3M0AR = crate::Reg<s3m0ar::S3M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s3m0ar;
#[doc = "S3M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3m1ar`]
module"]
pub type S3M1AR = crate::Reg<s3m1ar::S3M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s3m1ar;
#[doc = "S3FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3fcr`]
module"]
pub type S3FCR = crate::Reg<s3fcr::S3FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s3fcr;
#[doc = "S4CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4cr`]
module"]
pub type S4CR = crate::Reg<s4cr::S4CRrs>;
#[doc = "stream x configuration register"]
pub mod s4cr;
#[doc = "S4NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4ndtr`]
module"]
pub type S4NDTR = crate::Reg<s4ndtr::S4NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s4ndtr;
#[doc = "S4PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4par`]
module"]
pub type S4PAR = crate::Reg<s4par::S4PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s4par;
#[doc = "S4M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4m0ar`]
module"]
pub type S4M0AR = crate::Reg<s4m0ar::S4M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s4m0ar;
#[doc = "S4M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4m1ar`]
module"]
pub type S4M1AR = crate::Reg<s4m1ar::S4M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s4m1ar;
#[doc = "S4FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4fcr`]
module"]
pub type S4FCR = crate::Reg<s4fcr::S4FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s4fcr;
#[doc = "S5CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5cr`]
module"]
pub type S5CR = crate::Reg<s5cr::S5CRrs>;
#[doc = "stream x configuration register"]
pub mod s5cr;
#[doc = "S5NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5ndtr`]
module"]
pub type S5NDTR = crate::Reg<s5ndtr::S5NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s5ndtr;
#[doc = "S5PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5par`]
module"]
pub type S5PAR = crate::Reg<s5par::S5PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s5par;
#[doc = "S5M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5m0ar`]
module"]
pub type S5M0AR = crate::Reg<s5m0ar::S5M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s5m0ar;
#[doc = "S5M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5m1ar`]
module"]
pub type S5M1AR = crate::Reg<s5m1ar::S5M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s5m1ar;
#[doc = "S5FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5fcr`]
module"]
pub type S5FCR = crate::Reg<s5fcr::S5FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s5fcr;
#[doc = "S6CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6cr`]
module"]
pub type S6CR = crate::Reg<s6cr::S6CRrs>;
#[doc = "stream x configuration register"]
pub mod s6cr;
#[doc = "S6NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6ndtr`]
module"]
pub type S6NDTR = crate::Reg<s6ndtr::S6NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s6ndtr;
#[doc = "S6PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6par`]
module"]
pub type S6PAR = crate::Reg<s6par::S6PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s6par;
#[doc = "S6M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6m0ar`]
module"]
pub type S6M0AR = crate::Reg<s6m0ar::S6M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s6m0ar;
#[doc = "S6M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6m1ar`]
module"]
pub type S6M1AR = crate::Reg<s6m1ar::S6M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s6m1ar;
#[doc = "S6FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6fcr`]
module"]
pub type S6FCR = crate::Reg<s6fcr::S6FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s6fcr;
#[doc = "S7CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7cr`]
module"]
pub type S7CR = crate::Reg<s7cr::S7CRrs>;
#[doc = "stream x configuration register"]
pub mod s7cr;
#[doc = "S7NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7ndtr`]
module"]
pub type S7NDTR = crate::Reg<s7ndtr::S7NDTRrs>;
#[doc = "stream x number of data register"]
pub mod s7ndtr;
#[doc = "S7PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7par`]
module"]
pub type S7PAR = crate::Reg<s7par::S7PARrs>;
#[doc = "stream x peripheral address register"]
pub mod s7par;
#[doc = "S7M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7m0ar`]
module"]
pub type S7M0AR = crate::Reg<s7m0ar::S7M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod s7m0ar;
#[doc = "S7M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7m1ar`]
module"]
pub type S7M1AR = crate::Reg<s7m1ar::S7M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod s7m1ar;
#[doc = "S7FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7fcr`]
module"]
pub type S7FCR = crate::Reg<s7fcr::S7FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod s7fcr;
