#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timecr: TIMECR,
    timeisr: TIMEISR,
    timeicr: TIMEICR,
    timedier5: TIMEDIER5,
    cnter: CNTER,
    perer: PERER,
    reper: REPER,
    cmp1er: CMP1ER,
    cmp1cer: CMP1CER,
    cmp2er: CMP2ER,
    cmp3er: CMP3ER,
    cmp4er: CMP4ER,
    cpt1er: CPT1ER,
    cpt2er: CPT2ER,
    dter: DTER,
    sete1r: SETE1R,
    rste1r: RSTE1R,
    sete2r: SETE2R,
    rste2r: RSTE2R,
    eefer1: EEFER1,
    eefer2: EEFER2,
    rster: RSTER,
    chper: CHPER,
    cpt1ecr: CPT1ECR,
    cpt2ecr: CPT2ECR,
    outer: OUTER,
    flter: FLTER,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timecr(&self) -> &TIMECR {
        &self.timecr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timeisr(&self) -> &TIMEISR {
        &self.timeisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timeicr(&self) -> &TIMEICR {
        &self.timeicr
    }
    #[doc = "0x0c - TIMxDIER5"]
    #[inline(always)]
    pub const fn timedier5(&self) -> &TIMEDIER5 {
        &self.timedier5
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cnter(&self) -> &CNTER {
        &self.cnter
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perer(&self) -> &PERER {
        &self.perer
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn reper(&self) -> &REPER {
        &self.reper
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1er(&self) -> &CMP1ER {
        &self.cmp1er
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cer(&self) -> &CMP1CER {
        &self.cmp1cer
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2er(&self) -> &CMP2ER {
        &self.cmp2er
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3er(&self) -> &CMP3ER {
        &self.cmp3er
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4er(&self) -> &CMP4ER {
        &self.cmp4er
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1er(&self) -> &CPT1ER {
        &self.cpt1er
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2er(&self) -> &CPT2ER {
        &self.cpt2er
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dter(&self) -> &DTER {
        &self.dter
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn sete1r(&self) -> &SETE1R {
        &self.sete1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rste1r(&self) -> &RSTE1R {
        &self.rste1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn sete2r(&self) -> &SETE2R {
        &self.sete2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rste2r(&self) -> &RSTE2R {
        &self.rste2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefer1(&self) -> &EEFER1 {
        &self.eefer1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefer2(&self) -> &EEFER2 {
        &self.eefer2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rster(&self) -> &RSTER {
        &self.rster
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chper(&self) -> &CHPER {
        &self.chper
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1ecr(&self) -> &CPT1ECR {
        &self.cpt1ecr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2ecr(&self) -> &CPT2ECR {
        &self.cpt2ecr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outer(&self) -> &OUTER {
        &self.outer
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn flter(&self) -> &FLTER {
        &self.flter
    }
}
#[doc = "TIMECR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecr`]
module"]
pub type TIMECR = crate::Reg<timecr::TIMECRrs>;
#[doc = "Timerx Control Register"]
pub mod timecr;
#[doc = "TIMEISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeisr`]
module"]
pub type TIMEISR = crate::Reg<timeisr::TIMEISRrs>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timeisr;
#[doc = "TIMEICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeicr`]
module"]
pub type TIMEICR = crate::Reg<timeicr::TIMEICRrs>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timeicr;
#[doc = "TIMEDIER5 (rw) register accessor: TIMxDIER5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timedier5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timedier5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timedier5`]
module"]
pub type TIMEDIER5 = crate::Reg<timedier5::TIMEDIER5rs>;
#[doc = "TIMxDIER5"]
pub mod timedier5;
#[doc = "CNTER (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnter`]
module"]
pub type CNTER = crate::Reg<cnter::CNTERrs>;
#[doc = "Timerx Counter Register"]
pub mod cnter;
#[doc = "PERER (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perer`]
module"]
pub type PERER = crate::Reg<perer::PERERrs>;
#[doc = "Timerx Period Register"]
pub mod perer;
#[doc = "REPER (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reper`]
module"]
pub type REPER = crate::Reg<reper::REPERrs>;
#[doc = "Timerx Repetition Register"]
pub mod reper;
#[doc = "CMP1ER (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1er`]
module"]
pub type CMP1ER = crate::Reg<cmp1er::CMP1ERrs>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1er;
#[doc = "CMP1CER (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cer`]
module"]
pub type CMP1CER = crate::Reg<cmp1cer::CMP1CERrs>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cer;
#[doc = "CMP2ER (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2er`]
module"]
pub type CMP2ER = crate::Reg<cmp2er::CMP2ERrs>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2er;
#[doc = "CMP3ER (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3er`]
module"]
pub type CMP3ER = crate::Reg<cmp3er::CMP3ERrs>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3er;
#[doc = "CMP4ER (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4er`]
module"]
pub type CMP4ER = crate::Reg<cmp4er::CMP4ERrs>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4er;
#[doc = "CPT1ER (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1er::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1er`]
module"]
pub type CPT1ER = crate::Reg<cpt1er::CPT1ERrs>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1er;
#[doc = "CPT2ER (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2er::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2er`]
module"]
pub type CPT2ER = crate::Reg<cpt2er::CPT2ERrs>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2er;
#[doc = "DTER (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dter`]
module"]
pub type DTER = crate::Reg<dter::DTERrs>;
#[doc = "Timerx Deadtime Register"]
pub mod dter;
#[doc = "SETE1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sete1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sete1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sete1r`]
module"]
pub type SETE1R = crate::Reg<sete1r::SETE1Rrs>;
#[doc = "Timerx Output1 Set Register"]
pub mod sete1r;
#[doc = "RSTE1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rste1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rste1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rste1r`]
module"]
pub type RSTE1R = crate::Reg<rste1r::RSTE1Rrs>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rste1r;
#[doc = "SETE2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sete2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sete2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sete2r`]
module"]
pub type SETE2R = crate::Reg<sete2r::SETE2Rrs>;
#[doc = "Timerx Output2 Set Register"]
pub mod sete2r;
#[doc = "RSTE2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rste2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rste2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rste2r`]
module"]
pub type RSTE2R = crate::Reg<rste2r::RSTE2Rrs>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rste2r;
#[doc = "EEFER1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefer1`]
module"]
pub type EEFER1 = crate::Reg<eefer1::EEFER1rs>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefer1;
#[doc = "EEFER2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefer2`]
module"]
pub type EEFER2 = crate::Reg<eefer2::EEFER2rs>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefer2;
#[doc = "RSTER (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rster::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rster::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rster`]
module"]
pub type RSTER = crate::Reg<rster::RSTERrs>;
#[doc = "TimerA Reset Register"]
pub mod rster;
#[doc = "CHPER (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chper`]
module"]
pub type CHPER = crate::Reg<chper::CHPERrs>;
#[doc = "Timerx Chopper Register"]
pub mod chper;
#[doc = "CPT1ECR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1ecr`]
module"]
pub type CPT1ECR = crate::Reg<cpt1ecr::CPT1ECRrs>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ecr;
#[doc = "CPT2ECR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2ecr`]
module"]
pub type CPT2ECR = crate::Reg<cpt2ecr::CPT2ECRrs>;
#[doc = "CPT2xCR"]
pub mod cpt2ecr;
#[doc = "OUTER (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outer`]
module"]
pub type OUTER = crate::Reg<outer::OUTERrs>;
#[doc = "Timerx Output Register"]
pub mod outer;
#[doc = "FLTER (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flter`]
module"]
pub type FLTER = crate::Reg<flter::FLTERrs>;
#[doc = "Timerx Fault Register"]
pub mod flter;
