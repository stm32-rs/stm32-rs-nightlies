#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    timbcr: TIMBCR,
    timbisr: TIMBISR,
    timbicr: TIMBICR,
    timbdier: TIMBDIER,
    cntbr: CNTBR,
    perbr: PERBR,
    repbr: REPBR,
    cmp1br: CMP1BR,
    cmp1cbr: CMP1CBR,
    cmp2br: CMP2BR,
    cmp3br: CMP3BR,
    cmp4br: CMP4BR,
    cpt1br: CPT1BR,
    cpt2br: CPT2BR,
    dtbr: DTBR,
    setb1r: SETB1R,
    rstb1r: RSTB1R,
    setb2r: SETB2R,
    rstb2r: RSTB2R,
    eefbr1: EEFBR1,
    eefbr2: EEFBR2,
    rstbr: RSTBR,
    chpbr: CHPBR,
    cpt1bcr: CPT1BCR,
    cpt2bcr: CPT2BCR,
    outbr: OUTBR,
    fltbr: FLTBR,
}
impl RegisterBlock {
    ///0x00 - Timerx Control Register
    #[inline(always)]
    pub const fn timbcr(&self) -> &TIMBCR {
        &self.timbcr
    }
    ///0x04 - Timerx Interrupt Status Register
    #[inline(always)]
    pub const fn timbisr(&self) -> &TIMBISR {
        &self.timbisr
    }
    ///0x08 - Timerx Interrupt Clear Register
    #[inline(always)]
    pub const fn timbicr(&self) -> &TIMBICR {
        &self.timbicr
    }
    ///0x0c - TIMxDIER5
    #[inline(always)]
    pub const fn timbdier(&self) -> &TIMBDIER {
        &self.timbdier
    }
    ///0x10 - Timerx Counter Register
    #[inline(always)]
    pub const fn cntbr(&self) -> &CNTBR {
        &self.cntbr
    }
    ///0x14 - Timerx Period Register
    #[inline(always)]
    pub const fn perbr(&self) -> &PERBR {
        &self.perbr
    }
    ///0x18 - Timerx Repetition Register
    #[inline(always)]
    pub const fn repbr(&self) -> &REPBR {
        &self.repbr
    }
    ///0x1c - Timerx Compare 1 Register
    #[inline(always)]
    pub const fn cmp1br(&self) -> &CMP1BR {
        &self.cmp1br
    }
    ///0x20 - Timerx Compare 1 Compound Register
    #[inline(always)]
    pub const fn cmp1cbr(&self) -> &CMP1CBR {
        &self.cmp1cbr
    }
    ///0x24 - Timerx Compare 2 Register
    #[inline(always)]
    pub const fn cmp2br(&self) -> &CMP2BR {
        &self.cmp2br
    }
    ///0x28 - Timerx Compare 3 Register
    #[inline(always)]
    pub const fn cmp3br(&self) -> &CMP3BR {
        &self.cmp3br
    }
    ///0x2c - Timerx Compare 4 Register
    #[inline(always)]
    pub const fn cmp4br(&self) -> &CMP4BR {
        &self.cmp4br
    }
    ///0x30 - Timerx Capture 1 Register
    #[inline(always)]
    pub const fn cpt1br(&self) -> &CPT1BR {
        &self.cpt1br
    }
    ///0x34 - Timerx Capture 2 Register
    #[inline(always)]
    pub const fn cpt2br(&self) -> &CPT2BR {
        &self.cpt2br
    }
    ///0x38 - Timerx Deadtime Register
    #[inline(always)]
    pub const fn dtbr(&self) -> &DTBR {
        &self.dtbr
    }
    ///0x3c - Timerx Output1 Set Register
    #[inline(always)]
    pub const fn setb1r(&self) -> &SETB1R {
        &self.setb1r
    }
    ///0x40 - Timerx Output1 Reset Register
    #[inline(always)]
    pub const fn rstb1r(&self) -> &RSTB1R {
        &self.rstb1r
    }
    ///0x44 - Timerx Output2 Set Register
    #[inline(always)]
    pub const fn setb2r(&self) -> &SETB2R {
        &self.setb2r
    }
    ///0x48 - Timerx Output2 Reset Register
    #[inline(always)]
    pub const fn rstb2r(&self) -> &RSTB2R {
        &self.rstb2r
    }
    ///0x4c - Timerx External Event Filtering Register 1
    #[inline(always)]
    pub const fn eefbr1(&self) -> &EEFBR1 {
        &self.eefbr1
    }
    ///0x50 - Timerx External Event Filtering Register 2
    #[inline(always)]
    pub const fn eefbr2(&self) -> &EEFBR2 {
        &self.eefbr2
    }
    ///0x54 - TimerA Reset Register
    #[inline(always)]
    pub const fn rstbr(&self) -> &RSTBR {
        &self.rstbr
    }
    ///0x58 - Timerx Chopper Register
    #[inline(always)]
    pub const fn chpbr(&self) -> &CHPBR {
        &self.chpbr
    }
    ///0x5c - Timerx Capture 2 Control Register
    #[inline(always)]
    pub const fn cpt1bcr(&self) -> &CPT1BCR {
        &self.cpt1bcr
    }
    ///0x60 - CPT2xCR
    #[inline(always)]
    pub const fn cpt2bcr(&self) -> &CPT2BCR {
        &self.cpt2bcr
    }
    ///0x64 - Timerx Output Register
    #[inline(always)]
    pub const fn outbr(&self) -> &OUTBR {
        &self.outbr
    }
    ///0x68 - Timerx Fault Register
    #[inline(always)]
    pub const fn fltbr(&self) -> &FLTBR {
        &self.fltbr
    }
}
/**TIMBCR (rw) register accessor: Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`timbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:TIMBCR)

For information about available fields see [`mod@timbcr`]
module*/
pub type TIMBCR = crate::Reg<timbcr::TIMBCRrs>;
///Timerx Control Register
pub mod timbcr;
/**TIMBISR (r) register accessor: Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`timbisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:TIMBISR)

For information about available fields see [`mod@timbisr`]
module*/
pub type TIMBISR = crate::Reg<timbisr::TIMBISRrs>;
///Timerx Interrupt Status Register
pub mod timbisr;
/**TIMBICR (w) register accessor: Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timbicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:TIMBICR)

For information about available fields see [`mod@timbicr`]
module*/
pub type TIMBICR = crate::Reg<timbicr::TIMBICRrs>;
///Timerx Interrupt Clear Register
pub mod timbicr;
/**TIMBDIER (rw) register accessor: TIMxDIER5

You can [`read`](crate::Reg::read) this register and get [`timbdier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timbdier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:TIMBDIER)

For information about available fields see [`mod@timbdier`]
module*/
pub type TIMBDIER = crate::Reg<timbdier::TIMBDIERrs>;
///TIMxDIER5
pub mod timbdier;
/**CNTBR (rw) register accessor: Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CNTBR)

For information about available fields see [`mod@cntbr`]
module*/
pub type CNTBR = crate::Reg<cntbr::CNTBRrs>;
///Timerx Counter Register
pub mod cntbr;
/**PERBR (rw) register accessor: Timerx Period Register

You can [`read`](crate::Reg::read) this register and get [`perbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:PERBR)

For information about available fields see [`mod@perbr`]
module*/
pub type PERBR = crate::Reg<perbr::PERBRrs>;
///Timerx Period Register
pub mod perbr;
/**REPBR (rw) register accessor: Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:REPBR)

For information about available fields see [`mod@repbr`]
module*/
pub type REPBR = crate::Reg<repbr::REPBRrs>;
///Timerx Repetition Register
pub mod repbr;
/**CMP1BR (rw) register accessor: Timerx Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CMP1BR)

For information about available fields see [`mod@cmp1br`]
module*/
pub type CMP1BR = crate::Reg<cmp1br::CMP1BRrs>;
///Timerx Compare 1 Register
pub mod cmp1br;
/**CMP1CBR (rw) register accessor: Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1cbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CMP1CBR)

For information about available fields see [`mod@cmp1cbr`]
module*/
pub type CMP1CBR = crate::Reg<cmp1cbr::CMP1CBRrs>;
///Timerx Compare 1 Compound Register
pub mod cmp1cbr;
/**CMP2BR (rw) register accessor: Timerx Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`cmp2br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CMP2BR)

For information about available fields see [`mod@cmp2br`]
module*/
pub type CMP2BR = crate::Reg<cmp2br::CMP2BRrs>;
///Timerx Compare 2 Register
pub mod cmp2br;
/**CMP3BR (rw) register accessor: Timerx Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`cmp3br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CMP3BR)

For information about available fields see [`mod@cmp3br`]
module*/
pub type CMP3BR = crate::Reg<cmp3br::CMP3BRrs>;
///Timerx Compare 3 Register
pub mod cmp3br;
/**CMP4BR (rw) register accessor: Timerx Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`cmp4br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CMP4BR)

For information about available fields see [`mod@cmp4br`]
module*/
pub type CMP4BR = crate::Reg<cmp4br::CMP4BRrs>;
///Timerx Compare 4 Register
pub mod cmp4br;
/**CPT1BR (r) register accessor: Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CPT1BR)

For information about available fields see [`mod@cpt1br`]
module*/
pub type CPT1BR = crate::Reg<cpt1br::CPT1BRrs>;
///Timerx Capture 1 Register
pub mod cpt1br;
/**CPT2BR (r) register accessor: Timerx Capture 2 Register

You can [`read`](crate::Reg::read) this register and get [`cpt2br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CPT2BR)

For information about available fields see [`mod@cpt2br`]
module*/
pub type CPT2BR = crate::Reg<cpt2br::CPT2BRrs>;
///Timerx Capture 2 Register
pub mod cpt2br;
/**DTBR (rw) register accessor: Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:DTBR)

For information about available fields see [`mod@dtbr`]
module*/
pub type DTBR = crate::Reg<dtbr::DTBRrs>;
///Timerx Deadtime Register
pub mod dtbr;
/**SETB1R (rw) register accessor: Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`setb1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setb1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:SETB1R)

For information about available fields see [`mod@setb1r`]
module*/
pub type SETB1R = crate::Reg<setb1r::SETB1Rrs>;
///Timerx Output1 Set Register
pub mod setb1r;
/**RSTB1R (rw) register accessor: Timerx Output1 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstb1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstb1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:RSTB1R)

For information about available fields see [`mod@rstb1r`]
module*/
pub type RSTB1R = crate::Reg<rstb1r::RSTB1Rrs>;
///Timerx Output1 Reset Register
pub mod rstb1r;
/**SETB2R (rw) register accessor: Timerx Output2 Set Register

You can [`read`](crate::Reg::read) this register and get [`setb2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setb2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:SETB2R)

For information about available fields see [`mod@setb2r`]
module*/
pub type SETB2R = crate::Reg<setb2r::SETB2Rrs>;
///Timerx Output2 Set Register
pub mod setb2r;
/**RSTB2R (rw) register accessor: Timerx Output2 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstb2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstb2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:RSTB2R)

For information about available fields see [`mod@rstb2r`]
module*/
pub type RSTB2R = crate::Reg<rstb2r::RSTB2Rrs>;
///Timerx Output2 Reset Register
pub mod rstb2r;
/**EEFBR1 (rw) register accessor: Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:EEFBR1)

For information about available fields see [`mod@eefbr1`]
module*/
pub type EEFBR1 = crate::Reg<eefbr1::EEFBR1rs>;
///Timerx External Event Filtering Register 1
pub mod eefbr1;
/**EEFBR2 (rw) register accessor: Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:EEFBR2)

For information about available fields see [`mod@eefbr2`]
module*/
pub type EEFBR2 = crate::Reg<eefbr2::EEFBR2rs>;
///Timerx External Event Filtering Register 2
pub mod eefbr2;
/**RSTBR (rw) register accessor: TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:RSTBR)

For information about available fields see [`mod@rstbr`]
module*/
pub type RSTBR = crate::Reg<rstbr::RSTBRrs>;
///TimerA Reset Register
pub mod rstbr;
/**CHPBR (rw) register accessor: Timerx Chopper Register

You can [`read`](crate::Reg::read) this register and get [`chpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CHPBR)

For information about available fields see [`mod@chpbr`]
module*/
pub type CHPBR = crate::Reg<chpbr::CHPBRrs>;
///Timerx Chopper Register
pub mod chpbr;
/**CPT1BCR (rw) register accessor: Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CPT1BCR)

For information about available fields see [`mod@cpt1bcr`]
module*/
pub type CPT1BCR = crate::Reg<cpt1bcr::CPT1BCRrs>;
///Timerx Capture 2 Control Register
pub mod cpt1bcr;
/**CPT2BCR (rw) register accessor: CPT2xCR

You can [`read`](crate::Reg::read) this register and get [`cpt2bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt2bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:CPT2BCR)

For information about available fields see [`mod@cpt2bcr`]
module*/
pub type CPT2BCR = crate::Reg<cpt2bcr::CPT2BCRrs>;
///CPT2xCR
pub mod cpt2bcr;
/**OUTBR (rw) register accessor: Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:OUTBR)

For information about available fields see [`mod@outbr`]
module*/
pub type OUTBR = crate::Reg<outbr::OUTBRrs>;
///Timerx Output Register
pub mod outbr;
/**FLTBR (rw) register accessor: Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:FLTBR)

For information about available fields see [`mod@fltbr`]
module*/
pub type FLTBR = crate::Reg<fltbr::FLTBRrs>;
///Timerx Fault Register
pub mod fltbr;
