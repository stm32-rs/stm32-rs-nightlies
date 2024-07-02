#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - UCPD configuration register
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    ///0x04 - UCPD configuration register 2
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    ///0x08 - UCPD configuration register 3
    #[inline(always)]
    pub const fn cfg3(&self) -> &CFG3 {
        &self.cfg3
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
    ///0x20 - UCPD Tx Paysize Register
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
    ///0x2c - UCPD Rx Paysize Register
    #[inline(always)]
    pub const fn rx_paysz(&self) -> &RX_PAYSZ {
        &self.rx_paysz
    }
    ///0x30 - UCPD Receive Data Register
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    ///0x34 - UCPD Rx Ordered Set Extension Register
    #[inline(always)]
    pub const fn rx_ordext1(&self) -> &RX_ORDEXT1 {
        &self.rx_ordext1
    }
    ///0x38 - UCPD Rx Ordered Set Extension Register
    #[inline(always)]
    pub const fn rx_ordext2(&self) -> &RX_ORDEXT2 {
        &self.rx_ordext2
    }
    ///0x3f4 - UCPD IP ID register
    #[inline(always)]
    pub const fn ipver(&self) -> &IPVER {
        &self.ipver
    }
    ///0x3f8 - UCPD IP ID register
    #[inline(always)]
    pub const fn ipid(&self) -> &IPID {
        &self.ipid
    }
    ///0x3fc - UCPD IP ID register
    #[inline(always)]
    pub const fn mid(&self) -> &MID {
        &self.mid
    }
}
/**CFG1 (rw) register accessor: UCPD configuration register

You can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:CFG1)

For information about available fields see [`mod@cfg1`]
module*/
pub type CFG1 = crate::Reg<cfg1::CFG1rs>;
///UCPD configuration register
pub mod cfg1;
/**CFG2 (rw) register accessor: UCPD configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:CFG2)

For information about available fields see [`mod@cfg2`]
module*/
pub type CFG2 = crate::Reg<cfg2::CFG2rs>;
///UCPD configuration register 2
pub mod cfg2;
/**CFG3 (rw) register accessor: UCPD configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:CFG3)

For information about available fields see [`mod@cfg3`]
module*/
pub type CFG3 = crate::Reg<cfg3::CFG3rs>;
///UCPD configuration register 3
pub mod cfg3;
/**CR (rw) register accessor: UCPD control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:CR)

For information about available fields see [`mod@cr`]
module*/
pub type CR = crate::Reg<cr::CRrs>;
///UCPD control register
pub mod cr;
/**IMR (rw) register accessor: UCPD Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:IMR)

For information about available fields see [`mod@imr`]
module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///UCPD Interrupt Mask Register
pub mod imr;
/**SR (r) register accessor: UCPD Status Register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:SR)

For information about available fields see [`mod@sr`]
module*/
pub type SR = crate::Reg<sr::SRrs>;
///UCPD Status Register
pub mod sr;
/**ICR (rw) register accessor: UCPD Interrupt Clear Register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:ICR)

For information about available fields see [`mod@icr`]
module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///UCPD Interrupt Clear Register
pub mod icr;
/**TX_ORDSET (rw) register accessor: UCPD Tx Ordered Set Type Register

You can [`read`](crate::Reg::read) this register and get [`tx_ordset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ordset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:TX_ORDSET)

For information about available fields see [`mod@tx_ordset`]
module*/
pub type TX_ORDSET = crate::Reg<tx_ordset::TX_ORDSETrs>;
///UCPD Tx Ordered Set Type Register
pub mod tx_ordset;
/**TX_PAYSZ (rw) register accessor: UCPD Tx Paysize Register

You can [`read`](crate::Reg::read) this register and get [`tx_paysz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_paysz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:TX_PAYSZ)

For information about available fields see [`mod@tx_paysz`]
module*/
pub type TX_PAYSZ = crate::Reg<tx_paysz::TX_PAYSZrs>;
///UCPD Tx Paysize Register
pub mod tx_paysz;
/**TXDR (rw) register accessor: UCPD Tx Data Register

You can [`read`](crate::Reg::read) this register and get [`txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:TXDR)

For information about available fields see [`mod@txdr`]
module*/
pub type TXDR = crate::Reg<txdr::TXDRrs>;
///UCPD Tx Data Register
pub mod txdr;
/**RX_ORDSET (r) register accessor: UCPD Rx Ordered Set Register

You can [`read`](crate::Reg::read) this register and get [`rx_ordset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:RX_ORDSET)

For information about available fields see [`mod@rx_ordset`]
module*/
pub type RX_ORDSET = crate::Reg<rx_ordset::RX_ORDSETrs>;
///UCPD Rx Ordered Set Register
pub mod rx_ordset;
/**RX_PAYSZ (rw) register accessor: UCPD Rx Paysize Register

You can [`read`](crate::Reg::read) this register and get [`rx_paysz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_paysz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:RX_PAYSZ)

For information about available fields see [`mod@rx_paysz`]
module*/
pub type RX_PAYSZ = crate::Reg<rx_paysz::RX_PAYSZrs>;
///UCPD Rx Paysize Register
pub mod rx_paysz;
/**RXDR (r) register accessor: UCPD Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:RXDR)

For information about available fields see [`mod@rxdr`]
module*/
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
///UCPD Receive Data Register
pub mod rxdr;
/**RX_ORDEXT1 (rw) register accessor: UCPD Rx Ordered Set Extension Register

You can [`read`](crate::Reg::read) this register and get [`rx_ordext1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordext1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:RX_ORDEXT1)

For information about available fields see [`mod@rx_ordext1`]
module*/
pub type RX_ORDEXT1 = crate::Reg<rx_ordext1::RX_ORDEXT1rs>;
///UCPD Rx Ordered Set Extension Register
pub mod rx_ordext1;
/**RX_ORDEXT2 (rw) register accessor: UCPD Rx Ordered Set Extension Register

You can [`read`](crate::Reg::read) this register and get [`rx_ordext2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordext2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:RX_ORDEXT2)

For information about available fields see [`mod@rx_ordext2`]
module*/
pub type RX_ORDEXT2 = crate::Reg<rx_ordext2::RX_ORDEXT2rs>;
///UCPD Rx Ordered Set Extension Register
pub mod rx_ordext2;
/**IPVER (r) register accessor: UCPD IP ID register

You can [`read`](crate::Reg::read) this register and get [`ipver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:IPVER)

For information about available fields see [`mod@ipver`]
module*/
pub type IPVER = crate::Reg<ipver::IPVERrs>;
///UCPD IP ID register
pub mod ipver;
/**IPID (r) register accessor: UCPD IP ID register

You can [`read`](crate::Reg::read) this register and get [`ipid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:IPID)

For information about available fields see [`mod@ipid`]
module*/
pub type IPID = crate::Reg<ipid::IPIDrs>;
///UCPD IP ID register
pub mod ipid;
/**MID (r) register accessor: UCPD IP ID register

You can [`read`](crate::Reg::read) this register and get [`mid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:MID)

For information about available fields see [`mod@mid`]
module*/
pub type MID = crate::Reg<mid::MIDrs>;
///UCPD IP ID register
pub mod mid;
