#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck.
    #[inline(always)]
    pub const fn i2c_cr1(&self) -> &I2C_CR1 {
        &self.i2c_cr1
    }
    ///0x04 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
    #[inline(always)]
    pub const fn i2c_cr2(&self) -> &I2C_CR2 {
        &self.i2c_cr2
    }
    ///0x08 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
    #[inline(always)]
    pub const fn i2c_oar1(&self) -> &I2C_OAR1 {
        &self.i2c_oar1
    }
    ///0x0c - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
    #[inline(always)]
    pub const fn i2c_oar2(&self) -> &I2C_OAR2 {
        &self.i2c_oar2
    }
    ///0x10 - Access: No wait states
    #[inline(always)]
    pub const fn i2c_timingr(&self) -> &I2C_TIMINGR {
        &self.i2c_timingr
    }
    ///0x14 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
    #[inline(always)]
    pub const fn i2c_timeoutr(&self) -> &I2C_TIMEOUTR {
        &self.i2c_timeoutr
    }
    ///0x18 - Access: No wait states
    #[inline(always)]
    pub const fn i2c_isr(&self) -> &I2C_ISR {
        &self.i2c_isr
    }
    ///0x1c - Access: No wait states
    #[inline(always)]
    pub const fn i2c_icr(&self) -> &I2C_ICR {
        &self.i2c_icr
    }
    ///0x20 - Access: No wait states
    #[inline(always)]
    pub const fn i2c_pecr(&self) -> &I2C_PECR {
        &self.i2c_pecr
    }
    ///0x24 - Access: No wait states
    #[inline(always)]
    pub const fn i2c_rxdr(&self) -> &I2C_RXDR {
        &self.i2c_rxdr
    }
    ///0x28 - Access: No wait states
    #[inline(always)]
    pub const fn i2c_txdr(&self) -> &I2C_TXDR {
        &self.i2c_txdr
    }
    ///0x3f0 - I2C hardware configuration register
    #[inline(always)]
    pub const fn i2c_hwcfgr(&self) -> &I2C_HWCFGR {
        &self.i2c_hwcfgr
    }
    ///0x3f4 - I2C version register
    #[inline(always)]
    pub const fn i2c_verr(&self) -> &I2C_VERR {
        &self.i2c_verr
    }
    ///0x3f8 - I2C identification register
    #[inline(always)]
    pub const fn i2c_ipidr(&self) -> &I2C_IPIDR {
        &self.i2c_ipidr
    }
    ///0x3fc - I2C size identification register
    #[inline(always)]
    pub const fn i2c_sidr(&self) -> &I2C_SIDR {
        &self.i2c_sidr
    }
}
/**I2C_CR1 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck.

You can [`read`](crate::Reg::read) this register and get [`i2c_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_CR1)

For information about available fields see [`mod@i2c_cr1`]
module*/
pub type I2C_CR1 = crate::Reg<i2c_cr1::I2C_CR1rs>;
///Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck.
pub mod i2c_cr1;
/**I2C_CR2 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.

You can [`read`](crate::Reg::read) this register and get [`i2c_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_CR2)

For information about available fields see [`mod@i2c_cr2`]
module*/
pub type I2C_CR2 = crate::Reg<i2c_cr2::I2C_CR2rs>;
///Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
pub mod i2c_cr2;
/**I2C_OAR1 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.

You can [`read`](crate::Reg::read) this register and get [`i2c_oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_OAR1)

For information about available fields see [`mod@i2c_oar1`]
module*/
pub type I2C_OAR1 = crate::Reg<i2c_oar1::I2C_OAR1rs>;
///Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
pub mod i2c_oar1;
/**I2C_OAR2 (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.

You can [`read`](crate::Reg::read) this register and get [`i2c_oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_OAR2)

For information about available fields see [`mod@i2c_oar2`]
module*/
pub type I2C_OAR2 = crate::Reg<i2c_oar2::I2C_OAR2rs>;
///Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
pub mod i2c_oar2;
/**I2C_TIMINGR (rw) register accessor: Access: No wait states

You can [`read`](crate::Reg::read) this register and get [`i2c_timingr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timingr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_TIMINGR)

For information about available fields see [`mod@i2c_timingr`]
module*/
pub type I2C_TIMINGR = crate::Reg<i2c_timingr::I2C_TIMINGRrs>;
///Access: No wait states
pub mod i2c_timingr;
/**I2C_TIMEOUTR (rw) register accessor: Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.

You can [`read`](crate::Reg::read) this register and get [`i2c_timeoutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timeoutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_TIMEOUTR)

For information about available fields see [`mod@i2c_timeoutr`]
module*/
pub type I2C_TIMEOUTR = crate::Reg<i2c_timeoutr::I2C_TIMEOUTRrs>;
///Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.
pub mod i2c_timeoutr;
/**I2C_ISR (rw) register accessor: Access: No wait states

You can [`read`](crate::Reg::read) this register and get [`i2c_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_ISR)

For information about available fields see [`mod@i2c_isr`]
module*/
pub type I2C_ISR = crate::Reg<i2c_isr::I2C_ISRrs>;
///Access: No wait states
pub mod i2c_isr;
/**I2C_ICR (w) register accessor: Access: No wait states

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_ICR)

For information about available fields see [`mod@i2c_icr`]
module*/
pub type I2C_ICR = crate::Reg<i2c_icr::I2C_ICRrs>;
///Access: No wait states
pub mod i2c_icr;
/**I2C_PECR (r) register accessor: Access: No wait states

You can [`read`](crate::Reg::read) this register and get [`i2c_pecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_PECR)

For information about available fields see [`mod@i2c_pecr`]
module*/
pub type I2C_PECR = crate::Reg<i2c_pecr::I2C_PECRrs>;
///Access: No wait states
pub mod i2c_pecr;
/**I2C_RXDR (r) register accessor: Access: No wait states

You can [`read`](crate::Reg::read) this register and get [`i2c_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_RXDR)

For information about available fields see [`mod@i2c_rxdr`]
module*/
pub type I2C_RXDR = crate::Reg<i2c_rxdr::I2C_RXDRrs>;
///Access: No wait states
pub mod i2c_rxdr;
/**I2C_TXDR (rw) register accessor: Access: No wait states

You can [`read`](crate::Reg::read) this register and get [`i2c_txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_TXDR)

For information about available fields see [`mod@i2c_txdr`]
module*/
pub type I2C_TXDR = crate::Reg<i2c_txdr::I2C_TXDRrs>;
///Access: No wait states
pub mod i2c_txdr;
/**I2C_HWCFGR (r) register accessor: I2C hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`i2c_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_HWCFGR)

For information about available fields see [`mod@i2c_hwcfgr`]
module*/
pub type I2C_HWCFGR = crate::Reg<i2c_hwcfgr::I2C_HWCFGRrs>;
///I2C hardware configuration register
pub mod i2c_hwcfgr;
/**I2C_VERR (r) register accessor: I2C version register

You can [`read`](crate::Reg::read) this register and get [`i2c_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_VERR)

For information about available fields see [`mod@i2c_verr`]
module*/
pub type I2C_VERR = crate::Reg<i2c_verr::I2C_VERRrs>;
///I2C version register
pub mod i2c_verr;
/**I2C_IPIDR (r) register accessor: I2C identification register

You can [`read`](crate::Reg::read) this register and get [`i2c_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_IPIDR)

For information about available fields see [`mod@i2c_ipidr`]
module*/
pub type I2C_IPIDR = crate::Reg<i2c_ipidr::I2C_IPIDRrs>;
///I2C identification register
pub mod i2c_ipidr;
/**I2C_SIDR (r) register accessor: I2C size identification register

You can [`read`](crate::Reg::read) this register and get [`i2c_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_SIDR)

For information about available fields see [`mod@i2c_sidr`]
module*/
pub type I2C_SIDR = crate::Reg<i2c_sidr::I2C_SIDRrs>;
///I2C size identification register
pub mod i2c_sidr;
