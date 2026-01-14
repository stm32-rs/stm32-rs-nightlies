#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    pmcr: PMCR,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    cfgr: CFGR,
    _reserved6: [u8; 0x04],
    cccsr: CCCSR,
    ccvr: CCVR,
    cccr: CCCR,
    pwrcr: PWRCR,
    _reserved10: [u8; 0xf4],
    pkgr: PKGR,
    _reserved11: [u8; 0x01d8],
    ur0: UR0,
    ur1: UR1,
    ur2: UR2,
    ur3: UR3,
    ur4: UR4,
    ur5: UR5,
    ur6: UR6,
    ur7: UR7,
    ur8: UR8,
    ur9: UR9,
    ur10: UR10,
    ur11: UR11,
    ur12: UR12,
    ur13: UR13,
    ur14: UR14,
    ur15: UR15,
    ur16: UR16,
    ur17: UR17,
}
impl RegisterBlock {
    ///0x04 - peripheral mode configuration register
    #[inline(always)]
    pub const fn pmcr(&self) -> &PMCR {
        &self.pmcr
    }
    ///0x08 - external interrupt configuration register 1
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    ///0x0c - external interrupt configuration register 2
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    ///0x10 - external interrupt configuration register 3
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    ///0x14 - external interrupt configuration register 4
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
    ///0x18 - configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x20 - compensation cell control/status register
    #[inline(always)]
    pub const fn cccsr(&self) -> &CCCSR {
        &self.cccsr
    }
    ///0x24 - SYSCFG compensation cell value register
    #[inline(always)]
    pub const fn ccvr(&self) -> &CCVR {
        &self.ccvr
    }
    ///0x28 - SYSCFG compensation cell code register
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    ///0x2c - SYSCFG power control register
    #[inline(always)]
    pub const fn pwrcr(&self) -> &PWRCR {
        &self.pwrcr
    }
    ///0x124 - SYSCFG package register
    #[inline(always)]
    pub const fn pkgr(&self) -> &PKGR {
        &self.pkgr
    }
    ///0x300 - SYSCFG user register 0
    #[inline(always)]
    pub const fn ur0(&self) -> &UR0 {
        &self.ur0
    }
    ///0x304 - SYSCFG user register 1
    #[inline(always)]
    pub const fn ur1(&self) -> &UR1 {
        &self.ur1
    }
    ///0x308 - SYSCFG user register 2
    #[inline(always)]
    pub const fn ur2(&self) -> &UR2 {
        &self.ur2
    }
    ///0x30c - SYSCFG user register 3
    #[inline(always)]
    pub const fn ur3(&self) -> &UR3 {
        &self.ur3
    }
    ///0x310 - SYSCFG user register 4
    #[inline(always)]
    pub const fn ur4(&self) -> &UR4 {
        &self.ur4
    }
    ///0x314 - SYSCFG user register 5
    #[inline(always)]
    pub const fn ur5(&self) -> &UR5 {
        &self.ur5
    }
    ///0x318 - SYSCFG user register 6
    #[inline(always)]
    pub const fn ur6(&self) -> &UR6 {
        &self.ur6
    }
    ///0x31c - SYSCFG user register 7
    #[inline(always)]
    pub const fn ur7(&self) -> &UR7 {
        &self.ur7
    }
    ///0x320 - SYSCFG user register 8
    #[inline(always)]
    pub const fn ur8(&self) -> &UR8 {
        &self.ur8
    }
    ///0x324 - SYSCFG user register 9
    #[inline(always)]
    pub const fn ur9(&self) -> &UR9 {
        &self.ur9
    }
    ///0x328 - SYSCFG user register 10
    #[inline(always)]
    pub const fn ur10(&self) -> &UR10 {
        &self.ur10
    }
    ///0x32c - SYSCFG user register 11
    #[inline(always)]
    pub const fn ur11(&self) -> &UR11 {
        &self.ur11
    }
    ///0x330 - SYSCFG user register 12
    #[inline(always)]
    pub const fn ur12(&self) -> &UR12 {
        &self.ur12
    }
    ///0x334 - SYSCFG user register 13
    #[inline(always)]
    pub const fn ur13(&self) -> &UR13 {
        &self.ur13
    }
    ///0x338 - SYSCFG user register 14
    #[inline(always)]
    pub const fn ur14(&self) -> &UR14 {
        &self.ur14
    }
    ///0x33c - SYSCFG user register 15
    #[inline(always)]
    pub const fn ur15(&self) -> &UR15 {
        &self.ur15
    }
    ///0x340 - SYSCFG user register 16
    #[inline(always)]
    pub const fn ur16(&self) -> &UR16 {
        &self.ur16
    }
    ///0x344 - SYSCFG user register 17
    #[inline(always)]
    pub const fn ur17(&self) -> &UR17 {
        &self.ur17
    }
}
/**PMCR (rw) register accessor: peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:PMCR)

For information about available fields see [`mod@pmcr`] module*/
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
///peripheral mode configuration register
pub mod pmcr;
/**EXTICR1 (rw) register accessor: external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:EXTICR1)

For information about available fields see [`mod@exticr1`] module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///external interrupt configuration register 1
pub mod exticr1;
/**EXTICR2 (rw) register accessor: external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:EXTICR2)

For information about available fields see [`mod@exticr2`] module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///external interrupt configuration register 2
pub mod exticr2;
/**EXTICR3 (rw) register accessor: external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:EXTICR3)

For information about available fields see [`mod@exticr3`] module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///external interrupt configuration register 3
pub mod exticr3;
/**EXTICR4 (rw) register accessor: external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:EXTICR4)

For information about available fields see [`mod@exticr4`] module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///external interrupt configuration register 4
pub mod exticr4;
/**CFGR (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///configuration register
pub mod cfgr;
/**CCCSR (rw) register accessor: compensation cell control/status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:CCCSR)

For information about available fields see [`mod@cccsr`] module*/
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
///compensation cell control/status register
pub mod cccsr;
/**CCVR (r) register accessor: SYSCFG compensation cell value register

You can [`read`](crate::Reg::read) this register and get [`ccvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:CCVR)

For information about available fields see [`mod@ccvr`] module*/
pub type CCVR = crate::Reg<ccvr::CCVRrs>;
///SYSCFG compensation cell value register
pub mod ccvr;
/**CCCR (rw) register accessor: SYSCFG compensation cell code register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:CCCR)

For information about available fields see [`mod@cccr`] module*/
pub type CCCR = crate::Reg<cccr::CCCRrs>;
///SYSCFG compensation cell code register
pub mod cccr;
/**PWRCR (rw) register accessor: SYSCFG power control register

You can [`read`](crate::Reg::read) this register and get [`pwrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:PWRCR)

For information about available fields see [`mod@pwrcr`] module*/
pub type PWRCR = crate::Reg<pwrcr::PWRCRrs>;
///SYSCFG power control register
pub mod pwrcr;
/**PKGR (r) register accessor: SYSCFG package register

You can [`read`](crate::Reg::read) this register and get [`pkgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:PKGR)

For information about available fields see [`mod@pkgr`] module*/
pub type PKGR = crate::Reg<pkgr::PKGRrs>;
///SYSCFG package register
pub mod pkgr;
/**UR0 (r) register accessor: SYSCFG user register 0

You can [`read`](crate::Reg::read) this register and get [`ur0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR0)

For information about available fields see [`mod@ur0`] module*/
pub type UR0 = crate::Reg<ur0::UR0rs>;
///SYSCFG user register 0
pub mod ur0;
/**UR1 (rw) register accessor: SYSCFG user register 1

You can [`read`](crate::Reg::read) this register and get [`ur1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR1)

For information about available fields see [`mod@ur1`] module*/
pub type UR1 = crate::Reg<ur1::UR1rs>;
///SYSCFG user register 1
pub mod ur1;
/**UR2 (rw) register accessor: SYSCFG user register 2

You can [`read`](crate::Reg::read) this register and get [`ur2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR2)

For information about available fields see [`mod@ur2`] module*/
pub type UR2 = crate::Reg<ur2::UR2rs>;
///SYSCFG user register 2
pub mod ur2;
/**UR3 (rw) register accessor: SYSCFG user register 3

You can [`read`](crate::Reg::read) this register and get [`ur3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR3)

For information about available fields see [`mod@ur3`] module*/
pub type UR3 = crate::Reg<ur3::UR3rs>;
///SYSCFG user register 3
pub mod ur3;
/**UR4 (rw) register accessor: SYSCFG user register 4

You can [`read`](crate::Reg::read) this register and get [`ur4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR4)

For information about available fields see [`mod@ur4`] module*/
pub type UR4 = crate::Reg<ur4::UR4rs>;
///SYSCFG user register 4
pub mod ur4;
/**UR5 (r) register accessor: SYSCFG user register 5

You can [`read`](crate::Reg::read) this register and get [`ur5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR5)

For information about available fields see [`mod@ur5`] module*/
pub type UR5 = crate::Reg<ur5::UR5rs>;
///SYSCFG user register 5
pub mod ur5;
/**UR6 (r) register accessor: SYSCFG user register 6

You can [`read`](crate::Reg::read) this register and get [`ur6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR6)

For information about available fields see [`mod@ur6`] module*/
pub type UR6 = crate::Reg<ur6::UR6rs>;
///SYSCFG user register 6
pub mod ur6;
/**UR7 (r) register accessor: SYSCFG user register 7

You can [`read`](crate::Reg::read) this register and get [`ur7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR7)

For information about available fields see [`mod@ur7`] module*/
pub type UR7 = crate::Reg<ur7::UR7rs>;
///SYSCFG user register 7
pub mod ur7;
/**UR8 (r) register accessor: SYSCFG user register 8

You can [`read`](crate::Reg::read) this register and get [`ur8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR8)

For information about available fields see [`mod@ur8`] module*/
pub type UR8 = crate::Reg<ur8::UR8rs>;
///SYSCFG user register 8
pub mod ur8;
/**UR9 (r) register accessor: SYSCFG user register 9

You can [`read`](crate::Reg::read) this register and get [`ur9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR9)

For information about available fields see [`mod@ur9`] module*/
pub type UR9 = crate::Reg<ur9::UR9rs>;
///SYSCFG user register 9
pub mod ur9;
/**UR10 (r) register accessor: SYSCFG user register 10

You can [`read`](crate::Reg::read) this register and get [`ur10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR10)

For information about available fields see [`mod@ur10`] module*/
pub type UR10 = crate::Reg<ur10::UR10rs>;
///SYSCFG user register 10
pub mod ur10;
/**UR11 (r) register accessor: SYSCFG user register 11

You can [`read`](crate::Reg::read) this register and get [`ur11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR11)

For information about available fields see [`mod@ur11`] module*/
pub type UR11 = crate::Reg<ur11::UR11rs>;
///SYSCFG user register 11
pub mod ur11;
/**UR12 (r) register accessor: SYSCFG user register 12

You can [`read`](crate::Reg::read) this register and get [`ur12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR12)

For information about available fields see [`mod@ur12`] module*/
pub type UR12 = crate::Reg<ur12::UR12rs>;
///SYSCFG user register 12
pub mod ur12;
/**UR13 (r) register accessor: SYSCFG user register 13

You can [`read`](crate::Reg::read) this register and get [`ur13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR13)

For information about available fields see [`mod@ur13`] module*/
pub type UR13 = crate::Reg<ur13::UR13rs>;
///SYSCFG user register 13
pub mod ur13;
/**UR14 (rw) register accessor: SYSCFG user register 14

You can [`read`](crate::Reg::read) this register and get [`ur14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR14)

For information about available fields see [`mod@ur14`] module*/
pub type UR14 = crate::Reg<ur14::UR14rs>;
///SYSCFG user register 14
pub mod ur14;
/**UR15 (rw) register accessor: SYSCFG user register 15

You can [`read`](crate::Reg::read) this register and get [`ur15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR15)

For information about available fields see [`mod@ur15`] module*/
pub type UR15 = crate::Reg<ur15::UR15rs>;
///SYSCFG user register 15
pub mod ur15;
/**UR16 (r) register accessor: SYSCFG user register 16

You can [`read`](crate::Reg::read) this register and get [`ur16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR16)

For information about available fields see [`mod@ur16`] module*/
pub type UR16 = crate::Reg<ur16::UR16rs>;
///SYSCFG user register 16
pub mod ur16;
/**UR17 (r) register accessor: SYSCFG user register 17

You can [`read`](crate::Reg::read) this register and get [`ur17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR17)

For information about available fields see [`mod@ur17`] module*/
pub type UR17 = crate::Reg<ur17::UR17rs>;
///SYSCFG user register 17
pub mod ur17;
