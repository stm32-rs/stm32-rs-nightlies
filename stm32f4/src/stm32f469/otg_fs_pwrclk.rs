#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    ///0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
/**PCGCCTL (rw) register accessor: OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)

You can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#OTG_FS_PWRCLK:PCGCCTL)

For information about available fields see [`mod@pcgcctl`] module*/
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTLrs>;
///OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
pub mod pcgcctl;
