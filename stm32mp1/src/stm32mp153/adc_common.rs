#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csr: CSR,
    _reserved1: [u8; 0x04],
    ccr: CCR,
    cdr: CDR,
    cdr2: CDR2,
}
impl RegisterBlock {
    ///0x00 - ADC Common status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x08 - ADC common control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x0c - Common regular data register for dual mode
    #[inline(always)]
    pub const fn cdr(&self) -> &CDR {
        &self.cdr
    }
    ///0x10 - Common regular data register for dual mode
    #[inline(always)]
    pub const fn cdr2(&self) -> &CDR2 {
        &self.cdr2
    }
}
/**CSR (r) register accessor: ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC_common:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///ADC Common status register
pub mod csr;
/**CCR (rw) register accessor: ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC_common:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC common control register
pub mod ccr;
/**CDR (r) register accessor: Common regular data register for dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC_common:CDR)

For information about available fields see [`mod@cdr`] module*/
pub type CDR = crate::Reg<cdr::CDRrs>;
///Common regular data register for dual mode
pub mod cdr;
/**CDR2 (r) register accessor: Common regular data register for dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC_common:CDR2)

For information about available fields see [`mod@cdr2`] module*/
pub type CDR2 = crate::Reg<cdr2::CDR2rs>;
///Common regular data register for dual mode
pub mod cdr2;
