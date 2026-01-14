#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    _reserved2: [u8; 0x34],
    apb1fzr1: APB1FZR1,
    c2ap_b1fzr1: C2AP_B1FZR1,
    apb1fzr2: APB1FZR2,
    _reserved_5_c2apb: [u8; 0x04],
    apb2fzr: APB2FZR,
}
impl RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    ///0x04 - Debug MCU Configuration Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x3c - APB1 Low Freeze Register CPU1
    #[inline(always)]
    pub const fn apb1fzr1(&self) -> &APB1FZR1 {
        &self.apb1fzr1
    }
    ///0x40 - APB1 Low Freeze Register CPU2
    #[inline(always)]
    pub const fn c2ap_b1fzr1(&self) -> &C2AP_B1FZR1 {
        &self.c2ap_b1fzr1
    }
    ///0x44 - APB1 High Freeze Register CPU1
    #[inline(always)]
    pub const fn apb1fzr2(&self) -> &APB1FZR2 {
        &self.apb1fzr2
    }
    ///0x48 - APB2 Freeze Register CPU2
    #[inline(always)]
    pub const fn c2apb2fzr(&self) -> &C2APB2FZR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    ///0x48 - APB1 High Freeze Register CPU2
    #[inline(always)]
    pub const fn c2apb1fzr2(&self) -> &C2APB1FZR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    ///0x4c - APB2 Freeze Register CPU1
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
    }
}
/**IDCODE (r) register accessor: MCU Device ID Code Register

You can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:IDCODE)

For information about available fields see [`mod@idcode`] module*/
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
///MCU Device ID Code Register
pub mod idcode;
/**CR (rw) register accessor: Debug MCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Debug MCU Configuration Register
pub mod cr;
/**APB1FZR1 (rw) register accessor: APB1 Low Freeze Register CPU1

You can [`read`](crate::Reg::read) this register and get [`apb1fzr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:APB1FZR1)

For information about available fields see [`mod@apb1fzr1`] module*/
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1rs>;
///APB1 Low Freeze Register CPU1
pub mod apb1fzr1;
/**C2AP_B1FZR1 (rw) register accessor: APB1 Low Freeze Register CPU2

You can [`read`](crate::Reg::read) this register and get [`c2ap_b1fzr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ap_b1fzr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:C2AP_B1FZR1)

For information about available fields see [`mod@c2ap_b1fzr1`] module*/
pub type C2AP_B1FZR1 = crate::Reg<c2ap_b1fzr1::C2AP_B1FZR1rs>;
///APB1 Low Freeze Register CPU2
pub mod c2ap_b1fzr1;
/**APB1FZR2 (rw) register accessor: APB1 High Freeze Register CPU1

You can [`read`](crate::Reg::read) this register and get [`apb1fzr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:APB1FZR2)

For information about available fields see [`mod@apb1fzr2`] module*/
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2rs>;
///APB1 High Freeze Register CPU1
pub mod apb1fzr2;
/**C2APB1FZR2 (rw) register accessor: APB1 High Freeze Register CPU2

You can [`read`](crate::Reg::read) this register and get [`c2apb1fzr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1fzr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:C2APB1FZR2)

For information about available fields see [`mod@c2apb1fzr2`] module*/
pub type C2APB1FZR2 = crate::Reg<c2apb1fzr2::C2APB1FZR2rs>;
///APB1 High Freeze Register CPU2
pub mod c2apb1fzr2;
/**APB2FZR (rw) register accessor: APB2 Freeze Register CPU1

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:APB2FZR)

For information about available fields see [`mod@apb2fzr`] module*/
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
///APB2 Freeze Register CPU1
pub mod apb2fzr;
/**C2APB2FZR (rw) register accessor: APB2 Freeze Register CPU2

You can [`read`](crate::Reg::read) this register and get [`c2apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:C2APB2FZR)

For information about available fields see [`mod@c2apb2fzr`] module*/
pub type C2APB2FZR = crate::Reg<c2apb2fzr::C2APB2FZRrs>;
///APB2 Freeze Register CPU2
pub mod c2apb2fzr;
