#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb1fzr1: APB1FZR1,
    apb1fzr2: APB1FZR2,
    apb2fzr: APB2FZR,
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
    pub const fn apb1fzr1(&self) -> &APB1FZR1 {
        &self.apb1fzr1
    }
    ///0x0c - Debug MCU APB1 freeze register 2
    #[inline(always)]
    pub const fn apb1fzr2(&self) -> &APB1FZR2 {
        &self.apb1fzr2
    }
    ///0x10 - Debug MCU APB2 freeze register
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
    }
}
/**IDCODE (r) register accessor: DBGMCU_IDCODE

You can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#DBGMCU:IDCODE)

For information about available fields see [`mod@idcode`] module*/
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
///DBGMCU_IDCODE
pub mod idcode;
/**CR (rw) register accessor: Debug MCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Debug MCU configuration register
pub mod cr;
/**APB1FZR1 (rw) register accessor: Debug MCU APB1 freeze register1

You can [`read`](crate::Reg::read) this register and get [`apb1fzr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#DBGMCU:APB1FZR1)

For information about available fields see [`mod@apb1fzr1`] module*/
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1rs>;
///Debug MCU APB1 freeze register1
pub mod apb1fzr1;
/**APB1FZR2 (rw) register accessor: Debug MCU APB1 freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb1fzr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#DBGMCU:APB1FZR2)

For information about available fields see [`mod@apb1fzr2`] module*/
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2rs>;
///Debug MCU APB1 freeze register 2
pub mod apb1fzr2;
/**APB2FZR (rw) register accessor: Debug MCU APB2 freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#DBGMCU:APB2FZR)

For information about available fields see [`mod@apb2fzr`] module*/
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
///Debug MCU APB2 freeze register
pub mod apb2fzr;
