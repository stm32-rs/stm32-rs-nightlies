#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    comp1_sr: COMP1_SR,
    comp1_icfr: COMP1_ICFR,
    comp1_or: COMP1_OR,
    comp1_cfgr1: COMP1_CFGR1,
    comp1_cfgr2: COMP1_CFGR2,
}
impl RegisterBlock {
    ///0x00 - Comparator status register
    #[inline(always)]
    pub const fn comp1_sr(&self) -> &COMP1_SR {
        &self.comp1_sr
    }
    ///0x04 - Comparator interrupt clear flag register
    #[inline(always)]
    pub const fn comp1_icfr(&self) -> &COMP1_ICFR {
        &self.comp1_icfr
    }
    ///0x08 - Comparator option register
    #[inline(always)]
    pub const fn comp1_or(&self) -> &COMP1_OR {
        &self.comp1_or
    }
    ///0x0c - Comparator configuration register 1
    #[inline(always)]
    pub const fn comp1_cfgr1(&self) -> &COMP1_CFGR1 {
        &self.comp1_cfgr1
    }
    ///0x10 - Comparator configuration register 2
    #[inline(always)]
    pub const fn comp1_cfgr2(&self) -> &COMP1_CFGR2 {
        &self.comp1_cfgr2
    }
}
/**COMP1_SR (r) register accessor: Comparator status register

You can [`read`](crate::Reg::read) this register and get [`comp1_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#COMP1:COMP1_SR)

For information about available fields see [`mod@comp1_sr`] module*/
pub type COMP1_SR = crate::Reg<comp1_sr::COMP1_SRrs>;
///Comparator status register
pub mod comp1_sr;
/**COMP1_ICFR (w) register accessor: Comparator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_icfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#COMP1:COMP1_ICFR)

For information about available fields see [`mod@comp1_icfr`] module*/
pub type COMP1_ICFR = crate::Reg<comp1_icfr::COMP1_ICFRrs>;
///Comparator interrupt clear flag register
pub mod comp1_icfr;
/**COMP1_OR (rw) register accessor: Comparator option register

You can [`read`](crate::Reg::read) this register and get [`comp1_or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#COMP1:COMP1_OR)

For information about available fields see [`mod@comp1_or`] module*/
pub type COMP1_OR = crate::Reg<comp1_or::COMP1_ORrs>;
///Comparator option register
pub mod comp1_or;
/**COMP1_CFGR1 (rw) register accessor: Comparator configuration register 1

You can [`read`](crate::Reg::read) this register and get [`comp1_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#COMP1:COMP1_CFGR1)

For information about available fields see [`mod@comp1_cfgr1`] module*/
pub type COMP1_CFGR1 = crate::Reg<comp1_cfgr1::COMP1_CFGR1rs>;
///Comparator configuration register 1
pub mod comp1_cfgr1;
/**COMP1_CFGR2 (rw) register accessor: Comparator configuration register 2

You can [`read`](crate::Reg::read) this register and get [`comp1_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#COMP1:COMP1_CFGR2)

For information about available fields see [`mod@comp1_cfgr2`] module*/
pub type COMP1_CFGR2 = crate::Reg<comp1_cfgr2::COMP1_CFGR2rs>;
///Comparator configuration register 2
pub mod comp1_cfgr2;
