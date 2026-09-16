#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    lckvtr1: LCKVTR1,
    lckvtr2: LCKVTR2,
    _reserved3: [u8; 0xe8],
    vctr0: VCTR0,
    vctr1: VCTR1,
    vctr2: VCTR2,
    vctr3: VCTR3,
    vctr4: VCTR4,
    vctr5: VCTR5,
    vctr6: VCTR6,
    vctr7: VCTR7,
    vctr8: VCTR8,
    vctr9: VCTR9,
    vctr10: VCTR10,
    vctr11: VCTR11,
    vctr12: VCTR12,
    vctr13: VCTR13,
    vctr14: VCTR14,
    vctr15: VCTR15,
    vctr16: VCTR16,
    vctr17: VCTR17,
    vctr18: VCTR18,
    vctr19: VCTR19,
    vctr20: VCTR20,
    vctr21: VCTR21,
    vctr22: VCTR22,
    vctr23: VCTR23,
    vctr24: VCTR24,
    vctr25: VCTR25,
    vctr26: VCTR26,
    vctr27: VCTR27,
    vctr28: VCTR28,
    vctr29: VCTR29,
    vctr30: VCTR30,
    vctr31: VCTR31,
    vctr32: VCTR32,
    vctr33: VCTR33,
    vctr34: VCTR34,
    vctr35: VCTR35,
    vctr36: VCTR36,
    vctr37: VCTR37,
    vctr38: VCTR38,
    vctr39: VCTR39,
    vctr40: VCTR40,
    vctr41: VCTR41,
    vctr42: VCTR42,
    vctr43: VCTR43,
    vctr44: VCTR44,
    vctr45: VCTR45,
    vctr46: VCTR46,
    vctr47: VCTR47,
    vctr48: VCTR48,
    vctr49: VCTR49,
    vctr50: VCTR50,
    vctr51: VCTR51,
    vctr52: VCTR52,
    vctr53: VCTR53,
    vctr54: VCTR54,
    vctr55: VCTR55,
    vctr56: VCTR56,
    vctr57: VCTR57,
    vctr58: VCTR58,
    vctr59: VCTR59,
    vctr60: VCTR60,
    vctr61: VCTR61,
    vctr62: VCTR62,
    vctr63: VCTR63,
}
impl RegisterBlock {
    ///0x00 - MPCBB control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - MPCBB control register
    #[inline(always)]
    pub const fn lckvtr1(&self) -> &LCKVTR1 {
        &self.lckvtr1
    }
    ///0x14 - MPCBB control register
    #[inline(always)]
    pub const fn lckvtr2(&self) -> &LCKVTR2 {
        &self.lckvtr2
    }
    ///0x100 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr0(&self) -> &VCTR0 {
        &self.vctr0
    }
    ///0x104 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr1(&self) -> &VCTR1 {
        &self.vctr1
    }
    ///0x108 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr2(&self) -> &VCTR2 {
        &self.vctr2
    }
    ///0x10c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr3(&self) -> &VCTR3 {
        &self.vctr3
    }
    ///0x110 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr4(&self) -> &VCTR4 {
        &self.vctr4
    }
    ///0x114 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr5(&self) -> &VCTR5 {
        &self.vctr5
    }
    ///0x118 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr6(&self) -> &VCTR6 {
        &self.vctr6
    }
    ///0x11c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr7(&self) -> &VCTR7 {
        &self.vctr7
    }
    ///0x120 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr8(&self) -> &VCTR8 {
        &self.vctr8
    }
    ///0x124 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr9(&self) -> &VCTR9 {
        &self.vctr9
    }
    ///0x128 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr10(&self) -> &VCTR10 {
        &self.vctr10
    }
    ///0x12c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr11(&self) -> &VCTR11 {
        &self.vctr11
    }
    ///0x130 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr12(&self) -> &VCTR12 {
        &self.vctr12
    }
    ///0x134 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr13(&self) -> &VCTR13 {
        &self.vctr13
    }
    ///0x138 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr14(&self) -> &VCTR14 {
        &self.vctr14
    }
    ///0x13c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr15(&self) -> &VCTR15 {
        &self.vctr15
    }
    ///0x140 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr16(&self) -> &VCTR16 {
        &self.vctr16
    }
    ///0x144 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr17(&self) -> &VCTR17 {
        &self.vctr17
    }
    ///0x148 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr18(&self) -> &VCTR18 {
        &self.vctr18
    }
    ///0x14c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr19(&self) -> &VCTR19 {
        &self.vctr19
    }
    ///0x150 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr20(&self) -> &VCTR20 {
        &self.vctr20
    }
    ///0x154 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr21(&self) -> &VCTR21 {
        &self.vctr21
    }
    ///0x158 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr22(&self) -> &VCTR22 {
        &self.vctr22
    }
    ///0x15c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr23(&self) -> &VCTR23 {
        &self.vctr23
    }
    ///0x160 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr24(&self) -> &VCTR24 {
        &self.vctr24
    }
    ///0x164 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr25(&self) -> &VCTR25 {
        &self.vctr25
    }
    ///0x168 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr26(&self) -> &VCTR26 {
        &self.vctr26
    }
    ///0x16c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr27(&self) -> &VCTR27 {
        &self.vctr27
    }
    ///0x170 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr28(&self) -> &VCTR28 {
        &self.vctr28
    }
    ///0x174 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr29(&self) -> &VCTR29 {
        &self.vctr29
    }
    ///0x178 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr30(&self) -> &VCTR30 {
        &self.vctr30
    }
    ///0x17c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr31(&self) -> &VCTR31 {
        &self.vctr31
    }
    ///0x180 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr32(&self) -> &VCTR32 {
        &self.vctr32
    }
    ///0x184 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr33(&self) -> &VCTR33 {
        &self.vctr33
    }
    ///0x188 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr34(&self) -> &VCTR34 {
        &self.vctr34
    }
    ///0x18c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr35(&self) -> &VCTR35 {
        &self.vctr35
    }
    ///0x190 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr36(&self) -> &VCTR36 {
        &self.vctr36
    }
    ///0x194 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr37(&self) -> &VCTR37 {
        &self.vctr37
    }
    ///0x198 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr38(&self) -> &VCTR38 {
        &self.vctr38
    }
    ///0x19c - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr39(&self) -> &VCTR39 {
        &self.vctr39
    }
    ///0x1a0 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr40(&self) -> &VCTR40 {
        &self.vctr40
    }
    ///0x1a4 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr41(&self) -> &VCTR41 {
        &self.vctr41
    }
    ///0x1a8 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr42(&self) -> &VCTR42 {
        &self.vctr42
    }
    ///0x1ac - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr43(&self) -> &VCTR43 {
        &self.vctr43
    }
    ///0x1b0 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr44(&self) -> &VCTR44 {
        &self.vctr44
    }
    ///0x1b4 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr45(&self) -> &VCTR45 {
        &self.vctr45
    }
    ///0x1b8 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr46(&self) -> &VCTR46 {
        &self.vctr46
    }
    ///0x1bc - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr47(&self) -> &VCTR47 {
        &self.vctr47
    }
    ///0x1c0 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr48(&self) -> &VCTR48 {
        &self.vctr48
    }
    ///0x1c4 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr49(&self) -> &VCTR49 {
        &self.vctr49
    }
    ///0x1c8 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr50(&self) -> &VCTR50 {
        &self.vctr50
    }
    ///0x1cc - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr51(&self) -> &VCTR51 {
        &self.vctr51
    }
    ///0x1d0 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr52(&self) -> &VCTR52 {
        &self.vctr52
    }
    ///0x1d4 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr53(&self) -> &VCTR53 {
        &self.vctr53
    }
    ///0x1d8 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr54(&self) -> &VCTR54 {
        &self.vctr54
    }
    ///0x1dc - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr55(&self) -> &VCTR55 {
        &self.vctr55
    }
    ///0x1e0 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr56(&self) -> &VCTR56 {
        &self.vctr56
    }
    ///0x1e4 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr57(&self) -> &VCTR57 {
        &self.vctr57
    }
    ///0x1e8 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr58(&self) -> &VCTR58 {
        &self.vctr58
    }
    ///0x1ec - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr59(&self) -> &VCTR59 {
        &self.vctr59
    }
    ///0x1f0 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr60(&self) -> &VCTR60 {
        &self.vctr60
    }
    ///0x1f4 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr61(&self) -> &VCTR61 {
        &self.vctr61
    }
    ///0x1f8 - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr62(&self) -> &VCTR62 {
        &self.vctr62
    }
    ///0x1fc - MPCBBx vector register
    #[inline(always)]
    pub const fn vctr63(&self) -> &VCTR63 {
        &self.vctr63
    }
}
/**CR (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MPCBB control register
pub mod cr;
/**LCKVTR1 (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`lckvtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckvtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:LCKVTR1)

For information about available fields see [`mod@lckvtr1`] module*/
pub type LCKVTR1 = crate::Reg<lckvtr1::LCKVTR1rs>;
///MPCBB control register
pub mod lckvtr1;
/**LCKVTR2 (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`lckvtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckvtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:LCKVTR2)

For information about available fields see [`mod@lckvtr2`] module*/
pub type LCKVTR2 = crate::Reg<lckvtr2::LCKVTR2rs>;
///MPCBB control register
pub mod lckvtr2;
/**VCTR0 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR0)

For information about available fields see [`mod@vctr0`] module*/
pub type VCTR0 = crate::Reg<vctr0::VCTR0rs>;
///MPCBBx vector register
pub mod vctr0;
/**VCTR1 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR1)

For information about available fields see [`mod@vctr1`] module*/
pub type VCTR1 = crate::Reg<vctr1::VCTR1rs>;
///MPCBBx vector register
pub mod vctr1;
/**VCTR2 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR2)

For information about available fields see [`mod@vctr2`] module*/
pub type VCTR2 = crate::Reg<vctr2::VCTR2rs>;
///MPCBBx vector register
pub mod vctr2;
/**VCTR3 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR3)

For information about available fields see [`mod@vctr3`] module*/
pub type VCTR3 = crate::Reg<vctr3::VCTR3rs>;
///MPCBBx vector register
pub mod vctr3;
/**VCTR4 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR4)

For information about available fields see [`mod@vctr4`] module*/
pub type VCTR4 = crate::Reg<vctr4::VCTR4rs>;
///MPCBBx vector register
pub mod vctr4;
/**VCTR5 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR5)

For information about available fields see [`mod@vctr5`] module*/
pub type VCTR5 = crate::Reg<vctr5::VCTR5rs>;
///MPCBBx vector register
pub mod vctr5;
/**VCTR6 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR6)

For information about available fields see [`mod@vctr6`] module*/
pub type VCTR6 = crate::Reg<vctr6::VCTR6rs>;
///MPCBBx vector register
pub mod vctr6;
/**VCTR7 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR7)

For information about available fields see [`mod@vctr7`] module*/
pub type VCTR7 = crate::Reg<vctr7::VCTR7rs>;
///MPCBBx vector register
pub mod vctr7;
/**VCTR8 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR8)

For information about available fields see [`mod@vctr8`] module*/
pub type VCTR8 = crate::Reg<vctr8::VCTR8rs>;
///MPCBBx vector register
pub mod vctr8;
/**VCTR9 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR9)

For information about available fields see [`mod@vctr9`] module*/
pub type VCTR9 = crate::Reg<vctr9::VCTR9rs>;
///MPCBBx vector register
pub mod vctr9;
/**VCTR10 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR10)

For information about available fields see [`mod@vctr10`] module*/
pub type VCTR10 = crate::Reg<vctr10::VCTR10rs>;
///MPCBBx vector register
pub mod vctr10;
/**VCTR11 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR11)

For information about available fields see [`mod@vctr11`] module*/
pub type VCTR11 = crate::Reg<vctr11::VCTR11rs>;
///MPCBBx vector register
pub mod vctr11;
/**VCTR12 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR12)

For information about available fields see [`mod@vctr12`] module*/
pub type VCTR12 = crate::Reg<vctr12::VCTR12rs>;
///MPCBBx vector register
pub mod vctr12;
/**VCTR13 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR13)

For information about available fields see [`mod@vctr13`] module*/
pub type VCTR13 = crate::Reg<vctr13::VCTR13rs>;
///MPCBBx vector register
pub mod vctr13;
/**VCTR14 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR14)

For information about available fields see [`mod@vctr14`] module*/
pub type VCTR14 = crate::Reg<vctr14::VCTR14rs>;
///MPCBBx vector register
pub mod vctr14;
/**VCTR15 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR15)

For information about available fields see [`mod@vctr15`] module*/
pub type VCTR15 = crate::Reg<vctr15::VCTR15rs>;
///MPCBBx vector register
pub mod vctr15;
/**VCTR16 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR16)

For information about available fields see [`mod@vctr16`] module*/
pub type VCTR16 = crate::Reg<vctr16::VCTR16rs>;
///MPCBBx vector register
pub mod vctr16;
/**VCTR17 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR17)

For information about available fields see [`mod@vctr17`] module*/
pub type VCTR17 = crate::Reg<vctr17::VCTR17rs>;
///MPCBBx vector register
pub mod vctr17;
/**VCTR18 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR18)

For information about available fields see [`mod@vctr18`] module*/
pub type VCTR18 = crate::Reg<vctr18::VCTR18rs>;
///MPCBBx vector register
pub mod vctr18;
/**VCTR19 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR19)

For information about available fields see [`mod@vctr19`] module*/
pub type VCTR19 = crate::Reg<vctr19::VCTR19rs>;
///MPCBBx vector register
pub mod vctr19;
/**VCTR20 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR20)

For information about available fields see [`mod@vctr20`] module*/
pub type VCTR20 = crate::Reg<vctr20::VCTR20rs>;
///MPCBBx vector register
pub mod vctr20;
/**VCTR21 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR21)

For information about available fields see [`mod@vctr21`] module*/
pub type VCTR21 = crate::Reg<vctr21::VCTR21rs>;
///MPCBBx vector register
pub mod vctr21;
/**VCTR22 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR22)

For information about available fields see [`mod@vctr22`] module*/
pub type VCTR22 = crate::Reg<vctr22::VCTR22rs>;
///MPCBBx vector register
pub mod vctr22;
/**VCTR23 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR23)

For information about available fields see [`mod@vctr23`] module*/
pub type VCTR23 = crate::Reg<vctr23::VCTR23rs>;
///MPCBBx vector register
pub mod vctr23;
/**VCTR24 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR24)

For information about available fields see [`mod@vctr24`] module*/
pub type VCTR24 = crate::Reg<vctr24::VCTR24rs>;
///MPCBBx vector register
pub mod vctr24;
/**VCTR25 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR25)

For information about available fields see [`mod@vctr25`] module*/
pub type VCTR25 = crate::Reg<vctr25::VCTR25rs>;
///MPCBBx vector register
pub mod vctr25;
/**VCTR26 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR26)

For information about available fields see [`mod@vctr26`] module*/
pub type VCTR26 = crate::Reg<vctr26::VCTR26rs>;
///MPCBBx vector register
pub mod vctr26;
/**VCTR27 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR27)

For information about available fields see [`mod@vctr27`] module*/
pub type VCTR27 = crate::Reg<vctr27::VCTR27rs>;
///MPCBBx vector register
pub mod vctr27;
/**VCTR28 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR28)

For information about available fields see [`mod@vctr28`] module*/
pub type VCTR28 = crate::Reg<vctr28::VCTR28rs>;
///MPCBBx vector register
pub mod vctr28;
/**VCTR29 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR29)

For information about available fields see [`mod@vctr29`] module*/
pub type VCTR29 = crate::Reg<vctr29::VCTR29rs>;
///MPCBBx vector register
pub mod vctr29;
/**VCTR30 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR30)

For information about available fields see [`mod@vctr30`] module*/
pub type VCTR30 = crate::Reg<vctr30::VCTR30rs>;
///MPCBBx vector register
pub mod vctr30;
/**VCTR31 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR31)

For information about available fields see [`mod@vctr31`] module*/
pub type VCTR31 = crate::Reg<vctr31::VCTR31rs>;
///MPCBBx vector register
pub mod vctr31;
/**VCTR32 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR32)

For information about available fields see [`mod@vctr32`] module*/
pub type VCTR32 = crate::Reg<vctr32::VCTR32rs>;
///MPCBBx vector register
pub mod vctr32;
/**VCTR33 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR33)

For information about available fields see [`mod@vctr33`] module*/
pub type VCTR33 = crate::Reg<vctr33::VCTR33rs>;
///MPCBBx vector register
pub mod vctr33;
/**VCTR34 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR34)

For information about available fields see [`mod@vctr34`] module*/
pub type VCTR34 = crate::Reg<vctr34::VCTR34rs>;
///MPCBBx vector register
pub mod vctr34;
/**VCTR35 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR35)

For information about available fields see [`mod@vctr35`] module*/
pub type VCTR35 = crate::Reg<vctr35::VCTR35rs>;
///MPCBBx vector register
pub mod vctr35;
/**VCTR36 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR36)

For information about available fields see [`mod@vctr36`] module*/
pub type VCTR36 = crate::Reg<vctr36::VCTR36rs>;
///MPCBBx vector register
pub mod vctr36;
/**VCTR37 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR37)

For information about available fields see [`mod@vctr37`] module*/
pub type VCTR37 = crate::Reg<vctr37::VCTR37rs>;
///MPCBBx vector register
pub mod vctr37;
/**VCTR38 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR38)

For information about available fields see [`mod@vctr38`] module*/
pub type VCTR38 = crate::Reg<vctr38::VCTR38rs>;
///MPCBBx vector register
pub mod vctr38;
/**VCTR39 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR39)

For information about available fields see [`mod@vctr39`] module*/
pub type VCTR39 = crate::Reg<vctr39::VCTR39rs>;
///MPCBBx vector register
pub mod vctr39;
/**VCTR40 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR40)

For information about available fields see [`mod@vctr40`] module*/
pub type VCTR40 = crate::Reg<vctr40::VCTR40rs>;
///MPCBBx vector register
pub mod vctr40;
/**VCTR41 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR41)

For information about available fields see [`mod@vctr41`] module*/
pub type VCTR41 = crate::Reg<vctr41::VCTR41rs>;
///MPCBBx vector register
pub mod vctr41;
/**VCTR42 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR42)

For information about available fields see [`mod@vctr42`] module*/
pub type VCTR42 = crate::Reg<vctr42::VCTR42rs>;
///MPCBBx vector register
pub mod vctr42;
/**VCTR43 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR43)

For information about available fields see [`mod@vctr43`] module*/
pub type VCTR43 = crate::Reg<vctr43::VCTR43rs>;
///MPCBBx vector register
pub mod vctr43;
/**VCTR44 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR44)

For information about available fields see [`mod@vctr44`] module*/
pub type VCTR44 = crate::Reg<vctr44::VCTR44rs>;
///MPCBBx vector register
pub mod vctr44;
/**VCTR45 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR45)

For information about available fields see [`mod@vctr45`] module*/
pub type VCTR45 = crate::Reg<vctr45::VCTR45rs>;
///MPCBBx vector register
pub mod vctr45;
/**VCTR46 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR46)

For information about available fields see [`mod@vctr46`] module*/
pub type VCTR46 = crate::Reg<vctr46::VCTR46rs>;
///MPCBBx vector register
pub mod vctr46;
/**VCTR47 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR47)

For information about available fields see [`mod@vctr47`] module*/
pub type VCTR47 = crate::Reg<vctr47::VCTR47rs>;
///MPCBBx vector register
pub mod vctr47;
/**VCTR48 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR48)

For information about available fields see [`mod@vctr48`] module*/
pub type VCTR48 = crate::Reg<vctr48::VCTR48rs>;
///MPCBBx vector register
pub mod vctr48;
/**VCTR49 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR49)

For information about available fields see [`mod@vctr49`] module*/
pub type VCTR49 = crate::Reg<vctr49::VCTR49rs>;
///MPCBBx vector register
pub mod vctr49;
/**VCTR50 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR50)

For information about available fields see [`mod@vctr50`] module*/
pub type VCTR50 = crate::Reg<vctr50::VCTR50rs>;
///MPCBBx vector register
pub mod vctr50;
/**VCTR51 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR51)

For information about available fields see [`mod@vctr51`] module*/
pub type VCTR51 = crate::Reg<vctr51::VCTR51rs>;
///MPCBBx vector register
pub mod vctr51;
/**VCTR52 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR52)

For information about available fields see [`mod@vctr52`] module*/
pub type VCTR52 = crate::Reg<vctr52::VCTR52rs>;
///MPCBBx vector register
pub mod vctr52;
/**VCTR53 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR53)

For information about available fields see [`mod@vctr53`] module*/
pub type VCTR53 = crate::Reg<vctr53::VCTR53rs>;
///MPCBBx vector register
pub mod vctr53;
/**VCTR54 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR54)

For information about available fields see [`mod@vctr54`] module*/
pub type VCTR54 = crate::Reg<vctr54::VCTR54rs>;
///MPCBBx vector register
pub mod vctr54;
/**VCTR55 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR55)

For information about available fields see [`mod@vctr55`] module*/
pub type VCTR55 = crate::Reg<vctr55::VCTR55rs>;
///MPCBBx vector register
pub mod vctr55;
/**VCTR56 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR56)

For information about available fields see [`mod@vctr56`] module*/
pub type VCTR56 = crate::Reg<vctr56::VCTR56rs>;
///MPCBBx vector register
pub mod vctr56;
/**VCTR57 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR57)

For information about available fields see [`mod@vctr57`] module*/
pub type VCTR57 = crate::Reg<vctr57::VCTR57rs>;
///MPCBBx vector register
pub mod vctr57;
/**VCTR58 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR58)

For information about available fields see [`mod@vctr58`] module*/
pub type VCTR58 = crate::Reg<vctr58::VCTR58rs>;
///MPCBBx vector register
pub mod vctr58;
/**VCTR59 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR59)

For information about available fields see [`mod@vctr59`] module*/
pub type VCTR59 = crate::Reg<vctr59::VCTR59rs>;
///MPCBBx vector register
pub mod vctr59;
/**VCTR60 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR60)

For information about available fields see [`mod@vctr60`] module*/
pub type VCTR60 = crate::Reg<vctr60::VCTR60rs>;
///MPCBBx vector register
pub mod vctr60;
/**VCTR61 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR61)

For information about available fields see [`mod@vctr61`] module*/
pub type VCTR61 = crate::Reg<vctr61::VCTR61rs>;
///MPCBBx vector register
pub mod vctr61;
/**VCTR62 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR62)

For information about available fields see [`mod@vctr62`] module*/
pub type VCTR62 = crate::Reg<vctr62::VCTR62rs>;
///MPCBBx vector register
pub mod vctr62;
/**VCTR63 (rw) register accessor: MPCBBx vector register

You can [`read`](crate::Reg::read) this register and get [`vctr63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vctr63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_MPCBB1:VCTR63)

For information about available fields see [`mod@vctr63`] module*/
pub type VCTR63 = crate::Reg<vctr63::VCTR63rs>;
///MPCBBx vector register
pub mod vctr63;
