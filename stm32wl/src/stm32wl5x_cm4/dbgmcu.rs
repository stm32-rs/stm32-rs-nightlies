#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcoder: IDCODER,
    cr: CR,
    _reserved2: [u8; 0x34],
    apb1fzr1: APB1FZR1,
    c2apb1fzr1: C2APB1FZR1,
    apb1fzr2: APB1FZR2,
    c2apb1fzr2: C2APB1FZR2,
    apb2fzr: APB2FZR,
    c2apb2fzr: C2APB2FZR,
}
impl RegisterBlock {
    ///0x00 - DBGMCU Identity Code Register
    #[inline(always)]
    pub const fn idcoder(&self) -> &IDCODER {
        &self.idcoder
    }
    ///0x04 - DBGMCU Configuration Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x3c - DBGMCU CPU1 APB1 Peripheral Freeze Register 1
    #[inline(always)]
    pub const fn apb1fzr1(&self) -> &APB1FZR1 {
        &self.apb1fzr1
    }
    ///0x40 - DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \[dual core device
    #[inline(always)]
    pub const fn c2apb1fzr1(&self) -> &C2APB1FZR1 {
        &self.c2apb1fzr1
    }
    ///0x44 - DBGMCU CPU1 APB1 Peripheral Freeze Register 2
    #[inline(always)]
    pub const fn apb1fzr2(&self) -> &APB1FZR2 {
        &self.apb1fzr2
    }
    ///0x48 - DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \[dual core device
    #[inline(always)]
    pub const fn c2apb1fzr2(&self) -> &C2APB1FZR2 {
        &self.c2apb1fzr2
    }
    ///0x4c - DBGMCU CPU1 APB2 Peripheral Freeze Register
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
    }
    ///0x50 - DBGMCU CPU2 APB2 Peripheral Freeze Register \[dual core device
    #[inline(always)]
    pub const fn c2apb2fzr(&self) -> &C2APB2FZR {
        &self.c2apb2fzr
    }
}
/**IDCODER (r) register accessor: DBGMCU Identity Code Register

You can [`read`](crate::Reg::read) this register and get [`idcoder::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:IDCODER)

For information about available fields see [`mod@idcoder`] module*/
pub type IDCODER = crate::Reg<idcoder::IDCODERrs>;
///DBGMCU Identity Code Register
pub mod idcoder;
/**CR (rw) register accessor: DBGMCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DBGMCU Configuration Register
pub mod cr;
/**APB1FZR1 (rw) register accessor: DBGMCU CPU1 APB1 Peripheral Freeze Register 1

You can [`read`](crate::Reg::read) this register and get [`apb1fzr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:APB1FZR1)

For information about available fields see [`mod@apb1fzr1`] module*/
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1rs>;
///DBGMCU CPU1 APB1 Peripheral Freeze Register 1
pub mod apb1fzr1;
/**C2APB1FZR1 (rw) register accessor: DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \[dual core device

You can [`read`](crate::Reg::read) this register and get [`c2apb1fzr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1fzr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:C2APB1FZR1)

For information about available fields see [`mod@c2apb1fzr1`] module*/
pub type C2APB1FZR1 = crate::Reg<c2apb1fzr1::C2APB1FZR1rs>;
///DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \[dual core device
pub mod c2apb1fzr1;
/**APB1FZR2 (rw) register accessor: DBGMCU CPU1 APB1 Peripheral Freeze Register 2

You can [`read`](crate::Reg::read) this register and get [`apb1fzr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:APB1FZR2)

For information about available fields see [`mod@apb1fzr2`] module*/
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2rs>;
///DBGMCU CPU1 APB1 Peripheral Freeze Register 2
pub mod apb1fzr2;
/**C2APB1FZR2 (rw) register accessor: DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \[dual core device

You can [`read`](crate::Reg::read) this register and get [`c2apb1fzr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1fzr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:C2APB1FZR2)

For information about available fields see [`mod@c2apb1fzr2`] module*/
pub type C2APB1FZR2 = crate::Reg<c2apb1fzr2::C2APB1FZR2rs>;
///DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \[dual core device
pub mod c2apb1fzr2;
/**APB2FZR (rw) register accessor: DBGMCU CPU1 APB2 Peripheral Freeze Register

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:APB2FZR)

For information about available fields see [`mod@apb2fzr`] module*/
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
///DBGMCU CPU1 APB2 Peripheral Freeze Register
pub mod apb2fzr;
/**C2APB2FZR (rw) register accessor: DBGMCU CPU2 APB2 Peripheral Freeze Register \[dual core device

You can [`read`](crate::Reg::read) this register and get [`c2apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:C2APB2FZR)

For information about available fields see [`mod@c2apb2fzr`] module*/
pub type C2APB2FZR = crate::Reg<c2apb2fzr::C2APB2FZRrs>;
///DBGMCU CPU2 APB2 Peripheral Freeze Register \[dual core device
pub mod c2apb2fzr;
