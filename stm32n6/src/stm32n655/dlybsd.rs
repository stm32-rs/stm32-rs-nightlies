#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfg: CFG,
    status: STATUS,
}
impl RegisterBlock {
    ///0x00 - Delay block SDMMC DLL configuration
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    ///0x04 - Delay block SDMMC DLL status
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
}
/**CFG (rw) register accessor: Delay block SDMMC DLL configuration

You can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DLYBSD:CFG)

For information about available fields see [`mod@cfg`] module*/
pub type CFG = crate::Reg<cfg::CFGrs>;
///Delay block SDMMC DLL configuration
pub mod cfg;
/**STATUS (rw) register accessor: Delay block SDMMC DLL status

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DLYBSD:STATUS)

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUSrs>;
///Delay block SDMMC DLL status
pub mod status;
