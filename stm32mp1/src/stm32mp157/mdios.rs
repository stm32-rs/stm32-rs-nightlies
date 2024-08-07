#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mdios_cr: MDIOS_CR,
    mdios_wrfr: MDIOS_WRFR,
    mdios_cwrfr: MDIOS_CWRFR,
    mdios_rdfr: MDIOS_RDFR,
    mdios_crdfr: MDIOS_CRDFR,
    mdios_sr: MDIOS_SR,
    mdios_clrfr: MDIOS_CLRFR,
    _reserved7: [u8; 0xe4],
    mdios_dinr0: MDIOS_DINR0,
    mdios_dinr1: MDIOS_DINR1,
    mdios_dinr2: MDIOS_DINR2,
    mdios_dinr3: MDIOS_DINR3,
    mdios_dinr4: MDIOS_DINR4,
    mdios_dinr5: MDIOS_DINR5,
    mdios_dinr6: MDIOS_DINR6,
    mdios_dinr7: MDIOS_DINR7,
    mdios_dinr8: MDIOS_DINR8,
    mdios_dinr9: MDIOS_DINR9,
    mdios_dinr10: MDIOS_DINR10,
    mdios_dinr11: MDIOS_DINR11,
    mdios_dinr12: MDIOS_DINR12,
    mdios_dinr13: MDIOS_DINR13,
    mdios_dinr14: MDIOS_DINR14,
    mdios_dinr15: MDIOS_DINR15,
    mdios_dinr16: MDIOS_DINR16,
    mdios_dinr17: MDIOS_DINR17,
    mdios_dinr18: MDIOS_DINR18,
    mdios_dinr19: MDIOS_DINR19,
    mdios_dinr20: MDIOS_DINR20,
    mdios_dinr21: MDIOS_DINR21,
    mdios_dinr22: MDIOS_DINR22,
    mdios_dinr23: MDIOS_DINR23,
    mdios_dinr24: MDIOS_DINR24,
    mdios_dinr25: MDIOS_DINR25,
    mdios_dinr26: MDIOS_DINR26,
    mdios_dinr27: MDIOS_DINR27,
    mdios_dinr28: MDIOS_DINR28,
    mdios_dinr29: MDIOS_DINR29,
    mdios_dinr30: MDIOS_DINR30,
    mdios_dinr31: MDIOS_DINR31,
    mdios_doutr0: MDIOS_DOUTR0,
    mdios_doutr1: MDIOS_DOUTR1,
    mdios_doutr2: MDIOS_DOUTR2,
    mdios_doutr3: MDIOS_DOUTR3,
    mdios_doutr4: MDIOS_DOUTR4,
    mdios_doutr5: MDIOS_DOUTR5,
    mdios_doutr6: MDIOS_DOUTR6,
    mdios_doutr7: MDIOS_DOUTR7,
    mdios_doutr8: MDIOS_DOUTR8,
    mdios_doutr9: MDIOS_DOUTR9,
    mdios_doutr10: MDIOS_DOUTR10,
    mdios_doutr11: MDIOS_DOUTR11,
    mdios_doutr12: MDIOS_DOUTR12,
    mdios_doutr13: MDIOS_DOUTR13,
    mdios_doutr14: MDIOS_DOUTR14,
    mdios_doutr15: MDIOS_DOUTR15,
    mdios_doutr16: MDIOS_DOUTR16,
    mdios_doutr17: MDIOS_DOUTR17,
    mdios_doutr18: MDIOS_DOUTR18,
    mdios_doutr19: MDIOS_DOUTR19,
    mdios_doutr20: MDIOS_DOUTR20,
    mdios_doutr21: MDIOS_DOUTR21,
    mdios_doutr22: MDIOS_DOUTR22,
    mdios_doutr23: MDIOS_DOUTR23,
    mdios_doutr24: MDIOS_DOUTR24,
    mdios_doutr25: MDIOS_DOUTR25,
    mdios_doutr26: MDIOS_DOUTR26,
    mdios_doutr27: MDIOS_DOUTR27,
    mdios_doutr28: MDIOS_DOUTR28,
    mdios_doutr29: MDIOS_DOUTR29,
    mdios_doutr30: MDIOS_DOUTR30,
    mdios_doutr31: MDIOS_DOUTR31,
    _reserved71: [u8; 0x01f0],
    mdios_hwcfgr: MDIOS_HWCFGR,
    mdios_verr: MDIOS_VERR,
    mdios_ipidr: MDIOS_IPIDR,
    mdios_sidr: MDIOS_SIDR,
}
impl RegisterBlock {
    ///0x00 - MDIOS configuration register
    #[inline(always)]
    pub const fn mdios_cr(&self) -> &MDIOS_CR {
        &self.mdios_cr
    }
    ///0x04 - MDIOS write flag register
    #[inline(always)]
    pub const fn mdios_wrfr(&self) -> &MDIOS_WRFR {
        &self.mdios_wrfr
    }
    ///0x08 - MDIOS clear write flag register
    #[inline(always)]
    pub const fn mdios_cwrfr(&self) -> &MDIOS_CWRFR {
        &self.mdios_cwrfr
    }
    ///0x0c - MDIOS read flag register
    #[inline(always)]
    pub const fn mdios_rdfr(&self) -> &MDIOS_RDFR {
        &self.mdios_rdfr
    }
    ///0x10 - MDIOS clear read flag register
    #[inline(always)]
    pub const fn mdios_crdfr(&self) -> &MDIOS_CRDFR {
        &self.mdios_crdfr
    }
    ///0x14 - MDIOS status register
    #[inline(always)]
    pub const fn mdios_sr(&self) -> &MDIOS_SR {
        &self.mdios_sr
    }
    ///0x18 - MDIOS clear flag register
    #[inline(always)]
    pub const fn mdios_clrfr(&self) -> &MDIOS_CLRFR {
        &self.mdios_clrfr
    }
    ///0x100 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr0(&self) -> &MDIOS_DINR0 {
        &self.mdios_dinr0
    }
    ///0x104 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr1(&self) -> &MDIOS_DINR1 {
        &self.mdios_dinr1
    }
    ///0x108 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr2(&self) -> &MDIOS_DINR2 {
        &self.mdios_dinr2
    }
    ///0x10c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr3(&self) -> &MDIOS_DINR3 {
        &self.mdios_dinr3
    }
    ///0x110 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr4(&self) -> &MDIOS_DINR4 {
        &self.mdios_dinr4
    }
    ///0x114 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr5(&self) -> &MDIOS_DINR5 {
        &self.mdios_dinr5
    }
    ///0x118 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr6(&self) -> &MDIOS_DINR6 {
        &self.mdios_dinr6
    }
    ///0x11c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr7(&self) -> &MDIOS_DINR7 {
        &self.mdios_dinr7
    }
    ///0x120 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr8(&self) -> &MDIOS_DINR8 {
        &self.mdios_dinr8
    }
    ///0x124 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr9(&self) -> &MDIOS_DINR9 {
        &self.mdios_dinr9
    }
    ///0x128 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr10(&self) -> &MDIOS_DINR10 {
        &self.mdios_dinr10
    }
    ///0x12c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr11(&self) -> &MDIOS_DINR11 {
        &self.mdios_dinr11
    }
    ///0x130 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr12(&self) -> &MDIOS_DINR12 {
        &self.mdios_dinr12
    }
    ///0x134 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr13(&self) -> &MDIOS_DINR13 {
        &self.mdios_dinr13
    }
    ///0x138 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr14(&self) -> &MDIOS_DINR14 {
        &self.mdios_dinr14
    }
    ///0x13c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr15(&self) -> &MDIOS_DINR15 {
        &self.mdios_dinr15
    }
    ///0x140 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr16(&self) -> &MDIOS_DINR16 {
        &self.mdios_dinr16
    }
    ///0x144 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr17(&self) -> &MDIOS_DINR17 {
        &self.mdios_dinr17
    }
    ///0x148 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr18(&self) -> &MDIOS_DINR18 {
        &self.mdios_dinr18
    }
    ///0x14c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr19(&self) -> &MDIOS_DINR19 {
        &self.mdios_dinr19
    }
    ///0x150 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr20(&self) -> &MDIOS_DINR20 {
        &self.mdios_dinr20
    }
    ///0x154 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr21(&self) -> &MDIOS_DINR21 {
        &self.mdios_dinr21
    }
    ///0x158 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr22(&self) -> &MDIOS_DINR22 {
        &self.mdios_dinr22
    }
    ///0x15c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr23(&self) -> &MDIOS_DINR23 {
        &self.mdios_dinr23
    }
    ///0x160 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr24(&self) -> &MDIOS_DINR24 {
        &self.mdios_dinr24
    }
    ///0x164 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr25(&self) -> &MDIOS_DINR25 {
        &self.mdios_dinr25
    }
    ///0x168 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr26(&self) -> &MDIOS_DINR26 {
        &self.mdios_dinr26
    }
    ///0x16c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr27(&self) -> &MDIOS_DINR27 {
        &self.mdios_dinr27
    }
    ///0x170 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr28(&self) -> &MDIOS_DINR28 {
        &self.mdios_dinr28
    }
    ///0x174 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr29(&self) -> &MDIOS_DINR29 {
        &self.mdios_dinr29
    }
    ///0x178 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr30(&self) -> &MDIOS_DINR30 {
        &self.mdios_dinr30
    }
    ///0x17c - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_dinr31(&self) -> &MDIOS_DINR31 {
        &self.mdios_dinr31
    }
    ///0x180 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_doutr0(&self) -> &MDIOS_DOUTR0 {
        &self.mdios_doutr0
    }
    ///0x184 - MDIOS input data register
    #[inline(always)]
    pub const fn mdios_doutr1(&self) -> &MDIOS_DOUTR1 {
        &self.mdios_doutr1
    }
    ///0x188 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr2(&self) -> &MDIOS_DOUTR2 {
        &self.mdios_doutr2
    }
    ///0x18c - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr3(&self) -> &MDIOS_DOUTR3 {
        &self.mdios_doutr3
    }
    ///0x190 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr4(&self) -> &MDIOS_DOUTR4 {
        &self.mdios_doutr4
    }
    ///0x194 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr5(&self) -> &MDIOS_DOUTR5 {
        &self.mdios_doutr5
    }
    ///0x198 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr6(&self) -> &MDIOS_DOUTR6 {
        &self.mdios_doutr6
    }
    ///0x19c - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr7(&self) -> &MDIOS_DOUTR7 {
        &self.mdios_doutr7
    }
    ///0x1a0 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr8(&self) -> &MDIOS_DOUTR8 {
        &self.mdios_doutr8
    }
    ///0x1a4 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr9(&self) -> &MDIOS_DOUTR9 {
        &self.mdios_doutr9
    }
    ///0x1a8 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr10(&self) -> &MDIOS_DOUTR10 {
        &self.mdios_doutr10
    }
    ///0x1ac - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr11(&self) -> &MDIOS_DOUTR11 {
        &self.mdios_doutr11
    }
    ///0x1b0 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr12(&self) -> &MDIOS_DOUTR12 {
        &self.mdios_doutr12
    }
    ///0x1b4 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr13(&self) -> &MDIOS_DOUTR13 {
        &self.mdios_doutr13
    }
    ///0x1b8 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr14(&self) -> &MDIOS_DOUTR14 {
        &self.mdios_doutr14
    }
    ///0x1bc - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr15(&self) -> &MDIOS_DOUTR15 {
        &self.mdios_doutr15
    }
    ///0x1c0 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr16(&self) -> &MDIOS_DOUTR16 {
        &self.mdios_doutr16
    }
    ///0x1c4 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr17(&self) -> &MDIOS_DOUTR17 {
        &self.mdios_doutr17
    }
    ///0x1c8 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr18(&self) -> &MDIOS_DOUTR18 {
        &self.mdios_doutr18
    }
    ///0x1cc - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr19(&self) -> &MDIOS_DOUTR19 {
        &self.mdios_doutr19
    }
    ///0x1d0 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr20(&self) -> &MDIOS_DOUTR20 {
        &self.mdios_doutr20
    }
    ///0x1d4 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr21(&self) -> &MDIOS_DOUTR21 {
        &self.mdios_doutr21
    }
    ///0x1d8 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr22(&self) -> &MDIOS_DOUTR22 {
        &self.mdios_doutr22
    }
    ///0x1dc - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr23(&self) -> &MDIOS_DOUTR23 {
        &self.mdios_doutr23
    }
    ///0x1e0 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr24(&self) -> &MDIOS_DOUTR24 {
        &self.mdios_doutr24
    }
    ///0x1e4 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr25(&self) -> &MDIOS_DOUTR25 {
        &self.mdios_doutr25
    }
    ///0x1e8 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr26(&self) -> &MDIOS_DOUTR26 {
        &self.mdios_doutr26
    }
    ///0x1ec - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr27(&self) -> &MDIOS_DOUTR27 {
        &self.mdios_doutr27
    }
    ///0x1f0 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr28(&self) -> &MDIOS_DOUTR28 {
        &self.mdios_doutr28
    }
    ///0x1f4 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr29(&self) -> &MDIOS_DOUTR29 {
        &self.mdios_doutr29
    }
    ///0x1f8 - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr30(&self) -> &MDIOS_DOUTR30 {
        &self.mdios_doutr30
    }
    ///0x1fc - MDIOS output data register
    #[inline(always)]
    pub const fn mdios_doutr31(&self) -> &MDIOS_DOUTR31 {
        &self.mdios_doutr31
    }
    ///0x3f0 - MDIOS HW configuration register
    #[inline(always)]
    pub const fn mdios_hwcfgr(&self) -> &MDIOS_HWCFGR {
        &self.mdios_hwcfgr
    }
    ///0x3f4 - MDIOS version register
    #[inline(always)]
    pub const fn mdios_verr(&self) -> &MDIOS_VERR {
        &self.mdios_verr
    }
    ///0x3f8 - MDIOS identification register
    #[inline(always)]
    pub const fn mdios_ipidr(&self) -> &MDIOS_IPIDR {
        &self.mdios_ipidr
    }
    ///0x3fc - MDIOS size identification register
    #[inline(always)]
    pub const fn mdios_sidr(&self) -> &MDIOS_SIDR {
        &self.mdios_sidr
    }
}
/**MDIOS_CR (rw) register accessor: MDIOS configuration register

You can [`read`](crate::Reg::read) this register and get [`mdios_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_CR)

For information about available fields see [`mod@mdios_cr`]
module*/
pub type MDIOS_CR = crate::Reg<mdios_cr::MDIOS_CRrs>;
///MDIOS configuration register
pub mod mdios_cr;
/**MDIOS_WRFR (r) register accessor: MDIOS write flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_wrfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_WRFR)

For information about available fields see [`mod@mdios_wrfr`]
module*/
pub type MDIOS_WRFR = crate::Reg<mdios_wrfr::MDIOS_WRFRrs>;
///MDIOS write flag register
pub mod mdios_wrfr;
/**MDIOS_CWRFR (rw) register accessor: MDIOS clear write flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_cwrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_cwrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_CWRFR)

For information about available fields see [`mod@mdios_cwrfr`]
module*/
pub type MDIOS_CWRFR = crate::Reg<mdios_cwrfr::MDIOS_CWRFRrs>;
///MDIOS clear write flag register
pub mod mdios_cwrfr;
/**MDIOS_RDFR (r) register accessor: MDIOS read flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_rdfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_RDFR)

For information about available fields see [`mod@mdios_rdfr`]
module*/
pub type MDIOS_RDFR = crate::Reg<mdios_rdfr::MDIOS_RDFRrs>;
///MDIOS read flag register
pub mod mdios_rdfr;
/**MDIOS_CRDFR (rw) register accessor: MDIOS clear read flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_crdfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_crdfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_CRDFR)

For information about available fields see [`mod@mdios_crdfr`]
module*/
pub type MDIOS_CRDFR = crate::Reg<mdios_crdfr::MDIOS_CRDFRrs>;
///MDIOS clear read flag register
pub mod mdios_crdfr;
/**MDIOS_SR (r) register accessor: MDIOS status register

You can [`read`](crate::Reg::read) this register and get [`mdios_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_SR)

For information about available fields see [`mod@mdios_sr`]
module*/
pub type MDIOS_SR = crate::Reg<mdios_sr::MDIOS_SRrs>;
///MDIOS status register
pub mod mdios_sr;
/**MDIOS_CLRFR (rw) register accessor: MDIOS clear flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_clrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_clrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_CLRFR)

For information about available fields see [`mod@mdios_clrfr`]
module*/
pub type MDIOS_CLRFR = crate::Reg<mdios_clrfr::MDIOS_CLRFRrs>;
///MDIOS clear flag register
pub mod mdios_clrfr;
/**MDIOS_DINR0 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR0)

For information about available fields see [`mod@mdios_dinr0`]
module*/
pub type MDIOS_DINR0 = crate::Reg<mdios_dinr0::MDIOS_DINR0rs>;
///MDIOS input data register
pub mod mdios_dinr0;
/**MDIOS_DINR1 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR1)

For information about available fields see [`mod@mdios_dinr1`]
module*/
pub type MDIOS_DINR1 = crate::Reg<mdios_dinr1::MDIOS_DINR1rs>;
///MDIOS input data register
pub mod mdios_dinr1;
/**MDIOS_DINR2 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR2)

For information about available fields see [`mod@mdios_dinr2`]
module*/
pub type MDIOS_DINR2 = crate::Reg<mdios_dinr2::MDIOS_DINR2rs>;
///MDIOS input data register
pub mod mdios_dinr2;
/**MDIOS_DINR3 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR3)

For information about available fields see [`mod@mdios_dinr3`]
module*/
pub type MDIOS_DINR3 = crate::Reg<mdios_dinr3::MDIOS_DINR3rs>;
///MDIOS input data register
pub mod mdios_dinr3;
/**MDIOS_DINR4 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR4)

For information about available fields see [`mod@mdios_dinr4`]
module*/
pub type MDIOS_DINR4 = crate::Reg<mdios_dinr4::MDIOS_DINR4rs>;
///MDIOS input data register
pub mod mdios_dinr4;
/**MDIOS_DINR5 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR5)

For information about available fields see [`mod@mdios_dinr5`]
module*/
pub type MDIOS_DINR5 = crate::Reg<mdios_dinr5::MDIOS_DINR5rs>;
///MDIOS input data register
pub mod mdios_dinr5;
/**MDIOS_DINR6 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR6)

For information about available fields see [`mod@mdios_dinr6`]
module*/
pub type MDIOS_DINR6 = crate::Reg<mdios_dinr6::MDIOS_DINR6rs>;
///MDIOS input data register
pub mod mdios_dinr6;
/**MDIOS_DINR7 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR7)

For information about available fields see [`mod@mdios_dinr7`]
module*/
pub type MDIOS_DINR7 = crate::Reg<mdios_dinr7::MDIOS_DINR7rs>;
///MDIOS input data register
pub mod mdios_dinr7;
/**MDIOS_DINR8 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR8)

For information about available fields see [`mod@mdios_dinr8`]
module*/
pub type MDIOS_DINR8 = crate::Reg<mdios_dinr8::MDIOS_DINR8rs>;
///MDIOS input data register
pub mod mdios_dinr8;
/**MDIOS_DINR9 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR9)

For information about available fields see [`mod@mdios_dinr9`]
module*/
pub type MDIOS_DINR9 = crate::Reg<mdios_dinr9::MDIOS_DINR9rs>;
///MDIOS input data register
pub mod mdios_dinr9;
/**MDIOS_DINR10 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR10)

For information about available fields see [`mod@mdios_dinr10`]
module*/
pub type MDIOS_DINR10 = crate::Reg<mdios_dinr10::MDIOS_DINR10rs>;
///MDIOS input data register
pub mod mdios_dinr10;
/**MDIOS_DINR11 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR11)

For information about available fields see [`mod@mdios_dinr11`]
module*/
pub type MDIOS_DINR11 = crate::Reg<mdios_dinr11::MDIOS_DINR11rs>;
///MDIOS input data register
pub mod mdios_dinr11;
/**MDIOS_DINR12 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR12)

For information about available fields see [`mod@mdios_dinr12`]
module*/
pub type MDIOS_DINR12 = crate::Reg<mdios_dinr12::MDIOS_DINR12rs>;
///MDIOS input data register
pub mod mdios_dinr12;
/**MDIOS_DINR13 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR13)

For information about available fields see [`mod@mdios_dinr13`]
module*/
pub type MDIOS_DINR13 = crate::Reg<mdios_dinr13::MDIOS_DINR13rs>;
///MDIOS input data register
pub mod mdios_dinr13;
/**MDIOS_DINR14 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR14)

For information about available fields see [`mod@mdios_dinr14`]
module*/
pub type MDIOS_DINR14 = crate::Reg<mdios_dinr14::MDIOS_DINR14rs>;
///MDIOS input data register
pub mod mdios_dinr14;
/**MDIOS_DINR15 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR15)

For information about available fields see [`mod@mdios_dinr15`]
module*/
pub type MDIOS_DINR15 = crate::Reg<mdios_dinr15::MDIOS_DINR15rs>;
///MDIOS input data register
pub mod mdios_dinr15;
/**MDIOS_DINR16 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR16)

For information about available fields see [`mod@mdios_dinr16`]
module*/
pub type MDIOS_DINR16 = crate::Reg<mdios_dinr16::MDIOS_DINR16rs>;
///MDIOS input data register
pub mod mdios_dinr16;
/**MDIOS_DINR17 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR17)

For information about available fields see [`mod@mdios_dinr17`]
module*/
pub type MDIOS_DINR17 = crate::Reg<mdios_dinr17::MDIOS_DINR17rs>;
///MDIOS input data register
pub mod mdios_dinr17;
/**MDIOS_DINR18 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR18)

For information about available fields see [`mod@mdios_dinr18`]
module*/
pub type MDIOS_DINR18 = crate::Reg<mdios_dinr18::MDIOS_DINR18rs>;
///MDIOS input data register
pub mod mdios_dinr18;
/**MDIOS_DINR19 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR19)

For information about available fields see [`mod@mdios_dinr19`]
module*/
pub type MDIOS_DINR19 = crate::Reg<mdios_dinr19::MDIOS_DINR19rs>;
///MDIOS input data register
pub mod mdios_dinr19;
/**MDIOS_DINR20 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR20)

For information about available fields see [`mod@mdios_dinr20`]
module*/
pub type MDIOS_DINR20 = crate::Reg<mdios_dinr20::MDIOS_DINR20rs>;
///MDIOS input data register
pub mod mdios_dinr20;
/**MDIOS_DINR21 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR21)

For information about available fields see [`mod@mdios_dinr21`]
module*/
pub type MDIOS_DINR21 = crate::Reg<mdios_dinr21::MDIOS_DINR21rs>;
///MDIOS input data register
pub mod mdios_dinr21;
/**MDIOS_DINR22 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR22)

For information about available fields see [`mod@mdios_dinr22`]
module*/
pub type MDIOS_DINR22 = crate::Reg<mdios_dinr22::MDIOS_DINR22rs>;
///MDIOS input data register
pub mod mdios_dinr22;
/**MDIOS_DINR23 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR23)

For information about available fields see [`mod@mdios_dinr23`]
module*/
pub type MDIOS_DINR23 = crate::Reg<mdios_dinr23::MDIOS_DINR23rs>;
///MDIOS input data register
pub mod mdios_dinr23;
/**MDIOS_DINR24 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR24)

For information about available fields see [`mod@mdios_dinr24`]
module*/
pub type MDIOS_DINR24 = crate::Reg<mdios_dinr24::MDIOS_DINR24rs>;
///MDIOS input data register
pub mod mdios_dinr24;
/**MDIOS_DINR25 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR25)

For information about available fields see [`mod@mdios_dinr25`]
module*/
pub type MDIOS_DINR25 = crate::Reg<mdios_dinr25::MDIOS_DINR25rs>;
///MDIOS input data register
pub mod mdios_dinr25;
/**MDIOS_DINR26 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR26)

For information about available fields see [`mod@mdios_dinr26`]
module*/
pub type MDIOS_DINR26 = crate::Reg<mdios_dinr26::MDIOS_DINR26rs>;
///MDIOS input data register
pub mod mdios_dinr26;
/**MDIOS_DINR27 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR27)

For information about available fields see [`mod@mdios_dinr27`]
module*/
pub type MDIOS_DINR27 = crate::Reg<mdios_dinr27::MDIOS_DINR27rs>;
///MDIOS input data register
pub mod mdios_dinr27;
/**MDIOS_DINR28 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR28)

For information about available fields see [`mod@mdios_dinr28`]
module*/
pub type MDIOS_DINR28 = crate::Reg<mdios_dinr28::MDIOS_DINR28rs>;
///MDIOS input data register
pub mod mdios_dinr28;
/**MDIOS_DINR29 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR29)

For information about available fields see [`mod@mdios_dinr29`]
module*/
pub type MDIOS_DINR29 = crate::Reg<mdios_dinr29::MDIOS_DINR29rs>;
///MDIOS input data register
pub mod mdios_dinr29;
/**MDIOS_DINR30 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR30)

For information about available fields see [`mod@mdios_dinr30`]
module*/
pub type MDIOS_DINR30 = crate::Reg<mdios_dinr30::MDIOS_DINR30rs>;
///MDIOS input data register
pub mod mdios_dinr30;
/**MDIOS_DINR31 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DINR31)

For information about available fields see [`mod@mdios_dinr31`]
module*/
pub type MDIOS_DINR31 = crate::Reg<mdios_dinr31::MDIOS_DINR31rs>;
///MDIOS input data register
pub mod mdios_dinr31;
/**MDIOS_DOUTR0 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR0)

For information about available fields see [`mod@mdios_doutr0`]
module*/
pub type MDIOS_DOUTR0 = crate::Reg<mdios_doutr0::MDIOS_DOUTR0rs>;
///MDIOS input data register
pub mod mdios_doutr0;
/**MDIOS_DOUTR1 (r) register accessor: MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR1)

For information about available fields see [`mod@mdios_doutr1`]
module*/
pub type MDIOS_DOUTR1 = crate::Reg<mdios_doutr1::MDIOS_DOUTR1rs>;
///MDIOS input data register
pub mod mdios_doutr1;
/**MDIOS_DOUTR2 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR2)

For information about available fields see [`mod@mdios_doutr2`]
module*/
pub type MDIOS_DOUTR2 = crate::Reg<mdios_doutr2::MDIOS_DOUTR2rs>;
///MDIOS output data register
pub mod mdios_doutr2;
/**MDIOS_DOUTR3 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR3)

For information about available fields see [`mod@mdios_doutr3`]
module*/
pub type MDIOS_DOUTR3 = crate::Reg<mdios_doutr3::MDIOS_DOUTR3rs>;
///MDIOS output data register
pub mod mdios_doutr3;
/**MDIOS_DOUTR4 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR4)

For information about available fields see [`mod@mdios_doutr4`]
module*/
pub type MDIOS_DOUTR4 = crate::Reg<mdios_doutr4::MDIOS_DOUTR4rs>;
///MDIOS output data register
pub mod mdios_doutr4;
/**MDIOS_DOUTR5 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR5)

For information about available fields see [`mod@mdios_doutr5`]
module*/
pub type MDIOS_DOUTR5 = crate::Reg<mdios_doutr5::MDIOS_DOUTR5rs>;
///MDIOS output data register
pub mod mdios_doutr5;
/**MDIOS_DOUTR6 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR6)

For information about available fields see [`mod@mdios_doutr6`]
module*/
pub type MDIOS_DOUTR6 = crate::Reg<mdios_doutr6::MDIOS_DOUTR6rs>;
///MDIOS output data register
pub mod mdios_doutr6;
/**MDIOS_DOUTR7 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR7)

For information about available fields see [`mod@mdios_doutr7`]
module*/
pub type MDIOS_DOUTR7 = crate::Reg<mdios_doutr7::MDIOS_DOUTR7rs>;
///MDIOS output data register
pub mod mdios_doutr7;
/**MDIOS_DOUTR8 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR8)

For information about available fields see [`mod@mdios_doutr8`]
module*/
pub type MDIOS_DOUTR8 = crate::Reg<mdios_doutr8::MDIOS_DOUTR8rs>;
///MDIOS output data register
pub mod mdios_doutr8;
/**MDIOS_DOUTR9 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR9)

For information about available fields see [`mod@mdios_doutr9`]
module*/
pub type MDIOS_DOUTR9 = crate::Reg<mdios_doutr9::MDIOS_DOUTR9rs>;
///MDIOS output data register
pub mod mdios_doutr9;
/**MDIOS_DOUTR10 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR10)

For information about available fields see [`mod@mdios_doutr10`]
module*/
pub type MDIOS_DOUTR10 = crate::Reg<mdios_doutr10::MDIOS_DOUTR10rs>;
///MDIOS output data register
pub mod mdios_doutr10;
/**MDIOS_DOUTR11 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR11)

For information about available fields see [`mod@mdios_doutr11`]
module*/
pub type MDIOS_DOUTR11 = crate::Reg<mdios_doutr11::MDIOS_DOUTR11rs>;
///MDIOS output data register
pub mod mdios_doutr11;
/**MDIOS_DOUTR12 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR12)

For information about available fields see [`mod@mdios_doutr12`]
module*/
pub type MDIOS_DOUTR12 = crate::Reg<mdios_doutr12::MDIOS_DOUTR12rs>;
///MDIOS output data register
pub mod mdios_doutr12;
/**MDIOS_DOUTR13 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR13)

For information about available fields see [`mod@mdios_doutr13`]
module*/
pub type MDIOS_DOUTR13 = crate::Reg<mdios_doutr13::MDIOS_DOUTR13rs>;
///MDIOS output data register
pub mod mdios_doutr13;
/**MDIOS_DOUTR14 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR14)

For information about available fields see [`mod@mdios_doutr14`]
module*/
pub type MDIOS_DOUTR14 = crate::Reg<mdios_doutr14::MDIOS_DOUTR14rs>;
///MDIOS output data register
pub mod mdios_doutr14;
/**MDIOS_DOUTR15 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR15)

For information about available fields see [`mod@mdios_doutr15`]
module*/
pub type MDIOS_DOUTR15 = crate::Reg<mdios_doutr15::MDIOS_DOUTR15rs>;
///MDIOS output data register
pub mod mdios_doutr15;
/**MDIOS_DOUTR16 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR16)

For information about available fields see [`mod@mdios_doutr16`]
module*/
pub type MDIOS_DOUTR16 = crate::Reg<mdios_doutr16::MDIOS_DOUTR16rs>;
///MDIOS output data register
pub mod mdios_doutr16;
/**MDIOS_DOUTR17 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR17)

For information about available fields see [`mod@mdios_doutr17`]
module*/
pub type MDIOS_DOUTR17 = crate::Reg<mdios_doutr17::MDIOS_DOUTR17rs>;
///MDIOS output data register
pub mod mdios_doutr17;
/**MDIOS_DOUTR18 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR18)

For information about available fields see [`mod@mdios_doutr18`]
module*/
pub type MDIOS_DOUTR18 = crate::Reg<mdios_doutr18::MDIOS_DOUTR18rs>;
///MDIOS output data register
pub mod mdios_doutr18;
/**MDIOS_DOUTR19 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR19)

For information about available fields see [`mod@mdios_doutr19`]
module*/
pub type MDIOS_DOUTR19 = crate::Reg<mdios_doutr19::MDIOS_DOUTR19rs>;
///MDIOS output data register
pub mod mdios_doutr19;
/**MDIOS_DOUTR20 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR20)

For information about available fields see [`mod@mdios_doutr20`]
module*/
pub type MDIOS_DOUTR20 = crate::Reg<mdios_doutr20::MDIOS_DOUTR20rs>;
///MDIOS output data register
pub mod mdios_doutr20;
/**MDIOS_DOUTR21 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR21)

For information about available fields see [`mod@mdios_doutr21`]
module*/
pub type MDIOS_DOUTR21 = crate::Reg<mdios_doutr21::MDIOS_DOUTR21rs>;
///MDIOS output data register
pub mod mdios_doutr21;
/**MDIOS_DOUTR22 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR22)

For information about available fields see [`mod@mdios_doutr22`]
module*/
pub type MDIOS_DOUTR22 = crate::Reg<mdios_doutr22::MDIOS_DOUTR22rs>;
///MDIOS output data register
pub mod mdios_doutr22;
/**MDIOS_DOUTR23 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR23)

For information about available fields see [`mod@mdios_doutr23`]
module*/
pub type MDIOS_DOUTR23 = crate::Reg<mdios_doutr23::MDIOS_DOUTR23rs>;
///MDIOS output data register
pub mod mdios_doutr23;
/**MDIOS_DOUTR24 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR24)

For information about available fields see [`mod@mdios_doutr24`]
module*/
pub type MDIOS_DOUTR24 = crate::Reg<mdios_doutr24::MDIOS_DOUTR24rs>;
///MDIOS output data register
pub mod mdios_doutr24;
/**MDIOS_DOUTR25 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR25)

For information about available fields see [`mod@mdios_doutr25`]
module*/
pub type MDIOS_DOUTR25 = crate::Reg<mdios_doutr25::MDIOS_DOUTR25rs>;
///MDIOS output data register
pub mod mdios_doutr25;
/**MDIOS_DOUTR26 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR26)

For information about available fields see [`mod@mdios_doutr26`]
module*/
pub type MDIOS_DOUTR26 = crate::Reg<mdios_doutr26::MDIOS_DOUTR26rs>;
///MDIOS output data register
pub mod mdios_doutr26;
/**MDIOS_DOUTR27 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR27)

For information about available fields see [`mod@mdios_doutr27`]
module*/
pub type MDIOS_DOUTR27 = crate::Reg<mdios_doutr27::MDIOS_DOUTR27rs>;
///MDIOS output data register
pub mod mdios_doutr27;
/**MDIOS_DOUTR28 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR28)

For information about available fields see [`mod@mdios_doutr28`]
module*/
pub type MDIOS_DOUTR28 = crate::Reg<mdios_doutr28::MDIOS_DOUTR28rs>;
///MDIOS output data register
pub mod mdios_doutr28;
/**MDIOS_DOUTR29 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR29)

For information about available fields see [`mod@mdios_doutr29`]
module*/
pub type MDIOS_DOUTR29 = crate::Reg<mdios_doutr29::MDIOS_DOUTR29rs>;
///MDIOS output data register
pub mod mdios_doutr29;
/**MDIOS_DOUTR30 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR30)

For information about available fields see [`mod@mdios_doutr30`]
module*/
pub type MDIOS_DOUTR30 = crate::Reg<mdios_doutr30::MDIOS_DOUTR30rs>;
///MDIOS output data register
pub mod mdios_doutr30;
/**MDIOS_DOUTR31 (r) register accessor: MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR31)

For information about available fields see [`mod@mdios_doutr31`]
module*/
pub type MDIOS_DOUTR31 = crate::Reg<mdios_doutr31::MDIOS_DOUTR31rs>;
///MDIOS output data register
pub mod mdios_doutr31;
/**MDIOS_HWCFGR (r) register accessor: MDIOS HW configuration register

You can [`read`](crate::Reg::read) this register and get [`mdios_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_HWCFGR)

For information about available fields see [`mod@mdios_hwcfgr`]
module*/
pub type MDIOS_HWCFGR = crate::Reg<mdios_hwcfgr::MDIOS_HWCFGRrs>;
///MDIOS HW configuration register
pub mod mdios_hwcfgr;
/**MDIOS_VERR (r) register accessor: MDIOS version register

You can [`read`](crate::Reg::read) this register and get [`mdios_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_VERR)

For information about available fields see [`mod@mdios_verr`]
module*/
pub type MDIOS_VERR = crate::Reg<mdios_verr::MDIOS_VERRrs>;
///MDIOS version register
pub mod mdios_verr;
/**MDIOS_IPIDR (r) register accessor: MDIOS identification register

You can [`read`](crate::Reg::read) this register and get [`mdios_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_IPIDR)

For information about available fields see [`mod@mdios_ipidr`]
module*/
pub type MDIOS_IPIDR = crate::Reg<mdios_ipidr::MDIOS_IPIDRrs>;
///MDIOS identification register
pub mod mdios_ipidr;
/**MDIOS_SIDR (r) register accessor: MDIOS size identification register

You can [`read`](crate::Reg::read) this register and get [`mdios_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_SIDR)

For information about available fields see [`mod@mdios_sidr`]
module*/
pub type MDIOS_SIDR = crate::Reg<mdios_sidr::MDIOS_SIDRrs>;
///MDIOS size identification register
pub mod mdios_sidr;
