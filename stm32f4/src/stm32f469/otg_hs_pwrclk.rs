#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pcgcr: PCGCR,
}
impl RegisterBlock {
    ///0x00 - Power and clock gating control register
    #[inline(always)]
    pub const fn pcgcr(&self) -> &PCGCR {
        &self.pcgcr
    }
}
/**PCGCR (rw) register accessor: Power and clock gating control register

You can [`read`](crate::Reg::read) this register and get [`pcgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#OTG_HS_PWRCLK:PCGCR)

For information about available fields see [`mod@pcgcr`] module*/
pub type PCGCR = crate::Reg<pcgcr::PCGCRrs>;
///Power and clock gating control register
pub mod pcgcr;
