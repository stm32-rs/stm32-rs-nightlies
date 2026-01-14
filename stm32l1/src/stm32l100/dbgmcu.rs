#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb1_fz: APB1_FZ,
    apb2_fz: APB2_FZ,
}
impl RegisterBlock {
    ///0x00 - DBGMCU_IDCODE
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    ///0x04 - Debug MCU configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - Debug MCU APB1 freeze register1
    #[inline(always)]
    pub const fn apb1_fz(&self) -> &APB1_FZ {
        &self.apb1_fz
    }
    ///0x0c - Debug MCU APB1 freeze register 2
    #[inline(always)]
    pub const fn apb2_fz(&self) -> &APB2_FZ {
        &self.apb2_fz
    }
}
/**IDCODE (r) register accessor: DBGMCU_IDCODE

You can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#DBGMCU:IDCODE)

For information about available fields see [`mod@idcode`] module*/
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
///DBGMCU_IDCODE
pub mod idcode;
/**CR (rw) register accessor: Debug MCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Debug MCU configuration register
pub mod cr;
/**APB1_FZ (rw) register accessor: Debug MCU APB1 freeze register1

You can [`read`](crate::Reg::read) this register and get [`apb1_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#DBGMCU:APB1_FZ)

For information about available fields see [`mod@apb1_fz`] module*/
pub type APB1_FZ = crate::Reg<apb1_fz::APB1_FZrs>;
///Debug MCU APB1 freeze register1
pub mod apb1_fz;
/**APB2_FZ (rw) register accessor: Debug MCU APB1 freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb2_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#DBGMCU:APB2_FZ)

For information about available fields see [`mod@apb2_fz`] module*/
pub type APB2_FZ = crate::Reg<apb2_fz::APB2_FZrs>;
///Debug MCU APB1 freeze register 2
pub mod apb2_fz;
