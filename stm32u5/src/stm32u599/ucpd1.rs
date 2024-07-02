#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    cfgr3: CFGR3,
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
}
impl RegisterBlock {
    ///0x00 - UCPD configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x04 - UCPD configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x08 - UCPD configuration register 3
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
    ///0x0c - UCPD control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - UCPD Interrupt Mask Register
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x14 - UCPD Status Register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - UCPD Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x1c - UCPD Tx Ordered Set Type Register
    #[inline(always)]
    pub const fn tx_ordset(&self) -> &TX_ORDSET {
        &self.tx_ordset
    }
    ///0x20 - UCPD Tx payload size Register
    #[inline(always)]
    pub const fn tx_paysz(&self) -> &TX_PAYSZ {
        &self.tx_paysz
    }
    ///0x24 - UCPD Tx Data Register
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
    ///0x28 - UCPD Rx Ordered Set Register
    #[inline(always)]
    pub const fn rx_ordset(&self) -> &RX_ORDSET {
        &self.rx_ordset
    }
    ///0x2c - UCPD Rx payload size Register
    #[inline(always)]
    pub const fn rx_paysz(&self) -> &RX_PAYSZ {
        &self.rx_paysz
    }
    ///0x30 - UCPD Receive Data Register
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    ///0x34 - UCPD Rx Ordered Set Extension Register 1
    #[inline(always)]
    pub const fn rx_ordext1(&self) -> &RX_ORDEXT1 {
        &self.rx_ordext1
    }
    ///0x38 - UCPD Rx Ordered Set Extension Register 2
    #[inline(always)]
    pub const fn rx_ordext2(&self) -> &RX_ORDEXT2 {
        &self.rx_ordext2
    }
}
/**CFGR1 (rw) register accessor: UCPD configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:CFGR1)

For information about available fields see [`mod@cfgr1`]
module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///UCPD configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: UCPD configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:CFGR2)

For information about available fields see [`mod@cfgr2`]
module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///UCPD configuration register 2
pub mod cfgr2;
/**CFGR3 (rw) register accessor: UCPD configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:CFGR3)

For information about available fields see [`mod@cfgr3`]
module*/
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
///UCPD configuration register 3
pub mod cfgr3;
/**CR (rw) register accessor: UCPD control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:CR)

For information about available fields see [`mod@cr`]
module*/
pub type CR = crate::Reg<cr::CRrs>;
///UCPD control register
pub mod cr;
/**IMR (rw) register accessor: UCPD Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:IMR)

For information about available fields see [`mod@imr`]
module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///UCPD Interrupt Mask Register
pub mod imr;
/**SR (r) register accessor: UCPD Status Register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:SR)

For information about available fields see [`mod@sr`]
module*/
pub type SR = crate::Reg<sr::SRrs>;
///UCPD Status Register
pub mod sr;
/**ICR (w) register accessor: UCPD Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:ICR)

For information about available fields see [`mod@icr`]
module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///UCPD Interrupt Clear Register
pub mod icr;
/**TX_ORDSET (rw) register accessor: UCPD Tx Ordered Set Type Register

You can [`read`](crate::Reg::read) this register and get [`tx_ordset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ordset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:TX_ORDSET)

For information about available fields see [`mod@tx_ordset`]
module*/
pub type TX_ORDSET = crate::Reg<tx_ordset::TX_ORDSETrs>;
///UCPD Tx Ordered Set Type Register
pub mod tx_ordset;
/**TX_PAYSZ (rw) register accessor: UCPD Tx payload size Register

You can [`read`](crate::Reg::read) this register and get [`tx_paysz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_paysz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:TX_PAYSZ)

For information about available fields see [`mod@tx_paysz`]
module*/
pub type TX_PAYSZ = crate::Reg<tx_paysz::TX_PAYSZrs>;
///UCPD Tx payload size Register
pub mod tx_paysz;
/**TXDR (rw) register accessor: UCPD Tx Data Register

You can [`read`](crate::Reg::read) this register and get [`txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:TXDR)

For information about available fields see [`mod@txdr`]
module*/
pub type TXDR = crate::Reg<txdr::TXDRrs>;
///UCPD Tx Data Register
pub mod txdr;
/**RX_ORDSET (r) register accessor: UCPD Rx Ordered Set Register

You can [`read`](crate::Reg::read) this register and get [`rx_ordset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:RX_ORDSET)

For information about available fields see [`mod@rx_ordset`]
module*/
pub type RX_ORDSET = crate::Reg<rx_ordset::RX_ORDSETrs>;
///UCPD Rx Ordered Set Register
pub mod rx_ordset;
/**RX_PAYSZ (r) register accessor: UCPD Rx payload size Register

You can [`read`](crate::Reg::read) this register and get [`rx_paysz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:RX_PAYSZ)

For information about available fields see [`mod@rx_paysz`]
module*/
pub type RX_PAYSZ = crate::Reg<rx_paysz::RX_PAYSZrs>;
///UCPD Rx payload size Register
pub mod rx_paysz;
/**RXDR (r) register accessor: UCPD Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:RXDR)

For information about available fields see [`mod@rxdr`]
module*/
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
///UCPD Receive Data Register
pub mod rxdr;
/**RX_ORDEXT1 (rw) register accessor: UCPD Rx Ordered Set Extension Register 1

You can [`read`](crate::Reg::read) this register and get [`rx_ordext1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordext1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:RX_ORDEXT1)

For information about available fields see [`mod@rx_ordext1`]
module*/
pub type RX_ORDEXT1 = crate::Reg<rx_ordext1::RX_ORDEXT1rs>;
///UCPD Rx Ordered Set Extension Register 1
pub mod rx_ordext1;
/**RX_ORDEXT2 (rw) register accessor: UCPD Rx Ordered Set Extension Register 2

You can [`read`](crate::Reg::read) this register and get [`rx_ordext2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordext2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#UCPD1:RX_ORDEXT2)

For information about available fields see [`mod@rx_ordext2`]
module*/
pub type RX_ORDEXT2 = crate::Reg<rx_ordext2::RX_ORDEXT2rs>;
///UCPD Rx Ordered Set Extension Register 2
pub mod rx_ordext2;
