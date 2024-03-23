#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timfcr: TIMFCR,
    timfisr: TIMFISR,
    timficr: TIMFICR,
    timfdier: TIMFDIER,
    cntfr: CNTFR,
    perfr: PERFR,
    repfr: REPFR,
    cmp1fr: CMP1FR,
    cmp1cfr: CMP1CFR,
    cmp2fr: CMP2FR,
    cmp3fr: CMP3FR,
    cmp4fr: CMP4FR,
    cpt1fr: CPT1FR,
    cpt2fr: CPT2FR,
    dtfr: DTFR,
    setf1r: SETF1R,
    rstf1r: RSTF1R,
    setf2r: SETF2R,
    rstf2r: RSTF2R,
    eeffr1: EEFFR1,
    eeffr2: EEFFR2,
    rstfr: RSTFR,
    chpfr: CHPFR,
    cpt1fcr: CPT1FCR,
    cpt2fcr: CPT2FCR,
    outfr: OUTFR,
    fltfr: FLTFR,
    timfcr2: TIMFCR2,
    feefr3: FEEFR3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timfcr(&self) -> &TIMFCR {
        &self.timfcr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timfisr(&self) -> &TIMFISR {
        &self.timfisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timficr(&self) -> &TIMFICR {
        &self.timficr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timfdier(&self) -> &TIMFDIER {
        &self.timfdier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntfr(&self) -> &CNTFR {
        &self.cntfr
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perfr(&self) -> &PERFR {
        &self.perfr
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repfr(&self) -> &REPFR {
        &self.repfr
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1fr(&self) -> &CMP1FR {
        &self.cmp1fr
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cfr(&self) -> &CMP1CFR {
        &self.cmp1cfr
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2fr(&self) -> &CMP2FR {
        &self.cmp2fr
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3fr(&self) -> &CMP3FR {
        &self.cmp3fr
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4fr(&self) -> &CMP4FR {
        &self.cmp4fr
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1fr(&self) -> &CPT1FR {
        &self.cpt1fr
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2fr(&self) -> &CPT2FR {
        &self.cpt2fr
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtfr(&self) -> &DTFR {
        &self.dtfr
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn setf1r(&self) -> &SETF1R {
        &self.setf1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rstf1r(&self) -> &RSTF1R {
        &self.rstf1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn setf2r(&self) -> &SETF2R {
        &self.setf2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rstf2r(&self) -> &RSTF2R {
        &self.rstf2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eeffr1(&self) -> &EEFFR1 {
        &self.eeffr1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eeffr2(&self) -> &EEFFR2 {
        &self.eeffr2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstfr(&self) -> &RSTFR {
        &self.rstfr
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpfr(&self) -> &CHPFR {
        &self.chpfr
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1fcr(&self) -> &CPT1FCR {
        &self.cpt1fcr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2fcr(&self) -> &CPT2FCR {
        &self.cpt2fcr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outfr(&self) -> &OUTFR {
        &self.outfr
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltfr(&self) -> &FLTFR {
        &self.fltfr
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timfcr2(&self) -> &TIMFCR2 {
        &self.timfcr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn feefr3(&self) -> &FEEFR3 {
        &self.feefr3
    }
}
#[doc = "TIMFCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfcr`]
module"]
pub type TIMFCR = crate::Reg<timfcr::TIMFCRrs>;
#[doc = "Timerx Control Register"]
pub mod timfcr;
#[doc = "TIMFISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfisr`]
module"]
pub type TIMFISR = crate::Reg<timfisr::TIMFISRrs>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timfisr;
#[doc = "TIMFICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timficr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timficr`]
module"]
pub type TIMFICR = crate::Reg<timficr::TIMFICRrs>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timficr;
#[doc = "TIMFDIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfdier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timfdier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfdier`]
module"]
pub type TIMFDIER = crate::Reg<timfdier::TIMFDIERrs>;
#[doc = "TIMxDIER"]
pub mod timfdier;
#[doc = "CNTFR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntfr`]
module"]
pub type CNTFR = crate::Reg<cntfr::CNTFRrs>;
#[doc = "Timerx Counter Register"]
pub mod cntfr;
#[doc = "PERFR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perfr`]
module"]
pub type PERFR = crate::Reg<perfr::PERFRrs>;
#[doc = "Timerx Period Register"]
pub mod perfr;
#[doc = "REPFR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repfr`]
module"]
pub type REPFR = crate::Reg<repfr::REPFRrs>;
#[doc = "Timerx Repetition Register"]
pub mod repfr;
#[doc = "CMP1FR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1fr`]
module"]
pub type CMP1FR = crate::Reg<cmp1fr::CMP1FRrs>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1fr;
#[doc = "CMP1CFR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cfr`]
module"]
pub type CMP1CFR = crate::Reg<cmp1cfr::CMP1CFRrs>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cfr;
#[doc = "CMP2FR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2fr`]
module"]
pub type CMP2FR = crate::Reg<cmp2fr::CMP2FRrs>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2fr;
#[doc = "CMP3FR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3fr`]
module"]
pub type CMP3FR = crate::Reg<cmp3fr::CMP3FRrs>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3fr;
#[doc = "CMP4FR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4fr`]
module"]
pub type CMP4FR = crate::Reg<cmp4fr::CMP4FRrs>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4fr;
#[doc = "CPT1FR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1fr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1fr`]
module"]
pub type CPT1FR = crate::Reg<cpt1fr::CPT1FRrs>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1fr;
#[doc = "CPT2FR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2fr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2fr`]
module"]
pub type CPT2FR = crate::Reg<cpt2fr::CPT2FRrs>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2fr;
#[doc = "DTFR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfr`]
module"]
pub type DTFR = crate::Reg<dtfr::DTFRrs>;
#[doc = "Timerx Deadtime Register"]
pub mod dtfr;
#[doc = "SETF1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setf1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setf1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setf1r`]
module"]
pub type SETF1R = crate::Reg<setf1r::SETF1Rrs>;
#[doc = "Timerx Output1 Set Register"]
pub mod setf1r;
#[doc = "RSTF1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstf1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstf1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstf1r`]
module"]
pub type RSTF1R = crate::Reg<rstf1r::RSTF1Rrs>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstf1r;
#[doc = "SETF2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setf2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setf2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setf2r`]
module"]
pub type SETF2R = crate::Reg<setf2r::SETF2Rrs>;
#[doc = "Timerx Output2 Set Register"]
pub mod setf2r;
#[doc = "RSTF2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstf2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstf2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstf2r`]
module"]
pub type RSTF2R = crate::Reg<rstf2r::RSTF2Rrs>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstf2r;
#[doc = "EEFFR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeffr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeffr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeffr1`]
module"]
pub type EEFFR1 = crate::Reg<eeffr1::EEFFR1rs>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eeffr1;
#[doc = "EEFFR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeffr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeffr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeffr2`]
module"]
pub type EEFFR2 = crate::Reg<eeffr2::EEFFR2rs>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eeffr2;
#[doc = "RSTFR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstfr`]
module"]
pub type RSTFR = crate::Reg<rstfr::RSTFRrs>;
#[doc = "TimerA Reset Register"]
pub mod rstfr;
#[doc = "CHPFR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpfr`]
module"]
pub type CHPFR = crate::Reg<chpfr::CHPFRrs>;
#[doc = "Timerx Chopper Register"]
pub mod chpfr;
#[doc = "CPT1FCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1fcr`]
module"]
pub type CPT1FCR = crate::Reg<cpt1fcr::CPT1FCRrs>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1fcr;
#[doc = "CPT2FCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2fcr`]
module"]
pub type CPT2FCR = crate::Reg<cpt2fcr::CPT2FCRrs>;
#[doc = "CPT2xCR"]
pub mod cpt2fcr;
#[doc = "OUTFR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfr`]
module"]
pub type OUTFR = crate::Reg<outfr::OUTFRrs>;
#[doc = "Timerx Output Register"]
pub mod outfr;
#[doc = "FLTFR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltfr`]
module"]
pub type FLTFR = crate::Reg<fltfr::FLTFRrs>;
#[doc = "Timerx Fault Register"]
pub mod fltfr;
#[doc = "TIMFCR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timfcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfcr2`]
module"]
pub type TIMFCR2 = crate::Reg<timfcr2::TIMFCR2rs>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timfcr2;
#[doc = "FEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feefr3`]
module"]
pub type FEEFR3 = crate::Reg<feefr3::FEEFR3rs>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod feefr3;
