#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    csr: CSR,
    _reserved1: [u8; 0x04],
    ccr: CCR,
    cdr: CDR,
    cdr2: CDR2,
}
impl RegisterBlock {
    ///0x300 - ADC12 common status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x308 - ADC12 common control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x30c - ADC12 common regular data register for Dual mode
    #[inline(always)]
    pub const fn cdr(&self) -> &CDR {
        &self.cdr
    }
    ///0x310 - ADC12 common regular data register for Dual mode
    #[inline(always)]
    pub const fn cdr2(&self) -> &CDR2 {
        &self.cdr2
    }
}
/**CSR (r) register accessor: ADC12 common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC12:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///ADC12 common status register
pub mod csr;
/**CCR (rw) register accessor: ADC12 common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC12:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC12 common control register
pub mod ccr;
/**CDR (r) register accessor: ADC12 common regular data register for Dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC12:CDR)

For information about available fields see [`mod@cdr`] module*/
pub type CDR = crate::Reg<cdr::CDRrs>;
///ADC12 common regular data register for Dual mode
pub mod cdr;
/**CDR2 (r) register accessor: ADC12 common regular data register for Dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC12:CDR2)

For information about available fields see [`mod@cdr2`] module*/
pub type CDR2 = crate::Reg<cdr2::CDR2rs>;
///ADC12 common regular data register for Dual mode
pub mod cdr2;
