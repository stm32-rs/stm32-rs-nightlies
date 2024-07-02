#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ucpd_cfgr1: UCPD_CFGR1,
    ucpd_cfgr2: UCPD_CFGR2,
    ucpd_cfgr3: UCPD_CFGR3,
    ucpd_cr: UCPD_CR,
    ucpd_imr: UCPD_IMR,
    ucpd_sr: UCPD_SR,
    ucpd_icr: UCPD_ICR,
    ucpd_tx_ordsetr: UCPD_TX_ORDSETR,
    ucpd_tx_payszr: UCPD_TX_PAYSZR,
    ucpd_txdr: UCPD_TXDR,
    ucpd_rx_ordsetr: UCPD_RX_ORDSETR,
    ucpd_rx_payszr: UCPD_RX_PAYSZR,
    ucpd_rxdr: UCPD_RXDR,
    ucpd_rx_ordextr1: UCPD_RX_ORDEXTR1,
    ucpd_rx_ordextr2: UCPD_RX_ORDEXTR2,
}
impl RegisterBlock {
    ///0x00 - UCPD configuration register 1
    #[inline(always)]
    pub const fn ucpd_cfgr1(&self) -> &UCPD_CFGR1 {
        &self.ucpd_cfgr1
    }
    ///0x04 - UCPD configuration register 2
    #[inline(always)]
    pub const fn ucpd_cfgr2(&self) -> &UCPD_CFGR2 {
        &self.ucpd_cfgr2
    }
    ///0x08 - UCPD configuration register 3
    #[inline(always)]
    pub const fn ucpd_cfgr3(&self) -> &UCPD_CFGR3 {
        &self.ucpd_cfgr3
    }
    ///0x0c - UCPD control register
    #[inline(always)]
    pub const fn ucpd_cr(&self) -> &UCPD_CR {
        &self.ucpd_cr
    }
    ///0x10 - UCPD interrupt mask register
    #[inline(always)]
    pub const fn ucpd_imr(&self) -> &UCPD_IMR {
        &self.ucpd_imr
    }
    ///0x14 - UCPD status register
    #[inline(always)]
    pub const fn ucpd_sr(&self) -> &UCPD_SR {
        &self.ucpd_sr
    }
    ///0x18 - UCPD interrupt clear register
    #[inline(always)]
    pub const fn ucpd_icr(&self) -> &UCPD_ICR {
        &self.ucpd_icr
    }
    ///0x1c - UCPD Tx ordered set type register
    #[inline(always)]
    pub const fn ucpd_tx_ordsetr(&self) -> &UCPD_TX_ORDSETR {
        &self.ucpd_tx_ordsetr
    }
    ///0x20 - UCPD Tx payload size register
    #[inline(always)]
    pub const fn ucpd_tx_payszr(&self) -> &UCPD_TX_PAYSZR {
        &self.ucpd_tx_payszr
    }
    ///0x24 - UCPD Tx data register
    #[inline(always)]
    pub const fn ucpd_txdr(&self) -> &UCPD_TXDR {
        &self.ucpd_txdr
    }
    ///0x28 -
    #[inline(always)]
    pub const fn ucpd_rx_ordsetr(&self) -> &UCPD_RX_ORDSETR {
        &self.ucpd_rx_ordsetr
    }
    ///0x2c -
    #[inline(always)]
    pub const fn ucpd_rx_payszr(&self) -> &UCPD_RX_PAYSZR {
        &self.ucpd_rx_payszr
    }
    ///0x30 -
    #[inline(always)]
    pub const fn ucpd_rxdr(&self) -> &UCPD_RXDR {
        &self.ucpd_rxdr
    }
    ///0x34 - UCPD Rx ordered set extension register 1
    #[inline(always)]
    pub const fn ucpd_rx_ordextr1(&self) -> &UCPD_RX_ORDEXTR1 {
        &self.ucpd_rx_ordextr1
    }
    ///0x38 - UCPD Rx ordered set extension register 2
    #[inline(always)]
    pub const fn ucpd_rx_ordextr2(&self) -> &UCPD_RX_ORDEXTR2 {
        &self.ucpd_rx_ordextr2
    }
}
/**UCPD_CFGR1 (rw) register accessor: UCPD configuration register 1

You can [`read`](crate::Reg::read) this register and get [`ucpd_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_CFGR1)

For information about available fields see [`mod@ucpd_cfgr1`]
module*/
pub type UCPD_CFGR1 = crate::Reg<ucpd_cfgr1::UCPD_CFGR1rs>;
///UCPD configuration register 1
pub mod ucpd_cfgr1;
/**UCPD_CFGR2 (rw) register accessor: UCPD configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ucpd_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_CFGR2)

For information about available fields see [`mod@ucpd_cfgr2`]
module*/
pub type UCPD_CFGR2 = crate::Reg<ucpd_cfgr2::UCPD_CFGR2rs>;
///UCPD configuration register 2
pub mod ucpd_cfgr2;
/**UCPD_CFGR3 (rw) register accessor: UCPD configuration register 3

You can [`read`](crate::Reg::read) this register and get [`ucpd_cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_CFGR3)

For information about available fields see [`mod@ucpd_cfgr3`]
module*/
pub type UCPD_CFGR3 = crate::Reg<ucpd_cfgr3::UCPD_CFGR3rs>;
///UCPD configuration register 3
pub mod ucpd_cfgr3;
/**UCPD_CR (rw) register accessor: UCPD control register

You can [`read`](crate::Reg::read) this register and get [`ucpd_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_CR)

For information about available fields see [`mod@ucpd_cr`]
module*/
pub type UCPD_CR = crate::Reg<ucpd_cr::UCPD_CRrs>;
///UCPD control register
pub mod ucpd_cr;
/**UCPD_IMR (rw) register accessor: UCPD interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`ucpd_imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_IMR)

For information about available fields see [`mod@ucpd_imr`]
module*/
pub type UCPD_IMR = crate::Reg<ucpd_imr::UCPD_IMRrs>;
///UCPD interrupt mask register
pub mod ucpd_imr;
/**UCPD_SR (r) register accessor: UCPD status register

You can [`read`](crate::Reg::read) this register and get [`ucpd_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_SR)

For information about available fields see [`mod@ucpd_sr`]
module*/
pub type UCPD_SR = crate::Reg<ucpd_sr::UCPD_SRrs>;
///UCPD status register
pub mod ucpd_sr;
/**UCPD_ICR (w) register accessor: UCPD interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_ICR)

For information about available fields see [`mod@ucpd_icr`]
module*/
pub type UCPD_ICR = crate::Reg<ucpd_icr::UCPD_ICRrs>;
///UCPD interrupt clear register
pub mod ucpd_icr;
/**UCPD_TX_ORDSETR (rw) register accessor: UCPD Tx ordered set type register

You can [`read`](crate::Reg::read) this register and get [`ucpd_tx_ordsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_tx_ordsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_TX_ORDSETR)

For information about available fields see [`mod@ucpd_tx_ordsetr`]
module*/
pub type UCPD_TX_ORDSETR = crate::Reg<ucpd_tx_ordsetr::UCPD_TX_ORDSETRrs>;
///UCPD Tx ordered set type register
pub mod ucpd_tx_ordsetr;
/**UCPD_TX_PAYSZR (rw) register accessor: UCPD Tx payload size register

You can [`read`](crate::Reg::read) this register and get [`ucpd_tx_payszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_tx_payszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_TX_PAYSZR)

For information about available fields see [`mod@ucpd_tx_payszr`]
module*/
pub type UCPD_TX_PAYSZR = crate::Reg<ucpd_tx_payszr::UCPD_TX_PAYSZRrs>;
///UCPD Tx payload size register
pub mod ucpd_tx_payszr;
/**UCPD_TXDR (rw) register accessor: UCPD Tx data register

You can [`read`](crate::Reg::read) this register and get [`ucpd_txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_TXDR)

For information about available fields see [`mod@ucpd_txdr`]
module*/
pub type UCPD_TXDR = crate::Reg<ucpd_txdr::UCPD_TXDRrs>;
///UCPD Tx data register
pub mod ucpd_txdr;
/**UCPD_RX_ORDSETR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ucpd_rx_ordsetr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_RX_ORDSETR)

For information about available fields see [`mod@ucpd_rx_ordsetr`]
module*/
pub type UCPD_RX_ORDSETR = crate::Reg<ucpd_rx_ordsetr::UCPD_RX_ORDSETRrs>;
///
pub mod ucpd_rx_ordsetr;
/**UCPD_RX_PAYSZR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ucpd_rx_payszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_RX_PAYSZR)

For information about available fields see [`mod@ucpd_rx_payszr`]
module*/
pub type UCPD_RX_PAYSZR = crate::Reg<ucpd_rx_payszr::UCPD_RX_PAYSZRrs>;
///
pub mod ucpd_rx_payszr;
/**UCPD_RXDR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ucpd_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_RXDR)

For information about available fields see [`mod@ucpd_rxdr`]
module*/
pub type UCPD_RXDR = crate::Reg<ucpd_rxdr::UCPD_RXDRrs>;
///
pub mod ucpd_rxdr;
/**UCPD_RX_ORDEXTR1 (rw) register accessor: UCPD Rx ordered set extension register 1

You can [`read`](crate::Reg::read) this register and get [`ucpd_rx_ordextr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_rx_ordextr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_RX_ORDEXTR1)

For information about available fields see [`mod@ucpd_rx_ordextr1`]
module*/
pub type UCPD_RX_ORDEXTR1 = crate::Reg<ucpd_rx_ordextr1::UCPD_RX_ORDEXTR1rs>;
///UCPD Rx ordered set extension register 1
pub mod ucpd_rx_ordextr1;
/**UCPD_RX_ORDEXTR2 (rw) register accessor: UCPD Rx ordered set extension register 2

You can [`read`](crate::Reg::read) this register and get [`ucpd_rx_ordextr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_rx_ordextr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_RX_ORDEXTR2)

For information about available fields see [`mod@ucpd_rx_ordextr2`]
module*/
pub type UCPD_RX_ORDEXTR2 = crate::Reg<ucpd_rx_ordextr2::UCPD_RX_ORDEXTR2rs>;
///UCPD Rx ordered set extension register 2
pub mod ucpd_rx_ordextr2;
