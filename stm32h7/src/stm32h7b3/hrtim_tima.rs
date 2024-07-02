#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    timacr: TIMACR,
    timaisr: TIMAISR,
    timaicr: TIMAICR,
    timadier5: TIMADIER5,
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
}
impl RegisterBlock {
    ///0x00 - Timerx Control Register
    #[inline(always)]
    pub const fn timacr(&self) -> &TIMACR {
        &self.timacr
    }
    ///0x04 - Timerx Interrupt Status Register
    #[inline(always)]
    pub const fn timaisr(&self) -> &TIMAISR {
        &self.timaisr
    }
    ///0x08 - Timerx Interrupt Clear Register
    #[inline(always)]
    pub const fn timaicr(&self) -> &TIMAICR {
        &self.timaicr
    }
    ///0x0c - TIMxDIER5
    #[inline(always)]
    pub const fn timadier5(&self) -> &TIMADIER5 {
        &self.timadier5
    }
    ///0x10 - Timerx Counter Register
    #[inline(always)]
    pub const fn cntar(&self) -> &CNTAR {
        &self.cntar
    }
    ///0x14 - Timerx Period Register
    #[inline(always)]
    pub const fn perar(&self) -> &PERAR {
        &self.perar
    }
    ///0x18 - Timerx Repetition Register
    #[inline(always)]
    pub const fn repar(&self) -> &REPAR {
        &self.repar
    }
    ///0x1c - Timerx Compare 1 Register
    #[inline(always)]
    pub const fn cmp1ar(&self) -> &CMP1AR {
        &self.cmp1ar
    }
    ///0x20 - Timerx Compare 1 Compound Register
    #[inline(always)]
    pub const fn cmp1car(&self) -> &CMP1CAR {
        &self.cmp1car
    }
    ///0x24 - Timerx Compare 2 Register
    #[inline(always)]
    pub const fn cmp2ar(&self) -> &CMP2AR {
        &self.cmp2ar
    }
    ///0x28 - Timerx Compare 3 Register
    #[inline(always)]
    pub const fn cmp3ar(&self) -> &CMP3AR {
        &self.cmp3ar
    }
    ///0x2c - Timerx Compare 4 Register
    #[inline(always)]
    pub const fn cmp4ar(&self) -> &CMP4AR {
        &self.cmp4ar
    }
    ///0x30 - Timerx Capture 1 Register
    #[inline(always)]
    pub const fn cpt1ar(&self) -> &CPT1AR {
        &self.cpt1ar
    }
    ///0x34 - Timerx Capture 2 Register
    #[inline(always)]
    pub const fn cpt2ar(&self) -> &CPT2AR {
        &self.cpt2ar
    }
    ///0x38 - Timerx Deadtime Register
    #[inline(always)]
    pub const fn dtar(&self) -> &DTAR {
        &self.dtar
    }
    ///0x3c - Timerx Output1 Set Register
    #[inline(always)]
    pub const fn seta1r(&self) -> &SETA1R {
        &self.seta1r
    }
    ///0x40 - Timerx Output1 Reset Register
    #[inline(always)]
    pub const fn rsta1r(&self) -> &RSTA1R {
        &self.rsta1r
    }
    ///0x44 - Timerx Output2 Set Register
    #[inline(always)]
    pub const fn seta2r(&self) -> &SETA2R {
        &self.seta2r
    }
    ///0x48 - Timerx Output2 Reset Register
    #[inline(always)]
    pub const fn rsta2r(&self) -> &RSTA2R {
        &self.rsta2r
    }
    ///0x4c - Timerx External Event Filtering Register 1
    #[inline(always)]
    pub const fn eefar1(&self) -> &EEFAR1 {
        &self.eefar1
    }
    ///0x50 - Timerx External Event Filtering Register 2
    #[inline(always)]
    pub const fn eefar2(&self) -> &EEFAR2 {
        &self.eefar2
    }
    ///0x54 - TimerA Reset Register
    #[inline(always)]
    pub const fn rstar(&self) -> &RSTAR {
        &self.rstar
    }
    ///0x58 - Timerx Chopper Register
    #[inline(always)]
    pub const fn chpar(&self) -> &CHPAR {
        &self.chpar
    }
    ///0x5c - Timerx Capture 2 Control Register
    #[inline(always)]
    pub const fn cpt1acr(&self) -> &CPT1ACR {
        &self.cpt1acr
    }
    ///0x60 - CPT2xCR
    #[inline(always)]
    pub const fn cpt2acr(&self) -> &CPT2ACR {
        &self.cpt2acr
    }
    ///0x64 - Timerx Output Register
    #[inline(always)]
    pub const fn outar(&self) -> &OUTAR {
        &self.outar
    }
    ///0x68 - Timerx Fault Register
    #[inline(always)]
    pub const fn fltar(&self) -> &FLTAR {
        &self.fltar
    }
}
/**TIMACR (rw) register accessor: Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`timacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:TIMACR)

For information about available fields see [`mod@timacr`]
module*/
pub type TIMACR = crate::Reg<timacr::TIMACRrs>;
///Timerx Control Register
pub mod timacr;
/**TIMAISR (r) register accessor: Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`timaisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:TIMAISR)

For information about available fields see [`mod@timaisr`]
module*/
pub type TIMAISR = crate::Reg<timaisr::TIMAISRrs>;
///Timerx Interrupt Status Register
pub mod timaisr;
/**TIMAICR (w) register accessor: Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timaicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:TIMAICR)

For information about available fields see [`mod@timaicr`]
module*/
pub type TIMAICR = crate::Reg<timaicr::TIMAICRrs>;
///Timerx Interrupt Clear Register
pub mod timaicr;
/**TIMADIER5 (rw) register accessor: TIMxDIER5

You can [`read`](crate::Reg::read) this register and get [`timadier5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timadier5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:TIMADIER5)

For information about available fields see [`mod@timadier5`]
module*/
pub type TIMADIER5 = crate::Reg<timadier5::TIMADIER5rs>;
///TIMxDIER5
pub mod timadier5;
/**CNTAR (rw) register accessor: Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CNTAR)

For information about available fields see [`mod@cntar`]
module*/
pub type CNTAR = crate::Reg<cntar::CNTARrs>;
///Timerx Counter Register
pub mod cntar;
/**PERAR (rw) register accessor: Timerx Period Register

You can [`read`](crate::Reg::read) this register and get [`perar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:PERAR)

For information about available fields see [`mod@perar`]
module*/
pub type PERAR = crate::Reg<perar::PERARrs>;
///Timerx Period Register
pub mod perar;
/**REPAR (rw) register accessor: Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:REPAR)

For information about available fields see [`mod@repar`]
module*/
pub type REPAR = crate::Reg<repar::REPARrs>;
///Timerx Repetition Register
pub mod repar;
/**CMP1AR (rw) register accessor: Timerx Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CMP1AR)

For information about available fields see [`mod@cmp1ar`]
module*/
pub type CMP1AR = crate::Reg<cmp1ar::CMP1ARrs>;
///Timerx Compare 1 Register
pub mod cmp1ar;
/**CMP1CAR (rw) register accessor: Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1car::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1car::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CMP1CAR)

For information about available fields see [`mod@cmp1car`]
module*/
pub type CMP1CAR = crate::Reg<cmp1car::CMP1CARrs>;
///Timerx Compare 1 Compound Register
pub mod cmp1car;
/**CMP2AR (rw) register accessor: Timerx Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`cmp2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CMP2AR)

For information about available fields see [`mod@cmp2ar`]
module*/
pub type CMP2AR = crate::Reg<cmp2ar::CMP2ARrs>;
///Timerx Compare 2 Register
pub mod cmp2ar;
/**CMP3AR (rw) register accessor: Timerx Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`cmp3ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CMP3AR)

For information about available fields see [`mod@cmp3ar`]
module*/
pub type CMP3AR = crate::Reg<cmp3ar::CMP3ARrs>;
///Timerx Compare 3 Register
pub mod cmp3ar;
/**CMP4AR (rw) register accessor: Timerx Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`cmp4ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CMP4AR)

For information about available fields see [`mod@cmp4ar`]
module*/
pub type CMP4AR = crate::Reg<cmp4ar::CMP4ARrs>;
///Timerx Compare 4 Register
pub mod cmp4ar;
/**CPT1AR (r) register accessor: Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CPT1AR)

For information about available fields see [`mod@cpt1ar`]
module*/
pub type CPT1AR = crate::Reg<cpt1ar::CPT1ARrs>;
///Timerx Capture 1 Register
pub mod cpt1ar;
/**CPT2AR (r) register accessor: Timerx Capture 2 Register

You can [`read`](crate::Reg::read) this register and get [`cpt2ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CPT2AR)

For information about available fields see [`mod@cpt2ar`]
module*/
pub type CPT2AR = crate::Reg<cpt2ar::CPT2ARrs>;
///Timerx Capture 2 Register
pub mod cpt2ar;
/**DTAR (rw) register accessor: Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:DTAR)

For information about available fields see [`mod@dtar`]
module*/
pub type DTAR = crate::Reg<dtar::DTARrs>;
///Timerx Deadtime Register
pub mod dtar;
/**SETA1R (rw) register accessor: Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`seta1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seta1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:SETA1R)

For information about available fields see [`mod@seta1r`]
module*/
pub type SETA1R = crate::Reg<seta1r::SETA1Rrs>;
///Timerx Output1 Set Register
pub mod seta1r;
/**RSTA1R (rw) register accessor: Timerx Output1 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rsta1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsta1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:RSTA1R)

For information about available fields see [`mod@rsta1r`]
module*/
pub type RSTA1R = crate::Reg<rsta1r::RSTA1Rrs>;
///Timerx Output1 Reset Register
pub mod rsta1r;
/**SETA2R (rw) register accessor: Timerx Output2 Set Register

You can [`read`](crate::Reg::read) this register and get [`seta2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seta2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:SETA2R)

For information about available fields see [`mod@seta2r`]
module*/
pub type SETA2R = crate::Reg<seta2r::SETA2Rrs>;
///Timerx Output2 Set Register
pub mod seta2r;
/**RSTA2R (rw) register accessor: Timerx Output2 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rsta2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsta2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:RSTA2R)

For information about available fields see [`mod@rsta2r`]
module*/
pub type RSTA2R = crate::Reg<rsta2r::RSTA2Rrs>;
///Timerx Output2 Reset Register
pub mod rsta2r;
/**EEFAR1 (rw) register accessor: Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:EEFAR1)

For information about available fields see [`mod@eefar1`]
module*/
pub type EEFAR1 = crate::Reg<eefar1::EEFAR1rs>;
///Timerx External Event Filtering Register 1
pub mod eefar1;
/**EEFAR2 (rw) register accessor: Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:EEFAR2)

For information about available fields see [`mod@eefar2`]
module*/
pub type EEFAR2 = crate::Reg<eefar2::EEFAR2rs>;
///Timerx External Event Filtering Register 2
pub mod eefar2;
/**RSTAR (rw) register accessor: TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:RSTAR)

For information about available fields see [`mod@rstar`]
module*/
pub type RSTAR = crate::Reg<rstar::RSTARrs>;
///TimerA Reset Register
pub mod rstar;
/**CHPAR (rw) register accessor: Timerx Chopper Register

You can [`read`](crate::Reg::read) this register and get [`chpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CHPAR)

For information about available fields see [`mod@chpar`]
module*/
pub type CHPAR = crate::Reg<chpar::CHPARrs>;
///Timerx Chopper Register
pub mod chpar;
/**CPT1ACR (rw) register accessor: Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CPT1ACR)

For information about available fields see [`mod@cpt1acr`]
module*/
pub type CPT1ACR = crate::Reg<cpt1acr::CPT1ACRrs>;
///Timerx Capture 2 Control Register
pub mod cpt1acr;
/**CPT2ACR (rw) register accessor: CPT2xCR

You can [`read`](crate::Reg::read) this register and get [`cpt2acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt2acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:CPT2ACR)

For information about available fields see [`mod@cpt2acr`]
module*/
pub type CPT2ACR = crate::Reg<cpt2acr::CPT2ACRrs>;
///CPT2xCR
pub mod cpt2acr;
/**OUTAR (rw) register accessor: Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:OUTAR)

For information about available fields see [`mod@outar`]
module*/
pub type OUTAR = crate::Reg<outar::OUTARrs>;
///Timerx Output Register
pub mod outar;
/**FLTAR (rw) register accessor: Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:FLTAR)

For information about available fields see [`mod@fltar`]
module*/
pub type FLTAR = crate::Reg<fltar::FLTARrs>;
///Timerx Fault Register
pub mod fltar;
