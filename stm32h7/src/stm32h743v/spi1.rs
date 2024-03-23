#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cfg1: CFG1,
    cfg2: CFG2,
    ier: IER,
    sr: SR,
    ifcr: IFCR,
    _reserved7: [u8; 0x04],
    txdr: TXDR,
    _reserved8: [u8; 0x0c],
    rxdr: RXDR,
    _reserved9: [u8; 0x0c],
    crcpoly: CRCPOLY,
    txcrc: TXCRC,
    rxcrc: RXCRC,
    udrdr: UDRDR,
    i2scfgr: I2SCFGR,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    #[doc = "0x0c - configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - Interrupt/Status Flags Clear Register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x20 - Transmit Data Register"]
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
    #[doc = "0x30 - Receive Data Register"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    #[doc = "0x40 - Polynomial Register"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &CRCPOLY {
        &self.crcpoly
    }
    #[doc = "0x44 - Transmitter CRC Register"]
    #[inline(always)]
    pub const fn txcrc(&self) -> &TXCRC {
        &self.txcrc
    }
    #[doc = "0x48 - Receiver CRC Register"]
    #[inline(always)]
    pub const fn rxcrc(&self) -> &RXCRC {
        &self.rxcrc
    }
    #[doc = "0x4c - Underrun Data Register"]
    #[inline(always)]
    pub const fn udrdr(&self) -> &UDRDR {
        &self.udrdr
    }
    #[doc = "0x50 - configuration register"]
    #[inline(always)]
    pub const fn i2scfgr(&self) -> &I2SCFGR {
        &self.i2scfgr
    }
}
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CFG1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1rs>;
#[doc = "configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
pub type CFG2 = crate::Reg<cfg2::CFG2rs>;
#[doc = "configuration register 2"]
pub mod cfg2;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IFCR (w) register accessor: Interrupt/Status Flags Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "Interrupt/Status Flags Clear Register"]
pub mod ifcr;
#[doc = "TXDR (w) register accessor: Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
pub type TXDR = crate::Reg<txdr::TXDRrs>;
#[doc = "Transmit Data Register"]
pub mod txdr;
#[doc = "RXDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
#[doc = "Receive Data Register"]
pub mod rxdr;
#[doc = "CRCPOLY (rw) register accessor: Polynomial Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcpoly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`]
module"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLYrs>;
#[doc = "Polynomial Register"]
pub mod crcpoly;
#[doc = "TXCRC (rw) register accessor: Transmitter CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcrc`]
module"]
pub type TXCRC = crate::Reg<txcrc::TXCRCrs>;
#[doc = "Transmitter CRC Register"]
pub mod txcrc;
#[doc = "RXCRC (rw) register accessor: Receiver CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrc`]
module"]
pub type RXCRC = crate::Reg<rxcrc::RXCRCrs>;
#[doc = "Receiver CRC Register"]
pub mod rxcrc;
#[doc = "UDRDR (rw) register accessor: Underrun Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udrdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udrdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udrdr`]
module"]
pub type UDRDR = crate::Reg<udrdr::UDRDRrs>;
#[doc = "Underrun Data Register"]
pub mod udrdr;
#[doc = "I2SCFGR (rw) register accessor: configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2scfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2scfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2scfgr`]
module"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGRrs>;
#[doc = "configuration register"]
pub mod i2scfgr;
