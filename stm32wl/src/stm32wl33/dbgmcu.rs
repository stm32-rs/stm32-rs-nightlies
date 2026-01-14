#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    dbg_apb0_fz: DBG_APB0_FZ,
    dbg_apb1_fz: DBG_APB1_FZ,
}
impl RegisterBlock {
    ///0x00 - CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DBG_APB0_FZ register
    #[inline(always)]
    pub const fn dbg_apb0_fz(&self) -> &DBG_APB0_FZ {
        &self.dbg_apb0_fz
    }
    ///0x08 - DBG_APB1_FZ register
    #[inline(always)]
    pub const fn dbg_apb1_fz(&self) -> &DBG_APB1_FZ {
        &self.dbg_apb1_fz
    }
}
/**CR (rw) register accessor: CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CR register
pub mod cr;
/**DBG_APB0_FZ (rw) register accessor: DBG_APB0_FZ register

You can [`read`](crate::Reg::read) this register and get [`dbg_apb0_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb0_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DBGMCU:DBG_APB0_FZ)

For information about available fields see [`mod@dbg_apb0_fz`] module*/
pub type DBG_APB0_FZ = crate::Reg<dbg_apb0_fz::DBG_APB0_FZrs>;
///DBG_APB0_FZ register
pub mod dbg_apb0_fz;
/**DBG_APB1_FZ (rw) register accessor: DBG_APB1_FZ register

You can [`read`](crate::Reg::read) this register and get [`dbg_apb1_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb1_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DBGMCU:DBG_APB1_FZ)

For information about available fields see [`mod@dbg_apb1_fz`] module*/
pub type DBG_APB1_FZ = crate::Reg<dbg_apb1_fz::DBG_APB1_FZrs>;
///DBG_APB1_FZ register
pub mod dbg_apb1_fz;
