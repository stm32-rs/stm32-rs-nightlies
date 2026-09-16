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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_TIME:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Timerx Control Register
pub mod cr;
pub use crate::stm32h753v::hrtim_master::cmp1r;
pub use crate::stm32h753v::hrtim_master::cmp1r as cmp2r;
pub use crate::stm32h753v::hrtim_master::cmp1r as cmp3r;
pub use crate::stm32h753v::hrtim_master::cmp1r as cmp4r;
pub use crate::stm32h753v::hrtim_master::cntr;
pub use crate::stm32h753v::hrtim_master::perr;
pub use crate::stm32h753v::hrtim_master::repr;
pub use crate::stm32h753v::hrtim_master::CMP1R;
pub use crate::stm32h753v::hrtim_master::CMP1R as CMP2R;
pub use crate::stm32h753v::hrtim_master::CMP1R as CMP3R;
pub use crate::stm32h753v::hrtim_master::CMP1R as CMP4R;
pub use crate::stm32h753v::hrtim_master::CNTR;
pub use crate::stm32h753v::hrtim_master::PERR;
pub use crate::stm32h753v::hrtim_master::REPR;
pub use crate::stm32h753v::hrtim_tima::cmp1cr;
pub use crate::stm32h753v::hrtim_tima::cpt1r;
pub use crate::stm32h753v::hrtim_tima::cpt1r as cpt2r;
pub use crate::stm32h753v::hrtim_tima::dier;
pub use crate::stm32h753v::hrtim_tima::dtr;
pub use crate::stm32h753v::hrtim_tima::icr;
pub use crate::stm32h753v::hrtim_tima::isr;
pub use crate::stm32h753v::hrtim_tima::CMP1CR;
pub use crate::stm32h753v::hrtim_tima::CPT1R;
pub use crate::stm32h753v::hrtim_tima::CPT1R as CPT2R;
pub use crate::stm32h753v::hrtim_tima::DIER;
pub use crate::stm32h753v::hrtim_tima::DTR;
pub use crate::stm32h753v::hrtim_tima::ICR;
pub use crate::stm32h753v::hrtim_tima::ISR;
/**SET1R (rw) register accessor: Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`set1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_TIME:SET1R)

For information about available fields see [`mod@set1r`] module*/
pub type SET1R = crate::Reg<set1r::SET1Rrs>;
///Timerx Output1 Set Register
pub mod set1r;
/**RST1R (rw) register accessor: Timerx Output1 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rst1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_TIME:RST1R)

For information about available fields see [`mod@rst1r`] module*/
pub type RST1R = crate::Reg<rst1r::RST1Rrs>;
///Timerx Output1 Reset Register
pub mod rst1r;
pub use crate::stm32h753v::hrtim_tima::eefr1;
pub use crate::stm32h753v::hrtim_tima::eefr2;
pub use crate::stm32h753v::hrtim_tima::EEFR1;
pub use crate::stm32h753v::hrtim_tima::EEFR2;
pub use rst1r as rst2r;
pub use set1r as set2r;
pub use RST1R as RST2R;
pub use SET1R as SET2R;
/**RSTR (rw) register accessor: TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_TIME:RSTR)

For information about available fields see [`mod@rstr`] module*/
pub type RSTR = crate::Reg<rstr::RSTRrs>;
///TimerA Reset Register
pub mod rstr;
pub use crate::stm32h753v::hrtim_tima::chpr;
pub use crate::stm32h753v::hrtim_tima::CHPR;
/**CPT1CR (rw) register accessor: Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_TIME:CPT1CR)

For information about available fields see [`mod@cpt1cr`] module*/
pub type CPT1CR = crate::Reg<cpt1cr::CPT1CRrs>;
///Timerx Capture 2 Control Register
pub mod cpt1cr;
pub use crate::stm32h753v::hrtim_tima::fltr;
pub use crate::stm32h753v::hrtim_tima::outr;
pub use crate::stm32h753v::hrtim_tima::FLTR;
pub use crate::stm32h753v::hrtim_tima::OUTR;
pub use cpt1cr as cpt2cr;
pub use CPT1CR as CPT2CR;
