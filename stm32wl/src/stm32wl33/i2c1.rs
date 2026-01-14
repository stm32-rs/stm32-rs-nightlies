#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    i2c_cr1: I2C_CR1,
    i2c_cr2: I2C_CR2,
    i2c_oar1: I2C_OAR1,
    i2c_oar2: I2C_OAR2,
    i2c_timing: I2C_TIMING,
    i2c_timeout: I2C_TIMEOUT,
    i2c_isr: I2C_ISR,
    i2c_icr: I2C_ICR,
    i2c_pec: I2C_PEC,
    i2c_rxdr: I2C_RXDR,
    i2c_txdr: I2C_TXDR,
}
impl RegisterBlock {
    ///0x00 - I2C_CR1 register
    #[inline(always)]
    pub const fn i2c_cr1(&self) -> &I2C_CR1 {
        &self.i2c_cr1
    }
    ///0x04 - I2C_CR2 register
    #[inline(always)]
    pub const fn i2c_cr2(&self) -> &I2C_CR2 {
        &self.i2c_cr2
    }
    ///0x08 - I2C_OAR1 register
    #[inline(always)]
    pub const fn i2c_oar1(&self) -> &I2C_OAR1 {
        &self.i2c_oar1
    }
    ///0x0c - I2C_OAR2 register
    #[inline(always)]
    pub const fn i2c_oar2(&self) -> &I2C_OAR2 {
        &self.i2c_oar2
    }
    ///0x10 - I2C_TIMING register
    #[inline(always)]
    pub const fn i2c_timing(&self) -> &I2C_TIMING {
        &self.i2c_timing
    }
    ///0x14 - I2C_TIMEOUT register
    #[inline(always)]
    pub const fn i2c_timeout(&self) -> &I2C_TIMEOUT {
        &self.i2c_timeout
    }
    ///0x18 - I2C_ISR register
    #[inline(always)]
    pub const fn i2c_isr(&self) -> &I2C_ISR {
        &self.i2c_isr
    }
    ///0x1c - I2C_ICR register
    #[inline(always)]
    pub const fn i2c_icr(&self) -> &I2C_ICR {
        &self.i2c_icr
    }
    ///0x20 - I2C_PEC register
    #[inline(always)]
    pub const fn i2c_pec(&self) -> &I2C_PEC {
        &self.i2c_pec
    }
    ///0x24 - I2C_RXDR register
    #[inline(always)]
    pub const fn i2c_rxdr(&self) -> &I2C_RXDR {
        &self.i2c_rxdr
    }
    ///0x28 - I2C_TXDR register
    #[inline(always)]
    pub const fn i2c_txdr(&self) -> &I2C_TXDR {
        &self.i2c_txdr
    }
}
/**I2C_CR1 (rw) register accessor: I2C_CR1 register

You can [`read`](crate::Reg::read) this register and get [`i2c_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_CR1)

For information about available fields see [`mod@i2c_cr1`] module*/
pub type I2C_CR1 = crate::Reg<i2c_cr1::I2C_CR1rs>;
///I2C_CR1 register
pub mod i2c_cr1;
/**I2C_CR2 (rw) register accessor: I2C_CR2 register

You can [`read`](crate::Reg::read) this register and get [`i2c_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_CR2)

For information about available fields see [`mod@i2c_cr2`] module*/
pub type I2C_CR2 = crate::Reg<i2c_cr2::I2C_CR2rs>;
///I2C_CR2 register
pub mod i2c_cr2;
/**I2C_OAR1 (rw) register accessor: I2C_OAR1 register

You can [`read`](crate::Reg::read) this register and get [`i2c_oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_OAR1)

For information about available fields see [`mod@i2c_oar1`] module*/
pub type I2C_OAR1 = crate::Reg<i2c_oar1::I2C_OAR1rs>;
///I2C_OAR1 register
pub mod i2c_oar1;
/**I2C_OAR2 (rw) register accessor: I2C_OAR2 register

You can [`read`](crate::Reg::read) this register and get [`i2c_oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_OAR2)

For information about available fields see [`mod@i2c_oar2`] module*/
pub type I2C_OAR2 = crate::Reg<i2c_oar2::I2C_OAR2rs>;
///I2C_OAR2 register
pub mod i2c_oar2;
/**I2C_TIMING (rw) register accessor: I2C_TIMING register

You can [`read`](crate::Reg::read) this register and get [`i2c_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_TIMING)

For information about available fields see [`mod@i2c_timing`] module*/
pub type I2C_TIMING = crate::Reg<i2c_timing::I2C_TIMINGrs>;
///I2C_TIMING register
pub mod i2c_timing;
/**I2C_TIMEOUT (rw) register accessor: I2C_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`i2c_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_TIMEOUT)

For information about available fields see [`mod@i2c_timeout`] module*/
pub type I2C_TIMEOUT = crate::Reg<i2c_timeout::I2C_TIMEOUTrs>;
///I2C_TIMEOUT register
pub mod i2c_timeout;
/**I2C_ISR (rw) register accessor: I2C_ISR register

You can [`read`](crate::Reg::read) this register and get [`i2c_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_ISR)

For information about available fields see [`mod@i2c_isr`] module*/
pub type I2C_ISR = crate::Reg<i2c_isr::I2C_ISRrs>;
///I2C_ISR register
pub mod i2c_isr;
/**I2C_ICR (rw) register accessor: I2C_ICR register

You can [`read`](crate::Reg::read) this register and get [`i2c_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_ICR)

For information about available fields see [`mod@i2c_icr`] module*/
pub type I2C_ICR = crate::Reg<i2c_icr::I2C_ICRrs>;
///I2C_ICR register
pub mod i2c_icr;
/**I2C_PEC (r) register accessor: I2C_PEC register

You can [`read`](crate::Reg::read) this register and get [`i2c_pec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_PEC)

For information about available fields see [`mod@i2c_pec`] module*/
pub type I2C_PEC = crate::Reg<i2c_pec::I2C_PECrs>;
///I2C_PEC register
pub mod i2c_pec;
/**I2C_RXDR (r) register accessor: I2C_RXDR register

You can [`read`](crate::Reg::read) this register and get [`i2c_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_RXDR)

For information about available fields see [`mod@i2c_rxdr`] module*/
pub type I2C_RXDR = crate::Reg<i2c_rxdr::I2C_RXDRrs>;
///I2C_RXDR register
pub mod i2c_rxdr;
/**I2C_TXDR (rw) register accessor: I2C_TXDR register

You can [`read`](crate::Reg::read) this register and get [`i2c_txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C1:I2C_TXDR)

For information about available fields see [`mod@i2c_txdr`] module*/
pub type I2C_TXDR = crate::Reg<i2c_txdr::I2C_TXDRrs>;
///I2C_TXDR register
pub mod i2c_txdr;
