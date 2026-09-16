#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dbgmcu_idcode: DBGMCU_IDCODE,
    dbgmcu_cr: DBGMCU_CR,
    dbgmcu_apb1_fz: DBGMCU_APB1_FZ,
    dbgmcu_apb2_fz: DBGMCU_APB2_FZ,
}
impl RegisterBlock {
    ///0x00 - IDCODE
    #[inline(always)]
    pub const fn dbgmcu_idcode(&self) -> &DBGMCU_IDCODE {
        &self.dbgmcu_idcode
    }
    ///0x04 - Control Register
    #[inline(always)]
    pub const fn dbgmcu_cr(&self) -> &DBGMCU_CR {
        &self.dbgmcu_cr
    }
    ///0x08 - Debug MCU APB1 Freeze registe
    #[inline(always)]
    pub const fn dbgmcu_apb1_fz(&self) -> &DBGMCU_APB1_FZ {
        &self.dbgmcu_apb1_fz
    }
    ///0x0c - Debug MCU APB2 Freeze registe
    #[inline(always)]
    pub const fn dbgmcu_apb2_fz(&self) -> &DBGMCU_APB2_FZ {
        &self.dbgmcu_apb2_fz
    }
}
/**DBGMCU_IDCODE (r) register accessor: IDCODE

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#DBG:DBGMCU_IDCODE)

For information about available fields see [`mod@dbgmcu_idcode`] module*/
pub type DBGMCU_IDCODE = crate::Reg<dbgmcu_idcode::DBGMCU_IDCODErs>;
///IDCODE
pub mod dbgmcu_idcode;
/**DBGMCU_CR (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#DBG:DBGMCU_CR)

For information about available fields see [`mod@dbgmcu_cr`] module*/
pub type DBGMCU_CR = crate::Reg<dbgmcu_cr::DBGMCU_CRrs>;
///Control Register
pub mod dbgmcu_cr;
/**DBGMCU_APB1_FZ (rw) register accessor: Debug MCU APB1 Freeze registe

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb1_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb1_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#DBG:DBGMCU_APB1_FZ)

For information about available fields see [`mod@dbgmcu_apb1_fz`] module*/
pub type DBGMCU_APB1_FZ = crate::Reg<dbgmcu_apb1_fz::DBGMCU_APB1_FZrs>;
///Debug MCU APB1 Freeze registe
pub mod dbgmcu_apb1_fz;
/**DBGMCU_APB2_FZ (rw) register accessor: Debug MCU APB2 Freeze registe

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb2_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb2_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#DBG:DBGMCU_APB2_FZ)

For information about available fields see [`mod@dbgmcu_apb2_fz`] module*/
pub type DBGMCU_APB2_FZ = crate::Reg<dbgmcu_apb2_fz::DBGMCU_APB2_FZrs>;
///Debug MCU APB2 Freeze registe
pub mod dbgmcu_apb2_fz;
