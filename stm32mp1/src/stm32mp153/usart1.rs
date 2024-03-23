#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    brr: BRR,
    gtpr: GTPR,
    rtor: RTOR,
    rqr: RQR,
    isr: ISR,
    icr: ICR,
    rdr: RDR,
    tdr: TDR,
    presc: PRESC,
    _reserved12: [u8; 0x03bc],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - Control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - Control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    #[doc = "0x0c - Baud rate register"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    #[doc = "0x10 - Guard time and prescaler register"]
    #[inline(always)]
    pub const fn gtpr(&self) -> &GTPR {
        &self.gtpr
    }
    #[doc = "0x14 - Receiver timeout register"]
    #[inline(always)]
    pub const fn rtor(&self) -> &RTOR {
        &self.rtor
    }
    #[doc = "0x18 - Request register"]
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    #[doc = "0x1c - Interrupt &amp; status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x20 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x24 - Receive data register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    #[doc = "0x28 - Transmit data register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    #[doc = "0x2c - Prescaler register"]
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
    #[doc = "0x3ec - USART Hardware Configuration register 2"]
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    #[doc = "0x3f0 - USART Hardware Configuration register 1"]
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    #[doc = "0x3f4 - EXTI IP Version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - EXTI Identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - EXTI Size ID register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "CR1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3rs>;
#[doc = "Control register 3"]
pub mod cr3;
#[doc = "BRR (rw) register accessor: Baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRRrs>;
#[doc = "Baud rate register"]
pub mod brr;
#[doc = "GTPR (rw) register accessor: Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpr`]
module"]
pub type GTPR = crate::Reg<gtpr::GTPRrs>;
#[doc = "Guard time and prescaler register"]
pub mod gtpr;
#[doc = "RTOR (rw) register accessor: Receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtor`]
module"]
pub type RTOR = crate::Reg<rtor::RTORrs>;
#[doc = "Receiver timeout register"]
pub mod rtor;
#[doc = "RQR (w) register accessor: Request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqr`]
module"]
pub type RQR = crate::Reg<rqr::RQRrs>;
#[doc = "Request register"]
pub mod rqr;
#[doc = "ISR (r) register accessor: Interrupt &amp; status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Interrupt &amp; status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "RDR (r) register accessor: Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDRrs>;
#[doc = "Receive data register"]
pub mod rdr;
#[doc = "TDR (rw) register accessor: Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDRrs>;
#[doc = "Transmit data register"]
pub mod tdr;
#[doc = "PRESC (rw) register accessor: Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`presc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presc`]
module"]
pub type PRESC = crate::Reg<presc::PRESCrs>;
#[doc = "Prescaler register"]
pub mod presc;
#[doc = "HWCFGR2 (r) register accessor: USART Hardware Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr2`]
module"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
#[doc = "USART Hardware Configuration register 2"]
pub mod hwcfgr2;
#[doc = "HWCFGR1 (r) register accessor: USART Hardware Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr1`]
module"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
#[doc = "USART Hardware Configuration register 1"]
pub mod hwcfgr1;
#[doc = "VERR (r) register accessor: EXTI IP Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: EXTI Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: EXTI Size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "EXTI Size ID register"]
pub mod sidr;
