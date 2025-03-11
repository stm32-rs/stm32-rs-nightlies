#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    ///0x00 - Power and clock gating control register
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
/**PCGCCTL (rw) register accessor: Power and clock gating control register

You can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#OTG_HS_PWRCLK:PCGCCTL)

For information about available fields see [`mod@pcgcctl`] module*/
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTLrs>;
///Power and clock gating control register
pub mod pcgcctl;
