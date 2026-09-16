#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    memrmp: MEMRMP,
    pmc: PMC,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    _reserved6: [u8; 0x08],
    cmpcr: CMPCR,
}
impl RegisterBlock {
    ///0x00 - memory remap register
    #[inline(always)]
    pub const fn memrmp(&self) -> &MEMRMP {
        &self.memrmp
    }
    ///0x04 - peripheral mode configuration register
    #[inline(always)]
    pub const fn pmc(&self) -> &PMC {
        &self.pmc
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
    ///0x20 - Compensation cell control register
    #[inline(always)]
    pub const fn cmpcr(&self) -> &CMPCR {
        &self.cmpcr
    }
}
/**MEMRMP (rw) register accessor: memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#SYSCFG:MEMRMP)

For information about available fields see [`mod@memrmp`] module*/
pub type MEMRMP = crate::Reg<memrmp::MEMRMPrs>;
///memory remap register
pub mod memrmp;
/**PMC (rw) register accessor: peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#SYSCFG:PMC)

For information about available fields see [`mod@pmc`] module*/
pub type PMC = crate::Reg<pmc::PMCrs>;
///peripheral mode configuration register
pub mod pmc;
/**EXTICR1 (rw) register accessor: external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#SYSCFG:EXTICR1)

For information about available fields see [`mod@exticr1`] module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///external interrupt configuration register 1
pub mod exticr1;
/**EXTICR2 (rw) register accessor: external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#SYSCFG:EXTICR2)

For information about available fields see [`mod@exticr2`] module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///external interrupt configuration register 2
pub mod exticr2;
/**EXTICR3 (rw) register accessor: external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#SYSCFG:EXTICR3)

For information about available fields see [`mod@exticr3`] module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///external interrupt configuration register 3
pub mod exticr3;
/**EXTICR4 (rw) register accessor: external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#SYSCFG:EXTICR4)

For information about available fields see [`mod@exticr4`] module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///external interrupt configuration register 4
pub mod exticr4;
/**CMPCR (r) register accessor: Compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`cmpcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#SYSCFG:CMPCR)

For information about available fields see [`mod@cmpcr`] module*/
pub type CMPCR = crate::Reg<cmpcr::CMPCRrs>;
///Compensation cell control register
pub mod cmpcr;
