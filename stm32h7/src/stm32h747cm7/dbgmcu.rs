#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idc: IDC,
    cr: CR,
    _reserved2: [u8; 0x2c],
    apb3fz1: APB3FZ1,
    apb3fz2: APB3FZ2,
    apb1lfz1: APB1LFZ1,
    apb1lfz2: APB1LFZ2,
    _reserved6: [u8; 0x08],
    apb2fz1: APB2FZ1,
    apb2fz2: APB2FZ2,
    apb4fz1: APB4FZ1,
    apb4fz2: APB4FZ2,
}
impl RegisterBlock {
    ///0x00 - DBGMCU Identity Code Register
    #[inline(always)]
    pub const fn idc(&self) -> &IDC {
        &self.idc
    }
    ///0x04 - DBGMCU Configuration Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x34 - DBGMCU APB3 peripheral freeze register CPU1
    #[inline(always)]
    pub const fn apb3fz1(&self) -> &APB3FZ1 {
        &self.apb3fz1
    }
    ///0x38 - DBGMCU APB3 peripheral freeze register CPU2
    #[inline(always)]
    pub const fn apb3fz2(&self) -> &APB3FZ2 {
        &self.apb3fz2
    }
    ///0x3c - DBGMCU APB1L peripheral freeze register
    #[inline(always)]
    pub const fn apb1lfz1(&self) -> &APB1LFZ1 {
        &self.apb1lfz1
    }
    ///0x40 - DBGMCU APB1L peripheral freeze register CPU2
    #[inline(always)]
    pub const fn apb1lfz2(&self) -> &APB1LFZ2 {
        &self.apb1lfz2
    }
    ///0x4c - DBGMCU APB2 peripheral freeze register
    #[inline(always)]
    pub const fn apb2fz1(&self) -> &APB2FZ1 {
        &self.apb2fz1
    }
    ///0x50 - DBGMCU APB2 peripheral freeze register CPU2
    #[inline(always)]
    pub const fn apb2fz2(&self) -> &APB2FZ2 {
        &self.apb2fz2
    }
    ///0x54 - DBGMCU APB4 peripheral freeze register
    #[inline(always)]
    pub const fn apb4fz1(&self) -> &APB4FZ1 {
        &self.apb4fz1
    }
    ///0x58 - DBGMCU APB4 peripheral freeze register CPU2
    #[inline(always)]
    pub const fn apb4fz2(&self) -> &APB4FZ2 {
        &self.apb4fz2
    }
}
/**IDC (r) register accessor: DBGMCU Identity Code Register

You can [`read`](crate::Reg::read) this register and get [`idc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:IDC)

For information about available fields see [`mod@idc`] module*/
pub type IDC = crate::Reg<idc::IDCrs>;
///DBGMCU Identity Code Register
pub mod idc;
/**CR (rw) register accessor: DBGMCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DBGMCU Configuration Register
pub mod cr;
/**APB3FZ1 (rw) register accessor: DBGMCU APB3 peripheral freeze register CPU1

You can [`read`](crate::Reg::read) this register and get [`apb3fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB3FZ1)

For information about available fields see [`mod@apb3fz1`] module*/
pub type APB3FZ1 = crate::Reg<apb3fz1::APB3FZ1rs>;
///DBGMCU APB3 peripheral freeze register CPU1
pub mod apb3fz1;
/**APB3FZ2 (rw) register accessor: DBGMCU APB3 peripheral freeze register CPU2

You can [`read`](crate::Reg::read) this register and get [`apb3fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB3FZ2)

For information about available fields see [`mod@apb3fz2`] module*/
pub type APB3FZ2 = crate::Reg<apb3fz2::APB3FZ2rs>;
///DBGMCU APB3 peripheral freeze register CPU2
pub mod apb3fz2;
/**APB1LFZ1 (rw) register accessor: DBGMCU APB1L peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1lfz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB1LFZ1)

For information about available fields see [`mod@apb1lfz1`] module*/
pub type APB1LFZ1 = crate::Reg<apb1lfz1::APB1LFZ1rs>;
///DBGMCU APB1L peripheral freeze register
pub mod apb1lfz1;
/**APB1LFZ2 (rw) register accessor: DBGMCU APB1L peripheral freeze register CPU2

You can [`read`](crate::Reg::read) this register and get [`apb1lfz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB1LFZ2)

For information about available fields see [`mod@apb1lfz2`] module*/
pub type APB1LFZ2 = crate::Reg<apb1lfz2::APB1LFZ2rs>;
///DBGMCU APB1L peripheral freeze register CPU2
pub mod apb1lfz2;
/**APB2FZ1 (rw) register accessor: DBGMCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB2FZ1)

For information about available fields see [`mod@apb2fz1`] module*/
pub type APB2FZ1 = crate::Reg<apb2fz1::APB2FZ1rs>;
///DBGMCU APB2 peripheral freeze register
pub mod apb2fz1;
/**APB2FZ2 (rw) register accessor: DBGMCU APB2 peripheral freeze register CPU2

You can [`read`](crate::Reg::read) this register and get [`apb2fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB2FZ2)

For information about available fields see [`mod@apb2fz2`] module*/
pub type APB2FZ2 = crate::Reg<apb2fz2::APB2FZ2rs>;
///DBGMCU APB2 peripheral freeze register CPU2
pub mod apb2fz2;
/**APB4FZ1 (rw) register accessor: DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB4FZ1)

For information about available fields see [`mod@apb4fz1`] module*/
pub type APB4FZ1 = crate::Reg<apb4fz1::APB4FZ1rs>;
///DBGMCU APB4 peripheral freeze register
pub mod apb4fz1;
/**APB4FZ2 (rw) register accessor: DBGMCU APB4 peripheral freeze register CPU2

You can [`read`](crate::Reg::read) this register and get [`apb4fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:APB4FZ2)

For information about available fields see [`mod@apb4fz2`] module*/
pub type APB4FZ2 = crate::Reg<apb4fz2::APB4FZ2rs>;
///DBGMCU APB4 peripheral freeze register CPU2
pub mod apb4fz2;
