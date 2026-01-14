#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    otg_hs_pcgcr: OTG_HS_PCGCR,
}
impl RegisterBlock {
    ///0x00 - Power and clock gating control register
    #[inline(always)]
    pub const fn otg_hs_pcgcr(&self) -> &OTG_HS_PCGCR {
        &self.otg_hs_pcgcr
    }
}
/**OTG_HS_PCGCR (rw) register accessor: Power and clock gating control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_pcgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_pcgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_PWRCLK:OTG_HS_PCGCR)

For information about available fields see [`mod@otg_hs_pcgcr`] module*/
pub type OTG_HS_PCGCR = crate::Reg<otg_hs_pcgcr::OTG_HS_PCGCRrs>;
///Power and clock gating control register
pub mod otg_hs_pcgcr;
