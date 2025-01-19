#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dbgmcu_idcode: DBGMCU_IDCODE,
    dbgmcu_cr: DBGMCU_CR,
    dbgmcu_apb1fzr: DBGMCU_APB1FZR,
    dbgmcu_apb2fzr: DBGMCU_APB2FZR,
    _reserved4: [u8; 0xec],
    dbgmcu_sr: DBGMCU_SR,
    dbgmcu_dbg_auth_host: DBGMCU_DBG_AUTH_HOST,
    dbgmcu_dbg_auth_device: DBGMCU_DBG_AUTH_DEVICE,
    _reserved7: [u8; 0x0ec8],
    dbgmcu_pidr4: DBGMCU_PIDR4,
    _reserved8: [u8; 0x0c],
    dbgmcu_pidr0: DBGMCU_PIDR0,
    dbgmcu_pidr1: DBGMCU_PIDR1,
    dbgmcu_pidr2: DBGMCU_PIDR2,
    dbgmcu_pidr3: DBGMCU_PIDR3,
    dbgmcu_cidr0: DBGMCU_CIDR0,
    dbgmcu_cidr1: DBGMCU_CIDR1,
    dbgmcu_cidr2: DBGMCU_CIDR2,
    dbgmcu_cidr3: DBGMCU_CIDR3,
}
impl RegisterBlock {
    ///0x00 - DBGMCU device ID code register
    #[inline(always)]
    pub const fn dbgmcu_idcode(&self) -> &DBGMCU_IDCODE {
        &self.dbgmcu_idcode
    }
    ///0x04 - DBGMCU configuration register
    #[inline(always)]
    pub const fn dbgmcu_cr(&self) -> &DBGMCU_CR {
        &self.dbgmcu_cr
    }
    ///0x08 - DBGMCU APB1 freeze register
    #[inline(always)]
    pub const fn dbgmcu_apb1fzr(&self) -> &DBGMCU_APB1FZR {
        &self.dbgmcu_apb1fzr
    }
    ///0x0c - DBG APB2 freeze register
    #[inline(always)]
    pub const fn dbgmcu_apb2fzr(&self) -> &DBGMCU_APB2FZR {
        &self.dbgmcu_apb2fzr
    }
    ///0xfc - DBGMCU status register
    #[inline(always)]
    pub const fn dbgmcu_sr(&self) -> &DBGMCU_SR {
        &self.dbgmcu_sr
    }
    ///0x100 - DBGMCU debug authentication mailbox host register
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_host(&self) -> &DBGMCU_DBG_AUTH_HOST {
        &self.dbgmcu_dbg_auth_host
    }
    ///0x104 - DBGMCU debug authentication mailbox device register
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_device(&self) -> &DBGMCU_DBG_AUTH_DEVICE {
        &self.dbgmcu_dbg_auth_device
    }
    ///0xfd0 - DBGMCU CoreSight peripheral identity register 4
    #[inline(always)]
    pub const fn dbgmcu_pidr4(&self) -> &DBGMCU_PIDR4 {
        &self.dbgmcu_pidr4
    }
    ///0xfe0 - DBGMCU CoreSight peripheral identity register 0
    #[inline(always)]
    pub const fn dbgmcu_pidr0(&self) -> &DBGMCU_PIDR0 {
        &self.dbgmcu_pidr0
    }
    ///0xfe4 - DBGMCU CoreSight peripheral identity register 1
    #[inline(always)]
    pub const fn dbgmcu_pidr1(&self) -> &DBGMCU_PIDR1 {
        &self.dbgmcu_pidr1
    }
    ///0xfe8 - DBGMCU CoreSight peripheral identity register 2
    #[inline(always)]
    pub const fn dbgmcu_pidr2(&self) -> &DBGMCU_PIDR2 {
        &self.dbgmcu_pidr2
    }
    ///0xfec - DBGMCU CoreSight peripheral identity register 3
    #[inline(always)]
    pub const fn dbgmcu_pidr3(&self) -> &DBGMCU_PIDR3 {
        &self.dbgmcu_pidr3
    }
    ///0xff0 - DBGMCU CoreSight component identity register 0
    #[inline(always)]
    pub const fn dbgmcu_cidr0(&self) -> &DBGMCU_CIDR0 {
        &self.dbgmcu_cidr0
    }
    ///0xff4 - DBGMCU CoreSight component identity register 1
    #[inline(always)]
    pub const fn dbgmcu_cidr1(&self) -> &DBGMCU_CIDR1 {
        &self.dbgmcu_cidr1
    }
    ///0xff8 - DBGMCU CoreSight component identity register 2
    #[inline(always)]
    pub const fn dbgmcu_cidr2(&self) -> &DBGMCU_CIDR2 {
        &self.dbgmcu_cidr2
    }
    ///0xffc - DBGMCU CoreSight component identity register 3
    #[inline(always)]
    pub const fn dbgmcu_cidr3(&self) -> &DBGMCU_CIDR3 {
        &self.dbgmcu_cidr3
    }
}
/**DBGMCU_IDCODE (r) register accessor: DBGMCU device ID code register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_IDCODE)

For information about available fields see [`mod@dbgmcu_idcode`]
module*/
pub type DBGMCU_IDCODE = crate::Reg<dbgmcu_idcode::DBGMCU_IDCODErs>;
///DBGMCU device ID code register
pub mod dbgmcu_idcode;
/**DBGMCU_CR (rw) register accessor: DBGMCU configuration register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_CR)

For information about available fields see [`mod@dbgmcu_cr`]
module*/
pub type DBGMCU_CR = crate::Reg<dbgmcu_cr::DBGMCU_CRrs>;
///DBGMCU configuration register
pub mod dbgmcu_cr;
/**DBGMCU_APB1FZR (rw) register accessor: DBGMCU APB1 freeze register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb1fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb1fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_APB1FZR)

For information about available fields see [`mod@dbgmcu_apb1fzr`]
module*/
pub type DBGMCU_APB1FZR = crate::Reg<dbgmcu_apb1fzr::DBGMCU_APB1FZRrs>;
///DBGMCU APB1 freeze register
pub mod dbgmcu_apb1fzr;
/**DBGMCU_APB2FZR (rw) register accessor: DBG APB2 freeze register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_APB2FZR)

For information about available fields see [`mod@dbgmcu_apb2fzr`]
module*/
pub type DBGMCU_APB2FZR = crate::Reg<dbgmcu_apb2fzr::DBGMCU_APB2FZRrs>;
///DBG APB2 freeze register
pub mod dbgmcu_apb2fzr;
/**DBGMCU_SR (r) register accessor: DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_SR)

For information about available fields see [`mod@dbgmcu_sr`]
module*/
pub type DBGMCU_SR = crate::Reg<dbgmcu_sr::DBGMCU_SRrs>;
///DBGMCU status register
pub mod dbgmcu_sr;
/**DBGMCU_DBG_AUTH_HOST (rw) register accessor: DBGMCU debug authentication mailbox host register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_dbg_auth_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_dbg_auth_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_DBG_AUTH_HOST)

For information about available fields see [`mod@dbgmcu_dbg_auth_host`]
module*/
pub type DBGMCU_DBG_AUTH_HOST = crate::Reg<dbgmcu_dbg_auth_host::DBGMCU_DBG_AUTH_HOSTrs>;
///DBGMCU debug authentication mailbox host register
pub mod dbgmcu_dbg_auth_host;
/**DBGMCU_DBG_AUTH_DEVICE (r) register accessor: DBGMCU debug authentication mailbox device register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_dbg_auth_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_DBG_AUTH_DEVICE)

For information about available fields see [`mod@dbgmcu_dbg_auth_device`]
module*/
pub type DBGMCU_DBG_AUTH_DEVICE = crate::Reg<dbgmcu_dbg_auth_device::DBGMCU_DBG_AUTH_DEVICErs>;
///DBGMCU debug authentication mailbox device register
pub mod dbgmcu_dbg_auth_device;
/**DBGMCU_PIDR4 (r) register accessor: DBGMCU CoreSight peripheral identity register 4

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_PIDR4)

For information about available fields see [`mod@dbgmcu_pidr4`]
module*/
pub type DBGMCU_PIDR4 = crate::Reg<dbgmcu_pidr4::DBGMCU_PIDR4rs>;
///DBGMCU CoreSight peripheral identity register 4
pub mod dbgmcu_pidr4;
/**DBGMCU_PIDR0 (r) register accessor: DBGMCU CoreSight peripheral identity register 0

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_PIDR0)

For information about available fields see [`mod@dbgmcu_pidr0`]
module*/
pub type DBGMCU_PIDR0 = crate::Reg<dbgmcu_pidr0::DBGMCU_PIDR0rs>;
///DBGMCU CoreSight peripheral identity register 0
pub mod dbgmcu_pidr0;
/**DBGMCU_PIDR1 (r) register accessor: DBGMCU CoreSight peripheral identity register 1

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_PIDR1)

For information about available fields see [`mod@dbgmcu_pidr1`]
module*/
pub type DBGMCU_PIDR1 = crate::Reg<dbgmcu_pidr1::DBGMCU_PIDR1rs>;
///DBGMCU CoreSight peripheral identity register 1
pub mod dbgmcu_pidr1;
/**DBGMCU_PIDR2 (r) register accessor: DBGMCU CoreSight peripheral identity register 2

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_PIDR2)

For information about available fields see [`mod@dbgmcu_pidr2`]
module*/
pub type DBGMCU_PIDR2 = crate::Reg<dbgmcu_pidr2::DBGMCU_PIDR2rs>;
///DBGMCU CoreSight peripheral identity register 2
pub mod dbgmcu_pidr2;
/**DBGMCU_PIDR3 (r) register accessor: DBGMCU CoreSight peripheral identity register 3

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_PIDR3)

For information about available fields see [`mod@dbgmcu_pidr3`]
module*/
pub type DBGMCU_PIDR3 = crate::Reg<dbgmcu_pidr3::DBGMCU_PIDR3rs>;
///DBGMCU CoreSight peripheral identity register 3
pub mod dbgmcu_pidr3;
/**DBGMCU_CIDR0 (r) register accessor: DBGMCU CoreSight component identity register 0

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_CIDR0)

For information about available fields see [`mod@dbgmcu_cidr0`]
module*/
pub type DBGMCU_CIDR0 = crate::Reg<dbgmcu_cidr0::DBGMCU_CIDR0rs>;
///DBGMCU CoreSight component identity register 0
pub mod dbgmcu_cidr0;
/**DBGMCU_CIDR1 (r) register accessor: DBGMCU CoreSight component identity register 1

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_CIDR1)

For information about available fields see [`mod@dbgmcu_cidr1`]
module*/
pub type DBGMCU_CIDR1 = crate::Reg<dbgmcu_cidr1::DBGMCU_CIDR1rs>;
///DBGMCU CoreSight component identity register 1
pub mod dbgmcu_cidr1;
/**DBGMCU_CIDR2 (r) register accessor: DBGMCU CoreSight component identity register 2

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_CIDR2)

For information about available fields see [`mod@dbgmcu_cidr2`]
module*/
pub type DBGMCU_CIDR2 = crate::Reg<dbgmcu_cidr2::DBGMCU_CIDR2rs>;
///DBGMCU CoreSight component identity register 2
pub mod dbgmcu_cidr2;
/**DBGMCU_CIDR3 (r) register accessor: DBGMCU CoreSight component identity register 3

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_CIDR3)

For information about available fields see [`mod@dbgmcu_cidr3`]
module*/
pub type DBGMCU_CIDR3 = crate::Reg<dbgmcu_cidr3::DBGMCU_CIDR3rs>;
///DBGMCU CoreSight component identity register 3
pub mod dbgmcu_cidr3;
