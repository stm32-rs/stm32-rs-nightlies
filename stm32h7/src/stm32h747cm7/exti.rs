#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    d3pmr1: D3PMR1,
    d3pcr1l: D3PCR1L,
    d3pcr1h: D3PCR1H,
    _reserved6: [u8; 0x08],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    d3pmr2: D3PMR2,
    d3pcr2l: D3PCR2L,
    d3pcr2h: D3PCR2H,
    _reserved12: [u8; 0x08],
    rtsr3: RTSR3,
    ftsr3: FTSR3,
    swier3: SWIER3,
    d3pmr3: D3PMR3,
    _reserved16: [u8; 0x04],
    d3pcr3h: D3PCR3H,
    _reserved17: [u8; 0x28],
    c1imr1: C1IMR1,
    c1emr1: C1EMR1,
    c1pr1: C1PR1,
    _reserved20: [u8; 0x04],
    c1imr2: C1IMR2,
    c1emr2: C1EMR2,
    c1pr2: C1PR2,
    _reserved23: [u8; 0x04],
    c1imr3: C1IMR3,
    c1emr3: C1EMR3,
    c1pr3: C1PR3,
    _reserved26: [u8; 0x14],
    c2imr1: C2IMR1,
    c2emr1: C2EMR1,
    c2pr1: C2PR1,
    _reserved29: [u8; 0x04],
    c2imr2: C2IMR2,
    c2emr2: C2EMR2,
    c2pr2: C2PR2,
    _reserved32: [u8; 0x04],
    c2imr3: C2IMR3,
    c2emr3: C2EMR3,
    c2pr3: C2PR3,
}
impl RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    #[doc = "0x04 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    #[doc = "0x08 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    #[doc = "0x0c - EXTI D3 pending mask register"]
    #[inline(always)]
    pub const fn d3pmr1(&self) -> &D3PMR1 {
        &self.d3pmr1
    }
    #[doc = "0x10 - EXTI D3 pending clear selection register low"]
    #[inline(always)]
    pub const fn d3pcr1l(&self) -> &D3PCR1L {
        &self.d3pcr1l
    }
    #[doc = "0x14 - EXTI D3 pending clear selection register high"]
    #[inline(always)]
    pub const fn d3pcr1h(&self) -> &D3PCR1H {
        &self.d3pcr1h
    }
    #[doc = "0x20 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    #[doc = "0x24 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    #[doc = "0x28 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    #[doc = "0x2c - EXTI D3 pending mask register"]
    #[inline(always)]
    pub const fn d3pmr2(&self) -> &D3PMR2 {
        &self.d3pmr2
    }
    #[doc = "0x30 - EXTI D3 pending clear selection register low"]
    #[inline(always)]
    pub const fn d3pcr2l(&self) -> &D3PCR2L {
        &self.d3pcr2l
    }
    #[doc = "0x34 - EXTI D3 pending clear selection register high"]
    #[inline(always)]
    pub const fn d3pcr2h(&self) -> &D3PCR2H {
        &self.d3pcr2h
    }
    #[doc = "0x40 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr3(&self) -> &RTSR3 {
        &self.rtsr3
    }
    #[doc = "0x44 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr3(&self) -> &FTSR3 {
        &self.ftsr3
    }
    #[doc = "0x48 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier3(&self) -> &SWIER3 {
        &self.swier3
    }
    #[doc = "0x4c - EXTI D3 pending mask register"]
    #[inline(always)]
    pub const fn d3pmr3(&self) -> &D3PMR3 {
        &self.d3pmr3
    }
    #[doc = "0x54 - EXTI D3 pending clear selection register high"]
    #[inline(always)]
    pub const fn d3pcr3h(&self) -> &D3PCR3H {
        &self.d3pcr3h
    }
    #[doc = "0x80 - EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn c1imr1(&self) -> &C1IMR1 {
        &self.c1imr1
    }
    #[doc = "0x84 - EXTI event mask register"]
    #[inline(always)]
    pub const fn c1emr1(&self) -> &C1EMR1 {
        &self.c1emr1
    }
    #[doc = "0x88 - EXTI pending register"]
    #[inline(always)]
    pub const fn c1pr1(&self) -> &C1PR1 {
        &self.c1pr1
    }
    #[doc = "0x90 - EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn c1imr2(&self) -> &C1IMR2 {
        &self.c1imr2
    }
    #[doc = "0x94 - EXTI event mask register"]
    #[inline(always)]
    pub const fn c1emr2(&self) -> &C1EMR2 {
        &self.c1emr2
    }
    #[doc = "0x98 - EXTI pending register"]
    #[inline(always)]
    pub const fn c1pr2(&self) -> &C1PR2 {
        &self.c1pr2
    }
    #[doc = "0xa0 - EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn c1imr3(&self) -> &C1IMR3 {
        &self.c1imr3
    }
    #[doc = "0xa4 - EXTI event mask register"]
    #[inline(always)]
    pub const fn c1emr3(&self) -> &C1EMR3 {
        &self.c1emr3
    }
    #[doc = "0xa8 - EXTI pending register"]
    #[inline(always)]
    pub const fn c1pr3(&self) -> &C1PR3 {
        &self.c1pr3
    }
    #[doc = "0xc0 - CPU2 EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    #[doc = "0xc4 - CPU2 EXTI event mask register"]
    #[inline(always)]
    pub const fn c2emr1(&self) -> &C2EMR1 {
        &self.c2emr1
    }
    #[doc = "0xc8 - CPU2 EXTI pending register"]
    #[inline(always)]
    pub const fn c2pr1(&self) -> &C2PR1 {
        &self.c2pr1
    }
    #[doc = "0xd0 - CPU2 EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    #[doc = "0xd4 - CPU2 EXTI event mask register"]
    #[inline(always)]
    pub const fn c2emr2(&self) -> &C2EMR2 {
        &self.c2emr2
    }
    #[doc = "0xd8 - CPU2 EXTI pending register"]
    #[inline(always)]
    pub const fn c2pr2(&self) -> &C2PR2 {
        &self.c2pr2
    }
    #[doc = "0xe0 - CPU2 EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn c2imr3(&self) -> &C2IMR3 {
        &self.c2imr3
    }
    #[doc = "0xe4 - CPU2 EXTI event mask register"]
    #[inline(always)]
    pub const fn c2emr3(&self) -> &C2EMR3 {
        &self.c2emr3
    }
    #[doc = "0xe8 - CPU2 EXTI pending register"]
    #[inline(always)]
    pub const fn c2pr3(&self) -> &C2PR3 {
        &self.c2pr3
    }
}
#[doc = "RTSR1 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr1`]
module"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr1`]
module"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier1`]
module"]
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "D3PMR1 (rw) register accessor: EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pmr1`]
module"]
pub type D3PMR1 = crate::Reg<d3pmr1::D3PMR1rs>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr1;
#[doc = "D3PCR1L (rw) register accessor: EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr1l`]
module"]
pub type D3PCR1L = crate::Reg<d3pcr1l::D3PCR1Lrs>;
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr1l;
#[doc = "D3PCR1H (rw) register accessor: EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr1h`]
module"]
pub type D3PCR1H = crate::Reg<d3pcr1h::D3PCR1Hrs>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr1h;
#[doc = "RTSR2 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr2`]
module"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr2`]
module"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier2`]
module"]
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
#[doc = "EXTI software interrupt event register"]
pub mod swier2;
#[doc = "D3PMR2 (rw) register accessor: EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pmr2`]
module"]
pub type D3PMR2 = crate::Reg<d3pmr2::D3PMR2rs>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr2;
#[doc = "D3PCR2L (rw) register accessor: EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr2l`]
module"]
pub type D3PCR2L = crate::Reg<d3pcr2l::D3PCR2Lrs>;
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr2l;
#[doc = "D3PCR2H (rw) register accessor: EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr2h`]
module"]
pub type D3PCR2H = crate::Reg<d3pcr2h::D3PCR2Hrs>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr2h;
#[doc = "RTSR3 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr3`]
module"]
pub type RTSR3 = crate::Reg<rtsr3::RTSR3rs>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr3;
#[doc = "FTSR3 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr3`]
module"]
pub type FTSR3 = crate::Reg<ftsr3::FTSR3rs>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr3;
#[doc = "SWIER3 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier3`]
module"]
pub type SWIER3 = crate::Reg<swier3::SWIER3rs>;
#[doc = "EXTI software interrupt event register"]
pub mod swier3;
#[doc = "D3PMR3 (rw) register accessor: EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pmr3`]
module"]
pub type D3PMR3 = crate::Reg<d3pmr3::D3PMR3rs>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr3;
#[doc = "D3PCR3H (rw) register accessor: EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr3h`]
module"]
pub type D3PCR3H = crate::Reg<d3pcr3h::D3PCR3Hrs>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr3h;
#[doc = "C1IMR1 (rw) register accessor: EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1imr1`]
module"]
pub type C1IMR1 = crate::Reg<c1imr1::C1IMR1rs>;
#[doc = "EXTI interrupt mask register"]
pub mod c1imr1;
#[doc = "C1EMR1 (rw) register accessor: EXTI event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1emr1`]
module"]
pub type C1EMR1 = crate::Reg<c1emr1::C1EMR1rs>;
#[doc = "EXTI event mask register"]
pub mod c1emr1;
#[doc = "C1PR1 (rw) register accessor: EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1pr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1pr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1pr1`]
module"]
pub type C1PR1 = crate::Reg<c1pr1::C1PR1rs>;
#[doc = "EXTI pending register"]
pub mod c1pr1;
#[doc = "C1IMR2 (rw) register accessor: EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1imr2`]
module"]
pub type C1IMR2 = crate::Reg<c1imr2::C1IMR2rs>;
#[doc = "EXTI interrupt mask register"]
pub mod c1imr2;
#[doc = "C1EMR2 (rw) register accessor: EXTI event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1emr2`]
module"]
pub type C1EMR2 = crate::Reg<c1emr2::C1EMR2rs>;
#[doc = "EXTI event mask register"]
pub mod c1emr2;
#[doc = "C1PR2 (rw) register accessor: EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1pr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1pr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1pr2`]
module"]
pub type C1PR2 = crate::Reg<c1pr2::C1PR2rs>;
#[doc = "EXTI pending register"]
pub mod c1pr2;
#[doc = "C1IMR3 (rw) register accessor: EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1imr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1imr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1imr3`]
module"]
pub type C1IMR3 = crate::Reg<c1imr3::C1IMR3rs>;
#[doc = "EXTI interrupt mask register"]
pub mod c1imr3;
#[doc = "C1EMR3 (rw) register accessor: EXTI event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1emr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1emr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1emr3`]
module"]
pub type C1EMR3 = crate::Reg<c1emr3::C1EMR3rs>;
#[doc = "EXTI event mask register"]
pub mod c1emr3;
#[doc = "C1PR3 (rw) register accessor: EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1pr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1pr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1pr3`]
module"]
pub type C1PR3 = crate::Reg<c1pr3::C1PR3rs>;
#[doc = "EXTI pending register"]
pub mod c1pr3;
#[doc = "C2IMR1 (rw) register accessor: CPU2 EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr1`]
module"]
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr1;
#[doc = "C2EMR1 (rw) register accessor: CPU2 EXTI event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2emr1`]
module"]
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1rs>;
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr1;
#[doc = "C2PR1 (rw) register accessor: CPU2 EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2pr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2pr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2pr1`]
module"]
pub type C2PR1 = crate::Reg<c2pr1::C2PR1rs>;
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr1;
#[doc = "C2IMR2 (rw) register accessor: CPU2 EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr2`]
module"]
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr2;
#[doc = "C2EMR2 (rw) register accessor: CPU2 EXTI event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2emr2`]
module"]
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2rs>;
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr2;
#[doc = "C2PR2 (rw) register accessor: CPU2 EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2pr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2pr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2pr2`]
module"]
pub type C2PR2 = crate::Reg<c2pr2::C2PR2rs>;
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr2;
#[doc = "C2IMR3 (rw) register accessor: CPU2 EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr3`]
module"]
pub type C2IMR3 = crate::Reg<c2imr3::C2IMR3rs>;
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr3;
#[doc = "C2EMR3 (rw) register accessor: CPU2 EXTI event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2emr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2emr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2emr3`]
module"]
pub type C2EMR3 = crate::Reg<c2emr3::C2EMR3rs>;
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr3;
#[doc = "C2PR3 (rw) register accessor: CPU2 EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2pr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2pr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2pr3`]
module"]
pub type C2PR3 = crate::Reg<c2pr3::C2PR3rs>;
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr3;
