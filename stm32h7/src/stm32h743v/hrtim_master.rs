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
    _reserved8: [u8; 0x04],
    cmp2r: CMP2R,
    cmp3r: CMP3R,
    cmp4r: CMP4R,
}
impl RegisterBlock {
    ///0x00 - Master Timer Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Master Timer Interrupt Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x08 - Master Timer Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x0c - MDIER4
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - Master Timer Counter Register
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    ///0x14 - Master Timer Period Register
    #[inline(always)]
    pub const fn perr(&self) -> &PERR {
        &self.perr
    }
    ///0x18 - Master Timer Repetition Register
    #[inline(always)]
    pub const fn repr(&self) -> &REPR {
        &self.repr
    }
    ///0x1c - Master Timer Compare 1 Register
    #[inline(always)]
    pub const fn cmp1r(&self) -> &CMP1R {
        &self.cmp1r
    }
    ///0x24 - Master Timer Compare 2 Register
    #[inline(always)]
    pub const fn cmp2r(&self) -> &CMP2R {
        &self.cmp2r
    }
    ///0x28 - Master Timer Compare 3 Register
    #[inline(always)]
    pub const fn cmp3r(&self) -> &CMP3R {
        &self.cmp3r
    }
    ///0x2c - Master Timer Compare 4 Register
    #[inline(always)]
    pub const fn cmp4r(&self) -> &CMP4R {
        &self.cmp4r
    }
}
/**CR (rw) register accessor: Master Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Master Timer Control Register
pub mod cr;
/**ISR (r) register accessor: Master Timer Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///Master Timer Interrupt Status Register
pub mod isr;
/**ICR (w) register accessor: Master Timer Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///Master Timer Interrupt Clear Register
pub mod icr;
/**DIER (rw) register accessor: MDIER4

You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:DIER)

For information about available fields see [`mod@dier`] module*/
pub type DIER = crate::Reg<dier::DIERrs>;
///MDIER4
pub mod dier;
/**CNTR (rw) register accessor: Master Timer Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:CNTR)

For information about available fields see [`mod@cntr`] module*/
pub type CNTR = crate::Reg<cntr::CNTRrs>;
///Master Timer Counter Register
pub mod cntr;
/**PERR (rw) register accessor: Master Timer Period Register

You can [`read`](crate::Reg::read) this register and get [`perr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:PERR)

For information about available fields see [`mod@perr`] module*/
pub type PERR = crate::Reg<perr::PERRrs>;
///Master Timer Period Register
pub mod perr;
/**REPR (rw) register accessor: Master Timer Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:REPR)

For information about available fields see [`mod@repr`] module*/
pub type REPR = crate::Reg<repr::REPRrs>;
///Master Timer Repetition Register
pub mod repr;
/**CMP1R (rw) register accessor: Master Timer Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:CMP1R)

For information about available fields see [`mod@cmp1r`] module*/
pub type CMP1R = crate::Reg<cmp1r::CMP1Rrs>;
///Master Timer Compare 1 Register
pub mod cmp1r;
pub use cmp1r as cmp2r;
pub use cmp1r as cmp3r;
pub use cmp1r as cmp4r;
pub use CMP1R as CMP2R;
pub use CMP1R as CMP3R;
pub use CMP1R as CMP4R;
