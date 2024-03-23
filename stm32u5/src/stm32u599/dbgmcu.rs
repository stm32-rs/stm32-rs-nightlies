#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb1lfzr: APB1LFZR,
    apb1hfzr: APB1HFZR,
    apb2fzr: APB2FZR,
    apb3fzr: APB3FZR,
    _reserved6: [u8; 0x08],
    ahb1fzr: AHB1FZR,
    _reserved7: [u8; 0x04],
    ahb3fzr: AHB3FZR,
    _reserved8: [u8; 0xd0],
    sr: SR,
    dbgmcu_dbg_auth_host: DBGMCU_DBG_AUTH_HOST,
    dbg_auth_device: DBG_AUTH_DEVICE,
    _reserved11: [u8; 0x0ec8],
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
    #[doc = "0x00 - DBGMCU_IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    #[doc = "0x04 - Debug MCU configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - Debug MCU APB1L peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1lfzr(&self) -> &APB1LFZR {
        &self.apb1lfzr
    }
    #[doc = "0x0c - Debug MCU APB1H peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1hfzr(&self) -> &APB1HFZR {
        &self.apb1hfzr
    }
    #[doc = "0x10 - Debug MCU APB2 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb2fzr(&self) -> &APB2FZR {
        &self.apb2fzr
    }
    #[doc = "0x14 - Debug MCU APB3 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb3fzr(&self) -> &APB3FZR {
        &self.apb3fzr
    }
    #[doc = "0x20 - Debug MCU AHB1 peripheral freeze register"]
    #[inline(always)]
    pub const fn ahb1fzr(&self) -> &AHB1FZR {
        &self.ahb1fzr
    }
    #[doc = "0x28 - Debug MCU AHB3 peripheral freeze register"]
    #[inline(always)]
    pub const fn ahb3fzr(&self) -> &AHB3FZR {
        &self.ahb3fzr
    }
    #[doc = "0xfc - DBGMCU status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x100 - DBGMCU debug host authentication register"]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_host(&self) -> &DBGMCU_DBG_AUTH_HOST {
        &self.dbgmcu_dbg_auth_host
    }
    #[doc = "0x104 - DBGMCU debug device authentication register"]
    #[inline(always)]
    pub const fn dbg_auth_device(&self) -> &DBG_AUTH_DEVICE {
        &self.dbg_auth_device
    }
    #[doc = "0xfd0 - Debug MCU CoreSight peripheral identity register 4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &PIDR4 {
        &self.pidr4
    }
    #[doc = "0xfe0 - Debug MCU CoreSight peripheral identity register 0"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &PIDR0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Debug MCU CoreSight peripheral identity register 1"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &PIDR1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Debug MCU CoreSight peripheral identity register 2"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &PIDR2 {
        &self.pidr2
    }
    #[doc = "0xfec - Debug MCU CoreSight peripheral identity register 3"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &PIDR3 {
        &self.pidr3
    }
    #[doc = "0xff0 - Debug MCU CoreSight component identity register 0"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &CIDR0 {
        &self.cidr0
    }
    #[doc = "0xff4 - Debug MCU CoreSight component identity register 1"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &CIDR1 {
        &self.cidr1
    }
    #[doc = "0xff8 - Debug MCU CoreSight component identity register 2"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &CIDR2 {
        &self.cidr2
    }
    #[doc = "0xffc - Debug MCU CoreSight component identity register 3"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &CIDR3 {
        &self.cidr3
    }
}
#[doc = "IDCODE (r) register accessor: DBGMCU_IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`]
module"]
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "CR (rw) register accessor: Debug MCU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Debug MCU configuration register"]
pub mod cr;
#[doc = "APB1LFZR (rw) register accessor: Debug MCU APB1L peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lfzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lfzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lfzr`]
module"]
pub type APB1LFZR = crate::Reg<apb1lfzr::APB1LFZRrs>;
#[doc = "Debug MCU APB1L peripheral freeze register"]
pub mod apb1lfzr;
#[doc = "APB1HFZR (rw) register accessor: Debug MCU APB1H peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hfzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hfzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hfzr`]
module"]
pub type APB1HFZR = crate::Reg<apb1hfzr::APB1HFZRrs>;
#[doc = "Debug MCU APB1H peripheral freeze register"]
pub mod apb1hfzr;
#[doc = "APB2FZR (rw) register accessor: Debug MCU APB2 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2fzr`]
module"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZRrs>;
#[doc = "Debug MCU APB2 peripheral freeze register"]
pub mod apb2fzr;
#[doc = "APB3FZR (rw) register accessor: Debug MCU APB3 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3fzr`]
module"]
pub type APB3FZR = crate::Reg<apb3fzr::APB3FZRrs>;
#[doc = "Debug MCU APB3 peripheral freeze register"]
pub mod apb3fzr;
#[doc = "AHB1FZR (rw) register accessor: Debug MCU AHB1 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1fzr`]
module"]
pub type AHB1FZR = crate::Reg<ahb1fzr::AHB1FZRrs>;
#[doc = "Debug MCU AHB1 peripheral freeze register"]
pub mod ahb1fzr;
#[doc = "AHB3FZR (rw) register accessor: Debug MCU AHB3 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3fzr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3fzr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3fzr`]
module"]
pub type AHB3FZR = crate::Reg<ahb3fzr::AHB3FZRrs>;
#[doc = "Debug MCU AHB3 peripheral freeze register"]
pub mod ahb3fzr;
#[doc = "SR (r) register accessor: DBGMCU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "DBGMCU status register"]
pub mod sr;
#[doc = "DBGMCU_DBG_AUTH_HOST (r) register accessor: DBGMCU debug host authentication register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_dbg_auth_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgmcu_dbg_auth_host`]
module"]
pub type DBGMCU_DBG_AUTH_HOST = crate::Reg<dbgmcu_dbg_auth_host::DBGMCU_DBG_AUTH_HOSTrs>;
#[doc = "DBGMCU debug host authentication register"]
pub mod dbgmcu_dbg_auth_host;
#[doc = "DBG_AUTH_DEVICE (r) register accessor: DBGMCU debug device authentication register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_auth_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_auth_device`]
module"]
pub type DBG_AUTH_DEVICE = crate::Reg<dbg_auth_device::DBG_AUTH_DEVICErs>;
#[doc = "DBGMCU debug device authentication register"]
pub mod dbg_auth_device;
#[doc = "PIDR4 (r) register accessor: Debug MCU CoreSight peripheral identity register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`]
module"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4rs>;
#[doc = "Debug MCU CoreSight peripheral identity register 4"]
pub mod pidr4;
#[doc = "PIDR0 (r) register accessor: Debug MCU CoreSight peripheral identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`]
module"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0rs>;
#[doc = "Debug MCU CoreSight peripheral identity register 0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: Debug MCU CoreSight peripheral identity register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`]
module"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1rs>;
#[doc = "Debug MCU CoreSight peripheral identity register 1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: Debug MCU CoreSight peripheral identity register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`]
module"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2rs>;
#[doc = "Debug MCU CoreSight peripheral identity register 2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: Debug MCU CoreSight peripheral identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`]
module"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3rs>;
#[doc = "Debug MCU CoreSight peripheral identity register 3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: Debug MCU CoreSight component identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`]
module"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0rs>;
#[doc = "Debug MCU CoreSight component identity register 0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: Debug MCU CoreSight component identity register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`]
module"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1rs>;
#[doc = "Debug MCU CoreSight component identity register 1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: Debug MCU CoreSight component identity register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`]
module"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2rs>;
#[doc = "Debug MCU CoreSight component identity register 2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: Debug MCU CoreSight component identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`]
module"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3rs>;
#[doc = "Debug MCU CoreSight component identity register 3"]
pub mod cidr3;
