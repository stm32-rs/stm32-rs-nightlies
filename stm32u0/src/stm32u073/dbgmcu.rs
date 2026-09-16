#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb1fzr: APB1FZR,
    apb2fzr: APB2FZR,
    _reserved4: [u8; 0xec],
    sr: SR,
    dbg_auth_host: DBG_AUTH_HOST,
    dbg_auth_device: DBG_AUTH_DEVICE,
    _reserved7: [u8; 0x0ec8],
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
    ///0x00 - DBGMCU device ID code register
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    ///0x04 - DBGMCU configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - DBGMCU APB1 freeze register
    #[inline(always)]
    pub const fn apb1fzr(&self) -> &APB1FZR {
        &self.apb1fzr
    }
    ///0x0c - DBG APB2 freeze register
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
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
    ///0xfd0 - DBGMCU CoreSight peripheral identity register 4
    #[inline(always)]
    pub const fn pidr4(&self) -> &PIDR4 {
        &self.pidr4
    }
    ///0xfe0 - DBGMCU CoreSight peripheral identity register 0
    #[inline(always)]
    pub const fn pidr0(&self) -> &PIDR0 {
        &self.pidr0
    }
    ///0xfe4 - DBGMCU CoreSight peripheral identity register 1
    #[inline(always)]
    pub const fn pidr1(&self) -> &PIDR1 {
        &self.pidr1
    }
    ///0xfe8 - DBGMCU CoreSight peripheral identity register 2
    #[inline(always)]
    pub const fn pidr2(&self) -> &PIDR2 {
        &self.pidr2
    }
    ///0xfec - DBGMCU CoreSight peripheral identity register 3
    #[inline(always)]
    pub const fn pidr3(&self) -> &PIDR3 {
        &self.pidr3
    }
    ///0xff0 - DBGMCU CoreSight component identity register 0
    #[inline(always)]
    pub const fn cidr0(&self) -> &CIDR0 {
        &self.cidr0
    }
    ///0xff4 - DBGMCU CoreSight component identity register 1
    #[inline(always)]
    pub const fn cidr1(&self) -> &CIDR1 {
        &self.cidr1
    }
    ///0xff8 - DBGMCU CoreSight component identity register 2
    #[inline(always)]
    pub const fn cidr2(&self) -> &CIDR2 {
        &self.cidr2
    }
    ///0xffc - DBGMCU CoreSight component identity register 3
    #[inline(always)]
    pub const fn cidr3(&self) -> &CIDR3 {
        &self.cidr3
    }
}
/**IDCODE (r) register accessor: DBGMCU device ID code register

You can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:IDCODE)

For information about available fields see [`mod@idcode`] module*/
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
///DBGMCU device ID code register
pub mod idcode;
/**CR (rw) register accessor: DBGMCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DBGMCU configuration register
pub mod cr;
/**APB1FZR (rw) register accessor: DBGMCU APB1 freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:APB1FZR)

For information about available fields see [`mod@apb1fzr`] module*/
pub type APB1FZR = crate::Reg<apb1fzr::APB1FZRrs>;
///DBGMCU APB1 freeze register
pub mod apb1fzr;
/**APB2FZR (rw) register accessor: DBG APB2 freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:APB2FZR)

For information about available fields see [`mod@apb2fzr`] module*/
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
///DBG APB2 freeze register
pub mod apb2fzr;
/**SR (r) register accessor: DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DBGMCU status register
pub mod sr;
/**DBG_AUTH_HOST (rw) register accessor: DBGMCU debug authentication mailbox host register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:DBG_AUTH_HOST)

For information about available fields see [`mod@dbg_auth_host`] module*/
pub type DBG_AUTH_HOST = crate::Reg<dbg_auth_host::DBG_AUTH_HOSTrs>;
///DBGMCU debug authentication mailbox host register
pub mod dbg_auth_host;
/**DBG_AUTH_DEVICE (r) register accessor: DBGMCU debug authentication mailbox device register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:DBG_AUTH_DEVICE)

For information about available fields see [`mod@dbg_auth_device`] module*/
pub type DBG_AUTH_DEVICE = crate::Reg<dbg_auth_device::DBG_AUTH_DEVICErs>;
///DBGMCU debug authentication mailbox device register
pub mod dbg_auth_device;
/**PIDR4 (r) register accessor: DBGMCU CoreSight peripheral identity register 4

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:PIDR4)

For information about available fields see [`mod@pidr4`] module*/
pub type PIDR4 = crate::Reg<pidr4::PIDR4rs>;
///DBGMCU CoreSight peripheral identity register 4
pub mod pidr4;
/**PIDR0 (r) register accessor: DBGMCU CoreSight peripheral identity register 0

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:PIDR0)

For information about available fields see [`mod@pidr0`] module*/
pub type PIDR0 = crate::Reg<pidr0::PIDR0rs>;
///DBGMCU CoreSight peripheral identity register 0
pub mod pidr0;
/**PIDR1 (r) register accessor: DBGMCU CoreSight peripheral identity register 1

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:PIDR1)

For information about available fields see [`mod@pidr1`] module*/
pub type PIDR1 = crate::Reg<pidr1::PIDR1rs>;
///DBGMCU CoreSight peripheral identity register 1
pub mod pidr1;
/**PIDR2 (r) register accessor: DBGMCU CoreSight peripheral identity register 2

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:PIDR2)

For information about available fields see [`mod@pidr2`] module*/
pub type PIDR2 = crate::Reg<pidr2::PIDR2rs>;
///DBGMCU CoreSight peripheral identity register 2
pub mod pidr2;
/**PIDR3 (r) register accessor: DBGMCU CoreSight peripheral identity register 3

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:PIDR3)

For information about available fields see [`mod@pidr3`] module*/
pub type PIDR3 = crate::Reg<pidr3::PIDR3rs>;
///DBGMCU CoreSight peripheral identity register 3
pub mod pidr3;
/**CIDR0 (r) register accessor: DBGMCU CoreSight component identity register 0

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:CIDR0)

For information about available fields see [`mod@cidr0`] module*/
pub type CIDR0 = crate::Reg<cidr0::CIDR0rs>;
///DBGMCU CoreSight component identity register 0
pub mod cidr0;
/**CIDR1 (r) register accessor: DBGMCU CoreSight component identity register 1

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:CIDR1)

For information about available fields see [`mod@cidr1`] module*/
pub type CIDR1 = crate::Reg<cidr1::CIDR1rs>;
///DBGMCU CoreSight component identity register 1
pub mod cidr1;
/**CIDR2 (r) register accessor: DBGMCU CoreSight component identity register 2

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:CIDR2)

For information about available fields see [`mod@cidr2`] module*/
pub type CIDR2 = crate::Reg<cidr2::CIDR2rs>;
///DBGMCU CoreSight component identity register 2
pub mod cidr2;
/**CIDR3 (r) register accessor: DBGMCU CoreSight component identity register 3

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:CIDR3)

For information about available fields see [`mod@cidr3`] module*/
pub type CIDR3 = crate::Reg<cidr3::CIDR3rs>;
///DBGMCU CoreSight component identity register 3
pub mod cidr3;
