#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg1: CFG1,
    cfg2: CFG2,
    cfg3: CFG3,
    cr: CR,
    imr: IMR,
    sr: SR,
    icr: ICR,
    tx_ordset: TX_ORDSET,
    tx_paysz: TX_PAYSZ,
    txdr: TXDR,
    rx_ordset: RX_ORDSET,
    rx_paysz: RX_PAYSZ,
    rxdr: RXDR,
    rx_ordext1: RX_ORDEXT1,
    rx_ordext2: RX_ORDEXT2,
    _reserved15: [u8; 0x03b8],
    ipver: IPVER,
    ipid: IPID,
    mid: MID,
}
impl RegisterBlock {
    #[doc = "0x00 - UCPD configuration register"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    #[doc = "0x04 - UCPD configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    #[doc = "0x08 - UCPD configuration register 3"]
    #[inline(always)]
    pub const fn cfg3(&self) -> &CFG3 {
        &self.cfg3
    }
    #[doc = "0x0c - UCPD control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x10 - UCPD Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x14 - UCPD Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - UCPD Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x1c - UCPD Tx Ordered Set Type Register"]
    #[inline(always)]
    pub const fn tx_ordset(&self) -> &TX_ORDSET {
        &self.tx_ordset
    }
    #[doc = "0x20 - UCPD Tx Paysize Register"]
    #[inline(always)]
    pub const fn tx_paysz(&self) -> &TX_PAYSZ {
        &self.tx_paysz
    }
    #[doc = "0x24 - UCPD Tx Data Register"]
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
    #[doc = "0x28 - UCPD Rx Ordered Set Register"]
    #[inline(always)]
    pub const fn rx_ordset(&self) -> &RX_ORDSET {
        &self.rx_ordset
    }
    #[doc = "0x2c - UCPD Rx Paysize Register"]
    #[inline(always)]
    pub const fn rx_paysz(&self) -> &RX_PAYSZ {
        &self.rx_paysz
    }
    #[doc = "0x30 - UCPD Receive Data Register"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    #[doc = "0x34 - UCPD Rx Ordered Set Extension Register"]
    #[inline(always)]
    pub const fn rx_ordext1(&self) -> &RX_ORDEXT1 {
        &self.rx_ordext1
    }
    #[doc = "0x38 - UCPD Rx Ordered Set Extension Register"]
    #[inline(always)]
    pub const fn rx_ordext2(&self) -> &RX_ORDEXT2 {
        &self.rx_ordext2
    }
    #[doc = "0x3f4 - UCPD IP ID register"]
    #[inline(always)]
    pub const fn ipver(&self) -> &IPVER {
        &self.ipver
    }
    #[doc = "0x3f8 - UCPD IP ID register"]
    #[inline(always)]
    pub const fn ipid(&self) -> &IPID {
        &self.ipid
    }
    #[doc = "0x3fc - UCPD IP ID register"]
    #[inline(always)]
    pub const fn mid(&self) -> &MID {
        &self.mid
    }
}
#[doc = "CFG1 (rw) register accessor: UCPD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1rs>;
#[doc = "UCPD configuration register"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: UCPD configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
pub type CFG2 = crate::Reg<cfg2::CFG2rs>;
#[doc = "UCPD configuration register 2"]
pub mod cfg2;
#[doc = "CFG3 (rw) register accessor: UCPD configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg3`]
module"]
pub type CFG3 = crate::Reg<cfg3::CFG3rs>;
#[doc = "UCPD configuration register 3"]
pub mod cfg3;
#[doc = "CR (rw) register accessor: UCPD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "UCPD control register"]
pub mod cr;
#[doc = "IMR (rw) register accessor: UCPD Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMRrs>;
#[doc = "UCPD Interrupt Mask Register"]
pub mod imr;
#[doc = "SR (r) register accessor: UCPD Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "UCPD Status Register"]
pub mod sr;
#[doc = "ICR (rw) register accessor: UCPD Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "UCPD Interrupt Clear Register"]
pub mod icr;
#[doc = "TX_ORDSET (rw) register accessor: UCPD Tx Ordered Set Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ordset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ordset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ordset`]
module"]
pub type TX_ORDSET = crate::Reg<tx_ordset::TX_ORDSETrs>;
#[doc = "UCPD Tx Ordered Set Type Register"]
pub mod tx_ordset;
#[doc = "TX_PAYSZ (rw) register accessor: UCPD Tx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_paysz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_paysz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_paysz`]
module"]
pub type TX_PAYSZ = crate::Reg<tx_paysz::TX_PAYSZrs>;
#[doc = "UCPD Tx Paysize Register"]
pub mod tx_paysz;
#[doc = "TXDR (rw) register accessor: UCPD Tx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
pub type TXDR = crate::Reg<txdr::TXDRrs>;
#[doc = "UCPD Tx Data Register"]
pub mod txdr;
#[doc = "RX_ORDSET (r) register accessor: UCPD Rx Ordered Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordset`]
module"]
pub type RX_ORDSET = crate::Reg<rx_ordset::RX_ORDSETrs>;
#[doc = "UCPD Rx Ordered Set Register"]
pub mod rx_ordset;
#[doc = "RX_PAYSZ (rw) register accessor: UCPD Rx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_paysz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_paysz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_paysz`]
module"]
pub type RX_PAYSZ = crate::Reg<rx_paysz::RX_PAYSZrs>;
#[doc = "UCPD Rx Paysize Register"]
pub mod rx_paysz;
#[doc = "RXDR (r) register accessor: UCPD Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
#[doc = "UCPD Receive Data Register"]
pub mod rxdr;
#[doc = "RX_ORDEXT1 (rw) register accessor: UCPD Rx Ordered Set Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordext1`]
module"]
pub type RX_ORDEXT1 = crate::Reg<rx_ordext1::RX_ORDEXT1rs>;
#[doc = "UCPD Rx Ordered Set Extension Register"]
pub mod rx_ordext1;
#[doc = "RX_ORDEXT2 (rw) register accessor: UCPD Rx Ordered Set Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordext2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordext2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordext2`]
module"]
pub type RX_ORDEXT2 = crate::Reg<rx_ordext2::RX_ORDEXT2rs>;
#[doc = "UCPD Rx Ordered Set Extension Register"]
pub mod rx_ordext2;
#[doc = "IPVER (r) register accessor: UCPD IP ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipver`]
module"]
pub type IPVER = crate::Reg<ipver::IPVERrs>;
#[doc = "UCPD IP ID register"]
pub mod ipver;
#[doc = "IPID (r) register accessor: UCPD IP ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipid`]
module"]
pub type IPID = crate::Reg<ipid::IPIDrs>;
#[doc = "UCPD IP ID register"]
pub mod ipid;
#[doc = "MID (r) register accessor: UCPD IP ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid`]
module"]
pub type MID = crate::Reg<mid::MIDrs>;
#[doc = "UCPD IP ID register"]
pub mod mid;
