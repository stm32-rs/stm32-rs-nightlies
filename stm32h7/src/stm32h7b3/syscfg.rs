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
    ///0x18 - SYSCFG timer break lockup register
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
}
/**PMCR (rw) register accessor: peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:PMCR)

For information about available fields see [`mod@pmcr`] module*/
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
///peripheral mode configuration register
pub mod pmcr;
/**EXTICR1 (rw) register accessor: external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:EXTICR1)

For information about available fields see [`mod@exticr1`] module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///external interrupt configuration register 1
pub mod exticr1;
/**EXTICR2 (rw) register accessor: external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:EXTICR2)

For information about available fields see [`mod@exticr2`] module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///external interrupt configuration register 2
pub mod exticr2;
/**EXTICR3 (rw) register accessor: external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:EXTICR3)

For information about available fields see [`mod@exticr3`] module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///external interrupt configuration register 3
pub mod exticr3;
/**EXTICR4 (rw) register accessor: external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:EXTICR4)

For information about available fields see [`mod@exticr4`] module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///external interrupt configuration register 4
pub mod exticr4;
/**CCCSR (rw) register accessor: compensation cell control/status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:CCCSR)

For information about available fields see [`mod@cccsr`] module*/
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
///compensation cell control/status register
pub mod cccsr;
/**CCVR (r) register accessor: SYSCFG compensation cell value register

You can [`read`](crate::Reg::read) this register and get [`ccvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:CCVR)

For information about available fields see [`mod@ccvr`] module*/
pub type CCVR = crate::Reg<ccvr::CCVRrs>;
///SYSCFG compensation cell value register
pub mod ccvr;
/**CCCR (rw) register accessor: SYSCFG compensation cell code register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:CCCR)

For information about available fields see [`mod@cccr`] module*/
pub type CCCR = crate::Reg<cccr::CCCRrs>;
///SYSCFG compensation cell code register
pub mod cccr;
/**CFGR (rw) register accessor: SYSCFG timer break lockup register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///SYSCFG timer break lockup register
pub mod cfgr;
