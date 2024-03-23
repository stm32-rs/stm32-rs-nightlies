#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timbcr: TIMBCR,
    timbisr: TIMBISR,
    timbicr: TIMBICR,
    timbdier: TIMBDIER,
    cntr: CNTR,
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
    timbcr2: TIMBCR2,
    beefr3: BEEFR3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timbcr(&self) -> &TIMBCR {
        &self.timbcr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timbisr(&self) -> &TIMBISR {
        &self.timbisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timbicr(&self) -> &TIMBICR {
        &self.timbicr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timbdier(&self) -> &TIMBDIER {
        &self.timbdier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perbr(&self) -> &PERBR {
        &self.perbr
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repbr(&self) -> &REPBR {
        &self.repbr
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1br(&self) -> &CMP1BR {
        &self.cmp1br
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cbr(&self) -> &CMP1CBR {
        &self.cmp1cbr
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2br(&self) -> &CMP2BR {
        &self.cmp2br
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3br(&self) -> &CMP3BR {
        &self.cmp3br
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4br(&self) -> &CMP4BR {
        &self.cmp4br
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1br(&self) -> &CPT1BR {
        &self.cpt1br
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2br(&self) -> &CPT2BR {
        &self.cpt2br
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtbr(&self) -> &DTBR {
        &self.dtbr
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn setb1r(&self) -> &SETB1R {
        &self.setb1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rstb1r(&self) -> &RSTB1R {
        &self.rstb1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn setb2r(&self) -> &SETB2R {
        &self.setb2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rstb2r(&self) -> &RSTB2R {
        &self.rstb2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefbr1(&self) -> &EEFBR1 {
        &self.eefbr1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefbr2(&self) -> &EEFBR2 {
        &self.eefbr2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstbr(&self) -> &RSTBR {
        &self.rstbr
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpbr(&self) -> &CHPBR {
        &self.chpbr
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1bcr(&self) -> &CPT1BCR {
        &self.cpt1bcr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2bcr(&self) -> &CPT2BCR {
        &self.cpt2bcr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outbr(&self) -> &OUTBR {
        &self.outbr
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltbr(&self) -> &FLTBR {
        &self.fltbr
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timbcr2(&self) -> &TIMBCR2 {
        &self.timbcr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn beefr3(&self) -> &BEEFR3 {
        &self.beefr3
    }
}
#[doc = "TIMBCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbcr`]
module"]
pub type TIMBCR = crate::Reg<timbcr::TIMBCRrs>;
#[doc = "Timerx Control Register"]
pub mod timbcr;
#[doc = "TIMBISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbisr`]
module"]
pub type TIMBISR = crate::Reg<timbisr::TIMBISRrs>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timbisr;
#[doc = "TIMBICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbicr`]
module"]
pub type TIMBICR = crate::Reg<timbicr::TIMBICRrs>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timbicr;
#[doc = "TIMBDIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbdier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbdier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbdier`]
module"]
pub type TIMBDIER = crate::Reg<timbdier::TIMBDIERrs>;
#[doc = "TIMxDIER"]
pub mod timbdier;
#[doc = "CNTR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
pub type CNTR = crate::Reg<cntr::CNTRrs>;
#[doc = "Timerx Counter Register"]
pub mod cntr;
#[doc = "PERBR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbr`]
module"]
pub type PERBR = crate::Reg<perbr::PERBRrs>;
#[doc = "Timerx Period Register"]
pub mod perbr;
#[doc = "REPBR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repbr`]
module"]
pub type REPBR = crate::Reg<repbr::REPBRrs>;
#[doc = "Timerx Repetition Register"]
pub mod repbr;
#[doc = "CMP1BR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1br`]
module"]
pub type CMP1BR = crate::Reg<cmp1br::CMP1BRrs>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1br;
#[doc = "CMP1CBR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cbr`]
module"]
pub type CMP1CBR = crate::Reg<cmp1cbr::CMP1CBRrs>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cbr;
#[doc = "CMP2BR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2br`]
module"]
pub type CMP2BR = crate::Reg<cmp2br::CMP2BRrs>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2br;
#[doc = "CMP3BR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3br`]
module"]
pub type CMP3BR = crate::Reg<cmp3br::CMP3BRrs>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3br;
#[doc = "CMP4BR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4br`]
module"]
pub type CMP4BR = crate::Reg<cmp4br::CMP4BRrs>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4br;
#[doc = "CPT1BR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1br`]
module"]
pub type CPT1BR = crate::Reg<cpt1br::CPT1BRrs>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1br;
#[doc = "CPT2BR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2br`]
module"]
pub type CPT2BR = crate::Reg<cpt2br::CPT2BRrs>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2br;
#[doc = "DTBR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtbr`]
module"]
pub type DTBR = crate::Reg<dtbr::DTBRrs>;
#[doc = "Timerx Deadtime Register"]
pub mod dtbr;
#[doc = "SETB1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setb1r`]
module"]
pub type SETB1R = crate::Reg<setb1r::SETB1Rrs>;
#[doc = "Timerx Output1 Set Register"]
pub mod setb1r;
#[doc = "RSTB1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstb1r`]
module"]
pub type RSTB1R = crate::Reg<rstb1r::RSTB1Rrs>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstb1r;
#[doc = "SETB2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setb2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setb2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setb2r`]
module"]
pub type SETB2R = crate::Reg<setb2r::SETB2Rrs>;
#[doc = "Timerx Output2 Set Register"]
pub mod setb2r;
#[doc = "RSTB2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstb2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstb2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstb2r`]
module"]
pub type RSTB2R = crate::Reg<rstb2r::RSTB2Rrs>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstb2r;
#[doc = "EEFBR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefbr1`]
module"]
pub type EEFBR1 = crate::Reg<eefbr1::EEFBR1rs>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefbr1;
#[doc = "EEFBR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefbr2`]
module"]
pub type EEFBR2 = crate::Reg<eefbr2::EEFBR2rs>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefbr2;
#[doc = "RSTBR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstbr`]
module"]
pub type RSTBR = crate::Reg<rstbr::RSTBRrs>;
#[doc = "TimerA Reset Register"]
pub mod rstbr;
#[doc = "CHPBR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpbr`]
module"]
pub type CHPBR = crate::Reg<chpbr::CHPBRrs>;
#[doc = "Timerx Chopper Register"]
pub mod chpbr;
#[doc = "CPT1BCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1bcr`]
module"]
pub type CPT1BCR = crate::Reg<cpt1bcr::CPT1BCRrs>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1bcr;
#[doc = "CPT2BCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2bcr`]
module"]
pub type CPT2BCR = crate::Reg<cpt2bcr::CPT2BCRrs>;
#[doc = "CPT2xCR"]
pub mod cpt2bcr;
#[doc = "OUTBR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbr`]
module"]
pub type OUTBR = crate::Reg<outbr::OUTBRrs>;
#[doc = "Timerx Output Register"]
pub mod outbr;
#[doc = "FLTBR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltbr`]
module"]
pub type FLTBR = crate::Reg<fltbr::FLTBRrs>;
#[doc = "Timerx Fault Register"]
pub mod fltbr;
#[doc = "TIMBCR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbcr2`]
module"]
pub type TIMBCR2 = crate::Reg<timbcr2::TIMBCR2rs>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timbcr2;
#[doc = "BEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`beefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`beefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@beefr3`]
module"]
pub type BEEFR3 = crate::Reg<beefr3::BEEFR3rs>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod beefr3;
