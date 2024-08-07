#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    memrmp: MEMRMP,
    cfgr1: CFGR1,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    scsr: SCSR,
    cfgr2: CFGR2,
    swpr: SWPR,
    skr: SKR,
    swpr2: SWPR2,
    imr1: IMR1,
    imr2: IMR2,
    c2imr1: C2IMR1,
    c2imr2: C2IMR2,
    sipcr: SIPCR,
}
impl RegisterBlock {
    ///0x00 - memory remap register
    #[inline(always)]
    pub const fn memrmp(&self) -> &MEMRMP {
        &self.memrmp
    }
    ///0x04 - configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
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
    ///0x18 - SCSR
    #[inline(always)]
    pub const fn scsr(&self) -> &SCSR {
        &self.scsr
    }
    ///0x1c - CFGR2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x20 - SRAM2 write protection register
    #[inline(always)]
    pub const fn swpr(&self) -> &SWPR {
        &self.swpr
    }
    ///0x24 - SKR
    #[inline(always)]
    pub const fn skr(&self) -> &SKR {
        &self.skr
    }
    ///0x28 - SRAM2 write protection register 2
    #[inline(always)]
    pub const fn swpr2(&self) -> &SWPR2 {
        &self.swpr2
    }
    ///0x2c - CPU1 interrupt mask register 1
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    ///0x30 - CPU1 interrupt mask register 2
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    ///0x34 - CPU2 interrupt mask register 1
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    ///0x38 - CPU2 interrupt mask register 1
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    ///0x3c - secure IP control register
    #[inline(always)]
    pub const fn sipcr(&self) -> &SIPCR {
        &self.sipcr
    }
}
/**MEMRMP (rw) register accessor: memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:MEMRMP)

For information about available fields see [`mod@memrmp`]
module*/
pub type MEMRMP = crate::Reg<memrmp::MEMRMPrs>;
///memory remap register
pub mod memrmp;
/**CFGR1 (rw) register accessor: configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:CFGR1)

For information about available fields see [`mod@cfgr1`]
module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///configuration register 1
pub mod cfgr1;
/**EXTICR1 (rw) register accessor: external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:EXTICR1)

For information about available fields see [`mod@exticr1`]
module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///external interrupt configuration register 1
pub mod exticr1;
/**EXTICR2 (rw) register accessor: external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:EXTICR2)

For information about available fields see [`mod@exticr2`]
module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///external interrupt configuration register 2
pub mod exticr2;
/**EXTICR3 (rw) register accessor: external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:EXTICR3)

For information about available fields see [`mod@exticr3`]
module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///external interrupt configuration register 3
pub mod exticr3;
/**EXTICR4 (rw) register accessor: external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:EXTICR4)

For information about available fields see [`mod@exticr4`]
module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///external interrupt configuration register 4
pub mod exticr4;
/**SCSR (rw) register accessor: SCSR

You can [`read`](crate::Reg::read) this register and get [`scsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SCSR)

For information about available fields see [`mod@scsr`]
module*/
pub type SCSR = crate::Reg<scsr::SCSRrs>;
///SCSR
pub mod scsr;
/**CFGR2 (rw) register accessor: CFGR2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:CFGR2)

For information about available fields see [`mod@cfgr2`]
module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///CFGR2
pub mod cfgr2;
/**SWPR (w) register accessor: SRAM2 write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SWPR)

For information about available fields see [`mod@swpr`]
module*/
pub type SWPR = crate::Reg<swpr::SWPRrs>;
///SRAM2 write protection register
pub mod swpr;
/**SKR (w) register accessor: SKR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SKR)

For information about available fields see [`mod@skr`]
module*/
pub type SKR = crate::Reg<skr::SKRrs>;
///SKR
pub mod skr;
/**SWPR2 (w) register accessor: SRAM2 write protection register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SWPR2)

For information about available fields see [`mod@swpr2`]
module*/
pub type SWPR2 = crate::Reg<swpr2::SWPR2rs>;
///SRAM2 write protection register 2
pub mod swpr2;
/**IMR1 (rw) register accessor: CPU1 interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:IMR1)

For information about available fields see [`mod@imr1`]
module*/
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
///CPU1 interrupt mask register 1
pub mod imr1;
/**IMR2 (rw) register accessor: CPU1 interrupt mask register 2

You can [`read`](crate::Reg::read) this register and get [`imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:IMR2)

For information about available fields see [`mod@imr2`]
module*/
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
///CPU1 interrupt mask register 2
pub mod imr2;
/**C2IMR1 (rw) register accessor: CPU2 interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`c2imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:C2IMR1)

For information about available fields see [`mod@c2imr1`]
module*/
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
///CPU2 interrupt mask register 1
pub mod c2imr1;
/**C2IMR2 (rw) register accessor: CPU2 interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`c2imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:C2IMR2)

For information about available fields see [`mod@c2imr2`]
module*/
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
///CPU2 interrupt mask register 1
pub mod c2imr2;
/**SIPCR (rw) register accessor: secure IP control register

You can [`read`](crate::Reg::read) this register and get [`sipcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sipcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SIPCR)

For information about available fields see [`mod@sipcr`]
module*/
pub type SIPCR = crate::Reg<sipcr::SIPCRrs>;
///secure IP control register
pub mod sipcr;
