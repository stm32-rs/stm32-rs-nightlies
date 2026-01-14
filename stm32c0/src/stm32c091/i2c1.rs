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
}
impl RegisterBlock {
    ///0x00 - I2C control register 1
    #[inline(always)]
    pub const fn i2c_cr1(&self) -> &I2C_CR1 {
        &self.i2c_cr1
    }
    ///0x04 - I2C control register 2
    #[inline(always)]
    pub const fn i2c_cr2(&self) -> &I2C_CR2 {
        &self.i2c_cr2
    }
    ///0x08 - I2C own address 1 register
    #[inline(always)]
    pub const fn i2c_oar1(&self) -> &I2C_OAR1 {
        &self.i2c_oar1
    }
    ///0x0c - I2C own address 2 register
    #[inline(always)]
    pub const fn i2c_oar2(&self) -> &I2C_OAR2 {
        &self.i2c_oar2
    }
    ///0x10 - I2C timing register
    #[inline(always)]
    pub const fn i2c_timingr(&self) -> &I2C_TIMINGR {
        &self.i2c_timingr
    }
    ///0x14 - I2C timeout register
    #[inline(always)]
    pub const fn i2c_timeoutr(&self) -> &I2C_TIMEOUTR {
        &self.i2c_timeoutr
    }
    ///0x18 - I2C interrupt and status register
    #[inline(always)]
    pub const fn i2c_isr(&self) -> &I2C_ISR {
        &self.i2c_isr
    }
    ///0x1c - I2C interrupt clear register
    #[inline(always)]
    pub const fn i2c_icr(&self) -> &I2C_ICR {
        &self.i2c_icr
    }
    ///0x20 - I2C PEC register
    #[inline(always)]
    pub const fn i2c_pecr(&self) -> &I2C_PECR {
        &self.i2c_pecr
    }
    ///0x24 - I2C receive data register
    #[inline(always)]
    pub const fn i2c_rxdr(&self) -> &I2C_RXDR {
        &self.i2c_rxdr
    }
    ///0x28 - I2C transmit data register
    #[inline(always)]
    pub const fn i2c_txdr(&self) -> &I2C_TXDR {
        &self.i2c_txdr
    }
}
/**I2C_CR1 (rw) register accessor: I2C control register 1

You can [`read`](crate::Reg::read) this register and get [`i2c_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_CR1)

For information about available fields see [`mod@i2c_cr1`] module*/
pub type I2C_CR1 = crate::Reg<i2c_cr1::I2C_CR1rs>;
///I2C control register 1
pub mod i2c_cr1;
/**I2C_CR2 (rw) register accessor: I2C control register 2

You can [`read`](crate::Reg::read) this register and get [`i2c_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_CR2)

For information about available fields see [`mod@i2c_cr2`] module*/
pub type I2C_CR2 = crate::Reg<i2c_cr2::I2C_CR2rs>;
///I2C control register 2
pub mod i2c_cr2;
/**I2C_OAR1 (rw) register accessor: I2C own address 1 register

You can [`read`](crate::Reg::read) this register and get [`i2c_oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_OAR1)

For information about available fields see [`mod@i2c_oar1`] module*/
pub type I2C_OAR1 = crate::Reg<i2c_oar1::I2C_OAR1rs>;
///I2C own address 1 register
pub mod i2c_oar1;
/**I2C_OAR2 (rw) register accessor: I2C own address 2 register

You can [`read`](crate::Reg::read) this register and get [`i2c_oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_OAR2)

For information about available fields see [`mod@i2c_oar2`] module*/
pub type I2C_OAR2 = crate::Reg<i2c_oar2::I2C_OAR2rs>;
///I2C own address 2 register
pub mod i2c_oar2;
/**I2C_TIMINGR (rw) register accessor: I2C timing register

You can [`read`](crate::Reg::read) this register and get [`i2c_timingr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timingr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_TIMINGR)

For information about available fields see [`mod@i2c_timingr`] module*/
pub type I2C_TIMINGR = crate::Reg<i2c_timingr::I2C_TIMINGRrs>;
///I2C timing register
pub mod i2c_timingr;
/**I2C_TIMEOUTR (rw) register accessor: I2C timeout register

You can [`read`](crate::Reg::read) this register and get [`i2c_timeoutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timeoutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_TIMEOUTR)

For information about available fields see [`mod@i2c_timeoutr`] module*/
pub type I2C_TIMEOUTR = crate::Reg<i2c_timeoutr::I2C_TIMEOUTRrs>;
///I2C timeout register
pub mod i2c_timeoutr;
/**I2C_ISR (rw) register accessor: I2C interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`i2c_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_ISR)

For information about available fields see [`mod@i2c_isr`] module*/
pub type I2C_ISR = crate::Reg<i2c_isr::I2C_ISRrs>;
///I2C interrupt and status register
pub mod i2c_isr;
/**I2C_ICR (w) register accessor: I2C interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_ICR)

For information about available fields see [`mod@i2c_icr`] module*/
pub type I2C_ICR = crate::Reg<i2c_icr::I2C_ICRrs>;
///I2C interrupt clear register
pub mod i2c_icr;
/**I2C_PECR (r) register accessor: I2C PEC register

You can [`read`](crate::Reg::read) this register and get [`i2c_pecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_PECR)

For information about available fields see [`mod@i2c_pecr`] module*/
pub type I2C_PECR = crate::Reg<i2c_pecr::I2C_PECRrs>;
///I2C PEC register
pub mod i2c_pecr;
/**I2C_RXDR (r) register accessor: I2C receive data register

You can [`read`](crate::Reg::read) this register and get [`i2c_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_RXDR)

For information about available fields see [`mod@i2c_rxdr`] module*/
pub type I2C_RXDR = crate::Reg<i2c_rxdr::I2C_RXDRrs>;
///I2C receive data register
pub mod i2c_rxdr;
/**I2C_TXDR (rw) register accessor: I2C transmit data register

You can [`read`](crate::Reg::read) this register and get [`i2c_txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_TXDR)

For information about available fields see [`mod@i2c_txdr`] module*/
pub type I2C_TXDR = crate::Reg<i2c_txdr::I2C_TXDRrs>;
///I2C transmit data register
pub mod i2c_txdr;
