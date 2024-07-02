#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    adc12_ccr: ADC12_CCR,
}
impl RegisterBlock {
    ///0x08 - ADC_CCR system control register
    #[inline(always)]
    pub const fn adc12_ccr(&self) -> &ADC12_CCR {
        &self.adc12_ccr
    }
}
/**ADC12_CCR (rw) register accessor: ADC_CCR system control register

You can [`read`](crate::Reg::read) this register and get [`adc12_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC12:ADC12_CCR)

For information about available fields see [`mod@adc12_ccr`]
module*/
pub type ADC12_CCR = crate::Reg<adc12_ccr::ADC12_CCRrs>;
///ADC_CCR system control register
pub mod adc12_ccr;
