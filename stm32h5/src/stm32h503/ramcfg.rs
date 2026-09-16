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
    m2sear: M2SEAR,
    m2dear: M2DEAR,
    m2icr: M2ICR,
    m2wpr1: M2WPR1,
    _reserved10: [u8; 0x08],
    m2ecckeyr: M2ECCKEYR,
    m2erkeyr: M2ERKEYR,
    _reserved12: [u8; 0x18],
    m3ier: M3IER,
    m3isr: M3ISR,
    m3sear: M3SEAR,
    m3dear: M3DEAR,
    m3icr: M3ICR,
    _reserved17: [u8; 0x0c],
    m3ecckeyr: M3ECCKEYR,
    m3erkeyr: M3ERKEYR,
    _reserved19: [u8; 0x3c],
    m4erkeyr: M4ERKEYR,
    _reserved20: [u8; 0x14],
    m5cr: M5CR,
    m5ier: M5IER,
    m5isr: M5ISR,
    m5sear: M5SEAR,
    m5dear: M5DEAR,
    m5icr: M5ICR,
    _reserved26: [u8; 0x0c],
    m5ecckeyr: M5ECCKEYR,
    m5erkeyr: M5ERKEYR,
}
impl RegisterBlock {
    ///0x00 - RAMCFG memory 1 control register
    #[inline(always)]
    pub const fn m1cr(&self) -> &M1CR {
        &self.m1cr
    }
    ///0x08 - RAMCFG memory interrupt status register
    #[inline(always)]
    pub const fn m1isr(&self) -> &M1ISR {
        &self.m1isr
    }
    ///0x28 - RAMCFG memory 1 erase key register
    #[inline(always)]
    pub const fn m1erkeyr(&self) -> &M1ERKEYR {
        &self.m1erkeyr
    }
    ///0x40 - RAMCFG memory 2 control register
    #[inline(always)]
    pub const fn m2cr(&self) -> &M2CR {
        &self.m2cr
    }
    ///0x44 - RAMCFG memory 2 interrupt enable register
    #[inline(always)]
    pub const fn m2ier(&self) -> &M2IER {
        &self.m2ier
    }
    ///0x48 - RAMCFG memory interrupt status register
    #[inline(always)]
    pub const fn m2isr(&self) -> &M2ISR {
        &self.m2isr
    }
    ///0x4c - RAMCFG memory 2 ECC single error address register
    #[inline(always)]
    pub const fn m2sear(&self) -> &M2SEAR {
        &self.m2sear
    }
    ///0x50 - RAMCFG memory 2 ECC double error address register
    #[inline(always)]
    pub const fn m2dear(&self) -> &M2DEAR {
        &self.m2dear
    }
    ///0x54 - RAMCFG memory 2 interrupt clear register 2
    #[inline(always)]
    pub const fn m2icr(&self) -> &M2ICR {
        &self.m2icr
    }
    ///0x58 - RAMCFG memory 2 write protection register 1
    #[inline(always)]
    pub const fn m2wpr1(&self) -> &M2WPR1 {
        &self.m2wpr1
    }
    ///0x64 - RAMCFG memory 2 ECC key register
    #[inline(always)]
    pub const fn m2ecckeyr(&self) -> &M2ECCKEYR {
        &self.m2ecckeyr
    }
    ///0x68 - RAMCFG memory 2 erase key register
    #[inline(always)]
    pub const fn m2erkeyr(&self) -> &M2ERKEYR {
        &self.m2erkeyr
    }
    ///0x84 - RAMCFG memory 3 interrupt enable register
    #[inline(always)]
    pub const fn m3ier(&self) -> &M3IER {
        &self.m3ier
    }
    ///0x88 - RAMCFG memory interrupt status register
    #[inline(always)]
    pub const fn m3isr(&self) -> &M3ISR {
        &self.m3isr
    }
    ///0x8c - RAMCFG memory 3 ECC single error address register
    #[inline(always)]
    pub const fn m3sear(&self) -> &M3SEAR {
        &self.m3sear
    }
    ///0x90 - RAMCFG memory 3 ECC double error address register
    #[inline(always)]
    pub const fn m3dear(&self) -> &M3DEAR {
        &self.m3dear
    }
    ///0x94 - RAMCFG memory 3 interrupt clear register 3
    #[inline(always)]
    pub const fn m3icr(&self) -> &M3ICR {
        &self.m3icr
    }
    ///0xa4 - RAMCFG memory 3 ECC key register
    #[inline(always)]
    pub const fn m3ecckeyr(&self) -> &M3ECCKEYR {
        &self.m3ecckeyr
    }
    ///0xa8 - RAMCFG memory 3 erase key register
    #[inline(always)]
    pub const fn m3erkeyr(&self) -> &M3ERKEYR {
        &self.m3erkeyr
    }
    ///0xe8 - RAMCFG memory 4 erase key register
    #[inline(always)]
    pub const fn m4erkeyr(&self) -> &M4ERKEYR {
        &self.m4erkeyr
    }
    ///0x100 - RAMCFG memory 5 control register
    #[inline(always)]
    pub const fn m5cr(&self) -> &M5CR {
        &self.m5cr
    }
    ///0x104 - RAMCFG memory 5 interrupt enable register
    #[inline(always)]
    pub const fn m5ier(&self) -> &M5IER {
        &self.m5ier
    }
    ///0x108 - RAMCFG memory interrupt status register
    #[inline(always)]
    pub const fn m5isr(&self) -> &M5ISR {
        &self.m5isr
    }
    ///0x10c - RAMCFG memory 5 ECC single error address register
    #[inline(always)]
    pub const fn m5sear(&self) -> &M5SEAR {
        &self.m5sear
    }
    ///0x110 - RAMCFG memory 5 ECC double error address register
    #[inline(always)]
    pub const fn m5dear(&self) -> &M5DEAR {
        &self.m5dear
    }
    ///0x114 - RAMCFG memory 5 interrupt clear register 5
    #[inline(always)]
    pub const fn m5icr(&self) -> &M5ICR {
        &self.m5icr
    }
    ///0x124 - RAMCFG memory 5 ECC key register
    #[inline(always)]
    pub const fn m5ecckeyr(&self) -> &M5ECCKEYR {
        &self.m5ecckeyr
    }
    ///0x128 - RAMCFG memory 5 erase key register
    #[inline(always)]
    pub const fn m5erkeyr(&self) -> &M5ERKEYR {
        &self.m5erkeyr
    }
}
/**M1CR (rw) register accessor: RAMCFG memory 1 control register

You can [`read`](crate::Reg::read) this register and get [`m1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M1CR)

For information about available fields see [`mod@m1cr`] module*/
pub type M1CR = crate::Reg<m1cr::M1CRrs>;
///RAMCFG memory 1 control register
pub mod m1cr;
/**M1ISR (r) register accessor: RAMCFG memory interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M1ISR)

For information about available fields see [`mod@m1isr`] module*/
pub type M1ISR = crate::Reg<m1isr::M1ISRrs>;
///RAMCFG memory interrupt status register
pub mod m1isr;
/**M1ERKEYR (w) register accessor: RAMCFG memory 1 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M1ERKEYR)

For information about available fields see [`mod@m1erkeyr`] module*/
pub type M1ERKEYR = crate::Reg<m1erkeyr::M1ERKEYRrs>;
///RAMCFG memory 1 erase key register
pub mod m1erkeyr;
/**M2CR (rw) register accessor: RAMCFG memory 2 control register

You can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2CR)

For information about available fields see [`mod@m2cr`] module*/
pub type M2CR = crate::Reg<m2cr::M2CRrs>;
///RAMCFG memory 2 control register
pub mod m2cr;
/**M2IER (rw) register accessor: RAMCFG memory 2 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2IER)

For information about available fields see [`mod@m2ier`] module*/
pub type M2IER = crate::Reg<m2ier::M2IERrs>;
///RAMCFG memory 2 interrupt enable register
pub mod m2ier;
/**M2ISR (r) register accessor: RAMCFG memory interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2ISR)

For information about available fields see [`mod@m2isr`] module*/
pub type M2ISR = crate::Reg<m2isr::M2ISRrs>;
///RAMCFG memory interrupt status register
pub mod m2isr;
/**M2SEAR (r) register accessor: RAMCFG memory 2 ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m2sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2SEAR)

For information about available fields see [`mod@m2sear`] module*/
pub type M2SEAR = crate::Reg<m2sear::M2SEARrs>;
///RAMCFG memory 2 ECC single error address register
pub mod m2sear;
/**M2DEAR (r) register accessor: RAMCFG memory 2 ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m2dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2DEAR)

For information about available fields see [`mod@m2dear`] module*/
pub type M2DEAR = crate::Reg<m2dear::M2DEARrs>;
///RAMCFG memory 2 ECC double error address register
pub mod m2dear;
/**M2ICR (rw) register accessor: RAMCFG memory 2 interrupt clear register 2

You can [`read`](crate::Reg::read) this register and get [`m2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2ICR)

For information about available fields see [`mod@m2icr`] module*/
pub type M2ICR = crate::Reg<m2icr::M2ICRrs>;
///RAMCFG memory 2 interrupt clear register 2
pub mod m2icr;
/**M2WPR1 (rw) register accessor: RAMCFG memory 2 write protection register 1

You can [`read`](crate::Reg::read) this register and get [`m2wpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2WPR1)

For information about available fields see [`mod@m2wpr1`] module*/
pub type M2WPR1 = crate::Reg<m2wpr1::M2WPR1rs>;
///RAMCFG memory 2 write protection register 1
pub mod m2wpr1;
/**M2ECCKEYR (w) register accessor: RAMCFG memory 2 ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2ECCKEYR)

For information about available fields see [`mod@m2ecckeyr`] module*/
pub type M2ECCKEYR = crate::Reg<m2ecckeyr::M2ECCKEYRrs>;
///RAMCFG memory 2 ECC key register
pub mod m2ecckeyr;
/**M2ERKEYR (w) register accessor: RAMCFG memory 2 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M2ERKEYR)

For information about available fields see [`mod@m2erkeyr`] module*/
pub type M2ERKEYR = crate::Reg<m2erkeyr::M2ERKEYRrs>;
///RAMCFG memory 2 erase key register
pub mod m2erkeyr;
/**M3IER (rw) register accessor: RAMCFG memory 3 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m3ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3IER)

For information about available fields see [`mod@m3ier`] module*/
pub type M3IER = crate::Reg<m3ier::M3IERrs>;
///RAMCFG memory 3 interrupt enable register
pub mod m3ier;
/**M3ISR (r) register accessor: RAMCFG memory interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3ISR)

For information about available fields see [`mod@m3isr`] module*/
pub type M3ISR = crate::Reg<m3isr::M3ISRrs>;
///RAMCFG memory interrupt status register
pub mod m3isr;
/**M3SEAR (r) register accessor: RAMCFG memory 3 ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m3sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3SEAR)

For information about available fields see [`mod@m3sear`] module*/
pub type M3SEAR = crate::Reg<m3sear::M3SEARrs>;
///RAMCFG memory 3 ECC single error address register
pub mod m3sear;
/**M3DEAR (r) register accessor: RAMCFG memory 3 ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m3dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3DEAR)

For information about available fields see [`mod@m3dear`] module*/
pub type M3DEAR = crate::Reg<m3dear::M3DEARrs>;
///RAMCFG memory 3 ECC double error address register
pub mod m3dear;
/**M3ICR (rw) register accessor: RAMCFG memory 3 interrupt clear register 3

You can [`read`](crate::Reg::read) this register and get [`m3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3ICR)

For information about available fields see [`mod@m3icr`] module*/
pub type M3ICR = crate::Reg<m3icr::M3ICRrs>;
///RAMCFG memory 3 interrupt clear register 3
pub mod m3icr;
/**M3ECCKEYR (w) register accessor: RAMCFG memory 3 ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3ECCKEYR)

For information about available fields see [`mod@m3ecckeyr`] module*/
pub type M3ECCKEYR = crate::Reg<m3ecckeyr::M3ECCKEYRrs>;
///RAMCFG memory 3 ECC key register
pub mod m3ecckeyr;
/**M3ERKEYR (w) register accessor: RAMCFG memory 3 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3ERKEYR)

For information about available fields see [`mod@m3erkeyr`] module*/
pub type M3ERKEYR = crate::Reg<m3erkeyr::M3ERKEYRrs>;
///RAMCFG memory 3 erase key register
pub mod m3erkeyr;
/**M4ERKEYR (w) register accessor: RAMCFG memory 4 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M4ERKEYR)

For information about available fields see [`mod@m4erkeyr`] module*/
pub type M4ERKEYR = crate::Reg<m4erkeyr::M4ERKEYRrs>;
///RAMCFG memory 4 erase key register
pub mod m4erkeyr;
/**M5CR (rw) register accessor: RAMCFG memory 5 control register

You can [`read`](crate::Reg::read) this register and get [`m5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5CR)

For information about available fields see [`mod@m5cr`] module*/
pub type M5CR = crate::Reg<m5cr::M5CRrs>;
///RAMCFG memory 5 control register
pub mod m5cr;
/**M5IER (rw) register accessor: RAMCFG memory 5 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m5ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5IER)

For information about available fields see [`mod@m5ier`] module*/
pub type M5IER = crate::Reg<m5ier::M5IERrs>;
///RAMCFG memory 5 interrupt enable register
pub mod m5ier;
/**M5ISR (r) register accessor: RAMCFG memory interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5ISR)

For information about available fields see [`mod@m5isr`] module*/
pub type M5ISR = crate::Reg<m5isr::M5ISRrs>;
///RAMCFG memory interrupt status register
pub mod m5isr;
/**M5SEAR (r) register accessor: RAMCFG memory 5 ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m5sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5SEAR)

For information about available fields see [`mod@m5sear`] module*/
pub type M5SEAR = crate::Reg<m5sear::M5SEARrs>;
///RAMCFG memory 5 ECC single error address register
pub mod m5sear;
/**M5DEAR (r) register accessor: RAMCFG memory 5 ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m5dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5DEAR)

For information about available fields see [`mod@m5dear`] module*/
pub type M5DEAR = crate::Reg<m5dear::M5DEARrs>;
///RAMCFG memory 5 ECC double error address register
pub mod m5dear;
/**M5ICR (rw) register accessor: RAMCFG memory 5 interrupt clear register 5

You can [`read`](crate::Reg::read) this register and get [`m5icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5ICR)

For information about available fields see [`mod@m5icr`] module*/
pub type M5ICR = crate::Reg<m5icr::M5ICRrs>;
///RAMCFG memory 5 interrupt clear register 5
pub mod m5icr;
/**M5ECCKEYR (w) register accessor: RAMCFG memory 5 ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5ECCKEYR)

For information about available fields see [`mod@m5ecckeyr`] module*/
pub type M5ECCKEYR = crate::Reg<m5ecckeyr::M5ECCKEYRrs>;
///RAMCFG memory 5 ECC key register
pub mod m5ecckeyr;
/**M5ERKEYR (w) register accessor: RAMCFG memory 5 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M5ERKEYR)

For information about available fields see [`mod@m5erkeyr`] module*/
pub type M5ERKEYR = crate::Reg<m5erkeyr::M5ERKEYRrs>;
///RAMCFG memory 5 erase key register
pub mod m5erkeyr;
