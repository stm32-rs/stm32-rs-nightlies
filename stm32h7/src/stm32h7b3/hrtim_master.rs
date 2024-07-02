#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mcr: MCR,
    misr: MISR,
    micr: MICR,
    mdier4: MDIER4,
    mcntr: MCNTR,
    mper: MPER,
    mrep: MREP,
    mcmp1r: MCMP1R,
    _reserved8: [u8; 0x04],
    mcmp2r: MCMP2R,
    mcmp3r: MCMP3R,
    mcmp4r: MCMP4R,
}
impl RegisterBlock {
    ///0x00 - Master Timer Control Register
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    ///0x04 - Master Timer Interrupt Status Register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x08 - Master Timer Interrupt Clear Register
    #[inline(always)]
    pub const fn micr(&self) -> &MICR {
        &self.micr
    }
    ///0x0c - MDIER4
    #[inline(always)]
    pub const fn mdier4(&self) -> &MDIER4 {
        &self.mdier4
    }
    ///0x10 - Master Timer Counter Register
    #[inline(always)]
    pub const fn mcntr(&self) -> &MCNTR {
        &self.mcntr
    }
    ///0x14 - Master Timer Period Register
    #[inline(always)]
    pub const fn mper(&self) -> &MPER {
        &self.mper
    }
    ///0x18 - Master Timer Repetition Register
    #[inline(always)]
    pub const fn mrep(&self) -> &MREP {
        &self.mrep
    }
    ///0x1c - Master Timer Compare 1 Register
    #[inline(always)]
    pub const fn mcmp1r(&self) -> &MCMP1R {
        &self.mcmp1r
    }
    ///0x24 - Master Timer Compare 2 Register
    #[inline(always)]
    pub const fn mcmp2r(&self) -> &MCMP2R {
        &self.mcmp2r
    }
    ///0x28 - Master Timer Compare 3 Register
    #[inline(always)]
    pub const fn mcmp3r(&self) -> &MCMP3R {
        &self.mcmp3r
    }
    ///0x2c - Master Timer Compare 4 Register
    #[inline(always)]
    pub const fn mcmp4r(&self) -> &MCMP4R {
        &self.mcmp4r
    }
}
/**MCR (rw) register accessor: Master Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MCR)

For information about available fields see [`mod@mcr`]
module*/
pub type MCR = crate::Reg<mcr::MCRrs>;
///Master Timer Control Register
pub mod mcr;
/**MISR (r) register accessor: Master Timer Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MISR)

For information about available fields see [`mod@misr`]
module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///Master Timer Interrupt Status Register
pub mod misr;
/**MICR (w) register accessor: Master Timer Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`micr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MICR)

For information about available fields see [`mod@micr`]
module*/
pub type MICR = crate::Reg<micr::MICRrs>;
///Master Timer Interrupt Clear Register
pub mod micr;
/**MDIER4 (rw) register accessor: MDIER4

You can [`read`](crate::Reg::read) this register and get [`mdier4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdier4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MDIER4)

For information about available fields see [`mod@mdier4`]
module*/
pub type MDIER4 = crate::Reg<mdier4::MDIER4rs>;
///MDIER4
pub mod mdier4;
/**MCNTR (rw) register accessor: Master Timer Counter Register

You can [`read`](crate::Reg::read) this register and get [`mcntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MCNTR)

For information about available fields see [`mod@mcntr`]
module*/
pub type MCNTR = crate::Reg<mcntr::MCNTRrs>;
///Master Timer Counter Register
pub mod mcntr;
/**MPER (rw) register accessor: Master Timer Period Register

You can [`read`](crate::Reg::read) this register and get [`mper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MPER)

For information about available fields see [`mod@mper`]
module*/
pub type MPER = crate::Reg<mper::MPERrs>;
///Master Timer Period Register
pub mod mper;
/**MREP (rw) register accessor: Master Timer Repetition Register

You can [`read`](crate::Reg::read) this register and get [`mrep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MREP)

For information about available fields see [`mod@mrep`]
module*/
pub type MREP = crate::Reg<mrep::MREPrs>;
///Master Timer Repetition Register
pub mod mrep;
/**MCMP1R (rw) register accessor: Master Timer Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`mcmp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MCMP1R)

For information about available fields see [`mod@mcmp1r`]
module*/
pub type MCMP1R = crate::Reg<mcmp1r::MCMP1Rrs>;
///Master Timer Compare 1 Register
pub mod mcmp1r;
/**MCMP2R (rw) register accessor: Master Timer Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`mcmp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MCMP2R)

For information about available fields see [`mod@mcmp2r`]
module*/
pub type MCMP2R = crate::Reg<mcmp2r::MCMP2Rrs>;
///Master Timer Compare 2 Register
pub mod mcmp2r;
/**MCMP3R (rw) register accessor: Master Timer Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`mcmp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MCMP3R)

For information about available fields see [`mod@mcmp3r`]
module*/
pub type MCMP3R = crate::Reg<mcmp3r::MCMP3Rrs>;
///Master Timer Compare 3 Register
pub mod mcmp3r;
/**MCMP4R (rw) register accessor: Master Timer Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`mcmp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MCMP4R)

For information about available fields see [`mod@mcmp4r`]
module*/
pub type MCMP4R = crate::Reg<mcmp4r::MCMP4Rrs>;
///Master Timer Compare 4 Register
pub mod mcmp4r;
