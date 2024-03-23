#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    sr: SR,
    dr: DR,
    crcpr: CRCPR,
    rxcrcr: RXCRCR,
    txcrcr: TXCRCR,
    i2scfgr: I2SCFGR,
    i2spr: I2SPR,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - SPI control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - SPI status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x0c - SPI data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x10 - SPI CRC polynomial register"]
    #[inline(always)]
    pub const fn crcpr(&self) -> &CRCPR {
        &self.crcpr
    }
    #[doc = "0x14 - SPI Rx CRC register"]
    #[inline(always)]
    pub const fn rxcrcr(&self) -> &RXCRCR {
        &self.rxcrcr
    }
    #[doc = "0x18 - SPI Tx CRC register"]
    #[inline(always)]
    pub const fn txcrcr(&self) -> &TXCRCR {
        &self.txcrcr
    }
    #[doc = "0x1c - SPI_I2S configuration register"]
    #[inline(always)]
    pub const fn i2scfgr(&self) -> &I2SCFGR {
        &self.i2scfgr
    }
    #[doc = "0x20 - SPI_I2S prescaler register"]
    #[inline(always)]
    pub const fn i2spr(&self) -> &I2SPR {
        &self.i2spr
    }
}
#[doc = "CR1 (rw) register accessor: SPI control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "SPI control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: SPI control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "SPI control register 2"]
pub mod cr2;
#[doc = "SR (rw) register accessor: SPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "SPI status register"]
pub mod sr;
#[doc = "DR (rw) register accessor: SPI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "SPI data register"]
pub mod dr;
#[doc = "CRCPR (rw) register accessor: SPI CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpr`]
module"]
pub type CRCPR = crate::Reg<crcpr::CRCPRrs>;
#[doc = "SPI CRC polynomial register"]
pub mod crcpr;
#[doc = "RXCRCR (r) register accessor: SPI Rx CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrcr`]
module"]
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCRrs>;
#[doc = "SPI Rx CRC register"]
pub mod rxcrcr;
#[doc = "TXCRCR (r) register accessor: SPI Tx CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcrcr`]
module"]
pub type TXCRCR = crate::Reg<txcrcr::TXCRCRrs>;
#[doc = "SPI Tx CRC register"]
pub mod txcrcr;
#[doc = "I2SCFGR (rw) register accessor: SPI_I2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2scfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2scfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2scfgr`]
module"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGRrs>;
#[doc = "SPI_I2S configuration register"]
pub mod i2scfgr;
#[doc = "I2SPR (rw) register accessor: SPI_I2S prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2spr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2spr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2spr`]
module"]
pub type I2SPR = crate::Reg<i2spr::I2SPRrs>;
#[doc = "SPI_I2S prescaler register"]
pub mod i2spr;
