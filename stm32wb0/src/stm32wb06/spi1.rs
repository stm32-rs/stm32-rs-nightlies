#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    _reserved2: [u8; 0x02],
    sr: SR,
    _reserved3: [u8; 0x02],
    _reserved_3_dr: [u8; 0x02],
    _reserved4: [u8; 0x02],
    crcpr: CRCPR,
    _reserved5: [u8; 0x02],
    rxcrcr: RXCRCR,
    _reserved6: [u8; 0x02],
    txcrcr: TXCRCR,
    _reserved7: [u8; 0x02],
    i2scfgr: I2SCFGR,
    _reserved8: [u8; 0x02],
    i2spr: I2SPR,
}
impl RegisterBlock {
    ///0x00 - SPI1_CR1 register
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - SPI1_CR2 register
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - SPI1_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - Direct 8-bit access to data register
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x0c - SPI1_DR register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x10 - SPI1_CRCPR register
    #[inline(always)]
    pub const fn crcpr(&self) -> &CRCPR {
        &self.crcpr
    }
    ///0x14 - SPI1_RXCRCR register
    #[inline(always)]
    pub const fn rxcrcr(&self) -> &RXCRCR {
        &self.rxcrcr
    }
    ///0x18 - SPI1_TXCRCR register
    #[inline(always)]
    pub const fn txcrcr(&self) -> &TXCRCR {
        &self.txcrcr
    }
    ///0x1c - SPI1_I2SCFGR register
    #[inline(always)]
    pub const fn i2scfgr(&self) -> &I2SCFGR {
        &self.i2scfgr
    }
    ///0x20 - SPI1_I2SPR register
    #[inline(always)]
    pub const fn i2spr(&self) -> &I2SPR {
        &self.i2spr
    }
}
/**CR1 (rw) register accessor: SPI1_CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///SPI1_CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: SPI1_CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///SPI1_CR2 register
pub mod cr2;
/**SR (rw) register accessor: SPI1_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SPI1_SR register
pub mod sr;
/**DR (rw) register accessor: SPI1_DR register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///SPI1_DR register
pub mod dr;
/**DR8 (rw) register accessor: Direct 8-bit access to data register

You can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:DR8)

For information about available fields see [`mod@dr8`] module*/
pub type DR8 = crate::Reg<dr8::DR8rs>;
///Direct 8-bit access to data register
pub mod dr8;
/**CRCPR (rw) register accessor: SPI1_CRCPR register

You can [`read`](crate::Reg::read) this register and get [`crcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:CRCPR)

For information about available fields see [`mod@crcpr`] module*/
pub type CRCPR = crate::Reg<crcpr::CRCPRrs>;
///SPI1_CRCPR register
pub mod crcpr;
/**RXCRCR (r) register accessor: SPI1_RXCRCR register

You can [`read`](crate::Reg::read) this register and get [`rxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:RXCRCR)

For information about available fields see [`mod@rxcrcr`] module*/
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCRrs>;
///SPI1_RXCRCR register
pub mod rxcrcr;
/**TXCRCR (r) register accessor: SPI1_TXCRCR register

You can [`read`](crate::Reg::read) this register and get [`txcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:TXCRCR)

For information about available fields see [`mod@txcrcr`] module*/
pub type TXCRCR = crate::Reg<txcrcr::TXCRCRrs>;
///SPI1_TXCRCR register
pub mod txcrcr;
/**I2SCFGR (rw) register accessor: SPI1_I2SCFGR register

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:I2SCFGR)

For information about available fields see [`mod@i2scfgr`] module*/
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGRrs>;
///SPI1_I2SCFGR register
pub mod i2scfgr;
/**I2SPR (rw) register accessor: SPI1_I2SPR register

You can [`read`](crate::Reg::read) this register and get [`i2spr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:I2SPR)

For information about available fields see [`mod@i2spr`] module*/
pub type I2SPR = crate::Reg<i2spr::I2SPRrs>;
///SPI1_I2SPR register
pub mod i2spr;
