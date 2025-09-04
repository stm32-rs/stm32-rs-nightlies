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
    autocr: AUTOCR,
    _reserved_8_txdr: [u8; 0x04],
    _reserved9: [u8; 0x0c],
    _reserved_9_rxdr: [u8; 0x04],
    _reserved10: [u8; 0x0c],
    crcpoly: CRCPOLY,
    txcrc: TXCRC,
    rxcrc: RXCRC,
    udrdr: UDRDR,
}
impl RegisterBlock {
    ///0x00 - SPI control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - SPI control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - SPI configuration register 1
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    ///0x0c - SPI configuration register 2
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    ///0x10 - SPI interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x14 - SPI status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - SPI interrupt/status flags clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x1c - SPI autonomous mode control register
    #[inline(always)]
    pub const fn autocr(&self) -> &AUTOCR {
        &self.autocr
    }
    ///0x20 - Direct 8-bit access to transmit data register
    #[inline(always)]
    pub const fn txdr8(&self) -> &TXDR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - Direct 16-bit access to transmit data register
    #[inline(always)]
    pub const fn txdr16(&self) -> &TXDR16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - SPI transmit data register
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x30 - Direct 8-bit access to receive data register
    #[inline(always)]
    pub const fn rxdr8(&self) -> &RXDR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x30 - Direct 16-bit access to receive data register
    #[inline(always)]
    pub const fn rxdr16(&self) -> &RXDR16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x30 - SPI receive data register
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x40 - SPI polynomial register
    #[inline(always)]
    pub const fn crcpoly(&self) -> &CRCPOLY {
        &self.crcpoly
    }
    ///0x44 - SPI transmitter CRC register
    #[inline(always)]
    pub const fn txcrc(&self) -> &TXCRC {
        &self.txcrc
    }
    ///0x48 - SPI receiver CRC register
    #[inline(always)]
    pub const fn rxcrc(&self) -> &RXCRC {
        &self.rxcrc
    }
    ///0x4c - SPI underrun data register
    #[inline(always)]
    pub const fn udrdr(&self) -> &UDRDR {
        &self.udrdr
    }
}
/**CR1 (rw) register accessor: SPI control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///SPI control register 1
pub mod cr1;
/**CR2 (rw) register accessor: SPI control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///SPI control register 2
pub mod cr2;
/**CFG1 (rw) register accessor: SPI configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:CFG1)

For information about available fields see [`mod@cfg1`] module*/
pub type CFG1 = crate::Reg<cfg1::CFG1rs>;
///SPI configuration register 1
pub mod cfg1;
/**CFG2 (rw) register accessor: SPI configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:CFG2)

For information about available fields see [`mod@cfg2`] module*/
pub type CFG2 = crate::Reg<cfg2::CFG2rs>;
///SPI configuration register 2
pub mod cfg2;
/**IER (rw) register accessor: SPI interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///SPI interrupt enable register
pub mod ier;
/**SR (r) register accessor: SPI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SPI status register
pub mod sr;
/**IFCR (w) register accessor: SPI interrupt/status flags clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///SPI interrupt/status flags clear register
pub mod ifcr;
/**AUTOCR (rw) register accessor: SPI autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:AUTOCR)

For information about available fields see [`mod@autocr`] module*/
pub type AUTOCR = crate::Reg<autocr::AUTOCRrs>;
///SPI autonomous mode control register
pub mod autocr;
/**TXDR (w) register accessor: SPI transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:TXDR)

For information about available fields see [`mod@txdr`] module*/
pub type TXDR = crate::Reg<txdr::TXDRrs>;
///SPI transmit data register
pub mod txdr;
/**TXDR16 (w) register accessor: Direct 16-bit access to transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:TXDR16)

For information about available fields see [`mod@txdr16`] module*/
pub type TXDR16 = crate::Reg<txdr16::TXDR16rs>;
///Direct 16-bit access to transmit data register
pub mod txdr16;
/**TXDR8 (w) register accessor: Direct 8-bit access to transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:TXDR8)

For information about available fields see [`mod@txdr8`] module*/
pub type TXDR8 = crate::Reg<txdr8::TXDR8rs>;
///Direct 8-bit access to transmit data register
pub mod txdr8;
/**RXDR (r) register accessor: SPI receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:RXDR)

For information about available fields see [`mod@rxdr`] module*/
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
///SPI receive data register
pub mod rxdr;
/**RXDR16 (r) register accessor: Direct 16-bit access to receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:RXDR16)

For information about available fields see [`mod@rxdr16`] module*/
pub type RXDR16 = crate::Reg<rxdr16::RXDR16rs>;
///Direct 16-bit access to receive data register
pub mod rxdr16;
/**RXDR8 (r) register accessor: Direct 8-bit access to receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:RXDR8)

For information about available fields see [`mod@rxdr8`] module*/
pub type RXDR8 = crate::Reg<rxdr8::RXDR8rs>;
///Direct 8-bit access to receive data register
pub mod rxdr8;
/**CRCPOLY (rw) register accessor: SPI polynomial register

You can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:CRCPOLY)

For information about available fields see [`mod@crcpoly`] module*/
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLYrs>;
///SPI polynomial register
pub mod crcpoly;
/**TXCRC (r) register accessor: SPI transmitter CRC register

You can [`read`](crate::Reg::read) this register and get [`txcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:TXCRC)

For information about available fields see [`mod@txcrc`] module*/
pub type TXCRC = crate::Reg<txcrc::TXCRCrs>;
///SPI transmitter CRC register
pub mod txcrc;
/**RXCRC (r) register accessor: SPI receiver CRC register

You can [`read`](crate::Reg::read) this register and get [`rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:RXCRC)

For information about available fields see [`mod@rxcrc`] module*/
pub type RXCRC = crate::Reg<rxcrc::RXCRCrs>;
///SPI receiver CRC register
pub mod rxcrc;
/**UDRDR (rw) register accessor: SPI underrun data register

You can [`read`](crate::Reg::read) this register and get [`udrdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udrdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SPI1:UDRDR)

For information about available fields see [`mod@udrdr`] module*/
pub type UDRDR = crate::Reg<udrdr::UDRDRrs>;
///SPI underrun data register
pub mod udrdr;
