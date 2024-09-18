#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    fpccr: FPCCR,
    fpcar: FPCAR,
    fpscr: FPSCR,
}
impl RegisterBlock {
    ///0x00 - Floating-point context control register
    #[inline(always)]
    pub const fn fpccr(&self) -> &FPCCR {
        &self.fpccr
    }
    ///0x04 - Floating-point context address register
    #[inline(always)]
    pub const fn fpcar(&self) -> &FPCAR {
        &self.fpcar
    }
    ///0x08 - Floating-point status control register
    #[inline(always)]
    pub const fn fpscr(&self) -> &FPSCR {
        &self.fpscr
    }
}
/**FPCCR (rw) register accessor: Floating-point context control register

You can [`read`](crate::Reg::read) this register and get [`fpccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#FPU:FPCCR)

For information about available fields see [`mod@fpccr`]
module*/
pub type FPCCR = crate::Reg<fpccr::FPCCRrs>;
///Floating-point context control register
pub mod fpccr;
/**FPCAR (rw) register accessor: Floating-point context address register

You can [`read`](crate::Reg::read) this register and get [`fpcar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpcar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#FPU:FPCAR)

For information about available fields see [`mod@fpcar`]
module*/
pub type FPCAR = crate::Reg<fpcar::FPCARrs>;
///Floating-point context address register
pub mod fpcar;
/**FPSCR (rw) register accessor: Floating-point status control register

You can [`read`](crate::Reg::read) this register and get [`fpscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#FPU:FPSCR)

For information about available fields see [`mod@fpscr`]
module*/
pub type FPSCR = crate::Reg<fpscr::FPSCRrs>;
///Floating-point status control register
pub mod fpscr;
