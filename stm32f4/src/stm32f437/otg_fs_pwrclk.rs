#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    fs_pcgcctl: FS_PCGCCTL,
}
impl RegisterBlock {
    ///0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
    #[inline(always)]
    pub const fn fs_pcgcctl(&self) -> &FS_PCGCCTL {
        &self.fs_pcgcctl
    }
}
/**FS_PCGCCTL (rw) register accessor: OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)

You can [`read`](crate::Reg::read) this register and get [`fs_pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_FS_PWRCLK:FS_PCGCCTL)

For information about available fields see [`mod@fs_pcgcctl`] module*/
pub type FS_PCGCCTL = crate::Reg<fs_pcgcctl::FS_PCGCCTLrs>;
///OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
pub mod fs_pcgcctl;
