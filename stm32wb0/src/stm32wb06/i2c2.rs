#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    oar1: OAR1,
    oar2: OAR2,
    timing: TIMING,
    timeout: TIMEOUT,
    isr: ISR,
    icr: ICR,
    pecr: PECR,
    rxdr: RXDR,
    txdr: TXDR,
}
impl RegisterBlock {
    ///0x00 - I2C2_CR1 register
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - I2C2_CR2 register
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - I2C2_OAR1 register
    #[inline(always)]
    pub const fn oar1(&self) -> &OAR1 {
        &self.oar1
    }
    ///0x0c - I2C2_OAR2 register
    #[inline(always)]
    pub const fn oar2(&self) -> &OAR2 {
        &self.oar2
    }
    ///0x10 - I2C2_TIMING register
    #[inline(always)]
    pub const fn timing(&self) -> &TIMING {
        &self.timing
    }
    ///0x14 - I2C2_TIMEOUT register
    #[inline(always)]
    pub const fn timeout(&self) -> &TIMEOUT {
        &self.timeout
    }
    ///0x18 - I2C2_ISR register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x1c - I2C2_ICR register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x20 - I2C2_PECR register
    #[inline(always)]
    pub const fn pecr(&self) -> &PECR {
        &self.pecr
    }
    ///0x24 - I2C2_RXDR register
    #[inline(always)]
    pub const fn rxdr(&self) -> &RXDR {
        &self.rxdr
    }
    ///0x28 - I2C2_TXDR register
    #[inline(always)]
    pub const fn txdr(&self) -> &TXDR {
        &self.txdr
    }
}
/**CR1 (rw) register accessor: I2C2_CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///I2C2_CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: I2C2_CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///I2C2_CR2 register
pub mod cr2;
/**OAR1 (rw) register accessor: I2C2_OAR1 register

You can [`read`](crate::Reg::read) this register and get [`oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:OAR1)

For information about available fields see [`mod@oar1`] module*/
pub type OAR1 = crate::Reg<oar1::OAR1rs>;
///I2C2_OAR1 register
pub mod oar1;
/**OAR2 (rw) register accessor: I2C2_OAR2 register

You can [`read`](crate::Reg::read) this register and get [`oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:OAR2)

For information about available fields see [`mod@oar2`] module*/
pub type OAR2 = crate::Reg<oar2::OAR2rs>;
///I2C2_OAR2 register
pub mod oar2;
/**TIMING (rw) register accessor: I2C2_TIMING register

You can [`read`](crate::Reg::read) this register and get [`timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:TIMING)

For information about available fields see [`mod@timing`] module*/
pub type TIMING = crate::Reg<timing::TIMINGrs>;
///I2C2_TIMING register
pub mod timing;
/**TIMEOUT (rw) register accessor: I2C2_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:TIMEOUT)

For information about available fields see [`mod@timeout`] module*/
pub type TIMEOUT = crate::Reg<timeout::TIMEOUTrs>;
///I2C2_TIMEOUT register
pub mod timeout;
/**ISR (rw) register accessor: I2C2_ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///I2C2_ISR register
pub mod isr;
/**ICR (rw) register accessor: I2C2_ICR register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///I2C2_ICR register
pub mod icr;
/**PECR (r) register accessor: I2C2_PECR register

You can [`read`](crate::Reg::read) this register and get [`pecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:PECR)

For information about available fields see [`mod@pecr`] module*/
pub type PECR = crate::Reg<pecr::PECRrs>;
///I2C2_PECR register
pub mod pecr;
/**RXDR (r) register accessor: I2C2_RXDR register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:RXDR)

For information about available fields see [`mod@rxdr`] module*/
pub type RXDR = crate::Reg<rxdr::RXDRrs>;
///I2C2_RXDR register
pub mod rxdr;
/**TXDR (rw) register accessor: I2C2_TXDR register

You can [`read`](crate::Reg::read) this register and get [`txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:TXDR)

For information about available fields see [`mod@txdr`] module*/
pub type TXDR = crate::Reg<txdr::TXDRrs>;
///I2C2_TXDR register
pub mod txdr;
