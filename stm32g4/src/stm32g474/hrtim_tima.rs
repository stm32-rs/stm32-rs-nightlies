#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timacr: TIMACR,
    timaisr: TIMAISR,
    timaicr: TIMAICR,
    timadier: TIMADIER,
    cntar: CNTAR,
    perar: PERAR,
    repar: REPAR,
    cmp1ar: CMP1AR,
    cmp1car: CMP1CAR,
    cmp2ar: CMP2AR,
    cmp3ar: CMP3AR,
    cmp4ar: CMP4AR,
    cpt1ar: CPT1AR,
    cpt2ar: CPT2AR,
    dtar: DTAR,
    seta1r: SETA1R,
    rsta1r: RSTA1R,
    seta2r: SETA2R,
    rsta2r: RSTA2R,
    eefar1: EEFAR1,
    eefar2: EEFAR2,
    rstar: RSTAR,
    chpar: CHPAR,
    cpt1acr: CPT1ACR,
    cpt2acr: CPT2ACR,
    outar: OUTAR,
    fltar: FLTAR,
    timacr2: TIMACR2,
    aeefr3: AEEFR3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timacr(&self) -> &TIMACR {
        &self.timacr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timaisr(&self) -> &TIMAISR {
        &self.timaisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timaicr(&self) -> &TIMAICR {
        &self.timaicr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timadier(&self) -> &TIMADIER {
        &self.timadier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntar(&self) -> &CNTAR {
        &self.cntar
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perar(&self) -> &PERAR {
        &self.perar
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repar(&self) -> &REPAR {
        &self.repar
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1ar(&self) -> &CMP1AR {
        &self.cmp1ar
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1car(&self) -> &CMP1CAR {
        &self.cmp1car
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2ar(&self) -> &CMP2AR {
        &self.cmp2ar
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3ar(&self) -> &CMP3AR {
        &self.cmp3ar
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4ar(&self) -> &CMP4AR {
        &self.cmp4ar
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1ar(&self) -> &CPT1AR {
        &self.cpt1ar
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2ar(&self) -> &CPT2AR {
        &self.cpt2ar
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtar(&self) -> &DTAR {
        &self.dtar
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn seta1r(&self) -> &SETA1R {
        &self.seta1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rsta1r(&self) -> &RSTA1R {
        &self.rsta1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn seta2r(&self) -> &SETA2R {
        &self.seta2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rsta2r(&self) -> &RSTA2R {
        &self.rsta2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefar1(&self) -> &EEFAR1 {
        &self.eefar1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefar2(&self) -> &EEFAR2 {
        &self.eefar2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstar(&self) -> &RSTAR {
        &self.rstar
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpar(&self) -> &CHPAR {
        &self.chpar
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1acr(&self) -> &CPT1ACR {
        &self.cpt1acr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2acr(&self) -> &CPT2ACR {
        &self.cpt2acr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outar(&self) -> &OUTAR {
        &self.outar
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltar(&self) -> &FLTAR {
        &self.fltar
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timacr2(&self) -> &TIMACR2 {
        &self.timacr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn aeefr3(&self) -> &AEEFR3 {
        &self.aeefr3
    }
}
#[doc = "TIMACR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timacr`]
module"]
pub type TIMACR = crate::Reg<timacr::TIMACRrs>;
#[doc = "Timerx Control Register"]
pub mod timacr;
#[doc = "TIMAISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timaisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timaisr`]
module"]
pub type TIMAISR = crate::Reg<timaisr::TIMAISRrs>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timaisr;
#[doc = "TIMAICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timaicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timaicr`]
module"]
pub type TIMAICR = crate::Reg<timaicr::TIMAICRrs>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timaicr;
#[doc = "TIMADIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timadier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timadier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timadier`]
module"]
pub type TIMADIER = crate::Reg<timadier::TIMADIERrs>;
#[doc = "TIMxDIER"]
pub mod timadier;
#[doc = "CNTAR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntar`]
module"]
pub type CNTAR = crate::Reg<cntar::CNTARrs>;
#[doc = "Timerx Counter Register"]
pub mod cntar;
#[doc = "PERAR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perar`]
module"]
pub type PERAR = crate::Reg<perar::PERARrs>;
#[doc = "Timerx Period Register"]
pub mod perar;
#[doc = "REPAR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repar`]
module"]
pub type REPAR = crate::Reg<repar::REPARrs>;
#[doc = "Timerx Repetition Register"]
pub mod repar;
#[doc = "CMP1AR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1ar`]
module"]
pub type CMP1AR = crate::Reg<cmp1ar::CMP1ARrs>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1ar;
#[doc = "CMP1CAR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1car`]
module"]
pub type CMP1CAR = crate::Reg<cmp1car::CMP1CARrs>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1car;
#[doc = "CMP2AR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2ar`]
module"]
pub type CMP2AR = crate::Reg<cmp2ar::CMP2ARrs>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2ar;
#[doc = "CMP3AR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3ar`]
module"]
pub type CMP3AR = crate::Reg<cmp3ar::CMP3ARrs>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3ar;
#[doc = "CMP4AR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4ar`]
module"]
pub type CMP4AR = crate::Reg<cmp4ar::CMP4ARrs>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4ar;
#[doc = "CPT1AR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1ar`]
module"]
pub type CPT1AR = crate::Reg<cpt1ar::CPT1ARrs>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1ar;
#[doc = "CPT2AR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2ar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2ar`]
module"]
pub type CPT2AR = crate::Reg<cpt2ar::CPT2ARrs>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2ar;
#[doc = "DTAR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtar`]
module"]
pub type DTAR = crate::Reg<dtar::DTARrs>;
#[doc = "Timerx Deadtime Register"]
pub mod dtar;
#[doc = "SETA1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seta1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seta1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seta1r`]
module"]
pub type SETA1R = crate::Reg<seta1r::SETA1Rrs>;
#[doc = "Timerx Output1 Set Register"]
pub mod seta1r;
#[doc = "RSTA1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsta1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsta1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsta1r`]
module"]
pub type RSTA1R = crate::Reg<rsta1r::RSTA1Rrs>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rsta1r;
#[doc = "SETA2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seta2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seta2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seta2r`]
module"]
pub type SETA2R = crate::Reg<seta2r::SETA2Rrs>;
#[doc = "Timerx Output2 Set Register"]
pub mod seta2r;
#[doc = "RSTA2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsta2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsta2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsta2r`]
module"]
pub type RSTA2R = crate::Reg<rsta2r::RSTA2Rrs>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rsta2r;
#[doc = "EEFAR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefar1`]
module"]
pub type EEFAR1 = crate::Reg<eefar1::EEFAR1rs>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefar1;
#[doc = "EEFAR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefar2`]
module"]
pub type EEFAR2 = crate::Reg<eefar2::EEFAR2rs>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefar2;
#[doc = "RSTAR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstar`]
module"]
pub type RSTAR = crate::Reg<rstar::RSTARrs>;
#[doc = "TimerA Reset Register"]
pub mod rstar;
#[doc = "CHPAR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpar`]
module"]
pub type CHPAR = crate::Reg<chpar::CHPARrs>;
#[doc = "Timerx Chopper Register"]
pub mod chpar;
#[doc = "CPT1ACR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1acr`]
module"]
pub type CPT1ACR = crate::Reg<cpt1acr::CPT1ACRrs>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1acr;
#[doc = "CPT2ACR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2acr`]
module"]
pub type CPT2ACR = crate::Reg<cpt2acr::CPT2ACRrs>;
#[doc = "CPT2xCR"]
pub mod cpt2acr;
#[doc = "OUTAR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outar`]
module"]
pub type OUTAR = crate::Reg<outar::OUTARrs>;
#[doc = "Timerx Output Register"]
pub mod outar;
#[doc = "FLTAR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltar`]
module"]
pub type FLTAR = crate::Reg<fltar::FLTARrs>;
#[doc = "Timerx Fault Register"]
pub mod fltar;
#[doc = "TIMACR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timacr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timacr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timacr2`]
module"]
pub type TIMACR2 = crate::Reg<timacr2::TIMACR2rs>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timacr2;
#[doc = "AEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeefr3`]
module"]
pub type AEEFR3 = crate::Reg<aeefr3::AEEFR3rs>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod aeefr3;
