#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - SPI/I2S control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - SPI/I2S control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - SPI/I2S configuration register 1
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    ///0x0c - SPI/I2S configuration register 2
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    ///0x10 - SPI/I2S interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x14 - SPI/I2S status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - SPI/I2S interrupt/status flags clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x20 - SPI/I2S transmit data register
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
    ///0x30 - SPI/I2S receive data register
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    ///0x40 - SPI/I2S polynomial register
    #[inline(always)]
    pub const fn crcpoly(&self) -> &CRCPOLY {
        &self.crcpoly
    }
    ///0x44 - SPI/I2S transmitter CRC register
    #[inline(always)]
    pub const fn txcrc(&self) -> &TXCRC {
        &self.txcrc
    }
    ///0x48 - SPI/I2S receiver CRC register
    #[inline(always)]
    pub const fn rxcrc(&self) -> &RXCRC {
        &self.rxcrc
    }
    ///0x4c - SPI/I2S underrun data register
    #[inline(always)]
    pub const fn udrdr(&self) -> &UDRDR {
        &self.udrdr
    }
    ///0x50 - SPI/I2S configuration register
    #[inline(always)]
    pub const fn i2scfgr(&self) -> &I2SCFGR {
        &self.i2scfgr
    }
}
/**CR1 (rw) register accessor: SPI/I2S control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///SPI/I2S control register 1
pub mod cr1;
/**CR2 (rw) register accessor: SPI/I2S control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///SPI/I2S control register 2
pub mod cr2;
/**CFG1 (rw) register accessor: SPI/I2S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:CFG1)

For information about available fields see [`mod@cfg1`] module*/
pub type CFG1 = crate::Reg<cfg1::CFG1rs>;
///SPI/I2S configuration register 1
pub mod cfg1;
/**CFG2 (rw) register accessor: SPI/I2S configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:CFG2)

For information about available fields see [`mod@cfg2`] module*/
pub type CFG2 = crate::Reg<cfg2::CFG2rs>;
///SPI/I2S configuration register 2
pub mod cfg2;
/**IER (rw) register accessor: SPI/I2S interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///SPI/I2S interrupt enable register
pub mod ier;
/**SR (r) register accessor: SPI/I2S status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SPI/I2S status register
pub mod sr;
/**IFCR (w) register accessor: SPI/I2S interrupt/status flags clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///SPI/I2S interrupt/status flags clear register
pub mod ifcr;
/**TXDR (w) register accessor: SPI/I2S transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:TXDR)

For information about available fields see [`mod@txdr`] module*/
pub type TXDR = crate::Reg<txdr::TXDRrs>;
///SPI/I2S transmit data register
pub mod txdr;
/**RXDR (r) register accessor: SPI/I2S receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:RXDR)

For information about available fields see [`mod@rxdr`] module*/
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
///SPI/I2S receive data register
pub mod rxdr;
/**CRCPOLY (rw) register accessor: SPI/I2S polynomial register

You can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:CRCPOLY)

For information about available fields see [`mod@crcpoly`] module*/
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLYrs>;
///SPI/I2S polynomial register
pub mod crcpoly;
/**TXCRC (r) register accessor: SPI/I2S transmitter CRC register

You can [`read`](crate::Reg::read) this register and get [`txcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:TXCRC)

For information about available fields see [`mod@txcrc`] module*/
pub type TXCRC = crate::Reg<txcrc::TXCRCrs>;
///SPI/I2S transmitter CRC register
pub mod txcrc;
/**RXCRC (r) register accessor: SPI/I2S receiver CRC register

You can [`read`](crate::Reg::read) this register and get [`rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:RXCRC)

For information about available fields see [`mod@rxcrc`] module*/
pub type RXCRC = crate::Reg<rxcrc::RXCRCrs>;
///SPI/I2S receiver CRC register
pub mod rxcrc;
/**UDRDR (rw) register accessor: SPI/I2S underrun data register

You can [`read`](crate::Reg::read) this register and get [`udrdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udrdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:UDRDR)

For information about available fields see [`mod@udrdr`] module*/
pub type UDRDR = crate::Reg<udrdr::UDRDRrs>;
///SPI/I2S underrun data register
pub mod udrdr;
/**I2SCFGR (rw) register accessor: SPI/I2S configuration register

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:I2SCFGR)

For information about available fields see [`mod@i2scfgr`] module*/
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGRrs>;
///SPI/I2S configuration register
pub mod i2scfgr;
