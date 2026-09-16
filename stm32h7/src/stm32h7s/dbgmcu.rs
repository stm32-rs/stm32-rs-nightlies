#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idc: IDC,
    cr: CR,
    _reserved2: [u8; 0x18],
    ahb5fzr: AHB5FZR,
    ahb1fzr: AHB1FZR,
    _reserved4: [u8; 0x14],
    apb1fzr: APB1FZR,
    _reserved5: [u8; 0x0c],
    apb2fz: APB2FZ,
    _reserved6: [u8; 0x04],
    apb4fzr: APB4FZR,
    _reserved7: [u8; 0xa4],
    sr: SR,
    dbg_auth_host: DBG_AUTH_HOST,
    dbg_auth_device: DBG_AUTH_DEVICE,
    dbg_auth_ack: DBG_AUTH_ACK,
    _reserved11: [u8; 0x0ec4],
    pidr4: PIDR4,
    _reserved12: [u8; 0x0c],
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
    ///0x00 - DBGMCU identity code register
    #[inline(always)]
    pub const fn idc(&self) -> &IDC {
        &self.idc
    }
    ///0x04 - DBGMCU configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x20 - DBGMCU AHB5 peripheral freeze register
    #[inline(always)]
    pub const fn ahb5fzr(&self) -> &AHB5FZR {
        &self.ahb5fzr
    }
    ///0x24 - DBGMCU AHB1 peripheral freeze register
    #[inline(always)]
    pub const fn ahb1fzr(&self) -> &AHB1FZR {
        &self.ahb1fzr
    }
    ///0x3c - DBGMCU APB1 peripheral freeze register
    #[inline(always)]
    pub const fn apb1fzr(&self) -> &APB1FZR {
        &self.apb1fzr
    }
    ///0x4c - DBGMCU APB2 peripheral freeze register
    #[inline(always)]
    pub const fn apb2fz(&self) -> &APB2FZ {
        &self.apb2fz
    }
    ///0x54 - DBGMCU APB4 peripheral freeze register
    #[inline(always)]
    pub const fn apb4fzr(&self) -> &APB4FZR {
        &self.apb4fzr
    }
    ///0xfc - DBGMCU status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x100 - DBGMCU debug authentication mailbox host register
    #[inline(always)]
    pub const fn dbg_auth_host(&self) -> &DBG_AUTH_HOST {
        &self.dbg_auth_host
    }
    ///0x104 - DBGMCU debug authentication mailbox device register
    #[inline(always)]
    pub const fn dbg_auth_device(&self) -> &DBG_AUTH_DEVICE {
        &self.dbg_auth_device
    }
    ///0x108 - DBGMCU debug authentication mailbox acknowledge register
    #[inline(always)]
    pub const fn dbg_auth_ack(&self) -> &DBG_AUTH_ACK {
        &self.dbg_auth_ack
    }
    ///0xfd0 - DBGMCU peripheral identity register 4
    #[inline(always)]
    pub const fn pidr4(&self) -> &PIDR4 {
        &self.pidr4
    }
    ///0xfe0 - DBGMCU peripheral identity register 0
    #[inline(always)]
    pub const fn pidr0(&self) -> &PIDR0 {
        &self.pidr0
    }
    ///0xfe4 - DBGMCU peripheral identity register 1
    #[inline(always)]
    pub const fn pidr1(&self) -> &PIDR1 {
        &self.pidr1
    }
    ///0xfe8 - DBGMCU peripheral identity register 2
    #[inline(always)]
    pub const fn pidr2(&self) -> &PIDR2 {
        &self.pidr2
    }
    ///0xfec - DBGMCU peripheral identity register 3
    #[inline(always)]
    pub const fn pidr3(&self) -> &PIDR3 {
        &self.pidr3
    }
    ///0xff0 - DBGMCU component identity register
    #[inline(always)]
    pub const fn cidr0(&self) -> &CIDR0 {
        &self.cidr0
    }
    ///0xff4 - DBGMCU component identity register
    #[inline(always)]
    pub const fn cidr1(&self) -> &CIDR1 {
        &self.cidr1
    }
    ///0xff8 - DBGMCU component identity register
    #[inline(always)]
    pub const fn cidr2(&self) -> &CIDR2 {
        &self.cidr2
    }
    ///0xffc - DBGMCU component identity register
    #[inline(always)]
    pub const fn cidr3(&self) -> &CIDR3 {
        &self.cidr3
    }
}
/**IDC (r) register accessor: DBGMCU identity code register

You can [`read`](crate::Reg::read) this register and get [`idc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:IDC)

For information about available fields see [`mod@idc`] module*/
pub type IDC = crate::Reg<idc::IDCrs>;
///DBGMCU identity code register
pub mod idc;
/**CR (rw) register accessor: DBGMCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DBGMCU configuration register
pub mod cr;
/**AHB5FZR (rw) register accessor: DBGMCU AHB5 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb5fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:AHB5FZR)

For information about available fields see [`mod@ahb5fzr`] module*/
pub type AHB5FZR = crate::Reg<ahb5fzr::AHB5FZRrs>;
///DBGMCU AHB5 peripheral freeze register
pub mod ahb5fzr;
/**AHB1FZR (rw) register accessor: DBGMCU AHB1 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb1fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:AHB1FZR)

For information about available fields see [`mod@ahb1fzr`] module*/
pub type AHB1FZR = crate::Reg<ahb1fzr::AHB1FZRrs>;
///DBGMCU AHB1 peripheral freeze register
pub mod ahb1fzr;
/**APB1FZR (rw) register accessor: DBGMCU APB1 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:APB1FZR)

For information about available fields see [`mod@apb1fzr`] module*/
pub type APB1FZR = crate::Reg<apb1fzr::APB1FZRrs>;
///DBGMCU APB1 peripheral freeze register
pub mod apb1fzr;
/**APB2FZ (rw) register accessor: DBGMCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:APB2FZ)

For information about available fields see [`mod@apb2fz`] module*/
pub type APB2FZ = crate::Reg<apb2fz::APB2FZrs>;
///DBGMCU APB2 peripheral freeze register
pub mod apb2fz;
/**APB4FZR (rw) register accessor: DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:APB4FZR)

For information about available fields see [`mod@apb4fzr`] module*/
pub type APB4FZR = crate::Reg<apb4fzr::APB4FZRrs>;
///DBGMCU APB4 peripheral freeze register
pub mod apb4fzr;
/**SR (r) register accessor: DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DBGMCU status register
pub mod sr;
/**DBG_AUTH_HOST (rw) register accessor: DBGMCU debug authentication mailbox host register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:DBG_AUTH_HOST)

For information about available fields see [`mod@dbg_auth_host`] module*/
pub type DBG_AUTH_HOST = crate::Reg<dbg_auth_host::DBG_AUTH_HOSTrs>;
///DBGMCU debug authentication mailbox host register
pub mod dbg_auth_host;
/**DBG_AUTH_DEVICE (rw) register accessor: DBGMCU debug authentication mailbox device register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_device::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_device::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:DBG_AUTH_DEVICE)

For information about available fields see [`mod@dbg_auth_device`] module*/
pub type DBG_AUTH_DEVICE = crate::Reg<dbg_auth_device::DBG_AUTH_DEVICErs>;
///DBGMCU debug authentication mailbox device register
pub mod dbg_auth_device;
/**DBG_AUTH_ACK (rw) register accessor: DBGMCU debug authentication mailbox acknowledge register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_ack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_ack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:DBG_AUTH_ACK)

For information about available fields see [`mod@dbg_auth_ack`] module*/
pub type DBG_AUTH_ACK = crate::Reg<dbg_auth_ack::DBG_AUTH_ACKrs>;
///DBGMCU debug authentication mailbox acknowledge register
pub mod dbg_auth_ack;
/**PIDR4 (r) register accessor: DBGMCU peripheral identity register 4

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:PIDR4)

For information about available fields see [`mod@pidr4`] module*/
pub type PIDR4 = crate::Reg<pidr4::PIDR4rs>;
///DBGMCU peripheral identity register 4
pub mod pidr4;
/**PIDR0 (r) register accessor: DBGMCU peripheral identity register 0

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:PIDR0)

For information about available fields see [`mod@pidr0`] module*/
pub type PIDR0 = crate::Reg<pidr0::PIDR0rs>;
///DBGMCU peripheral identity register 0
pub mod pidr0;
/**PIDR1 (r) register accessor: DBGMCU peripheral identity register 1

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:PIDR1)

For information about available fields see [`mod@pidr1`] module*/
pub type PIDR1 = crate::Reg<pidr1::PIDR1rs>;
///DBGMCU peripheral identity register 1
pub mod pidr1;
/**PIDR2 (r) register accessor: DBGMCU peripheral identity register 2

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:PIDR2)

For information about available fields see [`mod@pidr2`] module*/
pub type PIDR2 = crate::Reg<pidr2::PIDR2rs>;
///DBGMCU peripheral identity register 2
pub mod pidr2;
/**PIDR3 (r) register accessor: DBGMCU peripheral identity register 3

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:PIDR3)

For information about available fields see [`mod@pidr3`] module*/
pub type PIDR3 = crate::Reg<pidr3::PIDR3rs>;
///DBGMCU peripheral identity register 3
pub mod pidr3;
/**CIDR0 (r) register accessor: DBGMCU component identity register

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:CIDR0)

For information about available fields see [`mod@cidr0`] module*/
pub type CIDR0 = crate::Reg<cidr0::CIDR0rs>;
///DBGMCU component identity register
pub mod cidr0;
/**CIDR1 (r) register accessor: DBGMCU component identity register

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:CIDR1)

For information about available fields see [`mod@cidr1`] module*/
pub type CIDR1 = crate::Reg<cidr1::CIDR1rs>;
///DBGMCU component identity register
pub mod cidr1;
/**CIDR2 (r) register accessor: DBGMCU component identity register

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:CIDR2)

For information about available fields see [`mod@cidr2`] module*/
pub type CIDR2 = crate::Reg<cidr2::CIDR2rs>;
///DBGMCU component identity register
pub mod cidr2;
/**CIDR3 (r) register accessor: DBGMCU component identity register

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:CIDR3)

For information about available fields see [`mod@cidr3`] module*/
pub type CIDR3 = crate::Reg<cidr3::CIDR3rs>;
///DBGMCU component identity register
pub mod cidr3;
