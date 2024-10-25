#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    timecr: TIMECR,
    timeisr: TIMEISR,
    timeicr: TIMEICR,
    timedier: TIMEDIER,
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
    timecr2: TIMECR2,
    eeefr3: EEEFR3,
}
impl RegisterBlock {
    ///0x00 - Timerx Control Register
    #[inline(always)]
    pub const fn timecr(&self) -> &TIMECR {
        &self.timecr
    }
    ///0x04 - Timerx Interrupt Status Register
    #[inline(always)]
    pub const fn timeisr(&self) -> &TIMEISR {
        &self.timeisr
    }
    ///0x08 - Timerx Interrupt Clear Register
    #[inline(always)]
    pub const fn timeicr(&self) -> &TIMEICR {
        &self.timeicr
    }
    ///0x0c - TIMxDIER
    #[inline(always)]
    pub const fn timedier(&self) -> &TIMEDIER {
        &self.timedier
    }
    ///0x10 - Timerx Counter Register
    #[inline(always)]
    pub const fn cnter(&self) -> &CNTER {
        &self.cnter
    }
    ///0x14 - Timerx Period Register
    #[inline(always)]
    pub const fn perer(&self) -> &PERER {
        &self.perer
    }
    ///0x18 - Timerx Repetition Register
    #[inline(always)]
    pub const fn reper(&self) -> &REPER {
        &self.reper
    }
    ///0x1c - Timerx Compare 1 Register
    #[inline(always)]
    pub const fn cmp1er(&self) -> &CMP1ER {
        &self.cmp1er
    }
    ///0x20 - Timerx Compare 1 Compound Register
    #[inline(always)]
    pub const fn cmp1cer(&self) -> &CMP1CER {
        &self.cmp1cer
    }
    ///0x24 - Timerx Compare 2 Register
    #[inline(always)]
    pub const fn cmp2er(&self) -> &CMP2ER {
        &self.cmp2er
    }
    ///0x28 - Timerx Compare 3 Register
    #[inline(always)]
    pub const fn cmp3er(&self) -> &CMP3ER {
        &self.cmp3er
    }
    ///0x2c - Timerx Compare 4 Register
    #[inline(always)]
    pub const fn cmp4er(&self) -> &CMP4ER {
        &self.cmp4er
    }
    ///0x30 - Timerx Capture 1 Register
    #[inline(always)]
    pub const fn cpt1er(&self) -> &CPT1ER {
        &self.cpt1er
    }
    ///0x34 - Timerx Capture 2 Register
    #[inline(always)]
    pub const fn cpt2er(&self) -> &CPT2ER {
        &self.cpt2er
    }
    ///0x38 - Timerx Deadtime Register
    #[inline(always)]
    pub const fn dter(&self) -> &DTER {
        &self.dter
    }
    ///0x3c - Timerx Output1 Set Register
    #[inline(always)]
    pub const fn sete1r(&self) -> &SETE1R {
        &self.sete1r
    }
    ///0x40 - Timerx Output1 Reset Register
    #[inline(always)]
    pub const fn rste1r(&self) -> &RSTE1R {
        &self.rste1r
    }
    ///0x44 - Timerx Output2 Set Register
    #[inline(always)]
    pub const fn sete2r(&self) -> &SETE2R {
        &self.sete2r
    }
    ///0x48 - Timerx Output2 Reset Register
    #[inline(always)]
    pub const fn rste2r(&self) -> &RSTE2R {
        &self.rste2r
    }
    ///0x4c - Timerx External Event Filtering Register 1
    #[inline(always)]
    pub const fn eefer1(&self) -> &EEFER1 {
        &self.eefer1
    }
    ///0x50 - Timerx External Event Filtering Register 2
    #[inline(always)]
    pub const fn eefer2(&self) -> &EEFER2 {
        &self.eefer2
    }
    ///0x54 - TimerA Reset Register
    #[inline(always)]
    pub const fn rster(&self) -> &RSTER {
        &self.rster
    }
    ///0x58 - Timerx Chopper Register
    #[inline(always)]
    pub const fn chper(&self) -> &CHPER {
        &self.chper
    }
    ///0x5c - Timerx Capture 2 Control Register
    #[inline(always)]
    pub const fn cpt1ecr(&self) -> &CPT1ECR {
        &self.cpt1ecr
    }
    ///0x60 - CPT2xCR
    #[inline(always)]
    pub const fn cpt2ecr(&self) -> &CPT2ECR {
        &self.cpt2ecr
    }
    ///0x64 - Timerx Output Register
    #[inline(always)]
    pub const fn outer(&self) -> &OUTER {
        &self.outer
    }
    ///0x68 - Timerx Fault Register
    #[inline(always)]
    pub const fn flter(&self) -> &FLTER {
        &self.flter
    }
    ///0x6c - HRTIM Timerx Control Register 2
    #[inline(always)]
    pub const fn timecr2(&self) -> &TIMECR2 {
        &self.timecr2
    }
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    #[inline(always)]
    pub const fn eeefr3(&self) -> &EEEFR3 {
        &self.eeefr3
    }
}
/**TIMECR (rw) register accessor: Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`timecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:TIMECR)

For information about available fields see [`mod@timecr`]
module*/
pub type TIMECR = crate::Reg<timecr::TIMECRrs>;
///Timerx Control Register
pub mod timecr;
/**TIMEISR (r) register accessor: Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`timeisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:TIMEISR)

For information about available fields see [`mod@timeisr`]
module*/
pub type TIMEISR = crate::Reg<timeisr::TIMEISRrs>;
///Timerx Interrupt Status Register
pub mod timeisr;
/**TIMEICR (w) register accessor: Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:TIMEICR)

For information about available fields see [`mod@timeicr`]
module*/
pub type TIMEICR = crate::Reg<timeicr::TIMEICRrs>;
///Timerx Interrupt Clear Register
pub mod timeicr;
/**TIMEDIER (rw) register accessor: TIMxDIER

You can [`read`](crate::Reg::read) this register and get [`timedier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timedier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:TIMEDIER)

For information about available fields see [`mod@timedier`]
module*/
pub type TIMEDIER = crate::Reg<timedier::TIMEDIERrs>;
///TIMxDIER
pub mod timedier;
/**CNTER (rw) register accessor: Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cnter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CNTER)

For information about available fields see [`mod@cnter`]
module*/
pub type CNTER = crate::Reg<cnter::CNTERrs>;
///Timerx Counter Register
pub mod cnter;
/**PERER (rw) register accessor: Timerx Period Register

You can [`read`](crate::Reg::read) this register and get [`perer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:PERER)

For information about available fields see [`mod@perer`]
module*/
pub type PERER = crate::Reg<perer::PERERrs>;
///Timerx Period Register
pub mod perer;
/**REPER (rw) register accessor: Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`reper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:REPER)

For information about available fields see [`mod@reper`]
module*/
pub type REPER = crate::Reg<reper::REPERrs>;
///Timerx Repetition Register
pub mod reper;
/**CMP1ER (rw) register accessor: Timerx Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CMP1ER)

For information about available fields see [`mod@cmp1er`]
module*/
pub type CMP1ER = crate::Reg<cmp1er::CMP1ERrs>;
///Timerx Compare 1 Register
pub mod cmp1er;
/**CMP1CER (rw) register accessor: Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1cer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CMP1CER)

For information about available fields see [`mod@cmp1cer`]
module*/
pub type CMP1CER = crate::Reg<cmp1cer::CMP1CERrs>;
///Timerx Compare 1 Compound Register
pub mod cmp1cer;
/**CMP2ER (rw) register accessor: Timerx Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`cmp2er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CMP2ER)

For information about available fields see [`mod@cmp2er`]
module*/
pub type CMP2ER = crate::Reg<cmp2er::CMP2ERrs>;
///Timerx Compare 2 Register
pub mod cmp2er;
/**CMP3ER (rw) register accessor: Timerx Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`cmp3er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CMP3ER)

For information about available fields see [`mod@cmp3er`]
module*/
pub type CMP3ER = crate::Reg<cmp3er::CMP3ERrs>;
///Timerx Compare 3 Register
pub mod cmp3er;
/**CMP4ER (rw) register accessor: Timerx Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`cmp4er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CMP4ER)

For information about available fields see [`mod@cmp4er`]
module*/
pub type CMP4ER = crate::Reg<cmp4er::CMP4ERrs>;
///Timerx Compare 4 Register
pub mod cmp4er;
/**CPT1ER (r) register accessor: Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1er::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CPT1ER)

For information about available fields see [`mod@cpt1er`]
module*/
pub type CPT1ER = crate::Reg<cpt1er::CPT1ERrs>;
///Timerx Capture 1 Register
pub mod cpt1er;
/**CPT2ER (r) register accessor: Timerx Capture 2 Register

You can [`read`](crate::Reg::read) this register and get [`cpt2er::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CPT2ER)

For information about available fields see [`mod@cpt2er`]
module*/
pub type CPT2ER = crate::Reg<cpt2er::CPT2ERrs>;
///Timerx Capture 2 Register
pub mod cpt2er;
/**DTER (rw) register accessor: Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:DTER)

For information about available fields see [`mod@dter`]
module*/
pub type DTER = crate::Reg<dter::DTERrs>;
///Timerx Deadtime Register
pub mod dter;
/**SETE1R (rw) register accessor: Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`sete1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sete1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:SETE1R)

For information about available fields see [`mod@sete1r`]
module*/
pub type SETE1R = crate::Reg<sete1r::SETE1Rrs>;
///Timerx Output1 Set Register
pub mod sete1r;
/**RSTE1R (rw) register accessor: Timerx Output1 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rste1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rste1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:RSTE1R)

For information about available fields see [`mod@rste1r`]
module*/
pub type RSTE1R = crate::Reg<rste1r::RSTE1Rrs>;
///Timerx Output1 Reset Register
pub mod rste1r;
/**SETE2R (rw) register accessor: Timerx Output2 Set Register

You can [`read`](crate::Reg::read) this register and get [`sete2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sete2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:SETE2R)

For information about available fields see [`mod@sete2r`]
module*/
pub type SETE2R = crate::Reg<sete2r::SETE2Rrs>;
///Timerx Output2 Set Register
pub mod sete2r;
/**RSTE2R (rw) register accessor: Timerx Output2 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rste2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rste2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:RSTE2R)

For information about available fields see [`mod@rste2r`]
module*/
pub type RSTE2R = crate::Reg<rste2r::RSTE2Rrs>;
///Timerx Output2 Reset Register
pub mod rste2r;
/**EEFER1 (rw) register accessor: Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:EEFER1)

For information about available fields see [`mod@eefer1`]
module*/
pub type EEFER1 = crate::Reg<eefer1::EEFER1rs>;
///Timerx External Event Filtering Register 1
pub mod eefer1;
/**EEFER2 (rw) register accessor: Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:EEFER2)

For information about available fields see [`mod@eefer2`]
module*/
pub type EEFER2 = crate::Reg<eefer2::EEFER2rs>;
///Timerx External Event Filtering Register 2
pub mod eefer2;
/**RSTER (rw) register accessor: TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rster::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rster::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:RSTER)

For information about available fields see [`mod@rster`]
module*/
pub type RSTER = crate::Reg<rster::RSTERrs>;
///TimerA Reset Register
pub mod rster;
/**CHPER (rw) register accessor: Timerx Chopper Register

You can [`read`](crate::Reg::read) this register and get [`chper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CHPER)

For information about available fields see [`mod@chper`]
module*/
pub type CHPER = crate::Reg<chper::CHPERrs>;
///Timerx Chopper Register
pub mod chper;
/**CPT1ECR (rw) register accessor: Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CPT1ECR)

For information about available fields see [`mod@cpt1ecr`]
module*/
pub type CPT1ECR = crate::Reg<cpt1ecr::CPT1ECRrs>;
///Timerx Capture 2 Control Register
pub mod cpt1ecr;
/**CPT2ECR (rw) register accessor: CPT2xCR

You can [`read`](crate::Reg::read) this register and get [`cpt2ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt2ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CPT2ECR)

For information about available fields see [`mod@cpt2ecr`]
module*/
pub type CPT2ECR = crate::Reg<cpt2ecr::CPT2ECRrs>;
///CPT2xCR
pub mod cpt2ecr;
/**OUTER (rw) register accessor: Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:OUTER)

For information about available fields see [`mod@outer`]
module*/
pub type OUTER = crate::Reg<outer::OUTERrs>;
///Timerx Output Register
pub mod outer;
/**FLTER (rw) register accessor: Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`flter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:FLTER)

For information about available fields see [`mod@flter`]
module*/
pub type FLTER = crate::Reg<flter::FLTERrs>;
///Timerx Fault Register
pub mod flter;
/**TIMECR2 (rw) register accessor: HRTIM Timerx Control Register 2

You can [`read`](crate::Reg::read) this register and get [`timecr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:TIMECR2)

For information about available fields see [`mod@timecr2`]
module*/
pub type TIMECR2 = crate::Reg<timecr2::TIMECR2rs>;
///HRTIM Timerx Control Register 2
pub mod timecr2;
/**EEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3

You can [`read`](crate::Reg::read) this register and get [`eeefr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeefr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:EEEFR3)

For information about available fields see [`mod@eeefr3`]
module*/
pub type EEEFR3 = crate::Reg<eeefr3::EEEFR3rs>;
///HRTIM Timerx External Event Filtering Register 3
pub mod eeefr3;
