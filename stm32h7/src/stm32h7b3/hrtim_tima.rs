#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    isr: ISR,
    icr: ICR,
    dier: DIER,
    cntr: CNTR,
    perr: PERR,
    repr: REPR,
    cmp1r: CMP1R,
    cmp1cr: CMP1CR,
    cmp2r: CMP2R,
    cmp3r: CMP3R,
    cmp4r: CMP4R,
    cpt1r: CPT1R,
    cpt2r: CPT2R,
    dtr: DTR,
    set1r: SET1R,
    rst1r: RST1R,
    set2r: SET2R,
    rst2r: RST2R,
    eefr1: EEFR1,
    eefr2: EEFR2,
    rstr: RSTR,
    chpr: CHPR,
    cpt1cr: CPT1CR,
    cpt2cr: CPT2CR,
    outr: OUTR,
    fltr: FLTR,
}
impl RegisterBlock {
    ///0x00 - Timerx Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Timerx Interrupt Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x08 - Timerx Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x0c - TIMxDIER5
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - Timerx Counter Register
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    ///0x14 - Timerx Period Register
    #[inline(always)]
    pub const fn perr(&self) -> &PERR {
        &self.perr
    }
    ///0x18 - Timerx Repetition Register
    #[inline(always)]
    pub const fn repr(&self) -> &REPR {
        &self.repr
    }
    ///0x1c - Timerx Compare 1 Register
    #[inline(always)]
    pub const fn cmp1r(&self) -> &CMP1R {
        &self.cmp1r
    }
    ///0x20 - Timerx Compare 1 Compound Register
    #[inline(always)]
    pub const fn cmp1cr(&self) -> &CMP1CR {
        &self.cmp1cr
    }
    ///0x24 - Timerx Compare 2 Register
    #[inline(always)]
    pub const fn cmp2r(&self) -> &CMP2R {
        &self.cmp2r
    }
    ///0x28 - Timerx Compare 3 Register
    #[inline(always)]
    pub const fn cmp3r(&self) -> &CMP3R {
        &self.cmp3r
    }
    ///0x2c - Timerx Compare 4 Register
    #[inline(always)]
    pub const fn cmp4r(&self) -> &CMP4R {
        &self.cmp4r
    }
    ///0x30 - Timerx Capture 1 Register
    #[inline(always)]
    pub const fn cpt1r(&self) -> &CPT1R {
        &self.cpt1r
    }
    ///0x34 - Timerx Capture 2 Register
    #[inline(always)]
    pub const fn cpt2r(&self) -> &CPT2R {
        &self.cpt2r
    }
    ///0x38 - Timerx Deadtime Register
    #[inline(always)]
    pub const fn dtr(&self) -> &DTR {
        &self.dtr
    }
    ///0x3c - Timerx Output1 Set Register
    #[inline(always)]
    pub const fn set1r(&self) -> &SET1R {
        &self.set1r
    }
    ///0x40 - Timerx Output1 Reset Register
    #[inline(always)]
    pub const fn rst1r(&self) -> &RST1R {
        &self.rst1r
    }
    ///0x44 - Timerx Output2 Set Register
    #[inline(always)]
    pub const fn set2r(&self) -> &SET2R {
        &self.set2r
    }
    ///0x48 - Timerx Output2 Reset Register
    #[inline(always)]
    pub const fn rst2r(&self) -> &RST2R {
        &self.rst2r
    }
    ///0x4c - Timerx External Event Filtering Register 1
    #[inline(always)]
    pub const fn eefr1(&self) -> &EEFR1 {
        &self.eefr1
    }
    ///0x50 - Timerx External Event Filtering Register 2
    #[inline(always)]
    pub const fn eefr2(&self) -> &EEFR2 {
        &self.eefr2
    }
    ///0x54 - TimerA Reset Register
    #[inline(always)]
    pub const fn rstr(&self) -> &RSTR {
        &self.rstr
    }
    ///0x58 - Timerx Chopper Register
    #[inline(always)]
    pub const fn chpr(&self) -> &CHPR {
        &self.chpr
    }
    ///0x5c - Timerx Capture 2 Control Register
    #[inline(always)]
    pub const fn cpt1cr(&self) -> &CPT1CR {
        &self.cpt1cr
    }
    ///0x60 - CPT2xCR
    #[inline(always)]
    pub const fn cpt2cr(&self) -> &CPT2CR {
        &self.cpt2cr
    }
    ///0x64 - Timerx Output Register
    #[inline(always)]
    pub const fn outr(&self) -> &OUTR {
        &self.outr
    }
    ///0x68 - Timerx Fault Register
    #[inline(always)]
    pub const fn fltr(&self) -> &FLTR {
        &self.fltr
    }
}
/**CR (rw) register accessor: Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Timerx Control Register
pub mod cr;
/**ISR (r) register accessor: Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///Timerx Interrupt Status Register
pub mod isr;
/**ICR (w) register accessor: Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///Timerx Interrupt Clear Register
pub mod icr;
/**DIER (rw) register accessor: TIMxDIER5

You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:DIER)

For information about available fields see [`mod@dier`] module*/
pub type DIER = crate::Reg<dier::DIERrs>;
///TIMxDIER5
pub mod dier;
pub use crate::stm32h7b3::hrtim_master::cmp1r;
pub use crate::stm32h7b3::hrtim_master::cntr;
pub use crate::stm32h7b3::hrtim_master::perr;
pub use crate::stm32h7b3::hrtim_master::repr;
pub use crate::stm32h7b3::hrtim_master::CMP1R;
pub use crate::stm32h7b3::hrtim_master::CNTR;
pub use crate::stm32h7b3::hrtim_master::PERR;
pub use crate::stm32h7b3::hrtim_master::REPR;
/**CMP1CR (rw) register accessor: Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:CMP1CR)

For information about available fields see [`mod@cmp1cr`] module*/
pub type CMP1CR = crate::Reg<cmp1cr::CMP1CRrs>;
///Timerx Compare 1 Compound Register
pub mod cmp1cr;
pub use crate::stm32h7b3::hrtim_master::cmp1r as cmp2r;
pub use crate::stm32h7b3::hrtim_master::cmp1r as cmp3r;
pub use crate::stm32h7b3::hrtim_master::cmp1r as cmp4r;
pub use crate::stm32h7b3::hrtim_master::CMP1R as CMP2R;
pub use crate::stm32h7b3::hrtim_master::CMP1R as CMP3R;
pub use crate::stm32h7b3::hrtim_master::CMP1R as CMP4R;
/**CPT1R (r) register accessor: Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:CPT1R)

For information about available fields see [`mod@cpt1r`] module*/
pub type CPT1R = crate::Reg<cpt1r::CPT1Rrs>;
///Timerx Capture 1 Register
pub mod cpt1r;
pub use cpt1r as cpt2r;
pub use CPT1R as CPT2R;
/**DTR (rw) register accessor: Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:DTR)

For information about available fields see [`mod@dtr`] module*/
pub type DTR = crate::Reg<dtr::DTRrs>;
///Timerx Deadtime Register
pub mod dtr;
/**SET1R (rw) register accessor: Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`set1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:SET1R)

For information about available fields see [`mod@set1r`] module*/
pub type SET1R = crate::Reg<set1r::SET1Rrs>;
///Timerx Output1 Set Register
pub mod set1r;
/**RST1R (rw) register accessor: Timerx Output1 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rst1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:RST1R)

For information about available fields see [`mod@rst1r`] module*/
pub type RST1R = crate::Reg<rst1r::RST1Rrs>;
///Timerx Output1 Reset Register
pub mod rst1r;
pub use rst1r as rst2r;
pub use set1r as set2r;
pub use RST1R as RST2R;
pub use SET1R as SET2R;
/**EEFR1 (rw) register accessor: Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:EEFR1)

For information about available fields see [`mod@eefr1`] module*/
pub type EEFR1 = crate::Reg<eefr1::EEFR1rs>;
///Timerx External Event Filtering Register 1
pub mod eefr1;
/**EEFR2 (rw) register accessor: Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:EEFR2)

For information about available fields see [`mod@eefr2`] module*/
pub type EEFR2 = crate::Reg<eefr2::EEFR2rs>;
///Timerx External Event Filtering Register 2
pub mod eefr2;
/**RSTR (rw) register accessor: TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:RSTR)

For information about available fields see [`mod@rstr`] module*/
pub type RSTR = crate::Reg<rstr::RSTRrs>;
///TimerA Reset Register
pub mod rstr;
/**CHPR (rw) register accessor: Timerx Chopper Register

You can [`read`](crate::Reg::read) this register and get [`chpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:CHPR)

For information about available fields see [`mod@chpr`] module*/
pub type CHPR = crate::Reg<chpr::CHPRrs>;
///Timerx Chopper Register
pub mod chpr;
/**CPT1CR (rw) register accessor: Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:CPT1CR)

For information about available fields see [`mod@cpt1cr`] module*/
pub type CPT1CR = crate::Reg<cpt1cr::CPT1CRrs>;
///Timerx Capture 2 Control Register
pub mod cpt1cr;
pub use cpt1cr as cpt2cr;
pub use CPT1CR as CPT2CR;
/**OUTR (rw) register accessor: Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:OUTR)

For information about available fields see [`mod@outr`] module*/
pub type OUTR = crate::Reg<outr::OUTRrs>;
///Timerx Output Register
pub mod outr;
/**FLTR (rw) register accessor: Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_TIMA:FLTR)

For information about available fields see [`mod@fltr`] module*/
pub type FLTR = crate::Reg<fltr::FLTRrs>;
///Timerx Fault Register
pub mod fltr;
