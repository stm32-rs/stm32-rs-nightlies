#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    m1cr: M1CR,
    _reserved1: [u8; 0x04],
    m1isr: M1ISR,
    _reserved2: [u8; 0x1c],
    m1erkeyr: M1ERKEYR,
    _reserved3: [u8; 0x14],
    m2cr: M2CR,
    m2ier: M2IER,
    m2isr: M2ISR,
    _reserved6: [u8; 0x04],
    m2pear: M2PEAR,
    m2icr: M2ICR,
    m2wpr1: M2WPR1,
    m2wpr2: M2WPR2,
    _reserved10: [u8; 0x08],
    m2erkeyr: M2ERKEYR,
}
impl RegisterBlock {
    ///0x00 - RAMCFG SRAM1 control register
    #[inline(always)]
    pub const fn m1cr(&self) -> &M1CR {
        &self.m1cr
    }
    ///0x08 - RAMCFG SRAM1 interrupt status register
    #[inline(always)]
    pub const fn m1isr(&self) -> &M1ISR {
        &self.m1isr
    }
    ///0x28 - RAMCFG SRAM1 erase key register
    #[inline(always)]
    pub const fn m1erkeyr(&self) -> &M1ERKEYR {
        &self.m1erkeyr
    }
    ///0x40 - RAMCFG SRAM2 control register
    #[inline(always)]
    pub const fn m2cr(&self) -> &M2CR {
        &self.m2cr
    }
    ///0x44 - RAMCFG SRAM2 interrupt enable register
    #[inline(always)]
    pub const fn m2ier(&self) -> &M2IER {
        &self.m2ier
    }
    ///0x48 - RAMCFG SRAM2 interrupt status register
    #[inline(always)]
    pub const fn m2isr(&self) -> &M2ISR {
        &self.m2isr
    }
    ///0x50 - RAMCFG SRAM2 parity error address register
    #[inline(always)]
    pub const fn m2pear(&self) -> &M2PEAR {
        &self.m2pear
    }
    ///0x54 - RAMCFG SRAM2 interrupt clear register
    #[inline(always)]
    pub const fn m2icr(&self) -> &M2ICR {
        &self.m2icr
    }
    ///0x58 - RAMCFG SRAM2 write protection register 1
    #[inline(always)]
    pub const fn m2wpr1(&self) -> &M2WPR1 {
        &self.m2wpr1
    }
    ///0x5c - RAMCFG SRAM2 write protection register 2
    #[inline(always)]
    pub const fn m2wpr2(&self) -> &M2WPR2 {
        &self.m2wpr2
    }
    ///0x68 - RAMCFG SRAM2 erase key register
    #[inline(always)]
    pub const fn m2erkeyr(&self) -> &M2ERKEYR {
        &self.m2erkeyr
    }
}
/**M1CR (rw) register accessor: RAMCFG SRAM1 control register

You can [`read`](crate::Reg::read) this register and get [`m1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M1CR)

For information about available fields see [`mod@m1cr`] module*/
pub type M1CR = crate::Reg<m1cr::M1CRrs>;
///RAMCFG SRAM1 control register
pub mod m1cr;
/**M1ISR (r) register accessor: RAMCFG SRAM1 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M1ISR)

For information about available fields see [`mod@m1isr`] module*/
pub type M1ISR = crate::Reg<m1isr::M1ISRrs>;
///RAMCFG SRAM1 interrupt status register
pub mod m1isr;
/**M1ERKEYR (w) register accessor: RAMCFG SRAM1 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M1ERKEYR)

For information about available fields see [`mod@m1erkeyr`] module*/
pub type M1ERKEYR = crate::Reg<m1erkeyr::M1ERKEYRrs>;
///RAMCFG SRAM1 erase key register
pub mod m1erkeyr;
/**M2CR (rw) register accessor: RAMCFG SRAM2 control register

You can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2CR)

For information about available fields see [`mod@m2cr`] module*/
pub type M2CR = crate::Reg<m2cr::M2CRrs>;
///RAMCFG SRAM2 control register
pub mod m2cr;
/**M2IER (rw) register accessor: RAMCFG SRAM2 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2IER)

For information about available fields see [`mod@m2ier`] module*/
pub type M2IER = crate::Reg<m2ier::M2IERrs>;
///RAMCFG SRAM2 interrupt enable register
pub mod m2ier;
/**M2ISR (r) register accessor: RAMCFG SRAM2 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2ISR)

For information about available fields see [`mod@m2isr`] module*/
pub type M2ISR = crate::Reg<m2isr::M2ISRrs>;
///RAMCFG SRAM2 interrupt status register
pub mod m2isr;
/**M2PEAR (r) register accessor: RAMCFG SRAM2 parity error address register

You can [`read`](crate::Reg::read) this register and get [`m2pear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2PEAR)

For information about available fields see [`mod@m2pear`] module*/
pub type M2PEAR = crate::Reg<m2pear::M2PEARrs>;
///RAMCFG SRAM2 parity error address register
pub mod m2pear;
/**M2ICR (rw) register accessor: RAMCFG SRAM2 interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`m2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2ICR)

For information about available fields see [`mod@m2icr`] module*/
pub type M2ICR = crate::Reg<m2icr::M2ICRrs>;
///RAMCFG SRAM2 interrupt clear register
pub mod m2icr;
/**M2WPR1 (rw) register accessor: RAMCFG SRAM2 write protection register 1

You can [`read`](crate::Reg::read) this register and get [`m2wpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2WPR1)

For information about available fields see [`mod@m2wpr1`] module*/
pub type M2WPR1 = crate::Reg<m2wpr1::M2WPR1rs>;
///RAMCFG SRAM2 write protection register 1
pub mod m2wpr1;
/**M2WPR2 (rw) register accessor: RAMCFG SRAM2 write protection register 2

You can [`read`](crate::Reg::read) this register and get [`m2wpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2WPR2)

For information about available fields see [`mod@m2wpr2`] module*/
pub type M2WPR2 = crate::Reg<m2wpr2::M2WPR2rs>;
///RAMCFG SRAM2 write protection register 2
pub mod m2wpr2;
/**M2ERKEYR (w) register accessor: RAMCFG SRAM2 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2ERKEYR)

For information about available fields see [`mod@m2erkeyr`] module*/
pub type M2ERKEYR = crate::Reg<m2erkeyr::M2ERKEYRrs>;
///RAMCFG SRAM2 erase key register
pub mod m2erkeyr;
