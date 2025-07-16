#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csr: CSR,
    ccr: CCR,
}
impl RegisterBlock {
    ///0x00 - VREF control and status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x04 - calibration control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
}
/**CSR (rw) register accessor: VREF control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#VREFBUF:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///VREF control and status register
pub mod csr;
/**CCR (rw) register accessor: calibration control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#VREFBUF:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///calibration control register
pub mod ccr;
