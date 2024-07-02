#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    adc12_csr: ADC12_CSR,
    _reserved1: [u8; 0x04],
    adc12_ccr: ADC12_CCR,
    adc12_cdr: ADC12_CDR,
    adc12_cdr2: ADC12_CDR2,
}
impl RegisterBlock {
    ///0x00 - ADC common status register
    #[inline(always)]
    pub const fn adc12_csr(&self) -> &ADC12_CSR {
        &self.adc12_csr
    }
    ///0x08 - ADC_CCR system control register
    #[inline(always)]
    pub const fn adc12_ccr(&self) -> &ADC12_CCR {
        &self.adc12_ccr
    }
    ///0x0c - ADC common regular data register for dual mode
    #[inline(always)]
    pub const fn adc12_cdr(&self) -> &ADC12_CDR {
        &self.adc12_cdr
    }
    ///0x10 - ADC common regular data register for 32-bit dual mode
    #[inline(always)]
    pub const fn adc12_cdr2(&self) -> &ADC12_CDR2 {
        &self.adc12_cdr2
    }
}
/**ADC12_CSR (r) register accessor: ADC common status register

You can [`read`](crate::Reg::read) this register and get [`adc12_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC12_Common:ADC12_CSR)

For information about available fields see [`mod@adc12_csr`]
module*/
pub type ADC12_CSR = crate::Reg<adc12_csr::ADC12_CSRrs>;
///ADC common status register
pub mod adc12_csr;
/**ADC12_CCR (rw) register accessor: ADC_CCR system control register

You can [`read`](crate::Reg::read) this register and get [`adc12_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC12_Common:ADC12_CCR)

For information about available fields see [`mod@adc12_ccr`]
module*/
pub type ADC12_CCR = crate::Reg<adc12_ccr::ADC12_CCRrs>;
///ADC_CCR system control register
pub mod adc12_ccr;
/**ADC12_CDR (r) register accessor: ADC common regular data register for dual mode

You can [`read`](crate::Reg::read) this register and get [`adc12_cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC12_Common:ADC12_CDR)

For information about available fields see [`mod@adc12_cdr`]
module*/
pub type ADC12_CDR = crate::Reg<adc12_cdr::ADC12_CDRrs>;
///ADC common regular data register for dual mode
pub mod adc12_cdr;
/**ADC12_CDR2 (r) register accessor: ADC common regular data register for 32-bit dual mode

You can [`read`](crate::Reg::read) this register and get [`adc12_cdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC12_Common:ADC12_CDR2)

For information about available fields see [`mod@adc12_cdr2`]
module*/
pub type ADC12_CDR2 = crate::Reg<adc12_cdr2::ADC12_CDR2rs>;
///ADC common regular data register for 32-bit dual mode
pub mod adc12_cdr2;
