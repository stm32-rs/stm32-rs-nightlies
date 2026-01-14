#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ccr: CCR,
}
impl RegisterBlock {
    ///0x04 - ADC common control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
}
/**CCR (rw) register accessor: ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F401.html#ADC_Common:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC common control register
pub mod ccr;
