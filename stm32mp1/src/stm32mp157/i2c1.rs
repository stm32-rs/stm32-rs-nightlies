#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_cr1: I2C_CR1,
    i2c_cr2: I2C_CR2,
    i2c_oar1: I2C_OAR1,
    i2c_oar2: I2C_OAR2,
    i2c_timingr: I2C_TIMINGR,
    i2c_timeoutr: I2C_TIMEOUTR,
    i2c_isr: I2C_ISR,
    i2c_icr: I2C_ICR,
    i2c_pecr: I2C_PECR,
    i2c_rxdr: I2C_RXDR,
    i2c_txdr: I2C_TXDR,
    _reserved11: [u8; 0x03c4],
    i2c_hwcfgr: I2C_HWCFGR,
    i2c_verr: I2C_VERR,
    i2c_ipidr: I2C_IPIDR,
    i2c_sidr: I2C_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck."]
    #[inline(always)]
    pub const fn i2c_cr1(&self) -> &I2C_CR1 {
        &self.i2c_cr1
    }
    #[doc = "0x04 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    #[inline(always)]
    pub const fn i2c_cr2(&self) -> &I2C_CR2 {
        &self.i2c_cr2
    }
    #[doc = "0x08 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    #[inline(always)]
    pub const fn i2c_oar1(&self) -> &I2C_OAR1 {
        &self.i2c_oar1
    }
    #[doc = "0x0c - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    #[inline(always)]
    pub const fn i2c_oar2(&self) -> &I2C_OAR2 {
        &self.i2c_oar2
    }
    #[doc = "0x10 - Access: No wait states"]
    #[inline(always)]
    pub const fn i2c_timingr(&self) -> &I2C_TIMINGR {
        &self.i2c_timingr
    }
    #[doc = "0x14 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    #[inline(always)]
    pub const fn i2c_timeoutr(&self) -> &I2C_TIMEOUTR {
        &self.i2c_timeoutr
    }
    #[doc = "0x18 - Access: No wait states"]
    #[inline(always)]
    pub const fn i2c_isr(&self) -> &I2C_ISR {
        &self.i2c_isr
    }
    #[doc = "0x1c - Access: No wait states"]
    #[inline(always)]
    pub const fn i2c_icr(&self) -> &I2C_ICR {
        &self.i2c_icr
    }
    #[doc = "0x20 - Access: No wait states"]
    #[inline(always)]
    pub const fn i2c_pecr(&self) -> &I2C_PECR {
        &self.i2c_pecr
    }
    #[doc = "0x24 - Access: No wait states"]
    #[inline(always)]
    pub const fn i2c_rxdr(&self) -> &I2C_RXDR {
        &self.i2c_rxdr
    }
    #[doc = "0x28 - Access: No wait states"]
    #[inline(always)]
    pub const fn i2c_txdr(&self) -> &I2C_TXDR {
        &self.i2c_txdr
    }
    #[doc = "0x3f0 - I2C hardware configuration register"]
    #[inline(always)]
    pub const fn i2c_hwcfgr(&self) -> &I2C_HWCFGR {
        &self.i2c_hwcfgr
    }
    #[doc = "0x3f4 - I2C version register"]
    #[inline(always)]
    pub const fn i2c_verr(&self) -> &I2C_VERR {
        &self.i2c_verr
    }
    #[doc = "0x3f8 - I2C identification register"]
    #[inline(always)]
    pub const fn i2c_ipidr(&self) -> &I2C_IPIDR {
        &self.i2c_ipidr
    }
    #[doc = "0x3fc - I2C size identification register"]
    #[inline(always)]
    pub const fn i2c_sidr(&self) -> &I2C_SIDR {
        &self.i2c_sidr
    }
}
#[doc = "I2C_CR1 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr1`]
module"]
pub type I2C_CR1 = crate::Reg<i2c_cr1::I2C_CR1rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck."]
pub mod i2c_cr1;
#[doc = "I2C_CR2 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr2`]
module"]
pub type I2C_CR2 = crate::Reg<i2c_cr2::I2C_CR2rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_cr2;
#[doc = "I2C_OAR1 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar1`]
module"]
pub type I2C_OAR1 = crate::Reg<i2c_oar1::I2C_OAR1rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_oar1;
#[doc = "I2C_OAR2 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar2`]
module"]
pub type I2C_OAR2 = crate::Reg<i2c_oar2::I2C_OAR2rs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_oar2;
#[doc = "I2C_TIMINGR (rw) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timingr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timingr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timingr`]
module"]
pub type I2C_TIMINGR = crate::Reg<i2c_timingr::I2C_TIMINGRrs>;
#[doc = "Access: No wait states"]
pub mod i2c_timingr;
#[doc = "I2C_TIMEOUTR (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timeoutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timeoutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timeoutr`]
module"]
pub type I2C_TIMEOUTR = crate::Reg<i2c_timeoutr::I2C_TIMEOUTRrs>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_timeoutr;
#[doc = "I2C_ISR (rw) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_isr`]
module"]
pub type I2C_ISR = crate::Reg<i2c_isr::I2C_ISRrs>;
#[doc = "Access: No wait states"]
pub mod i2c_isr;
#[doc = "I2C_ICR (w) register accessor: Access: No wait states\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_icr`]
module"]
pub type I2C_ICR = crate::Reg<i2c_icr::I2C_ICRrs>;
#[doc = "Access: No wait states"]
pub mod i2c_icr;
#[doc = "I2C_PECR (r) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_pecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_pecr`]
module"]
pub type I2C_PECR = crate::Reg<i2c_pecr::I2C_PECRrs>;
#[doc = "Access: No wait states"]
pub mod i2c_pecr;
#[doc = "I2C_RXDR (r) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_rxdr`]
module"]
pub type I2C_RXDR = crate::Reg<i2c_rxdr::I2C_RXDRrs>;
#[doc = "Access: No wait states"]
pub mod i2c_rxdr;
#[doc = "I2C_TXDR (rw) register accessor: Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_txdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_txdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_txdr`]
module"]
pub type I2C_TXDR = crate::Reg<i2c_txdr::I2C_TXDRrs>;
#[doc = "Access: No wait states"]
pub mod i2c_txdr;
#[doc = "I2C_HWCFGR (r) register accessor: I2C hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_hwcfgr`]
module"]
pub type I2C_HWCFGR = crate::Reg<i2c_hwcfgr::I2C_HWCFGRrs>;
#[doc = "I2C hardware configuration register"]
pub mod i2c_hwcfgr;
#[doc = "I2C_VERR (r) register accessor: I2C version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_verr`]
module"]
pub type I2C_VERR = crate::Reg<i2c_verr::I2C_VERRrs>;
#[doc = "I2C version register"]
pub mod i2c_verr;
#[doc = "I2C_IPIDR (r) register accessor: I2C identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ipidr`]
module"]
pub type I2C_IPIDR = crate::Reg<i2c_ipidr::I2C_IPIDRrs>;
#[doc = "I2C identification register"]
pub mod i2c_ipidr;
#[doc = "I2C_SIDR (r) register accessor: I2C size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sidr`]
module"]
pub type I2C_SIDR = crate::Reg<i2c_sidr::I2C_SIDRrs>;
#[doc = "I2C size identification register"]
pub mod i2c_sidr;
