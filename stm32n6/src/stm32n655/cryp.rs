#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dinr: DINR,
    doutr: DOUTR,
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
    ///0x00 - CRYP control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - CRYP status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - CRYP data input register
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    ///0x0c - CRYP data output register
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    ///0x10 - CRYP DMA control register
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    ///0x14 - CRYP interrupt mask set/clear register
    #[inline(always)]
    pub const fn imscr(&self) -> &IMSCR {
        &self.imscr
    }
    ///0x18 - CRYP raw interrupt status register
    #[inline(always)]
    pub const fn risr(&self) -> &RISR {
        &self.risr
    }
    ///0x1c - CRYP masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x20 - CRYP key register 0L
    #[inline(always)]
    pub const fn k0lr(&self) -> &K0LR {
        &self.k0lr
    }
    ///0x24 - CRYP key register 0R
    #[inline(always)]
    pub const fn k0rr(&self) -> &K0RR {
        &self.k0rr
    }
    ///0x28 - CRYP key register 1L
    #[inline(always)]
    pub const fn k1lr(&self) -> &K1LR {
        &self.k1lr
    }
    ///0x2c - CRYP key register 1R
    #[inline(always)]
    pub const fn k1rr(&self) -> &K1RR {
        &self.k1rr
    }
    ///0x30 - CRYP key register 2L
    #[inline(always)]
    pub const fn k2lr(&self) -> &K2LR {
        &self.k2lr
    }
    ///0x34 - CRYP key register 2R
    #[inline(always)]
    pub const fn k2rr(&self) -> &K2RR {
        &self.k2rr
    }
    ///0x38 - CRYP key register 3L
    #[inline(always)]
    pub const fn k3lr(&self) -> &K3LR {
        &self.k3lr
    }
    ///0x3c - CRYP key register 3R
    #[inline(always)]
    pub const fn k3rr(&self) -> &K3RR {
        &self.k3rr
    }
    ///0x40 - CRYP initialization vector register 0L
    #[inline(always)]
    pub const fn iv0lr(&self) -> &IV0LR {
        &self.iv0lr
    }
    ///0x44 - CRYP initialization vector register 0R
    #[inline(always)]
    pub const fn iv0rr(&self) -> &IV0RR {
        &self.iv0rr
    }
    ///0x48 - CRYP initialization vector register 1L
    #[inline(always)]
    pub const fn iv1lr(&self) -> &IV1LR {
        &self.iv1lr
    }
    ///0x4c - CRYP initialization vector register 1R
    #[inline(always)]
    pub const fn iv1rr(&self) -> &IV1RR {
        &self.iv1rr
    }
    ///0x50 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm0r(&self) -> &CSGCMCCM0R {
        &self.csgcmccm0r
    }
    ///0x54 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm1r(&self) -> &CSGCMCCM1R {
        &self.csgcmccm1r
    }
    ///0x58 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm2r(&self) -> &CSGCMCCM2R {
        &self.csgcmccm2r
    }
    ///0x5c - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm3r(&self) -> &CSGCMCCM3R {
        &self.csgcmccm3r
    }
    ///0x60 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm4r(&self) -> &CSGCMCCM4R {
        &self.csgcmccm4r
    }
    ///0x64 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm5r(&self) -> &CSGCMCCM5R {
        &self.csgcmccm5r
    }
    ///0x68 - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm6r(&self) -> &CSGCMCCM6R {
        &self.csgcmccm6r
    }
    ///0x6c - CRYP context swap GCM-CCM registers
    #[inline(always)]
    pub const fn csgcmccm7r(&self) -> &CSGCMCCM7R {
        &self.csgcmccm7r
    }
    ///0x70 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm0r(&self) -> &CSGCM0R {
        &self.csgcm0r
    }
    ///0x74 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm1r(&self) -> &CSGCM1R {
        &self.csgcm1r
    }
    ///0x78 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm2r(&self) -> &CSGCM2R {
        &self.csgcm2r
    }
    ///0x7c - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm3r(&self) -> &CSGCM3R {
        &self.csgcm3r
    }
    ///0x80 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm4r(&self) -> &CSGCM4R {
        &self.csgcm4r
    }
    ///0x84 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm5r(&self) -> &CSGCM5R {
        &self.csgcm5r
    }
    ///0x88 - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm6r(&self) -> &CSGCM6R {
        &self.csgcm6r
    }
    ///0x8c - CRYP context swap GCM registers
    #[inline(always)]
    pub const fn csgcm7r(&self) -> &CSGCM7R {
        &self.csgcm7r
    }
}
/**CR (rw) register accessor: CRYP control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CRYP control register
pub mod cr;
/**SR (r) register accessor: CRYP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///CRYP status register
pub mod sr;
/**DINR (rw) register accessor: CRYP data input register

You can [`read`](crate::Reg::read) this register and get [`dinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:DINR)

For information about available fields see [`mod@dinr`] module*/
pub type DINR = crate::Reg<dinr::DINRrs>;
///CRYP data input register
pub mod dinr;
/**DOUTR (r) register accessor: CRYP data output register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:DOUTR)

For information about available fields see [`mod@doutr`] module*/
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
///CRYP data output register
pub mod doutr;
/**DMACR (rw) register accessor: CRYP DMA control register

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:DMACR)

For information about available fields see [`mod@dmacr`] module*/
pub type DMACR = crate::Reg<dmacr::DMACRrs>;
///CRYP DMA control register
pub mod dmacr;
/**IMSCR (rw) register accessor: CRYP interrupt mask set/clear register

You can [`read`](crate::Reg::read) this register and get [`imscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:IMSCR)

For information about available fields see [`mod@imscr`] module*/
pub type IMSCR = crate::Reg<imscr::IMSCRrs>;
///CRYP interrupt mask set/clear register
pub mod imscr;
/**RISR (r) register accessor: CRYP raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`risr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:RISR)

For information about available fields see [`mod@risr`] module*/
pub type RISR = crate::Reg<risr::RISRrs>;
///CRYP raw interrupt status register
pub mod risr;
/**MISR (r) register accessor: CRYP masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///CRYP masked interrupt status register
pub mod misr;
/**K0LR (w) register accessor: CRYP key register 0L

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K0LR)

For information about available fields see [`mod@k0lr`] module*/
pub type K0LR = crate::Reg<k0lr::K0LRrs>;
///CRYP key register 0L
pub mod k0lr;
/**K0RR (w) register accessor: CRYP key register 0R

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k0rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K0RR)

For information about available fields see [`mod@k0rr`] module*/
pub type K0RR = crate::Reg<k0rr::K0RRrs>;
///CRYP key register 0R
pub mod k0rr;
/**K1LR (w) register accessor: CRYP key register 1L

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K1LR)

For information about available fields see [`mod@k1lr`] module*/
pub type K1LR = crate::Reg<k1lr::K1LRrs>;
///CRYP key register 1L
pub mod k1lr;
/**K1RR (w) register accessor: CRYP key register 1R

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k1rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K1RR)

For information about available fields see [`mod@k1rr`] module*/
pub type K1RR = crate::Reg<k1rr::K1RRrs>;
///CRYP key register 1R
pub mod k1rr;
/**K2LR (w) register accessor: CRYP key register 2L

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K2LR)

For information about available fields see [`mod@k2lr`] module*/
pub type K2LR = crate::Reg<k2lr::K2LRrs>;
///CRYP key register 2L
pub mod k2lr;
/**K2RR (w) register accessor: CRYP key register 2R

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k2rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K2RR)

For information about available fields see [`mod@k2rr`] module*/
pub type K2RR = crate::Reg<k2rr::K2RRrs>;
///CRYP key register 2R
pub mod k2rr;
/**K3LR (w) register accessor: CRYP key register 3L

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3lr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K3LR)

For information about available fields see [`mod@k3lr`] module*/
pub type K3LR = crate::Reg<k3lr::K3LRrs>;
///CRYP key register 3L
pub mod k3lr;
/**K3RR (w) register accessor: CRYP key register 3R

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3rr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:K3RR)

For information about available fields see [`mod@k3rr`] module*/
pub type K3RR = crate::Reg<k3rr::K3RRrs>;
///CRYP key register 3R
pub mod k3rr;
/**IV0LR (rw) register accessor: CRYP initialization vector register 0L

You can [`read`](crate::Reg::read) this register and get [`iv0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:IV0LR)

For information about available fields see [`mod@iv0lr`] module*/
pub type IV0LR = crate::Reg<iv0lr::IV0LRrs>;
///CRYP initialization vector register 0L
pub mod iv0lr;
/**IV0RR (rw) register accessor: CRYP initialization vector register 0R

You can [`read`](crate::Reg::read) this register and get [`iv0rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:IV0RR)

For information about available fields see [`mod@iv0rr`] module*/
pub type IV0RR = crate::Reg<iv0rr::IV0RRrs>;
///CRYP initialization vector register 0R
pub mod iv0rr;
/**IV1LR (rw) register accessor: CRYP initialization vector register 1L

You can [`read`](crate::Reg::read) this register and get [`iv1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:IV1LR)

For information about available fields see [`mod@iv1lr`] module*/
pub type IV1LR = crate::Reg<iv1lr::IV1LRrs>;
///CRYP initialization vector register 1L
pub mod iv1lr;
/**IV1RR (rw) register accessor: CRYP initialization vector register 1R

You can [`read`](crate::Reg::read) this register and get [`iv1rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv1rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:IV1RR)

For information about available fields see [`mod@iv1rr`] module*/
pub type IV1RR = crate::Reg<iv1rr::IV1RRrs>;
///CRYP initialization vector register 1R
pub mod iv1rr;
/**CSGCMCCM0R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM0R)

For information about available fields see [`mod@csgcmccm0r`] module*/
pub type CSGCMCCM0R = crate::Reg<csgcmccm0r::CSGCMCCM0Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm0r;
/**CSGCMCCM1R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM1R)

For information about available fields see [`mod@csgcmccm1r`] module*/
pub type CSGCMCCM1R = crate::Reg<csgcmccm1r::CSGCMCCM1Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm1r;
/**CSGCMCCM2R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM2R)

For information about available fields see [`mod@csgcmccm2r`] module*/
pub type CSGCMCCM2R = crate::Reg<csgcmccm2r::CSGCMCCM2Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm2r;
/**CSGCMCCM3R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM3R)

For information about available fields see [`mod@csgcmccm3r`] module*/
pub type CSGCMCCM3R = crate::Reg<csgcmccm3r::CSGCMCCM3Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm3r;
/**CSGCMCCM4R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM4R)

For information about available fields see [`mod@csgcmccm4r`] module*/
pub type CSGCMCCM4R = crate::Reg<csgcmccm4r::CSGCMCCM4Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm4r;
/**CSGCMCCM5R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM5R)

For information about available fields see [`mod@csgcmccm5r`] module*/
pub type CSGCMCCM5R = crate::Reg<csgcmccm5r::CSGCMCCM5Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm5r;
/**CSGCMCCM6R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM6R)

For information about available fields see [`mod@csgcmccm6r`] module*/
pub type CSGCMCCM6R = crate::Reg<csgcmccm6r::CSGCMCCM6Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm6r;
/**CSGCMCCM7R (rw) register accessor: CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM7R)

For information about available fields see [`mod@csgcmccm7r`] module*/
pub type CSGCMCCM7R = crate::Reg<csgcmccm7r::CSGCMCCM7Rrs>;
///CRYP context swap GCM-CCM registers
pub mod csgcmccm7r;
/**CSGCM0R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM0R)

For information about available fields see [`mod@csgcm0r`] module*/
pub type CSGCM0R = crate::Reg<csgcm0r::CSGCM0Rrs>;
///CRYP context swap GCM registers
pub mod csgcm0r;
/**CSGCM1R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM1R)

For information about available fields see [`mod@csgcm1r`] module*/
pub type CSGCM1R = crate::Reg<csgcm1r::CSGCM1Rrs>;
///CRYP context swap GCM registers
pub mod csgcm1r;
/**CSGCM2R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM2R)

For information about available fields see [`mod@csgcm2r`] module*/
pub type CSGCM2R = crate::Reg<csgcm2r::CSGCM2Rrs>;
///CRYP context swap GCM registers
pub mod csgcm2r;
/**CSGCM3R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM3R)

For information about available fields see [`mod@csgcm3r`] module*/
pub type CSGCM3R = crate::Reg<csgcm3r::CSGCM3Rrs>;
///CRYP context swap GCM registers
pub mod csgcm3r;
/**CSGCM4R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM4R)

For information about available fields see [`mod@csgcm4r`] module*/
pub type CSGCM4R = crate::Reg<csgcm4r::CSGCM4Rrs>;
///CRYP context swap GCM registers
pub mod csgcm4r;
/**CSGCM5R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM5R)

For information about available fields see [`mod@csgcm5r`] module*/
pub type CSGCM5R = crate::Reg<csgcm5r::CSGCM5Rrs>;
///CRYP context swap GCM registers
pub mod csgcm5r;
/**CSGCM6R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM6R)

For information about available fields see [`mod@csgcm6r`] module*/
pub type CSGCM6R = crate::Reg<csgcm6r::CSGCM6Rrs>;
///CRYP context swap GCM registers
pub mod csgcm6r;
/**CSGCM7R (rw) register accessor: CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM7R)

For information about available fields see [`mod@csgcm7r`] module*/
pub type CSGCM7R = crate::Reg<csgcm7r::CSGCM7Rrs>;
///CRYP context swap GCM registers
pub mod csgcm7r;
