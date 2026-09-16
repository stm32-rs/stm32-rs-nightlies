#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    rpr1: RPR1,
    fpr1: FPR1,
    tzenr1: TZENR1,
    _reserved6: [u8; 0x08],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    rpr2: RPR2,
    fpr2: FPR2,
    tzenr2: TZENR2,
    _reserved12: [u8; 0x08],
    rtsr3: RTSR3,
    ftsr3: FTSR3,
    swier3: SWIER3,
    rpr3: RPR3,
    fpr3: FPR3,
    tzenr3: TZENR3,
    _reserved18: [u8; 0x08],
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    _reserved22: [u8; 0x10],
    imr1: IMR1,
    emr1: EMR1,
    _reserved24: [u8; 0x08],
    imr2: IMR2,
    emr2: EMR2,
    _reserved26: [u8; 0x08],
    imr3: IMR3,
    emr3: EMR3,
    _reserved28: [u8; 0x18],
    c2imr1: C2IMR1,
    c2emr1: C2EMR1,
    _reserved30: [u8; 0x08],
    c2imr2: C2IMR2,
    c2emr2: C2EMR2,
    _reserved32: [u8; 0x08],
    c2imr3: C2IMR3,
    c2emr3: C2EMR3,
    _reserved34: [u8; 0x02d8],
    hwcfgr13: HWCFGR13,
    hwcfgr12: HWCFGR12,
    hwcfgr11: HWCFGR11,
    hwcfgr10: HWCFGR10,
    hwcfgr9: HWCFGR9,
    hwcfgr8: HWCFGR8,
    hwcfgr7: HWCFGR7,
    hwcfgr6: HWCFGR6,
    hwcfgr5: HWCFGR5,
    hwcfgr4: HWCFGR4,
    hwcfgr3: HWCFGR3,
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    ///0x04 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    ///0x08 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    ///0x0c - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn rpr1(&self) -> &RPR1 {
        &self.rpr1
    }
    ///0x10 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn fpr1(&self) -> &FPR1 {
        &self.fpr1
    }
    ///0x14 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.
    #[inline(always)]
    pub const fn tzenr1(&self) -> &TZENR1 {
        &self.tzenr1
    }
    ///0x20 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    ///0x24 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    ///0x28 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    ///0x2c - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn rpr2(&self) -> &RPR2 {
        &self.rpr2
    }
    ///0x30 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn fpr2(&self) -> &FPR2 {
        &self.fpr2
    }
    ///0x34 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.
    #[inline(always)]
    pub const fn tzenr2(&self) -> &TZENR2 {
        &self.tzenr2
    }
    ///0x40 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn rtsr3(&self) -> &RTSR3 {
        &self.rtsr3
    }
    ///0x44 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn ftsr3(&self) -> &FTSR3 {
        &self.ftsr3
    }
    ///0x48 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn swier3(&self) -> &SWIER3 {
        &self.swier3
    }
    ///0x4c - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn rpr3(&self) -> &RPR3 {
        &self.rpr3
    }
    ///0x50 - Contains only register bits for configurable events.
    #[inline(always)]
    pub const fn fpr3(&self) -> &FPR3 {
        &self.fpr3
    }
    ///0x54 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.
    #[inline(always)]
    pub const fn tzenr3(&self) -> &TZENR3 {
        &self.tzenr3
    }
    ///0x60 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    ///0x64 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    ///0x68 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    ///0x6c - EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
    ///0x80 - Contains register bits for configurable events and Direct events.
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    ///0x84 - EXTI CPU wakeup with event mask register
    #[inline(always)]
    pub const fn emr1(&self) -> &EMR1 {
        &self.emr1
    }
    ///0x90 - Contains register bits for configurable events and direct events.
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    ///0x94 - EXTI CPU wakeup with event mask register
    #[inline(always)]
    pub const fn emr2(&self) -> &EMR2 {
        &self.emr2
    }
    ///0xa0 - Contains register bits for configurable events and direct events.
    #[inline(always)]
    pub const fn imr3(&self) -> &IMR3 {
        &self.imr3
    }
    ///0xa4 - EXTI CPU wakeup with event mask register
    #[inline(always)]
    pub const fn emr3(&self) -> &EMR3 {
        &self.emr3
    }
    ///0xc0 - Contains register bits for configurable events and Direct events.
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    ///0xc4 - EXTI CPU2 wakeup with event mask register
    #[inline(always)]
    pub const fn c2emr1(&self) -> &C2EMR1 {
        &self.c2emr1
    }
    ///0xd0 - Contains register bits for configurable events and direct events.
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    ///0xd4 - EXTI CPU2 wakeup with event mask register
    #[inline(always)]
    pub const fn c2emr2(&self) -> &C2EMR2 {
        &self.c2emr2
    }
    ///0xe0 - Contains register bits for configurable events and direct events.
    #[inline(always)]
    pub const fn c2imr3(&self) -> &C2IMR3 {
        &self.c2imr3
    }
    ///0xe4 - EXTI CPU2 wakeup with event mask register
    #[inline(always)]
    pub const fn c2emr3(&self) -> &C2EMR3 {
        &self.c2emr3
    }
    ///0x3c0 - EXTI hardware configuration register 13
    #[inline(always)]
    pub const fn hwcfgr13(&self) -> &HWCFGR13 {
        &self.hwcfgr13
    }
    ///0x3c4 - EXTI hardware configuration register 12
    #[inline(always)]
    pub const fn hwcfgr12(&self) -> &HWCFGR12 {
        &self.hwcfgr12
    }
    ///0x3c8 - EXTI hardware configuration register 11
    #[inline(always)]
    pub const fn hwcfgr11(&self) -> &HWCFGR11 {
        &self.hwcfgr11
    }
    ///0x3cc - EXTI hardware configuration register 10
    #[inline(always)]
    pub const fn hwcfgr10(&self) -> &HWCFGR10 {
        &self.hwcfgr10
    }
    ///0x3d0 - EXTI hardware configuration register 9
    #[inline(always)]
    pub const fn hwcfgr9(&self) -> &HWCFGR9 {
        &self.hwcfgr9
    }
    ///0x3d4 - EXTI hardware configuration register 8
    #[inline(always)]
    pub const fn hwcfgr8(&self) -> &HWCFGR8 {
        &self.hwcfgr8
    }
    ///0x3d8 - EXTI hardware configuration register 7
    #[inline(always)]
    pub const fn hwcfgr7(&self) -> &HWCFGR7 {
        &self.hwcfgr7
    }
    ///0x3dc - EXTI hardware configuration register 6
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    ///0x3e0 - EXTI hardware configuration register 5
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    ///0x3e4 - EXTI hardware configuration register 4
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    ///0x3e8 - EXTI hardware configuration register 3
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    ///0x3ec - EXTI hardware configuration register 2
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3f0 - EXTI hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f4 - EXTI IP version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - EXTI identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - EXTI size ID register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**RTSR1 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:RTSR1)

For information about available fields see [`mod@rtsr1`] module*/
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
///Contains only register bits for configurable events.
pub mod rtsr1;
/**FTSR1 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:FTSR1)

For information about available fields see [`mod@ftsr1`] module*/
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
///Contains only register bits for configurable events.
pub mod ftsr1;
/**SWIER1 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:SWIER1)

For information about available fields see [`mod@swier1`] module*/
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
///Contains only register bits for configurable events.
pub mod swier1;
/**RPR1 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:RPR1)

For information about available fields see [`mod@rpr1`] module*/
pub type RPR1 = crate::Reg<rpr1::RPR1rs>;
///Contains only register bits for configurable events.
pub mod rpr1;
/**FPR1 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`fpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:FPR1)

For information about available fields see [`mod@fpr1`] module*/
pub type FPR1 = crate::Reg<fpr1::FPR1rs>;
///Contains only register bits for configurable events.
pub mod fpr1;
/**TZENR1 (rw) register accessor: This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.

You can [`read`](crate::Reg::read) this register and get [`tzenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:TZENR1)

For information about available fields see [`mod@tzenr1`] module*/
pub type TZENR1 = crate::Reg<tzenr1::TZENR1rs>;
///This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.
pub mod tzenr1;
/**RTSR2 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:RTSR2)

For information about available fields see [`mod@rtsr2`] module*/
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
///Contains only register bits for configurable events.
pub mod rtsr2;
/**FTSR2 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:FTSR2)

For information about available fields see [`mod@ftsr2`] module*/
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
///Contains only register bits for configurable events.
pub mod ftsr2;
/**SWIER2 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`swier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:SWIER2)

For information about available fields see [`mod@swier2`] module*/
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
///Contains only register bits for configurable events.
pub mod swier2;
/**RPR2 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:RPR2)

For information about available fields see [`mod@rpr2`] module*/
pub type RPR2 = crate::Reg<rpr2::RPR2rs>;
///Contains only register bits for configurable events.
pub mod rpr2;
/**FPR2 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`fpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:FPR2)

For information about available fields see [`mod@fpr2`] module*/
pub type FPR2 = crate::Reg<fpr2::FPR2rs>;
///Contains only register bits for configurable events.
pub mod fpr2;
/**TZENR2 (rw) register accessor: This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.

You can [`read`](crate::Reg::read) this register and get [`tzenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:TZENR2)

For information about available fields see [`mod@tzenr2`] module*/
pub type TZENR2 = crate::Reg<tzenr2::TZENR2rs>;
///This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.
pub mod tzenr2;
/**RTSR3 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rtsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:RTSR3)

For information about available fields see [`mod@rtsr3`] module*/
pub type RTSR3 = crate::Reg<rtsr3::RTSR3rs>;
///Contains only register bits for configurable events.
pub mod rtsr3;
/**FTSR3 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`ftsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:FTSR3)

For information about available fields see [`mod@ftsr3`] module*/
pub type FTSR3 = crate::Reg<ftsr3::FTSR3rs>;
///Contains only register bits for configurable events.
pub mod ftsr3;
/**SWIER3 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`swier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:SWIER3)

For information about available fields see [`mod@swier3`] module*/
pub type SWIER3 = crate::Reg<swier3::SWIER3rs>;
///Contains only register bits for configurable events.
pub mod swier3;
/**RPR3 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rpr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:RPR3)

For information about available fields see [`mod@rpr3`] module*/
pub type RPR3 = crate::Reg<rpr3::RPR3rs>;
///Contains only register bits for configurable events.
pub mod rpr3;
/**FPR3 (rw) register accessor: Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`fpr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:FPR3)

For information about available fields see [`mod@fpr3`] module*/
pub type FPR3 = crate::Reg<fpr3::FPR3rs>;
///Contains only register bits for configurable events.
pub mod fpr3;
/**TZENR3 (rw) register accessor: This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.

You can [`read`](crate::Reg::read) this register and get [`tzenr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzenr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:TZENR3)

For information about available fields see [`mod@tzenr3`] module*/
pub type TZENR3 = crate::Reg<tzenr3::TZENR3rs>;
///This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.
pub mod tzenr3;
/**EXTICR1 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EXTICR1)

For information about available fields see [`mod@exticr1`] module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
pub mod exticr1;
/**EXTICR2 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EXTICR2)

For information about available fields see [`mod@exticr2`] module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
pub mod exticr2;
/**EXTICR3 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EXTICR3)

For information about available fields see [`mod@exticr3`] module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
pub mod exticr3;
/**EXTICR4 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EXTICR4)

For information about available fields see [`mod@exticr4`] module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
pub mod exticr4;
/**IMR1 (rw) register accessor: Contains register bits for configurable events and Direct events.

You can [`read`](crate::Reg::read) this register and get [`imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:IMR1)

For information about available fields see [`mod@imr1`] module*/
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
///Contains register bits for configurable events and Direct events.
pub mod imr1;
/**EMR1 (rw) register accessor: EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EMR1)

For information about available fields see [`mod@emr1`] module*/
pub type EMR1 = crate::Reg<emr1::EMR1rs>;
///EXTI CPU wakeup with event mask register
pub mod emr1;
/**IMR2 (rw) register accessor: Contains register bits for configurable events and direct events.

You can [`read`](crate::Reg::read) this register and get [`imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:IMR2)

For information about available fields see [`mod@imr2`] module*/
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
///Contains register bits for configurable events and direct events.
pub mod imr2;
/**EMR2 (rw) register accessor: EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EMR2)

For information about available fields see [`mod@emr2`] module*/
pub type EMR2 = crate::Reg<emr2::EMR2rs>;
///EXTI CPU wakeup with event mask register
pub mod emr2;
/**IMR3 (rw) register accessor: Contains register bits for configurable events and direct events.

You can [`read`](crate::Reg::read) this register and get [`imr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:IMR3)

For information about available fields see [`mod@imr3`] module*/
pub type IMR3 = crate::Reg<imr3::IMR3rs>;
///Contains register bits for configurable events and direct events.
pub mod imr3;
/**EMR3 (rw) register accessor: EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EMR3)

For information about available fields see [`mod@emr3`] module*/
pub type EMR3 = crate::Reg<emr3::EMR3rs>;
///EXTI CPU wakeup with event mask register
pub mod emr3;
/**C2IMR1 (rw) register accessor: Contains register bits for configurable events and Direct events.

You can [`read`](crate::Reg::read) this register and get [`c2imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:C2IMR1)

For information about available fields see [`mod@c2imr1`] module*/
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
///Contains register bits for configurable events and Direct events.
pub mod c2imr1;
/**C2EMR1 (rw) register accessor: EXTI CPU2 wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:C2EMR1)

For information about available fields see [`mod@c2emr1`] module*/
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1rs>;
///EXTI CPU2 wakeup with event mask register
pub mod c2emr1;
/**C2IMR2 (rw) register accessor: Contains register bits for configurable events and direct events.

You can [`read`](crate::Reg::read) this register and get [`c2imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:C2IMR2)

For information about available fields see [`mod@c2imr2`] module*/
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
///Contains register bits for configurable events and direct events.
pub mod c2imr2;
/**C2EMR2 (rw) register accessor: EXTI CPU2 wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:C2EMR2)

For information about available fields see [`mod@c2emr2`] module*/
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2rs>;
///EXTI CPU2 wakeup with event mask register
pub mod c2emr2;
/**C2IMR3 (rw) register accessor: Contains register bits for configurable events and direct events.

You can [`read`](crate::Reg::read) this register and get [`c2imr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:C2IMR3)

For information about available fields see [`mod@c2imr3`] module*/
pub type C2IMR3 = crate::Reg<c2imr3::C2IMR3rs>;
///Contains register bits for configurable events and direct events.
pub mod c2imr3;
/**C2EMR3 (rw) register accessor: EXTI CPU2 wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:C2EMR3)

For information about available fields see [`mod@c2emr3`] module*/
pub type C2EMR3 = crate::Reg<c2emr3::C2EMR3rs>;
///EXTI CPU2 wakeup with event mask register
pub mod c2emr3;
/**HWCFGR13 (r) register accessor: EXTI hardware configuration register 13

You can [`read`](crate::Reg::read) this register and get [`hwcfgr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR13)

For information about available fields see [`mod@hwcfgr13`] module*/
pub type HWCFGR13 = crate::Reg<hwcfgr13::HWCFGR13rs>;
///EXTI hardware configuration register 13
pub mod hwcfgr13;
/**HWCFGR12 (r) register accessor: EXTI hardware configuration register 12

You can [`read`](crate::Reg::read) this register and get [`hwcfgr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR12)

For information about available fields see [`mod@hwcfgr12`] module*/
pub type HWCFGR12 = crate::Reg<hwcfgr12::HWCFGR12rs>;
///EXTI hardware configuration register 12
pub mod hwcfgr12;
/**HWCFGR11 (r) register accessor: EXTI hardware configuration register 11

You can [`read`](crate::Reg::read) this register and get [`hwcfgr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR11)

For information about available fields see [`mod@hwcfgr11`] module*/
pub type HWCFGR11 = crate::Reg<hwcfgr11::HWCFGR11rs>;
///EXTI hardware configuration register 11
pub mod hwcfgr11;
/**HWCFGR10 (r) register accessor: EXTI hardware configuration register 10

You can [`read`](crate::Reg::read) this register and get [`hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR10)

For information about available fields see [`mod@hwcfgr10`] module*/
pub type HWCFGR10 = crate::Reg<hwcfgr10::HWCFGR10rs>;
///EXTI hardware configuration register 10
pub mod hwcfgr10;
/**HWCFGR9 (r) register accessor: EXTI hardware configuration register 9

You can [`read`](crate::Reg::read) this register and get [`hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR9)

For information about available fields see [`mod@hwcfgr9`] module*/
pub type HWCFGR9 = crate::Reg<hwcfgr9::HWCFGR9rs>;
///EXTI hardware configuration register 9
pub mod hwcfgr9;
/**HWCFGR8 (r) register accessor: EXTI hardware configuration register 8

You can [`read`](crate::Reg::read) this register and get [`hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR8)

For information about available fields see [`mod@hwcfgr8`] module*/
pub type HWCFGR8 = crate::Reg<hwcfgr8::HWCFGR8rs>;
///EXTI hardware configuration register 8
pub mod hwcfgr8;
/**HWCFGR7 (r) register accessor: EXTI hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR7)

For information about available fields see [`mod@hwcfgr7`] module*/
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7rs>;
///EXTI hardware configuration register 7
pub mod hwcfgr7;
/**HWCFGR6 (r) register accessor: EXTI hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR6)

For information about available fields see [`mod@hwcfgr6`] module*/
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
///EXTI hardware configuration register 6
pub mod hwcfgr6;
/**HWCFGR5 (r) register accessor: EXTI hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR5)

For information about available fields see [`mod@hwcfgr5`] module*/
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
///EXTI hardware configuration register 5
pub mod hwcfgr5;
/**HWCFGR4 (r) register accessor: EXTI hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR4)

For information about available fields see [`mod@hwcfgr4`] module*/
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
///EXTI hardware configuration register 4
pub mod hwcfgr4;
/**HWCFGR3 (r) register accessor: EXTI hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR3)

For information about available fields see [`mod@hwcfgr3`] module*/
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
///EXTI hardware configuration register 3
pub mod hwcfgr3;
/**HWCFGR2 (r) register accessor: EXTI hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///EXTI hardware configuration register 2
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: EXTI hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///EXTI hardware configuration register 1
pub mod hwcfgr1;
/**VERR (r) register accessor: EXTI IP version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///EXTI IP version register
pub mod verr;
/**IPIDR (r) register accessor: EXTI identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///EXTI identification register
pub mod ipidr;
/**SIDR (r) register accessor: EXTI size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///EXTI size ID register
pub mod sidr;
