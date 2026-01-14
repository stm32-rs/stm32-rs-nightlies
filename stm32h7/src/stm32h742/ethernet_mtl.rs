#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mtlomr: MTLOMR,
    _reserved1: [u8; 0x1c],
    mtlisr: MTLISR,
    _reserved2: [u8; 0xdc],
    mtltx_qomr: MTLTX_QOMR,
    mtltx_qur: MTLTX_QUR,
    mtltx_qdr: MTLTX_QDR,
    _reserved5: [u8; 0x20],
    mtlqicsr: MTLQICSR,
    mtlrx_qomr: MTLRX_QOMR,
    mtlrx_qmpocr: MTLRX_QMPOCR,
    mtlrx_qdr: MTLRX_QDR,
}
impl RegisterBlock {
    ///0x00 - Operating mode Register
    #[inline(always)]
    pub const fn mtlomr(&self) -> &MTLOMR {
        &self.mtlomr
    }
    ///0x20 - Interrupt status Register
    #[inline(always)]
    pub const fn mtlisr(&self) -> &MTLISR {
        &self.mtlisr
    }
    ///0x100 - Tx queue operating mode Register
    #[inline(always)]
    pub const fn mtltx_qomr(&self) -> &MTLTX_QOMR {
        &self.mtltx_qomr
    }
    ///0x104 - Tx queue underflow register
    #[inline(always)]
    pub const fn mtltx_qur(&self) -> &MTLTX_QUR {
        &self.mtltx_qur
    }
    ///0x108 - Tx queue debug Register
    #[inline(always)]
    pub const fn mtltx_qdr(&self) -> &MTLTX_QDR {
        &self.mtltx_qdr
    }
    ///0x12c - Queue interrupt control status Register
    #[inline(always)]
    pub const fn mtlqicsr(&self) -> &MTLQICSR {
        &self.mtlqicsr
    }
    ///0x130 - Rx queue operating mode register
    #[inline(always)]
    pub const fn mtlrx_qomr(&self) -> &MTLRX_QOMR {
        &self.mtlrx_qomr
    }
    ///0x134 - Rx queue missed packet and overflow counter register
    #[inline(always)]
    pub const fn mtlrx_qmpocr(&self) -> &MTLRX_QMPOCR {
        &self.mtlrx_qmpocr
    }
    ///0x138 - Rx queue debug register
    #[inline(always)]
    pub const fn mtlrx_qdr(&self) -> &MTLRX_QDR {
        &self.mtlrx_qdr
    }
}
/**MTLOMR (rw) register accessor: Operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtlomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLOMR)

For information about available fields see [`mod@mtlomr`] module*/
pub type MTLOMR = crate::Reg<mtlomr::MTLOMRrs>;
///Operating mode Register
pub mod mtlomr;
/**MTLISR (r) register accessor: Interrupt status Register

You can [`read`](crate::Reg::read) this register and get [`mtlisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLISR)

For information about available fields see [`mod@mtlisr`] module*/
pub type MTLISR = crate::Reg<mtlisr::MTLISRrs>;
///Interrupt status Register
pub mod mtlisr;
/**MTLTxQOMR (rw) register accessor: Tx queue operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_qomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_qomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLTxQOMR)

For information about available fields see [`mod@mtltx_qomr`] module*/
#[doc(alias = "MTLTxQOMR")]
pub type MTLTX_QOMR = crate::Reg<mtltx_qomr::MTLTX_QOMRrs>;
///Tx queue operating mode Register
pub mod mtltx_qomr;
/**MTLTxQUR (r) register accessor: Tx queue underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_qur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLTxQUR)

For information about available fields see [`mod@mtltx_qur`] module*/
#[doc(alias = "MTLTxQUR")]
pub type MTLTX_QUR = crate::Reg<mtltx_qur::MTLTX_QURrs>;
///Tx queue underflow register
pub mod mtltx_qur;
/**MTLTxQDR (r) register accessor: Tx queue debug Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_qdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLTxQDR)

For information about available fields see [`mod@mtltx_qdr`] module*/
#[doc(alias = "MTLTxQDR")]
pub type MTLTX_QDR = crate::Reg<mtltx_qdr::MTLTX_QDRrs>;
///Tx queue debug Register
pub mod mtltx_qdr;
/**MTLQICSR (rw) register accessor: Queue interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlqicsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlqicsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLQICSR)

For information about available fields see [`mod@mtlqicsr`] module*/
pub type MTLQICSR = crate::Reg<mtlqicsr::MTLQICSRrs>;
///Queue interrupt control status Register
pub mod mtlqicsr;
/**MTLRxQOMR (rw) register accessor: Rx queue operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_qomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_qomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLRxQOMR)

For information about available fields see [`mod@mtlrx_qomr`] module*/
#[doc(alias = "MTLRxQOMR")]
pub type MTLRX_QOMR = crate::Reg<mtlrx_qomr::MTLRX_QOMRrs>;
///Rx queue operating mode register
pub mod mtlrx_qomr;
/**MTLRxQMPOCR (r) register accessor: Rx queue missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_qmpocr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLRxQMPOCR)

For information about available fields see [`mod@mtlrx_qmpocr`] module*/
#[doc(alias = "MTLRxQMPOCR")]
pub type MTLRX_QMPOCR = crate::Reg<mtlrx_qmpocr::MTLRX_QMPOCRrs>;
///Rx queue missed packet and overflow counter register
pub mod mtlrx_qmpocr;
/**MTLRxQDR (r) register accessor: Rx queue debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_qdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLRxQDR)

For information about available fields see [`mod@mtlrx_qdr`] module*/
#[doc(alias = "MTLRxQDR")]
pub type MTLRX_QDR = crate::Reg<mtlrx_qdr::MTLRX_QDRrs>;
///Rx queue debug register
pub mod mtlrx_qdr;
