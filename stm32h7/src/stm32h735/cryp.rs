#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    din: DIN,
    dout: DOUT,
    dmacr: DMACR,
    imscr: IMSCR,
    risr: RISR,
    misr: MISR,
    k0lr: K0LR,
    k0rr: K0RR,
    k1lr: K1LR,
    k1rr: K1RR,
    k2lr: K2LR,
    k2rr: K2RR,
    k3lr: K3LR,
    k3rr: K3RR,
    iv0lr: IV0LR,
    iv0rr: IV0RR,
    iv1lr: IV1LR,
    iv1rr: IV1RR,
    csgcmccm0r: CSGCMCCM0R,
    csgcmccm1r: CSGCMCCM1R,
    csgcmccm2r: CSGCMCCM2R,
    csgcmccm3r: CSGCMCCM3R,
    csgcmccm4r: CSGCMCCM4R,
    csgcmccm5r: CSGCMCCM5R,
    csgcmccm6r: CSGCMCCM6R,
    csgcmccm7r: CSGCMCCM7R,
    csgcm0r: CSGCM0R,
    csgcm1r: CSGCM1R,
    csgcm2r: CSGCM2R,
    csgcm3r: CSGCM3R,
    csgcm4r: CSGCM4R,
    csgcm5r: CSGCM5R,
    csgcm6r: CSGCM6R,
    csgcm7r: CSGCM7R,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - data input register"]
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    #[doc = "0x0c - data output register"]
    #[inline(always)]
    pub const fn dout(&self) -> &DOUT {
        &self.dout
    }
    #[doc = "0x10 - DMA control register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    #[doc = "0x14 - interrupt mask set/clear register"]
    #[inline(always)]
    pub const fn imscr(&self) -> &IMSCR {
        &self.imscr
    }
    #[doc = "0x18 - raw interrupt status register"]
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    #[doc = "0x1c - masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    #[doc = "0x20 - key registers"]
    #[inline(always)]
    pub const fn k0lr(&self) -> &K0LR {
        &self.k0lr
    }
    #[doc = "0x24 - key registers"]
    #[inline(always)]
    pub const fn k0rr(&self) -> &K0RR {
        &self.k0rr
    }
    #[doc = "0x28 - key registers"]
    #[inline(always)]
    pub const fn k1lr(&self) -> &K1LR {
        &self.k1lr
    }
    #[doc = "0x2c - key registers"]
    #[inline(always)]
    pub const fn k1rr(&self) -> &K1RR {
        &self.k1rr
    }
    #[doc = "0x30 - key registers"]
    #[inline(always)]
    pub const fn k2lr(&self) -> &K2LR {
        &self.k2lr
    }
    #[doc = "0x34 - key registers"]
    #[inline(always)]
    pub const fn k2rr(&self) -> &K2RR {
        &self.k2rr
    }
    #[doc = "0x38 - key registers"]
    #[inline(always)]
    pub const fn k3lr(&self) -> &K3LR {
        &self.k3lr
    }
    #[doc = "0x3c - key registers"]
    #[inline(always)]
    pub const fn k3rr(&self) -> &K3RR {
        &self.k3rr
    }
    #[doc = "0x40 - Initialization vector register 0L"]
    #[inline(always)]
    pub const fn iv0lr(&self) -> &IV0LR {
        &self.iv0lr
    }
    #[doc = "0x44 - initialization vector register 0R"]
    #[inline(always)]
    pub const fn iv0rr(&self) -> &IV0RR {
        &self.iv0rr
    }
    #[doc = "0x48 - Initialization vector register 1L"]
    #[inline(always)]
    pub const fn iv1lr(&self) -> &IV1LR {
        &self.iv1lr
    }
    #[doc = "0x4c - Initialization vector register 1R"]
    #[inline(always)]
    pub const fn iv1rr(&self) -> &IV1RR {
        &self.iv1rr
    }
    #[doc = "0x50 - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm0r(&self) -> &CSGCMCCM0R {
        &self.csgcmccm0r
    }
    #[doc = "0x54 - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm1r(&self) -> &CSGCMCCM1R {
        &self.csgcmccm1r
    }
    #[doc = "0x58 - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm2r(&self) -> &CSGCMCCM2R {
        &self.csgcmccm2r
    }
    #[doc = "0x5c - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm3r(&self) -> &CSGCMCCM3R {
        &self.csgcmccm3r
    }
    #[doc = "0x60 - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm4r(&self) -> &CSGCMCCM4R {
        &self.csgcmccm4r
    }
    #[doc = "0x64 - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm5r(&self) -> &CSGCMCCM5R {
        &self.csgcmccm5r
    }
    #[doc = "0x68 - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm6r(&self) -> &CSGCMCCM6R {
        &self.csgcmccm6r
    }
    #[doc = "0x6c - context swap register"]
    #[inline(always)]
    pub const fn csgcmccm7r(&self) -> &CSGCMCCM7R {
        &self.csgcmccm7r
    }
    #[doc = "0x70 - context swap register"]
    #[inline(always)]
    pub const fn csgcm0r(&self) -> &CSGCM0R {
        &self.csgcm0r
    }
    #[doc = "0x74 - context swap register"]
    #[inline(always)]
    pub const fn csgcm1r(&self) -> &CSGCM1R {
        &self.csgcm1r
    }
    #[doc = "0x78 - context swap register"]
    #[inline(always)]
    pub const fn csgcm2r(&self) -> &CSGCM2R {
        &self.csgcm2r
    }
    #[doc = "0x7c - context swap register"]
    #[inline(always)]
    pub const fn csgcm3r(&self) -> &CSGCM3R {
        &self.csgcm3r
    }
    #[doc = "0x80 - context swap register"]
    #[inline(always)]
    pub const fn csgcm4r(&self) -> &CSGCM4R {
        &self.csgcm4r
    }
    #[doc = "0x84 - context swap register"]
    #[inline(always)]
    pub const fn csgcm5r(&self) -> &CSGCM5R {
        &self.csgcm5r
    }
    #[doc = "0x88 - context swap register"]
    #[inline(always)]
    pub const fn csgcm6r(&self) -> &CSGCM6R {
        &self.csgcm6r
    }
    #[doc = "0x8c - context swap register"]
    #[inline(always)]
    pub const fn csgcm7r(&self) -> &CSGCM7R {
        &self.csgcm7r
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "DIN (rw) register accessor: data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din`]
module"]
pub type DIN = crate::Reg<din::DINrs>;
#[doc = "data input register"]
pub mod din;
#[doc = "DOUT (r) register accessor: data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout`]
module"]
pub type DOUT = crate::Reg<dout::DOUTrs>;
#[doc = "data output register"]
pub mod dout;
#[doc = "DMACR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "IMSCR (rw) register accessor: interrupt mask set/clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imscr`]
module"]
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "RISR (r) register accessor: raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`risr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@risr`]
module"]
pub type RISR = crate::Reg<risr::RISRrs>;
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "MISR (r) register accessor: masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
pub type MISR = crate::Reg<misr::MISRrs>;
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "K0LR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k0lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k0lr`]
module"]
pub type K0LR = crate::Reg<k0lr::K0LRrs>;
#[doc = "key registers"]
pub mod k0lr;
#[doc = "K0RR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k0rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k0rr`]
module"]
pub type K0RR = crate::Reg<k0rr::K0RRrs>;
#[doc = "key registers"]
pub mod k0rr;
#[doc = "K1LR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k1lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k1lr`]
module"]
pub type K1LR = crate::Reg<k1lr::K1LRrs>;
#[doc = "key registers"]
pub mod k1lr;
#[doc = "K1RR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k1rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k1rr`]
module"]
pub type K1RR = crate::Reg<k1rr::K1RRrs>;
#[doc = "key registers"]
pub mod k1rr;
#[doc = "K2LR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k2lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k2lr`]
module"]
pub type K2LR = crate::Reg<k2lr::K2LRrs>;
#[doc = "key registers"]
pub mod k2lr;
#[doc = "K2RR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k2rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k2rr`]
module"]
pub type K2RR = crate::Reg<k2rr::K2RRrs>;
#[doc = "key registers"]
pub mod k2rr;
#[doc = "K3LR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k3lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k3lr`]
module"]
pub type K3LR = crate::Reg<k3lr::K3LRrs>;
#[doc = "key registers"]
pub mod k3lr;
#[doc = "K3RR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k3rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k3rr`]
module"]
pub type K3RR = crate::Reg<k3rr::K3RRrs>;
#[doc = "key registers"]
pub mod k3rr;
#[doc = "IV0LR (rw) register accessor: Initialization vector register 0L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv0lr`]
module"]
pub type IV0LR = crate::Reg<iv0lr::IV0LRrs>;
#[doc = "Initialization vector register 0L"]
pub mod iv0lr;
#[doc = "IV0RR (rw) register accessor: initialization vector register 0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0rr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0rr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv0rr`]
module"]
pub type IV0RR = crate::Reg<iv0rr::IV0RRrs>;
#[doc = "initialization vector register 0R"]
pub mod iv0rr;
#[doc = "IV1LR (rw) register accessor: Initialization vector register 1L\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv1lr`]
module"]
pub type IV1LR = crate::Reg<iv1lr::IV1LRrs>;
#[doc = "Initialization vector register 1L"]
pub mod iv1lr;
#[doc = "IV1RR (rw) register accessor: Initialization vector register 1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1rr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1rr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv1rr`]
module"]
pub type IV1RR = crate::Reg<iv1rr::IV1RRrs>;
#[doc = "Initialization vector register 1R"]
pub mod iv1rr;
#[doc = "CSGCMCCM0R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm0r`]
module"]
pub type CSGCMCCM0R = crate::Reg<csgcmccm0r::CSGCMCCM0Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm0r;
#[doc = "CSGCMCCM1R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm1r`]
module"]
pub type CSGCMCCM1R = crate::Reg<csgcmccm1r::CSGCMCCM1Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm1r;
#[doc = "CSGCMCCM2R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm2r`]
module"]
pub type CSGCMCCM2R = crate::Reg<csgcmccm2r::CSGCMCCM2Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm2r;
#[doc = "CSGCMCCM3R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm3r`]
module"]
pub type CSGCMCCM3R = crate::Reg<csgcmccm3r::CSGCMCCM3Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm3r;
#[doc = "CSGCMCCM4R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm4r`]
module"]
pub type CSGCMCCM4R = crate::Reg<csgcmccm4r::CSGCMCCM4Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm4r;
#[doc = "CSGCMCCM5R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm5r`]
module"]
pub type CSGCMCCM5R = crate::Reg<csgcmccm5r::CSGCMCCM5Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm5r;
#[doc = "CSGCMCCM6R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm6r`]
module"]
pub type CSGCMCCM6R = crate::Reg<csgcmccm6r::CSGCMCCM6Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm6r;
#[doc = "CSGCMCCM7R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmccm7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmccm7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcmccm7r`]
module"]
pub type CSGCMCCM7R = crate::Reg<csgcmccm7r::CSGCMCCM7Rrs>;
#[doc = "context swap register"]
pub mod csgcmccm7r;
#[doc = "CSGCM0R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm0r`]
module"]
pub type CSGCM0R = crate::Reg<csgcm0r::CSGCM0Rrs>;
#[doc = "context swap register"]
pub mod csgcm0r;
#[doc = "CSGCM1R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm1r`]
module"]
pub type CSGCM1R = crate::Reg<csgcm1r::CSGCM1Rrs>;
#[doc = "context swap register"]
pub mod csgcm1r;
#[doc = "CSGCM2R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm2r`]
module"]
pub type CSGCM2R = crate::Reg<csgcm2r::CSGCM2Rrs>;
#[doc = "context swap register"]
pub mod csgcm2r;
#[doc = "CSGCM3R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm3r`]
module"]
pub type CSGCM3R = crate::Reg<csgcm3r::CSGCM3Rrs>;
#[doc = "context swap register"]
pub mod csgcm3r;
#[doc = "CSGCM4R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm4r`]
module"]
pub type CSGCM4R = crate::Reg<csgcm4r::CSGCM4Rrs>;
#[doc = "context swap register"]
pub mod csgcm4r;
#[doc = "CSGCM5R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm5r`]
module"]
pub type CSGCM5R = crate::Reg<csgcm5r::CSGCM5Rrs>;
#[doc = "context swap register"]
pub mod csgcm5r;
#[doc = "CSGCM6R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm6r`]
module"]
pub type CSGCM6R = crate::Reg<csgcm6r::CSGCM6Rrs>;
#[doc = "context swap register"]
pub mod csgcm6r;
#[doc = "CSGCM7R (rw) register accessor: context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcm7r`]
module"]
pub type CSGCM7R = crate::Reg<csgcm7r::CSGCM7Rrs>;
#[doc = "context swap register"]
pub mod csgcm7r;
