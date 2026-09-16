#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfgr1: CFGR1,
    rcr: RCR,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    cfgr2: CFGR2,
    _reserved7: [u8; 0x34],
    cfgr3: CFGR3,
}
impl RegisterBlock {
    ///0x00 - configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x04 - CCM SRAM protection register
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
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
    ///0x18 - configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x50 - configuration register 3
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
}
/**CFGR1 (rw) register accessor: configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///configuration register 1
pub mod cfgr1;
/**EXTICR1 (rw) register accessor: external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:EXTICR1)

For information about available fields see [`mod@exticr1`] module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///external interrupt configuration register 1
pub mod exticr1;
/**EXTICR2 (rw) register accessor: external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:EXTICR2)

For information about available fields see [`mod@exticr2`] module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///external interrupt configuration register 2
pub mod exticr2;
/**EXTICR3 (rw) register accessor: external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:EXTICR3)

For information about available fields see [`mod@exticr3`] module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///external interrupt configuration register 3
pub mod exticr3;
/**EXTICR4 (rw) register accessor: external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:EXTICR4)

For information about available fields see [`mod@exticr4`] module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///external interrupt configuration register 4
pub mod exticr4;
/**CFGR2 (rw) register accessor: configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///configuration register 2
pub mod cfgr2;
/**RCR (rw) register accessor: CCM SRAM protection register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:RCR)

For information about available fields see [`mod@rcr`] module*/
pub type RCR = crate::Reg<rcr::RCRrs>;
///CCM SRAM protection register
pub mod rcr;
/**CFGR3 (rw) register accessor: configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:CFGR3)

For information about available fields see [`mod@cfgr3`] module*/
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
///configuration register 3
pub mod cfgr3;
