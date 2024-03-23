#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    oar1: OAR1,
    oar2: OAR2,
    timingr: TIMINGR,
    timeoutr: TIMEOUTR,
    isr: ISR,
    icr: ICR,
    pecr: PECR,
    rxdr: RXDR,
    txdr: TXDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    #[inline(always)]
    pub const fn oar1(&self) -> &OAR1 {
        &self.oar1
    }
    #[doc = "0x0c - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    #[inline(always)]
    pub const fn oar2(&self) -> &OAR2 {
        &self.oar2
    }
    #[doc = "0x10 - Access: No wait states"]
    #[inline(always)]
    pub const fn timingr(&self) -> &TIMINGR {
        &self.timingr
    }
    #[doc = "0x14 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
    #[inline(always)]
    pub const fn timeoutr(&self) -> &TIMEOUTR {
        &self.timeoutr
    }
    #[doc = "0x18 - Access: No wait states"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x1c - Access: No wait states"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x20 - Access: No wait states"]
    #[inline(always)]
    pub const fn pecr(&self) -> &PECR {
        &self.pecr
    }
    #[doc = "0x24 - Access: No wait states"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    #[doc = "0x28 - Access: No wait states"]
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
}
#[doc = "CR1 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod cr2;
#[doc = "OAR1 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar1`]
module"]
pub type OAR1 = crate::Reg<oar1::OAR1rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod oar1;
#[doc = "OAR2 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar2`]
module"]
pub type OAR2 = crate::Reg<oar2::OAR2rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod oar2;
#[doc = "TIMINGR (rw) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timingr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timingr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr`]
module"]
pub type TIMINGR = crate::Reg<timingr::TIMINGRrs>;
#[doc = "Access: No wait states"]
pub mod timingr;
#[doc = "TIMEOUTR (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeoutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeoutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeoutr`]
module"]
pub type TIMEOUTR = crate::Reg<timeoutr::TIMEOUTRrs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK."]
pub mod timeoutr;
#[doc = "ISR (rw) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Access: No wait states"]
pub mod isr;
#[doc = "ICR (w) register accessor: Access: No wait states\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "Access: No wait states"]
pub mod icr;
#[doc = "PECR (r) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pecr`]
module"]
pub type PECR = crate::Reg<pecr::PECRrs>;
#[doc = "Access: No wait states"]
pub mod pecr;
#[doc = "RXDR (r) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
#[doc = "Access: No wait states"]
pub mod rxdr;
#[doc = "TXDR (rw) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
pub type TXDR = crate::Reg<txdr::TXDRrs>;
#[doc = "Access: No wait states"]
pub mod txdr;
