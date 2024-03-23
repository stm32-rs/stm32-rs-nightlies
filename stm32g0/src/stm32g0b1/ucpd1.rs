#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    cfgr3: CFGR3,
    cr: CR,
    imr: IMR,
    sr: SR,
    icr: ICR,
    tx_ordsetr: TX_ORDSETR,
    tx_payszr: TX_PAYSZR,
    txdr: TXDR,
    rx_ordsetr: RX_ORDSETR,
    rx_payszr: RX_PAYSZR,
    rxdr: RXDR,
    rx_ordextr1: RX_ORDEXTR1,
    rx_ordextr2: RX_ORDEXTR2,
}
impl RegisterBlock {
    #[doc = "0x00 - UCPD configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x04 - UCPD configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x08 - UCPD configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
    #[doc = "0x0c - UCPD control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x10 - UCPD interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x14 - UCPD status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - UCPD interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x1c - UCPD Tx ordered set type register"]
    #[inline(always)]
    pub const fn tx_ordsetr(&self) -> &TX_ORDSETR {
        &self.tx_ordsetr
    }
    #[doc = "0x20 - UCPD Tx payload size register"]
    #[inline(always)]
    pub const fn tx_payszr(&self) -> &TX_PAYSZR {
        &self.tx_payszr
    }
    #[doc = "0x24 - UCPD Tx data register"]
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
    #[doc = "0x28 - UCPD Rx ordered set register"]
    #[inline(always)]
    pub const fn rx_ordsetr(&self) -> &RX_ORDSETR {
        &self.rx_ordsetr
    }
    #[doc = "0x2c - UCPD Rx payload size register"]
    #[inline(always)]
    pub const fn rx_payszr(&self) -> &RX_PAYSZR {
        &self.rx_payszr
    }
    #[doc = "0x30 - UCPD receive data register"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    #[doc = "0x34 - UCPD Rx ordered set extension register 1"]
    #[inline(always)]
    pub const fn rx_ordextr1(&self) -> &RX_ORDEXTR1 {
        &self.rx_ordextr1
    }
    #[doc = "0x38 - UCPD Rx ordered set extension register 2"]
    #[inline(always)]
    pub const fn rx_ordextr2(&self) -> &RX_ORDEXTR2 {
        &self.rx_ordextr2
    }
}
#[doc = "CFGR1 (rw) register accessor: UCPD configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "UCPD configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: UCPD configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "UCPD configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: UCPD configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`]
module"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
#[doc = "UCPD configuration register 3"]
pub mod cfgr3;
#[doc = "CR (rw) register accessor: UCPD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "UCPD control register"]
pub mod cr;
#[doc = "IMR (rw) register accessor: UCPD interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMRrs>;
#[doc = "UCPD interrupt mask register"]
pub mod imr;
#[doc = "SR (r) register accessor: UCPD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "UCPD status register"]
pub mod sr;
#[doc = "ICR (w) register accessor: UCPD interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "UCPD interrupt clear register"]
pub mod icr;
#[doc = "TX_ORDSETR (rw) register accessor: UCPD Tx ordered set type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ordsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ordsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ordsetr`]
module"]
pub type TX_ORDSETR = crate::Reg<tx_ordsetr::TX_ORDSETRrs>;
#[doc = "UCPD Tx ordered set type register"]
pub mod tx_ordsetr;
#[doc = "TX_PAYSZR (rw) register accessor: UCPD Tx payload size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_payszr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_payszr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_payszr`]
module"]
pub type TX_PAYSZR = crate::Reg<tx_payszr::TX_PAYSZRrs>;
#[doc = "UCPD Tx payload size register"]
pub mod tx_payszr;
#[doc = "TXDR (rw) register accessor: UCPD Tx data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
pub type TXDR = crate::Reg<txdr::TXDRrs>;
#[doc = "UCPD Tx data register"]
pub mod txdr;
#[doc = "RX_ORDSETR (r) register accessor: UCPD Rx ordered set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordsetr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordsetr`]
module"]
pub type RX_ORDSETR = crate::Reg<rx_ordsetr::RX_ORDSETRrs>;
#[doc = "UCPD Rx ordered set register"]
pub mod rx_ordsetr;
#[doc = "RX_PAYSZR (r) register accessor: UCPD Rx payload size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_payszr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_payszr`]
module"]
pub type RX_PAYSZR = crate::Reg<rx_payszr::RX_PAYSZRrs>;
#[doc = "UCPD Rx payload size register"]
pub mod rx_payszr;
#[doc = "RXDR (r) register accessor: UCPD receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
#[doc = "UCPD receive data register"]
pub mod rxdr;
#[doc = "RX_ORDEXTR1 (rw) register accessor: UCPD Rx ordered set extension register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordextr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordextr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordextr1`]
module"]
pub type RX_ORDEXTR1 = crate::Reg<rx_ordextr1::RX_ORDEXTR1rs>;
#[doc = "UCPD Rx ordered set extension register 1"]
pub mod rx_ordextr1;
#[doc = "RX_ORDEXTR2 (rw) register accessor: UCPD Rx ordered set extension register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordextr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordextr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordextr2`]
module"]
pub type RX_ORDEXTR2 = crate::Reg<rx_ordextr2::RX_ORDEXTR2rs>;
#[doc = "UCPD Rx ordered set extension register 2"]
pub mod rx_ordextr2;
