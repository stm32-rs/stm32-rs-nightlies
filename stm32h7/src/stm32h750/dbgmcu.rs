#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idc: IDC,
    cr: CR,
    _reserved2: [u8; 0x2c],
    apb3fz1: APB3FZ1,
    _reserved3: [u8; 0x04],
    apb1lfz1: APB1LFZ1,
    _reserved4: [u8; 0x0c],
    apb2fz1: APB2FZ1,
    _reserved5: [u8; 0x04],
    apb4fz1: APB4FZ1,
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
    ///0x34 - DBGMCU APB3 peripheral freeze register
    #[inline(always)]
    pub const fn apb3fz1(&self) -> &APB3FZ1 {
        &self.apb3fz1
    }
    ///0x3c - DBGMCU APB1L peripheral freeze register
    #[inline(always)]
    pub const fn apb1lfz1(&self) -> &APB1LFZ1 {
        &self.apb1lfz1
    }
    ///0x4c - DBGMCU APB2 peripheral freeze register
    #[inline(always)]
    pub const fn apb2fz1(&self) -> &APB2FZ1 {
        &self.apb2fz1
    }
    ///0x54 - DBGMCU APB4 peripheral freeze register
    #[inline(always)]
    pub const fn apb4fz1(&self) -> &APB4FZ1 {
        &self.apb4fz1
    }
}
/**IDC (r) register accessor: DBGMCU Identity Code Register

You can [`read`](crate::Reg::read) this register and get [`idc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DBGMCU:IDC)

For information about available fields see [`mod@idc`] module*/
pub type IDC = crate::Reg<idc::IDCrs>;
///DBGMCU Identity Code Register
pub mod idc;
/**CR (rw) register accessor: DBGMCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DBGMCU Configuration Register
pub mod cr;
/**APB3FZ1 (rw) register accessor: DBGMCU APB3 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb3fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DBGMCU:APB3FZ1)

For information about available fields see [`mod@apb3fz1`] module*/
pub type APB3FZ1 = crate::Reg<apb3fz1::APB3FZ1rs>;
///DBGMCU APB3 peripheral freeze register
pub mod apb3fz1;
/**APB1LFZ1 (rw) register accessor: DBGMCU APB1L peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1lfz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DBGMCU:APB1LFZ1)

For information about available fields see [`mod@apb1lfz1`] module*/
pub type APB1LFZ1 = crate::Reg<apb1lfz1::APB1LFZ1rs>;
///DBGMCU APB1L peripheral freeze register
pub mod apb1lfz1;
/**APB2FZ1 (rw) register accessor: DBGMCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DBGMCU:APB2FZ1)

For information about available fields see [`mod@apb2fz1`] module*/
pub type APB2FZ1 = crate::Reg<apb2fz1::APB2FZ1rs>;
///DBGMCU APB2 peripheral freeze register
pub mod apb2fz1;
/**APB4FZ1 (rw) register accessor: DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DBGMCU:APB4FZ1)

For information about available fields see [`mod@apb4fz1`] module*/
pub type APB4FZ1 = crate::Reg<apb4fz1::APB4FZ1rs>;
///DBGMCU APB4 peripheral freeze register
pub mod apb4fz1;
