#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - data input register
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    ///0x0c - data output register
    #[inline(always)]
    pub const fn dout(&self) -> &DOUT {
        &self.dout
    }
    ///0x10 - DMA control register
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    ///0x14 - interrupt mask set/clear register
    #[inline(always)]
    pub const fn imscr(&self) -> &IMSCR {
        &self.imscr
    }
    ///0x18 - raw interrupt status register
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    ///0x1c - masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x20 - key registers
    #[inline(always)]
    pub const fn k0lr(&self) -> &K0LR {
        &self.k0lr
    }
    ///0x24 - key registers
    #[inline(always)]
    pub const fn k0rr(&self) -> &K0RR {
        &self.k0rr
    }
    ///0x28 - key registers
    #[inline(always)]
    pub const fn k1lr(&self) -> &K1LR {
        &self.k1lr
    }
    ///0x2c - key registers
    #[inline(always)]
    pub const fn k1rr(&self) -> &K1RR {
        &self.k1rr
    }
    ///0x30 - key registers
    #[inline(always)]
    pub const fn k2lr(&self) -> &K2LR {
        &self.k2lr
    }
    ///0x34 - key registers
    #[inline(always)]
    pub const fn k2rr(&self) -> &K2RR {
        &self.k2rr
    }
    ///0x38 - key registers
    #[inline(always)]
    pub const fn k3lr(&self) -> &K3LR {
        &self.k3lr
    }
    ///0x3c - key registers
    #[inline(always)]
    pub const fn k3rr(&self) -> &K3RR {
        &self.k3rr
    }
    ///0x40 - Initialization vector register 0L
    #[inline(always)]
    pub const fn iv0lr(&self) -> &IV0LR {
        &self.iv0lr
    }
    ///0x44 - initialization vector register 0R
    #[inline(always)]
    pub const fn iv0rr(&self) -> &IV0RR {
        &self.iv0rr
    }
    ///0x48 - Initialization vector register 1L
    #[inline(always)]
    pub const fn iv1lr(&self) -> &IV1LR {
        &self.iv1lr
    }
    ///0x4c - Initialization vector register 1R
    #[inline(always)]
    pub const fn iv1rr(&self) -> &IV1RR {
        &self.iv1rr
    }
    ///0x50 - context swap register
    #[inline(always)]
    pub const fn csgcmccm0r(&self) -> &CSGCMCCM0R {
        &self.csgcmccm0r
    }
    ///0x54 - context swap register
    #[inline(always)]
    pub const fn csgcmccm1r(&self) -> &CSGCMCCM1R {
        &self.csgcmccm1r
    }
    ///0x58 - context swap register
    #[inline(always)]
    pub const fn csgcmccm2r(&self) -> &CSGCMCCM2R {
        &self.csgcmccm2r
    }
    ///0x5c - context swap register
    #[inline(always)]
    pub const fn csgcmccm3r(&self) -> &CSGCMCCM3R {
        &self.csgcmccm3r
    }
    ///0x60 - context swap register
    #[inline(always)]
    pub const fn csgcmccm4r(&self) -> &CSGCMCCM4R {
        &self.csgcmccm4r
    }
    ///0x64 - context swap register
    #[inline(always)]
    pub const fn csgcmccm5r(&self) -> &CSGCMCCM5R {
        &self.csgcmccm5r
    }
    ///0x68 - context swap register
    #[inline(always)]
    pub const fn csgcmccm6r(&self) -> &CSGCMCCM6R {
        &self.csgcmccm6r
    }
    ///0x6c - context swap register
    #[inline(always)]
    pub const fn csgcmccm7r(&self) -> &CSGCMCCM7R {
        &self.csgcmccm7r
    }
    ///0x70 - context swap register
    #[inline(always)]
    pub const fn csgcm0r(&self) -> &CSGCM0R {
        &self.csgcm0r
    }
    ///0x74 - context swap register
    #[inline(always)]
    pub const fn csgcm1r(&self) -> &CSGCM1R {
        &self.csgcm1r
    }
    ///0x78 - context swap register
    #[inline(always)]
    pub const fn csgcm2r(&self) -> &CSGCM2R {
        &self.csgcm2r
    }
    ///0x7c - context swap register
    #[inline(always)]
    pub const fn csgcm3r(&self) -> &CSGCM3R {
        &self.csgcm3r
    }
    ///0x80 - context swap register
    #[inline(always)]
    pub const fn csgcm4r(&self) -> &CSGCM4R {
        &self.csgcm4r
    }
    ///0x84 - context swap register
    #[inline(always)]
    pub const fn csgcm5r(&self) -> &CSGCM5R {
        &self.csgcm5r
    }
    ///0x88 - context swap register
    #[inline(always)]
    pub const fn csgcm6r(&self) -> &CSGCM6R {
        &self.csgcm6r
    }
    ///0x8c - context swap register
    #[inline(always)]
    pub const fn csgcm7r(&self) -> &CSGCM7R {
        &self.csgcm7r
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**DIN (rw) register accessor: data input register

You can [`read`](crate::Reg::read) this register and get [`din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:DIN)

For information about available fields see [`mod@din`] module*/
pub type DIN = crate::Reg<din::DINrs>;
///data input register
pub mod din;
/**DOUT (r) register accessor: data output register

You can [`read`](crate::Reg::read) this register and get [`dout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:DOUT)

For information about available fields see [`mod@dout`] module*/
pub type DOUT = crate::Reg<dout::DOUTrs>;
///data output register
pub mod dout;
/**DMACR (rw) register accessor: DMA control register

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:DMACR)

For information about available fields see [`mod@dmacr`] module*/
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
///DMA control register
pub mod dmacr;
/**IMSCR (rw) register accessor: interrupt mask set/clear register

You can [`read`](crate::Reg::read) this register and get [`imscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:IMSCR)

For information about available fields see [`mod@imscr`] module*/
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
///interrupt mask set/clear register
pub mod imscr;
/**RISR (r) register accessor: raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`risr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:RISR)

For information about available fields see [`mod@risr`] module*/
pub type RISR = crate::Reg<risr::RISRrs>;
///raw interrupt status register
pub mod risr;
/**MISR (r) register accessor: masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///masked interrupt status register
pub mod misr;
/**K0LR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K0LR)

For information about available fields see [`mod@k0lr`] module*/
pub type K0LR = crate::Reg<k0lr::K0LRrs>;
///key registers
pub mod k0lr;
/**K0RR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K0RR)

For information about available fields see [`mod@k0rr`] module*/
pub type K0RR = crate::Reg<k0rr::K0RRrs>;
///key registers
pub mod k0rr;
/**K1LR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K1LR)

For information about available fields see [`mod@k1lr`] module*/
pub type K1LR = crate::Reg<k1lr::K1LRrs>;
///key registers
pub mod k1lr;
/**K1RR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K1RR)

For information about available fields see [`mod@k1rr`] module*/
pub type K1RR = crate::Reg<k1rr::K1RRrs>;
///key registers
pub mod k1rr;
/**K2LR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K2LR)

For information about available fields see [`mod@k2lr`] module*/
pub type K2LR = crate::Reg<k2lr::K2LRrs>;
///key registers
pub mod k2lr;
/**K2RR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K2RR)

For information about available fields see [`mod@k2rr`] module*/
pub type K2RR = crate::Reg<k2rr::K2RRrs>;
///key registers
pub mod k2rr;
/**K3LR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K3LR)

For information about available fields see [`mod@k3lr`] module*/
pub type K3LR = crate::Reg<k3lr::K3LRrs>;
///key registers
pub mod k3lr;
/**K3RR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:K3RR)

For information about available fields see [`mod@k3rr`] module*/
pub type K3RR = crate::Reg<k3rr::K3RRrs>;
///key registers
pub mod k3rr;
/**IV0LR (rw) register accessor: Initialization vector register 0L

You can [`read`](crate::Reg::read) this register and get [`iv0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:IV0LR)

For information about available fields see [`mod@iv0lr`] module*/
pub type IV0LR = crate::Reg<iv0lr::IV0LRrs>;
///Initialization vector register 0L
pub mod iv0lr;
/**IV0RR (rw) register accessor: initialization vector register 0R

You can [`read`](crate::Reg::read) this register and get [`iv0rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:IV0RR)

For information about available fields see [`mod@iv0rr`] module*/
pub type IV0RR = crate::Reg<iv0rr::IV0RRrs>;
///initialization vector register 0R
pub mod iv0rr;
/**IV1LR (rw) register accessor: Initialization vector register 1L

You can [`read`](crate::Reg::read) this register and get [`iv1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:IV1LR)

For information about available fields see [`mod@iv1lr`] module*/
pub type IV1LR = crate::Reg<iv1lr::IV1LRrs>;
///Initialization vector register 1L
pub mod iv1lr;
/**IV1RR (rw) register accessor: Initialization vector register 1R

You can [`read`](crate::Reg::read) this register and get [`iv1rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:IV1RR)

For information about available fields see [`mod@iv1rr`] module*/
pub type IV1RR = crate::Reg<iv1rr::IV1RRrs>;
///Initialization vector register 1R
pub mod iv1rr;
/**CSGCMCCM0R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM0R)

For information about available fields see [`mod@csgcmccm0r`] module*/
pub type CSGCMCCM0R = crate::Reg<csgcmccm0r::CSGCMCCM0Rrs>;
///context swap register
pub mod csgcmccm0r;
/**CSGCMCCM1R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM1R)

For information about available fields see [`mod@csgcmccm1r`] module*/
pub type CSGCMCCM1R = crate::Reg<csgcmccm1r::CSGCMCCM1Rrs>;
///context swap register
pub mod csgcmccm1r;
/**CSGCMCCM2R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM2R)

For information about available fields see [`mod@csgcmccm2r`] module*/
pub type CSGCMCCM2R = crate::Reg<csgcmccm2r::CSGCMCCM2Rrs>;
///context swap register
pub mod csgcmccm2r;
/**CSGCMCCM3R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM3R)

For information about available fields see [`mod@csgcmccm3r`] module*/
pub type CSGCMCCM3R = crate::Reg<csgcmccm3r::CSGCMCCM3Rrs>;
///context swap register
pub mod csgcmccm3r;
/**CSGCMCCM4R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM4R)

For information about available fields see [`mod@csgcmccm4r`] module*/
pub type CSGCMCCM4R = crate::Reg<csgcmccm4r::CSGCMCCM4Rrs>;
///context swap register
pub mod csgcmccm4r;
/**CSGCMCCM5R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM5R)

For information about available fields see [`mod@csgcmccm5r`] module*/
pub type CSGCMCCM5R = crate::Reg<csgcmccm5r::CSGCMCCM5Rrs>;
///context swap register
pub mod csgcmccm5r;
/**CSGCMCCM6R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM6R)

For information about available fields see [`mod@csgcmccm6r`] module*/
pub type CSGCMCCM6R = crate::Reg<csgcmccm6r::CSGCMCCM6Rrs>;
///context swap register
pub mod csgcmccm6r;
/**CSGCMCCM7R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCMCCM7R)

For information about available fields see [`mod@csgcmccm7r`] module*/
pub type CSGCMCCM7R = crate::Reg<csgcmccm7r::CSGCMCCM7Rrs>;
///context swap register
pub mod csgcmccm7r;
/**CSGCM0R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM0R)

For information about available fields see [`mod@csgcm0r`] module*/
pub type CSGCM0R = crate::Reg<csgcm0r::CSGCM0Rrs>;
///context swap register
pub mod csgcm0r;
/**CSGCM1R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM1R)

For information about available fields see [`mod@csgcm1r`] module*/
pub type CSGCM1R = crate::Reg<csgcm1r::CSGCM1Rrs>;
///context swap register
pub mod csgcm1r;
/**CSGCM2R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM2R)

For information about available fields see [`mod@csgcm2r`] module*/
pub type CSGCM2R = crate::Reg<csgcm2r::CSGCM2Rrs>;
///context swap register
pub mod csgcm2r;
/**CSGCM3R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM3R)

For information about available fields see [`mod@csgcm3r`] module*/
pub type CSGCM3R = crate::Reg<csgcm3r::CSGCM3Rrs>;
///context swap register
pub mod csgcm3r;
/**CSGCM4R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM4R)

For information about available fields see [`mod@csgcm4r`] module*/
pub type CSGCM4R = crate::Reg<csgcm4r::CSGCM4Rrs>;
///context swap register
pub mod csgcm4r;
/**CSGCM5R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM5R)

For information about available fields see [`mod@csgcm5r`] module*/
pub type CSGCM5R = crate::Reg<csgcm5r::CSGCM5Rrs>;
///context swap register
pub mod csgcm5r;
/**CSGCM6R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM6R)

For information about available fields see [`mod@csgcm6r`] module*/
pub type CSGCM6R = crate::Reg<csgcm6r::CSGCM6Rrs>;
///context swap register
pub mod csgcm6r;
/**CSGCM7R (rw) register accessor: context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:CSGCM7R)

For information about available fields see [`mod@csgcm7r`] module*/
pub type CSGCM7R = crate::Reg<csgcm7r::CSGCM7Rrs>;
///context swap register
pub mod csgcm7r;
