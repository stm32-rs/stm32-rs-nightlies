#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gtzc1_tzic_ier1: GTZC1_TZIC_IER1,
    gtzc1_tzic_ier2: GTZC1_TZIC_IER2,
    gtzc1_tzic_ier3: GTZC1_TZIC_IER3,
    gtzc1_tzic_ier4: GTZC1_TZIC_IER4,
    gtzc1_tzic_sr1: GTZC1_TZIC_SR1,
    gtzc1_tzic_sr2: GTZC1_TZIC_SR2,
    gtzc1_tzic_sr3: GTZC1_TZIC_SR3,
    gtzc1_tzic_sr4: GTZC1_TZIC_SR4,
    gtzc1_tzic_fcr1: GTZC1_TZIC_FCR1,
    gtzc1_tzic_fcr2: GTZC1_TZIC_FCR2,
    gtzc1_tzic_fcr3: GTZC1_TZIC_FCR3,
    gtzc1_tzic_fcr4: GTZC1_TZIC_FCR4,
}
impl RegisterBlock {
    #[doc = "0x00 - TZIC interrupt enable register 1"]
    #[inline(always)]
    pub const fn gtzc1_tzic_ier1(&self) -> &GTZC1_TZIC_IER1 {
        &self.gtzc1_tzic_ier1
    }
    #[doc = "0x04 - GTZC1 TZIC interrupt enable register 2"]
    #[inline(always)]
    pub const fn gtzc1_tzic_ier2(&self) -> &GTZC1_TZIC_IER2 {
        &self.gtzc1_tzic_ier2
    }
    #[doc = "0x08 - GTZC1 TZIC interrupt enable register 3"]
    #[inline(always)]
    pub const fn gtzc1_tzic_ier3(&self) -> &GTZC1_TZIC_IER3 {
        &self.gtzc1_tzic_ier3
    }
    #[doc = "0x0c - GTZC1 TZIC interrupt enable register 4"]
    #[inline(always)]
    pub const fn gtzc1_tzic_ier4(&self) -> &GTZC1_TZIC_IER4 {
        &self.gtzc1_tzic_ier4
    }
    #[doc = "0x10 - TZIC status register 1"]
    #[inline(always)]
    pub const fn gtzc1_tzic_sr1(&self) -> &GTZC1_TZIC_SR1 {
        &self.gtzc1_tzic_sr1
    }
    #[doc = "0x14 - TZIC status register 2"]
    #[inline(always)]
    pub const fn gtzc1_tzic_sr2(&self) -> &GTZC1_TZIC_SR2 {
        &self.gtzc1_tzic_sr2
    }
    #[doc = "0x18 - TZIC status register 3"]
    #[inline(always)]
    pub const fn gtzc1_tzic_sr3(&self) -> &GTZC1_TZIC_SR3 {
        &self.gtzc1_tzic_sr3
    }
    #[doc = "0x1c - GTZC1 TZIC status register 4"]
    #[inline(always)]
    pub const fn gtzc1_tzic_sr4(&self) -> &GTZC1_TZIC_SR4 {
        &self.gtzc1_tzic_sr4
    }
    #[doc = "0x20 - TZIC flag clear register 1"]
    #[inline(always)]
    pub const fn gtzc1_tzic_fcr1(&self) -> &GTZC1_TZIC_FCR1 {
        &self.gtzc1_tzic_fcr1
    }
    #[doc = "0x24 - TZIC flag clear register 2"]
    #[inline(always)]
    pub const fn gtzc1_tzic_fcr2(&self) -> &GTZC1_TZIC_FCR2 {
        &self.gtzc1_tzic_fcr2
    }
    #[doc = "0x28 - TZIC flag clear register 3"]
    #[inline(always)]
    pub const fn gtzc1_tzic_fcr3(&self) -> &GTZC1_TZIC_FCR3 {
        &self.gtzc1_tzic_fcr3
    }
    #[doc = "0x2c - GTZC1 TZIC flag clear register 4"]
    #[inline(always)]
    pub const fn gtzc1_tzic_fcr4(&self) -> &GTZC1_TZIC_FCR4 {
        &self.gtzc1_tzic_fcr4
    }
}
#[doc = "GTZC1_TZIC_IER1 (rw) register accessor: TZIC interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_ier1`]
module"]
pub type GTZC1_TZIC_IER1 = crate::Reg<gtzc1_tzic_ier1::GTZC1_TZIC_IER1rs>;
#[doc = "TZIC interrupt enable register 1"]
pub mod gtzc1_tzic_ier1;
#[doc = "GTZC1_TZIC_IER2 (rw) register accessor: GTZC1 TZIC interrupt enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_ier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_ier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_ier2`]
module"]
pub type GTZC1_TZIC_IER2 = crate::Reg<gtzc1_tzic_ier2::GTZC1_TZIC_IER2rs>;
#[doc = "GTZC1 TZIC interrupt enable register 2"]
pub mod gtzc1_tzic_ier2;
#[doc = "GTZC1_TZIC_IER3 (rw) register accessor: GTZC1 TZIC interrupt enable register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_ier3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_ier3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_ier3`]
module"]
pub type GTZC1_TZIC_IER3 = crate::Reg<gtzc1_tzic_ier3::GTZC1_TZIC_IER3rs>;
#[doc = "GTZC1 TZIC interrupt enable register 3"]
pub mod gtzc1_tzic_ier3;
#[doc = "GTZC1_TZIC_IER4 (rw) register accessor: GTZC1 TZIC interrupt enable register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_ier4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_ier4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_ier4`]
module"]
pub type GTZC1_TZIC_IER4 = crate::Reg<gtzc1_tzic_ier4::GTZC1_TZIC_IER4rs>;
#[doc = "GTZC1 TZIC interrupt enable register 4"]
pub mod gtzc1_tzic_ier4;
#[doc = "GTZC1_TZIC_SR1 (r) register accessor: TZIC status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_sr1`]
module"]
pub type GTZC1_TZIC_SR1 = crate::Reg<gtzc1_tzic_sr1::GTZC1_TZIC_SR1rs>;
#[doc = "TZIC status register 1"]
pub mod gtzc1_tzic_sr1;
#[doc = "GTZC1_TZIC_SR2 (r) register accessor: TZIC status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_sr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_sr2`]
module"]
pub type GTZC1_TZIC_SR2 = crate::Reg<gtzc1_tzic_sr2::GTZC1_TZIC_SR2rs>;
#[doc = "TZIC status register 2"]
pub mod gtzc1_tzic_sr2;
#[doc = "GTZC1_TZIC_SR3 (r) register accessor: TZIC status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_sr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_sr3`]
module"]
pub type GTZC1_TZIC_SR3 = crate::Reg<gtzc1_tzic_sr3::GTZC1_TZIC_SR3rs>;
#[doc = "TZIC status register 3"]
pub mod gtzc1_tzic_sr3;
#[doc = "GTZC1_TZIC_SR4 (r) register accessor: GTZC1 TZIC status register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_sr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_sr4`]
module"]
pub type GTZC1_TZIC_SR4 = crate::Reg<gtzc1_tzic_sr4::GTZC1_TZIC_SR4rs>;
#[doc = "GTZC1 TZIC status register 4"]
pub mod gtzc1_tzic_sr4;
#[doc = "GTZC1_TZIC_FCR1 (w) register accessor: TZIC flag clear register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_fcr1`]
module"]
pub type GTZC1_TZIC_FCR1 = crate::Reg<gtzc1_tzic_fcr1::GTZC1_TZIC_FCR1rs>;
#[doc = "TZIC flag clear register 1"]
pub mod gtzc1_tzic_fcr1;
#[doc = "GTZC1_TZIC_FCR2 (w) register accessor: TZIC flag clear register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_fcr2`]
module"]
pub type GTZC1_TZIC_FCR2 = crate::Reg<gtzc1_tzic_fcr2::GTZC1_TZIC_FCR2rs>;
#[doc = "TZIC flag clear register 2"]
pub mod gtzc1_tzic_fcr2;
#[doc = "GTZC1_TZIC_FCR3 (w) register accessor: TZIC flag clear register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_fcr3`]
module"]
pub type GTZC1_TZIC_FCR3 = crate::Reg<gtzc1_tzic_fcr3::GTZC1_TZIC_FCR3rs>;
#[doc = "TZIC flag clear register 3"]
pub mod gtzc1_tzic_fcr3;
#[doc = "GTZC1_TZIC_FCR4 (w) register accessor: GTZC1 TZIC flag clear register 4\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzic_fcr4`]
module"]
pub type GTZC1_TZIC_FCR4 = crate::Reg<gtzc1_tzic_fcr4::GTZC1_TZIC_FCR4rs>;
#[doc = "GTZC1 TZIC flag clear register 4"]
pub mod gtzc1_tzic_fcr4;
