#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timdcr: TIMDCR,
    timdisr: TIMDISR,
    timdicr: TIMDICR,
    timddier5: TIMDDIER5,
    cntdr: CNTDR,
    perdr: PERDR,
    repdr: REPDR,
    cmp1dr: CMP1DR,
    cmp1cdr: CMP1CDR,
    cmp2dr: CMP2DR,
    cmp3dr: CMP3DR,
    cmp4dr: CMP4DR,
    cpt1dr: CPT1DR,
    cpt2dr: CPT2DR,
    dtdr: DTDR,
    setd1r: SETD1R,
    rstd1r: RSTD1R,
    setd2r: SETD2R,
    rstd2r: RSTD2R,
    eefdr1: EEFDR1,
    eefdr2: EEFDR2,
    rstdr: RSTDR,
    chpdr: CHPDR,
    cpt1dcr: CPT1DCR,
    cpt2dcr: CPT2DCR,
    outdr: OUTDR,
    fltdr: FLTDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timdcr(&self) -> &TIMDCR {
        &self.timdcr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timdisr(&self) -> &TIMDISR {
        &self.timdisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timdicr(&self) -> &TIMDICR {
        &self.timdicr
    }
    #[doc = "0x0c - TIMxDIER5"]
    #[inline(always)]
    pub const fn timddier5(&self) -> &TIMDDIER5 {
        &self.timddier5
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntdr(&self) -> &CNTDR {
        &self.cntdr
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perdr(&self) -> &PERDR {
        &self.perdr
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repdr(&self) -> &REPDR {
        &self.repdr
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1dr(&self) -> &CMP1DR {
        &self.cmp1dr
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cdr(&self) -> &CMP1CDR {
        &self.cmp1cdr
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2dr(&self) -> &CMP2DR {
        &self.cmp2dr
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3dr(&self) -> &CMP3DR {
        &self.cmp3dr
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4dr(&self) -> &CMP4DR {
        &self.cmp4dr
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1dr(&self) -> &CPT1DR {
        &self.cpt1dr
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2dr(&self) -> &CPT2DR {
        &self.cpt2dr
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtdr(&self) -> &DTDR {
        &self.dtdr
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn setd1r(&self) -> &SETD1R {
        &self.setd1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rstd1r(&self) -> &RSTD1R {
        &self.rstd1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn setd2r(&self) -> &SETD2R {
        &self.setd2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rstd2r(&self) -> &RSTD2R {
        &self.rstd2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefdr1(&self) -> &EEFDR1 {
        &self.eefdr1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefdr2(&self) -> &EEFDR2 {
        &self.eefdr2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstdr(&self) -> &RSTDR {
        &self.rstdr
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpdr(&self) -> &CHPDR {
        &self.chpdr
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1dcr(&self) -> &CPT1DCR {
        &self.cpt1dcr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2dcr(&self) -> &CPT2DCR {
        &self.cpt2dcr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outdr(&self) -> &OUTDR {
        &self.outdr
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltdr(&self) -> &FLTDR {
        &self.fltdr
    }
}
#[doc = "TIMDCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timdcr`]
module"]
pub type TIMDCR = crate::Reg<timdcr::TIMDCRrs>;
#[doc = "Timerx Control Register"]
pub mod timdcr;
#[doc = "TIMDISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timdisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timdisr`]
module"]
pub type TIMDISR = crate::Reg<timdisr::TIMDISRrs>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timdisr;
#[doc = "TIMDICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timdicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timdicr`]
module"]
pub type TIMDICR = crate::Reg<timdicr::TIMDICRrs>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timdicr;
#[doc = "TIMDDIER5 (rw) register accessor: TIMxDIER5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timddier5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timddier5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timddier5`]
module"]
pub type TIMDDIER5 = crate::Reg<timddier5::TIMDDIER5rs>;
#[doc = "TIMxDIER5"]
pub mod timddier5;
#[doc = "CNTDR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntdr`]
module"]
pub type CNTDR = crate::Reg<cntdr::CNTDRrs>;
#[doc = "Timerx Counter Register"]
pub mod cntdr;
#[doc = "PERDR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perdr`]
module"]
pub type PERDR = crate::Reg<perdr::PERDRrs>;
#[doc = "Timerx Period Register"]
pub mod perdr;
#[doc = "REPDR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repdr`]
module"]
pub type REPDR = crate::Reg<repdr::REPDRrs>;
#[doc = "Timerx Repetition Register"]
pub mod repdr;
#[doc = "CMP1DR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1dr`]
module"]
pub type CMP1DR = crate::Reg<cmp1dr::CMP1DRrs>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1dr;
#[doc = "CMP1CDR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cdr`]
module"]
pub type CMP1CDR = crate::Reg<cmp1cdr::CMP1CDRrs>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cdr;
#[doc = "CMP2DR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2dr`]
module"]
pub type CMP2DR = crate::Reg<cmp2dr::CMP2DRrs>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2dr;
#[doc = "CMP3DR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3dr`]
module"]
pub type CMP3DR = crate::Reg<cmp3dr::CMP3DRrs>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3dr;
#[doc = "CMP4DR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4dr`]
module"]
pub type CMP4DR = crate::Reg<cmp4dr::CMP4DRrs>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4dr;
#[doc = "CPT1DR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1dr`]
module"]
pub type CPT1DR = crate::Reg<cpt1dr::CPT1DRrs>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1dr;
#[doc = "CPT2DR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2dr`]
module"]
pub type CPT2DR = crate::Reg<cpt2dr::CPT2DRrs>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2dr;
#[doc = "DTDR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtdr`]
module"]
pub type DTDR = crate::Reg<dtdr::DTDRrs>;
#[doc = "Timerx Deadtime Register"]
pub mod dtdr;
#[doc = "SETD1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setd1r`]
module"]
pub type SETD1R = crate::Reg<setd1r::SETD1Rrs>;
#[doc = "Timerx Output1 Set Register"]
pub mod setd1r;
#[doc = "RSTD1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstd1r`]
module"]
pub type RSTD1R = crate::Reg<rstd1r::RSTD1Rrs>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstd1r;
#[doc = "SETD2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setd2r`]
module"]
pub type SETD2R = crate::Reg<setd2r::SETD2Rrs>;
#[doc = "Timerx Output2 Set Register"]
pub mod setd2r;
#[doc = "RSTD2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstd2r`]
module"]
pub type RSTD2R = crate::Reg<rstd2r::RSTD2Rrs>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstd2r;
#[doc = "EEFDR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefdr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefdr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefdr1`]
module"]
pub type EEFDR1 = crate::Reg<eefdr1::EEFDR1rs>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefdr1;
#[doc = "EEFDR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefdr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefdr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefdr2`]
module"]
pub type EEFDR2 = crate::Reg<eefdr2::EEFDR2rs>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefdr2;
#[doc = "RSTDR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstdr`]
module"]
pub type RSTDR = crate::Reg<rstdr::RSTDRrs>;
#[doc = "TimerA Reset Register"]
pub mod rstdr;
#[doc = "CHPDR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpdr`]
module"]
pub type CHPDR = crate::Reg<chpdr::CHPDRrs>;
#[doc = "Timerx Chopper Register"]
pub mod chpdr;
#[doc = "CPT1DCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1dcr`]
module"]
pub type CPT1DCR = crate::Reg<cpt1dcr::CPT1DCRrs>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1dcr;
#[doc = "CPT2DCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2dcr`]
module"]
pub type CPT2DCR = crate::Reg<cpt2dcr::CPT2DCRrs>;
#[doc = "CPT2xCR"]
pub mod cpt2dcr;
#[doc = "OUTDR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outdr`]
module"]
pub type OUTDR = crate::Reg<outdr::OUTDRrs>;
#[doc = "Timerx Output Register"]
pub mod outdr;
#[doc = "FLTDR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltdr`]
module"]
pub type FLTDR = crate::Reg<fltdr::FLTDRrs>;
#[doc = "Timerx Fault Register"]
pub mod fltdr;
