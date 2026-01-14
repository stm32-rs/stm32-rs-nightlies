#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csr: CSR,
    ccr: CCR,
    cdr: CDR,
}
impl RegisterBlock {
    ///0x00 - ADC Common status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x04 - ADC common control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x08 - ADC common regular data register for dual and triple modes
    #[inline(always)]
    pub const fn cdr(&self) -> &CDR {
        &self.cdr
    }
}
/**CSR (r) register accessor: ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#C_ADC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///ADC Common status register
pub mod csr;
/**CCR (rw) register accessor: ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#C_ADC:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC common control register
pub mod ccr;
/**CDR (r) register accessor: ADC common regular data register for dual and triple modes

You can [`read`](crate::Reg::read) this register and get [`cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#C_ADC:CDR)

For information about available fields see [`mod@cdr`] module*/
pub type CDR = crate::Reg<cdr::CDRrs>;
///ADC common regular data register for dual and triple modes
pub mod cdr;
