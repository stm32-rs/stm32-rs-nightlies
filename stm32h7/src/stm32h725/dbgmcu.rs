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
    _reserved4: [u8; 0x04],
    apb1hfz1: APB1HFZ1,
    _reserved5: [u8; 0x04],
    apb2fz1: APB2FZ1,
    _reserved6: [u8; 0x04],
    apb4fz1: APB4FZ1,
    _reserved7: [u8; 0x0f78],
    pidr4: PIDR4,
    _reserved8: [u8; 0x0c],
    pidr0: PIDR0,
    pidr1: PIDR1,
    pidr2: PIDR2,
    pidr3: PIDR3,
    cidr0: CIDR0,
    cidr1: CIDR1,
    cidr2: CIDR2,
    cidr3: CIDR3,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn idc(&self) -> &IDC {
        &self.idc
    }
    ///0x04 -
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x34 -
    #[inline(always)]
    pub const fn apb3fz1(&self) -> &APB3FZ1 {
        &self.apb3fz1
    }
    ///0x3c -
    #[inline(always)]
    pub const fn apb1lfz1(&self) -> &APB1LFZ1 {
        &self.apb1lfz1
    }
    ///0x44 -
    #[inline(always)]
    pub const fn apb1hfz1(&self) -> &APB1HFZ1 {
        &self.apb1hfz1
    }
    ///0x4c -
    #[inline(always)]
    pub const fn apb2fz1(&self) -> &APB2FZ1 {
        &self.apb2fz1
    }
    ///0x54 -
    #[inline(always)]
    pub const fn apb4fz1(&self) -> &APB4FZ1 {
        &self.apb4fz1
    }
    ///0xfd0 -
    #[inline(always)]
    pub const fn pidr4(&self) -> &PIDR4 {
        &self.pidr4
    }
    ///0xfe0 -
    #[inline(always)]
    pub const fn pidr0(&self) -> &PIDR0 {
        &self.pidr0
    }
    ///0xfe4 -
    #[inline(always)]
    pub const fn pidr1(&self) -> &PIDR1 {
        &self.pidr1
    }
    ///0xfe8 -
    #[inline(always)]
    pub const fn pidr2(&self) -> &PIDR2 {
        &self.pidr2
    }
    ///0xfec -
    #[inline(always)]
    pub const fn pidr3(&self) -> &PIDR3 {
        &self.pidr3
    }
    ///0xff0 -
    #[inline(always)]
    pub const fn cidr0(&self) -> &CIDR0 {
        &self.cidr0
    }
    ///0xff4 -
    #[inline(always)]
    pub const fn cidr1(&self) -> &CIDR1 {
        &self.cidr1
    }
    ///0xff8 -
    #[inline(always)]
    pub const fn cidr2(&self) -> &CIDR2 {
        &self.cidr2
    }
    ///0xffc -
    #[inline(always)]
    pub const fn cidr3(&self) -> &CIDR3 {
        &self.cidr3
    }
}
/**IDC (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`idc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:IDC)

For information about available fields see [`mod@idc`] module*/
pub type IDC = crate::Reg<idc::IDCrs>;
///
pub mod idc;
/**CR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///
pub mod cr;
/**APB3FZ1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb3fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:APB3FZ1)

For information about available fields see [`mod@apb3fz1`] module*/
pub type APB3FZ1 = crate::Reg<apb3fz1::APB3FZ1rs>;
///
pub mod apb3fz1;
/**APB1LFZ1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1lfz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:APB1LFZ1)

For information about available fields see [`mod@apb1lfz1`] module*/
pub type APB1LFZ1 = crate::Reg<apb1lfz1::APB1LFZ1rs>;
///
pub mod apb1lfz1;
/**APB1HFZ1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1hfz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:APB1HFZ1)

For information about available fields see [`mod@apb1hfz1`] module*/
pub type APB1HFZ1 = crate::Reg<apb1hfz1::APB1HFZ1rs>;
///
pub mod apb1hfz1;
/**APB2FZ1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb2fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:APB2FZ1)

For information about available fields see [`mod@apb2fz1`] module*/
pub type APB2FZ1 = crate::Reg<apb2fz1::APB2FZ1rs>;
///
pub mod apb2fz1;
/**APB4FZ1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:APB4FZ1)

For information about available fields see [`mod@apb4fz1`] module*/
pub type APB4FZ1 = crate::Reg<apb4fz1::APB4FZ1rs>;
///
pub mod apb4fz1;
/**PIDR4 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:PIDR4)

For information about available fields see [`mod@pidr4`] module*/
pub type PIDR4 = crate::Reg<pidr4::PIDR4rs>;
///
pub mod pidr4;
/**PIDR0 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:PIDR0)

For information about available fields see [`mod@pidr0`] module*/
pub type PIDR0 = crate::Reg<pidr0::PIDR0rs>;
///
pub mod pidr0;
/**PIDR1 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:PIDR1)

For information about available fields see [`mod@pidr1`] module*/
pub type PIDR1 = crate::Reg<pidr1::PIDR1rs>;
///
pub mod pidr1;
/**PIDR2 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:PIDR2)

For information about available fields see [`mod@pidr2`] module*/
pub type PIDR2 = crate::Reg<pidr2::PIDR2rs>;
///
pub mod pidr2;
/**PIDR3 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:PIDR3)

For information about available fields see [`mod@pidr3`] module*/
pub type PIDR3 = crate::Reg<pidr3::PIDR3rs>;
///
pub mod pidr3;
/**CIDR0 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:CIDR0)

For information about available fields see [`mod@cidr0`] module*/
pub type CIDR0 = crate::Reg<cidr0::CIDR0rs>;
///
pub mod cidr0;
/**CIDR1 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:CIDR1)

For information about available fields see [`mod@cidr1`] module*/
pub type CIDR1 = crate::Reg<cidr1::CIDR1rs>;
///
pub mod cidr1;
/**CIDR2 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:CIDR2)

For information about available fields see [`mod@cidr2`] module*/
pub type CIDR2 = crate::Reg<cidr2::CIDR2rs>;
///
pub mod cidr2;
/**CIDR3 (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:CIDR3)

For information about available fields see [`mod@cidr3`] module*/
pub type CIDR3 = crate::Reg<cidr3::CIDR3rs>;
///
pub mod cidr3;
