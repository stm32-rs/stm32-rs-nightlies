#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    imr1: IMR1,
    emr1: EMR1,
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    pr1: PR1,
    _reserved6: [u8; 0x08],
    imr2: IMR2,
    emr2: EMR2,
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    pr2: PR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    #[doc = "0x04 - Event mask register"]
    #[inline(always)]
    pub const fn emr1(&self) -> &EMR1 {
        &self.emr1
    }
    #[doc = "0x08 - Rising Trigger selection register"]
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    #[doc = "0x0c - Falling Trigger selection register"]
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    #[doc = "0x10 - Software interrupt event register"]
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    #[doc = "0x14 - Pending register"]
    #[inline(always)]
    pub const fn pr1(&self) -> &PR1 {
        &self.pr1
    }
    #[doc = "0x20 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    #[doc = "0x24 - Event mask register"]
    #[inline(always)]
    pub const fn emr2(&self) -> &EMR2 {
        &self.emr2
    }
    #[doc = "0x28 - Rising Trigger selection register"]
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    #[doc = "0x2c - Falling Trigger selection register"]
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    #[doc = "0x30 - Software interrupt event register"]
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    #[doc = "0x34 - Pending register"]
    #[inline(always)]
    pub const fn pr2(&self) -> &PR2 {
        &self.pr2
    }
}
#[doc = "IMR1 (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
#[doc = "Interrupt mask register"]
pub mod imr1;
#[doc = "EMR1 (rw) register accessor: Event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr1`]
module"]
pub type EMR1 = crate::Reg<emr1::EMR1rs>;
#[doc = "Event mask register"]
pub mod emr1;
#[doc = "RTSR1 (rw) register accessor: Rising Trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr1`]
module"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
#[doc = "Rising Trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: Falling Trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr1`]
module"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
#[doc = "Falling Trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier1`]
module"]
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
#[doc = "Software interrupt event register"]
pub mod swier1;
#[doc = "PR1 (rw) register accessor: Pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1`]
module"]
pub type PR1 = crate::Reg<pr1::PR1rs>;
#[doc = "Pending register"]
pub mod pr1;
#[doc = "IMR2 (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
#[doc = "Interrupt mask register"]
pub mod imr2;
#[doc = "EMR2 (rw) register accessor: Event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr2`]
module"]
pub type EMR2 = crate::Reg<emr2::EMR2rs>;
#[doc = "Event mask register"]
pub mod emr2;
#[doc = "RTSR2 (rw) register accessor: Rising Trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr2`]
module"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
#[doc = "Rising Trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 (rw) register accessor: Falling Trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr2`]
module"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
#[doc = "Falling Trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 (rw) register accessor: Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier2`]
module"]
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
#[doc = "Software interrupt event register"]
pub mod swier2;
#[doc = "PR2 (rw) register accessor: Pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr2`]
module"]
pub type PR2 = crate::Reg<pr2::PR2rs>;
#[doc = "Pending register"]
pub mod pr2;
