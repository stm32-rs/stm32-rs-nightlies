#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb1lfzr: APB1LFZR,
    apb1hfzr: APB1HFZR,
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
    pub const fn apb1lfzr(&self) -> &APB1LFZR {
        &self.apb1lfzr
    }
    ///0x0c - Debug MCU APB1 freeze register 2
    #[inline(always)]
    pub const fn apb1hfzr(&self) -> &APB1HFZR {
        &self.apb1hfzr
    }
    ///0x10 - Debug MCU APB2 freeze register
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
    }
}
/**IDCODE (r) register accessor: DBGMCU_IDCODE

You can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DBGMCU:IDCODE)

For information about available fields see [`mod@idcode`] module*/
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
///DBGMCU_IDCODE
pub mod idcode;
/**CR (rw) register accessor: Debug MCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Debug MCU configuration register
pub mod cr;
/**APB1LFZR (rw) register accessor: Debug MCU APB1 freeze register1

You can [`read`](crate::Reg::read) this register and get [`apb1lfzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DBGMCU:APB1LFZR)

For information about available fields see [`mod@apb1lfzr`] module*/
pub type APB1LFZR = crate::Reg<apb1lfzr::APB1LFZRrs>;
///Debug MCU APB1 freeze register1
pub mod apb1lfzr;
/**APB1HFZR (rw) register accessor: Debug MCU APB1 freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb1hfzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DBGMCU:APB1HFZR)

For information about available fields see [`mod@apb1hfzr`] module*/
pub type APB1HFZR = crate::Reg<apb1hfzr::APB1HFZRrs>;
///Debug MCU APB1 freeze register 2
pub mod apb1hfzr;
/**APB2FZR (rw) register accessor: Debug MCU APB2 freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DBGMCU:APB2FZR)

For information about available fields see [`mod@apb2fzr`] module*/
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
///Debug MCU APB2 freeze register
pub mod apb2fzr;
