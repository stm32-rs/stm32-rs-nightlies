#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    timccr: TIMCCR,
    timcisr: TIMCISR,
    timcicr: TIMCICR,
    timcdier5: TIMCDIER5,
    cntcr: CNTCR,
    percr: PERCR,
    repcr: REPCR,
    cmp1cr: CMP1CR,
    cmp1ccr: CMP1CCR,
    cmp2cr: CMP2CR,
    cmp3cr: CMP3CR,
    cmp4cr: CMP4CR,
    cpt1cr: CPT1CR,
    cpt2cr: CPT2CR,
    dtcr: DTCR,
    setc1r: SETC1R,
    rstc1r: RSTC1R,
    setc2r: SETC2R,
    rstc2r: RSTC2R,
    eefcr1: EEFCR1,
    eefcr2: EEFCR2,
    rstcr: RSTCR,
    chpcr: CHPCR,
    cpt1ccr: CPT1CCR,
    cpt2ccr: CPT2CCR,
    outcr: OUTCR,
    fltcr: FLTCR,
}
impl RegisterBlock {
    ///0x00 - Timerx Control Register
    #[inline(always)]
    pub const fn timccr(&self) -> &TIMCCR {
        &self.timccr
    }
    ///0x04 - Timerx Interrupt Status Register
    #[inline(always)]
    pub const fn timcisr(&self) -> &TIMCISR {
        &self.timcisr
    }
    ///0x08 - Timerx Interrupt Clear Register
    #[inline(always)]
    pub const fn timcicr(&self) -> &TIMCICR {
        &self.timcicr
    }
    ///0x0c - TIMxDIER5
    #[inline(always)]
    pub const fn timcdier5(&self) -> &TIMCDIER5 {
        &self.timcdier5
    }
    ///0x10 - Timerx Counter Register
    #[inline(always)]
    pub const fn cntcr(&self) -> &CNTCR {
        &self.cntcr
    }
    ///0x14 - Timerx Period Register
    #[inline(always)]
    pub const fn percr(&self) -> &PERCR {
        &self.percr
    }
    ///0x18 - Timerx Repetition Register
    #[inline(always)]
    pub const fn repcr(&self) -> &REPCR {
        &self.repcr
    }
    ///0x1c - Timerx Compare 1 Register
    #[inline(always)]
    pub const fn cmp1cr(&self) -> &CMP1CR {
        &self.cmp1cr
    }
    ///0x20 - Timerx Compare 1 Compound Register
    #[inline(always)]
    pub const fn cmp1ccr(&self) -> &CMP1CCR {
        &self.cmp1ccr
    }
    ///0x24 - Timerx Compare 2 Register
    #[inline(always)]
    pub const fn cmp2cr(&self) -> &CMP2CR {
        &self.cmp2cr
    }
    ///0x28 - Timerx Compare 3 Register
    #[inline(always)]
    pub const fn cmp3cr(&self) -> &CMP3CR {
        &self.cmp3cr
    }
    ///0x2c - Timerx Compare 4 Register
    #[inline(always)]
    pub const fn cmp4cr(&self) -> &CMP4CR {
        &self.cmp4cr
    }
    ///0x30 - Timerx Capture 1 Register
    #[inline(always)]
    pub const fn cpt1cr(&self) -> &CPT1CR {
        &self.cpt1cr
    }
    ///0x34 - Timerx Capture 2 Register
    #[inline(always)]
    pub const fn cpt2cr(&self) -> &CPT2CR {
        &self.cpt2cr
    }
    ///0x38 - Timerx Deadtime Register
    #[inline(always)]
    pub const fn dtcr(&self) -> &DTCR {
        &self.dtcr
    }
    ///0x3c - Timerx Output1 Set Register
    #[inline(always)]
    pub const fn setc1r(&self) -> &SETC1R {
        &self.setc1r
    }
    ///0x40 - Timerx Output1 Reset Register
    #[inline(always)]
    pub const fn rstc1r(&self) -> &RSTC1R {
        &self.rstc1r
    }
    ///0x44 - Timerx Output2 Set Register
    #[inline(always)]
    pub const fn setc2r(&self) -> &SETC2R {
        &self.setc2r
    }
    ///0x48 - Timerx Output2 Reset Register
    #[inline(always)]
    pub const fn rstc2r(&self) -> &RSTC2R {
        &self.rstc2r
    }
    ///0x4c - Timerx External Event Filtering Register 1
    #[inline(always)]
    pub const fn eefcr1(&self) -> &EEFCR1 {
        &self.eefcr1
    }
    ///0x50 - Timerx External Event Filtering Register 2
    #[inline(always)]
    pub const fn eefcr2(&self) -> &EEFCR2 {
        &self.eefcr2
    }
    ///0x54 - TimerA Reset Register
    #[inline(always)]
    pub const fn rstcr(&self) -> &RSTCR {
        &self.rstcr
    }
    ///0x58 - Timerx Chopper Register
    #[inline(always)]
    pub const fn chpcr(&self) -> &CHPCR {
        &self.chpcr
    }
    ///0x5c - Timerx Capture 2 Control Register
    #[inline(always)]
    pub const fn cpt1ccr(&self) -> &CPT1CCR {
        &self.cpt1ccr
    }
    ///0x60 - CPT2xCR
    #[inline(always)]
    pub const fn cpt2ccr(&self) -> &CPT2CCR {
        &self.cpt2ccr
    }
    ///0x64 - Timerx Output Register
    #[inline(always)]
    pub const fn outcr(&self) -> &OUTCR {
        &self.outcr
    }
    ///0x68 - Timerx Fault Register
    #[inline(always)]
    pub const fn fltcr(&self) -> &FLTCR {
        &self.fltcr
    }
}
/**TIMCCR (rw) register accessor: Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`timccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:TIMCCR)

For information about available fields see [`mod@timccr`]
module*/
pub type TIMCCR = crate::Reg<timccr::TIMCCRrs>;
///Timerx Control Register
pub mod timccr;
/**TIMCISR (r) register accessor: Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`timcisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:TIMCISR)

For information about available fields see [`mod@timcisr`]
module*/
pub type TIMCISR = crate::Reg<timcisr::TIMCISRrs>;
///Timerx Interrupt Status Register
pub mod timcisr;
/**TIMCICR (w) register accessor: Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timcicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:TIMCICR)

For information about available fields see [`mod@timcicr`]
module*/
pub type TIMCICR = crate::Reg<timcicr::TIMCICRrs>;
///Timerx Interrupt Clear Register
pub mod timcicr;
/**TIMCDIER5 (rw) register accessor: TIMxDIER5

You can [`read`](crate::Reg::read) this register and get [`timcdier5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timcdier5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:TIMCDIER5)

For information about available fields see [`mod@timcdier5`]
module*/
pub type TIMCDIER5 = crate::Reg<timcdier5::TIMCDIER5rs>;
///TIMxDIER5
pub mod timcdier5;
/**CNTCR (rw) register accessor: Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CNTCR)

For information about available fields see [`mod@cntcr`]
module*/
pub type CNTCR = crate::Reg<cntcr::CNTCRrs>;
///Timerx Counter Register
pub mod cntcr;
/**PERCR (rw) register accessor: Timerx Period Register

You can [`read`](crate::Reg::read) this register and get [`percr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`percr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:PERCR)

For information about available fields see [`mod@percr`]
module*/
pub type PERCR = crate::Reg<percr::PERCRrs>;
///Timerx Period Register
pub mod percr;
/**REPCR (rw) register accessor: Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:REPCR)

For information about available fields see [`mod@repcr`]
module*/
pub type REPCR = crate::Reg<repcr::REPCRrs>;
///Timerx Repetition Register
pub mod repcr;
/**CMP1CR (rw) register accessor: Timerx Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CMP1CR)

For information about available fields see [`mod@cmp1cr`]
module*/
pub type CMP1CR = crate::Reg<cmp1cr::CMP1CRrs>;
///Timerx Compare 1 Register
pub mod cmp1cr;
/**CMP1CCR (rw) register accessor: Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CMP1CCR)

For information about available fields see [`mod@cmp1ccr`]
module*/
pub type CMP1CCR = crate::Reg<cmp1ccr::CMP1CCRrs>;
///Timerx Compare 1 Compound Register
pub mod cmp1ccr;
/**CMP2CR (rw) register accessor: Timerx Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`cmp2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CMP2CR)

For information about available fields see [`mod@cmp2cr`]
module*/
pub type CMP2CR = crate::Reg<cmp2cr::CMP2CRrs>;
///Timerx Compare 2 Register
pub mod cmp2cr;
/**CMP3CR (rw) register accessor: Timerx Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`cmp3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CMP3CR)

For information about available fields see [`mod@cmp3cr`]
module*/
pub type CMP3CR = crate::Reg<cmp3cr::CMP3CRrs>;
///Timerx Compare 3 Register
pub mod cmp3cr;
/**CMP4CR (rw) register accessor: Timerx Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`cmp4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CMP4CR)

For information about available fields see [`mod@cmp4cr`]
module*/
pub type CMP4CR = crate::Reg<cmp4cr::CMP4CRrs>;
///Timerx Compare 4 Register
pub mod cmp4cr;
/**CPT1CR (r) register accessor: Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CPT1CR)

For information about available fields see [`mod@cpt1cr`]
module*/
pub type CPT1CR = crate::Reg<cpt1cr::CPT1CRrs>;
///Timerx Capture 1 Register
pub mod cpt1cr;
/**CPT2CR (r) register accessor: Timerx Capture 2 Register

You can [`read`](crate::Reg::read) this register and get [`cpt2cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CPT2CR)

For information about available fields see [`mod@cpt2cr`]
module*/
pub type CPT2CR = crate::Reg<cpt2cr::CPT2CRrs>;
///Timerx Capture 2 Register
pub mod cpt2cr;
/**DTCR (rw) register accessor: Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:DTCR)

For information about available fields see [`mod@dtcr`]
module*/
pub type DTCR = crate::Reg<dtcr::DTCRrs>;
///Timerx Deadtime Register
pub mod dtcr;
/**SETC1R (rw) register accessor: Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`setc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:SETC1R)

For information about available fields see [`mod@setc1r`]
module*/
pub type SETC1R = crate::Reg<setc1r::SETC1Rrs>;
///Timerx Output1 Set Register
pub mod setc1r;
/**RSTC1R (rw) register accessor: Timerx Output1 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:RSTC1R)

For information about available fields see [`mod@rstc1r`]
module*/
pub type RSTC1R = crate::Reg<rstc1r::RSTC1Rrs>;
///Timerx Output1 Reset Register
pub mod rstc1r;
/**SETC2R (rw) register accessor: Timerx Output2 Set Register

You can [`read`](crate::Reg::read) this register and get [`setc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:SETC2R)

For information about available fields see [`mod@setc2r`]
module*/
pub type SETC2R = crate::Reg<setc2r::SETC2Rrs>;
///Timerx Output2 Set Register
pub mod setc2r;
/**RSTC2R (rw) register accessor: Timerx Output2 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:RSTC2R)

For information about available fields see [`mod@rstc2r`]
module*/
pub type RSTC2R = crate::Reg<rstc2r::RSTC2Rrs>;
///Timerx Output2 Reset Register
pub mod rstc2r;
/**EEFCR1 (rw) register accessor: Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:EEFCR1)

For information about available fields see [`mod@eefcr1`]
module*/
pub type EEFCR1 = crate::Reg<eefcr1::EEFCR1rs>;
///Timerx External Event Filtering Register 1
pub mod eefcr1;
/**EEFCR2 (rw) register accessor: Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:EEFCR2)

For information about available fields see [`mod@eefcr2`]
module*/
pub type EEFCR2 = crate::Reg<eefcr2::EEFCR2rs>;
///Timerx External Event Filtering Register 2
pub mod eefcr2;
/**RSTCR (rw) register accessor: TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:RSTCR)

For information about available fields see [`mod@rstcr`]
module*/
pub type RSTCR = crate::Reg<rstcr::RSTCRrs>;
///TimerA Reset Register
pub mod rstcr;
/**CHPCR (rw) register accessor: Timerx Chopper Register

You can [`read`](crate::Reg::read) this register and get [`chpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CHPCR)

For information about available fields see [`mod@chpcr`]
module*/
pub type CHPCR = crate::Reg<chpcr::CHPCRrs>;
///Timerx Chopper Register
pub mod chpcr;
/**CPT1CCR (rw) register accessor: Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CPT1CCR)

For information about available fields see [`mod@cpt1ccr`]
module*/
pub type CPT1CCR = crate::Reg<cpt1ccr::CPT1CCRrs>;
///Timerx Capture 2 Control Register
pub mod cpt1ccr;
/**CPT2CCR (rw) register accessor: CPT2xCR

You can [`read`](crate::Reg::read) this register and get [`cpt2ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt2ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CPT2CCR)

For information about available fields see [`mod@cpt2ccr`]
module*/
pub type CPT2CCR = crate::Reg<cpt2ccr::CPT2CCRrs>;
///CPT2xCR
pub mod cpt2ccr;
/**OUTCR (rw) register accessor: Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:OUTCR)

For information about available fields see [`mod@outcr`]
module*/
pub type OUTCR = crate::Reg<outcr::OUTCRrs>;
///Timerx Output Register
pub mod outcr;
/**FLTCR (rw) register accessor: Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:FLTCR)

For information about available fields see [`mod@fltcr`]
module*/
pub type FLTCR = crate::Reg<fltcr::FLTCRrs>;
///Timerx Fault Register
pub mod fltcr;
