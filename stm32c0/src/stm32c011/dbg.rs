#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dbg_idcode: DBG_IDCODE,
    dbg_cr: DBG_CR,
    dbg_apb_fz1: DBG_APB_FZ1,
    dbg_apb_fz2: DBG_APB_FZ2,
}
impl RegisterBlock {
    ///0x00 - DBG device ID code register
    #[inline(always)]
    pub const fn dbg_idcode(&self) -> &DBG_IDCODE {
        &self.dbg_idcode
    }
    ///0x04 - DBG configuration register
    #[inline(always)]
    pub const fn dbg_cr(&self) -> &DBG_CR {
        &self.dbg_cr
    }
    ///0x08 - DBG APB freeze register 1
    #[inline(always)]
    pub const fn dbg_apb_fz1(&self) -> &DBG_APB_FZ1 {
        &self.dbg_apb_fz1
    }
    ///0x0c - DBG APB freeze register 2
    #[inline(always)]
    pub const fn dbg_apb_fz2(&self) -> &DBG_APB_FZ2 {
        &self.dbg_apb_fz2
    }
}
/**DBG_IDCODE (r) register accessor: DBG device ID code register

You can [`read`](crate::Reg::read) this register and get [`dbg_idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#DBG:DBG_IDCODE)

For information about available fields see [`mod@dbg_idcode`]
module*/
pub type DBG_IDCODE = crate::Reg<dbg_idcode::DBG_IDCODErs>;
///DBG device ID code register
pub mod dbg_idcode;
/**DBG_CR (rw) register accessor: DBG configuration register

You can [`read`](crate::Reg::read) this register and get [`dbg_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#DBG:DBG_CR)

For information about available fields see [`mod@dbg_cr`]
module*/
pub type DBG_CR = crate::Reg<dbg_cr::DBG_CRrs>;
///DBG configuration register
pub mod dbg_cr;
/**DBG_APB_FZ1 (rw) register accessor: DBG APB freeze register 1

You can [`read`](crate::Reg::read) this register and get [`dbg_apb_fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb_fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#DBG:DBG_APB_FZ1)

For information about available fields see [`mod@dbg_apb_fz1`]
module*/
pub type DBG_APB_FZ1 = crate::Reg<dbg_apb_fz1::DBG_APB_FZ1rs>;
///DBG APB freeze register 1
pub mod dbg_apb_fz1;
/**DBG_APB_FZ2 (rw) register accessor: DBG APB freeze register 2

You can [`read`](crate::Reg::read) this register and get [`dbg_apb_fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb_fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#DBG:DBG_APB_FZ2)

For information about available fields see [`mod@dbg_apb_fz2`]
module*/
pub type DBG_APB_FZ2 = crate::Reg<dbg_apb_fz2::DBG_APB_FZ2rs>;
///DBG APB freeze register 2
pub mod dbg_apb_fz2;
