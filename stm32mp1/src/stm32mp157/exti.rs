#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    exti_rtsr1: EXTI_RTSR1,
    exti_ftsr1: EXTI_FTSR1,
    exti_swier1: EXTI_SWIER1,
    exti_rpr1: EXTI_RPR1,
    exti_fpr1: EXTI_FPR1,
    exti_tzenr1: EXTI_TZENR1,
    _reserved6: [u8; 0x08],
    exti_rtsr2: EXTI_RTSR2,
    exti_ftsr2: EXTI_FTSR2,
    exti_swier2: EXTI_SWIER2,
    exti_rpr2: EXTI_RPR2,
    exti_fpr2: EXTI_FPR2,
    exti_tzenr2: EXTI_TZENR2,
    _reserved12: [u8; 0x08],
    exti_rtsr3: EXTI_RTSR3,
    exti_ftsr3: EXTI_FTSR3,
    exti_swier3: EXTI_SWIER3,
    exti_rpr3: EXTI_RPR3,
    exti_fpr3: EXTI_FPR3,
    exti_tzenr3: EXTI_TZENR3,
    _reserved18: [u8; 0x08],
    exti_exticr1: EXTI_EXTICR1,
    exti_exticr2: EXTI_EXTICR2,
    exti_exticr3: EXTI_EXTICR3,
    exti_exticr4: EXTI_EXTICR4,
    _reserved22: [u8; 0x10],
    exti_imr1: EXTI_IMR1,
    exti_emr1: EXTI_EMR1,
    _reserved24: [u8; 0x08],
    exti_imr2: EXTI_IMR2,
    exti_emr2: EXTI_EMR2,
    _reserved26: [u8; 0x08],
    exti_imr3: EXTI_IMR3,
    exti_emr3: EXTI_EMR3,
    _reserved28: [u8; 0x18],
    exti_c2imr1: EXTI_C2IMR1,
    exti_c2emr1: EXTI_C2EMR1,
    _reserved30: [u8; 0x08],
    exti_c2imr2: EXTI_C2IMR2,
    exti_c2emr2: EXTI_C2EMR2,
    _reserved32: [u8; 0x08],
    exti_c2imr3: EXTI_C2IMR3,
    exti_c2emr3: EXTI_C2EMR3,
    _reserved34: [u8; 0x02d8],
    exti_hwcfgr13: EXTI_HWCFGR13,
    exti_hwcfgr12: EXTI_HWCFGR12,
    exti_hwcfgr11: EXTI_HWCFGR11,
    exti_hwcfgr10: EXTI_HWCFGR10,
    exti_hwcfgr9: EXTI_HWCFGR9,
    exti_hwcfgr8: EXTI_HWCFGR8,
    exti_hwcfgr7: EXTI_HWCFGR7,
    exti_hwcfgr6: EXTI_HWCFGR6,
    exti_hwcfgr5: EXTI_HWCFGR5,
    exti_hwcfgr4: EXTI_HWCFGR4,
    exti_hwcfgr3: EXTI_HWCFGR3,
    exti_hwcfgr2: EXTI_HWCFGR2,
    exti_hwcfgr1: EXTI_HWCFGR1,
    exti_verr: EXTI_VERR,
    exti_ipidr: EXTI_IPIDR,
    exti_sidr: EXTI_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_rtsr1(&self) -> &EXTI_RTSR1 {
        &self.exti_rtsr1
    }
    #[doc = "0x04 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_ftsr1(&self) -> &EXTI_FTSR1 {
        &self.exti_ftsr1
    }
    #[doc = "0x08 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_swier1(&self) -> &EXTI_SWIER1 {
        &self.exti_swier1
    }
    #[doc = "0x0c - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_rpr1(&self) -> &EXTI_RPR1 {
        &self.exti_rpr1
    }
    #[doc = "0x10 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_fpr1(&self) -> &EXTI_FPR1 {
        &self.exti_fpr1
    }
    #[doc = "0x14 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    #[inline(always)]
    pub const fn exti_tzenr1(&self) -> &EXTI_TZENR1 {
        &self.exti_tzenr1
    }
    #[doc = "0x20 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_rtsr2(&self) -> &EXTI_RTSR2 {
        &self.exti_rtsr2
    }
    #[doc = "0x24 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_ftsr2(&self) -> &EXTI_FTSR2 {
        &self.exti_ftsr2
    }
    #[doc = "0x28 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_swier2(&self) -> &EXTI_SWIER2 {
        &self.exti_swier2
    }
    #[doc = "0x2c - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_rpr2(&self) -> &EXTI_RPR2 {
        &self.exti_rpr2
    }
    #[doc = "0x30 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_fpr2(&self) -> &EXTI_FPR2 {
        &self.exti_fpr2
    }
    #[doc = "0x34 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    #[inline(always)]
    pub const fn exti_tzenr2(&self) -> &EXTI_TZENR2 {
        &self.exti_tzenr2
    }
    #[doc = "0x40 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_rtsr3(&self) -> &EXTI_RTSR3 {
        &self.exti_rtsr3
    }
    #[doc = "0x44 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_ftsr3(&self) -> &EXTI_FTSR3 {
        &self.exti_ftsr3
    }
    #[doc = "0x48 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_swier3(&self) -> &EXTI_SWIER3 {
        &self.exti_swier3
    }
    #[doc = "0x4c - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_rpr3(&self) -> &EXTI_RPR3 {
        &self.exti_rpr3
    }
    #[doc = "0x50 - Contains only register bits for configurable events."]
    #[inline(always)]
    pub const fn exti_fpr3(&self) -> &EXTI_FPR3 {
        &self.exti_fpr3
    }
    #[doc = "0x54 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    #[inline(always)]
    pub const fn exti_tzenr3(&self) -> &EXTI_TZENR3 {
        &self.exti_tzenr3
    }
    #[doc = "0x60 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    #[inline(always)]
    pub const fn exti_exticr1(&self) -> &EXTI_EXTICR1 {
        &self.exti_exticr1
    }
    #[doc = "0x64 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    #[inline(always)]
    pub const fn exti_exticr2(&self) -> &EXTI_EXTICR2 {
        &self.exti_exticr2
    }
    #[doc = "0x68 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    #[inline(always)]
    pub const fn exti_exticr3(&self) -> &EXTI_EXTICR3 {
        &self.exti_exticr3
    }
    #[doc = "0x6c - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    #[inline(always)]
    pub const fn exti_exticr4(&self) -> &EXTI_EXTICR4 {
        &self.exti_exticr4
    }
    #[doc = "0x80 - Contains register bits for configurable events and Direct events."]
    #[inline(always)]
    pub const fn exti_imr1(&self) -> &EXTI_IMR1 {
        &self.exti_imr1
    }
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn exti_emr1(&self) -> &EXTI_EMR1 {
        &self.exti_emr1
    }
    #[doc = "0x90 - Contains register bits for configurable events and direct events."]
    #[inline(always)]
    pub const fn exti_imr2(&self) -> &EXTI_IMR2 {
        &self.exti_imr2
    }
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn exti_emr2(&self) -> &EXTI_EMR2 {
        &self.exti_emr2
    }
    #[doc = "0xa0 - Contains register bits for configurable events and direct events."]
    #[inline(always)]
    pub const fn exti_imr3(&self) -> &EXTI_IMR3 {
        &self.exti_imr3
    }
    #[doc = "0xa4 - EXTI CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn exti_emr3(&self) -> &EXTI_EMR3 {
        &self.exti_emr3
    }
    #[doc = "0xc0 - Contains register bits for configurable events and Direct events."]
    #[inline(always)]
    pub const fn exti_c2imr1(&self) -> &EXTI_C2IMR1 {
        &self.exti_c2imr1
    }
    #[doc = "0xc4 - EXTI CPU2 wakeup with event mask register"]
    #[inline(always)]
    pub const fn exti_c2emr1(&self) -> &EXTI_C2EMR1 {
        &self.exti_c2emr1
    }
    #[doc = "0xd0 - Contains register bits for configurable events and direct events."]
    #[inline(always)]
    pub const fn exti_c2imr2(&self) -> &EXTI_C2IMR2 {
        &self.exti_c2imr2
    }
    #[doc = "0xd4 - EXTI CPU2 wakeup with event mask register"]
    #[inline(always)]
    pub const fn exti_c2emr2(&self) -> &EXTI_C2EMR2 {
        &self.exti_c2emr2
    }
    #[doc = "0xe0 - Contains register bits for configurable events and direct events."]
    #[inline(always)]
    pub const fn exti_c2imr3(&self) -> &EXTI_C2IMR3 {
        &self.exti_c2imr3
    }
    #[doc = "0xe4 - EXTI CPU2 wakeup with event mask register"]
    #[inline(always)]
    pub const fn exti_c2emr3(&self) -> &EXTI_C2EMR3 {
        &self.exti_c2emr3
    }
    #[doc = "0x3c0 - EXTI hardware configuration register 13"]
    #[inline(always)]
    pub const fn exti_hwcfgr13(&self) -> &EXTI_HWCFGR13 {
        &self.exti_hwcfgr13
    }
    #[doc = "0x3c4 - EXTI hardware configuration register 12"]
    #[inline(always)]
    pub const fn exti_hwcfgr12(&self) -> &EXTI_HWCFGR12 {
        &self.exti_hwcfgr12
    }
    #[doc = "0x3c8 - EXTI hardware configuration register 11"]
    #[inline(always)]
    pub const fn exti_hwcfgr11(&self) -> &EXTI_HWCFGR11 {
        &self.exti_hwcfgr11
    }
    #[doc = "0x3cc - EXTI hardware configuration register 10"]
    #[inline(always)]
    pub const fn exti_hwcfgr10(&self) -> &EXTI_HWCFGR10 {
        &self.exti_hwcfgr10
    }
    #[doc = "0x3d0 - EXTI hardware configuration register 9"]
    #[inline(always)]
    pub const fn exti_hwcfgr9(&self) -> &EXTI_HWCFGR9 {
        &self.exti_hwcfgr9
    }
    #[doc = "0x3d4 - EXTI hardware configuration register 8"]
    #[inline(always)]
    pub const fn exti_hwcfgr8(&self) -> &EXTI_HWCFGR8 {
        &self.exti_hwcfgr8
    }
    #[doc = "0x3d8 - EXTI hardware configuration register 7"]
    #[inline(always)]
    pub const fn exti_hwcfgr7(&self) -> &EXTI_HWCFGR7 {
        &self.exti_hwcfgr7
    }
    #[doc = "0x3dc - EXTI hardware configuration register 6"]
    #[inline(always)]
    pub const fn exti_hwcfgr6(&self) -> &EXTI_HWCFGR6 {
        &self.exti_hwcfgr6
    }
    #[doc = "0x3e0 - EXTI hardware configuration register 5"]
    #[inline(always)]
    pub const fn exti_hwcfgr5(&self) -> &EXTI_HWCFGR5 {
        &self.exti_hwcfgr5
    }
    #[doc = "0x3e4 - EXTI hardware configuration register 4"]
    #[inline(always)]
    pub const fn exti_hwcfgr4(&self) -> &EXTI_HWCFGR4 {
        &self.exti_hwcfgr4
    }
    #[doc = "0x3e8 - EXTI hardware configuration register 3"]
    #[inline(always)]
    pub const fn exti_hwcfgr3(&self) -> &EXTI_HWCFGR3 {
        &self.exti_hwcfgr3
    }
    #[doc = "0x3ec - EXTI hardware configuration register 2"]
    #[inline(always)]
    pub const fn exti_hwcfgr2(&self) -> &EXTI_HWCFGR2 {
        &self.exti_hwcfgr2
    }
    #[doc = "0x3f0 - EXTI hardware configuration register 1"]
    #[inline(always)]
    pub const fn exti_hwcfgr1(&self) -> &EXTI_HWCFGR1 {
        &self.exti_hwcfgr1
    }
    #[doc = "0x3f4 - EXTI IP version register"]
    #[inline(always)]
    pub const fn exti_verr(&self) -> &EXTI_VERR {
        &self.exti_verr
    }
    #[doc = "0x3f8 - EXTI identification register"]
    #[inline(always)]
    pub const fn exti_ipidr(&self) -> &EXTI_IPIDR {
        &self.exti_ipidr
    }
    #[doc = "0x3fc - EXTI size ID register"]
    #[inline(always)]
    pub const fn exti_sidr(&self) -> &EXTI_SIDR {
        &self.exti_sidr
    }
}
#[doc = "EXTI_RTSR1 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rtsr1`]
module"]
pub type EXTI_RTSR1 = crate::Reg<exti_rtsr1::EXTI_RTSR1rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr1;
#[doc = "EXTI_FTSR1 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_ftsr1`]
module"]
pub type EXTI_FTSR1 = crate::Reg<exti_ftsr1::EXTI_FTSR1rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr1;
#[doc = "EXTI_SWIER1 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_swier1`]
module"]
pub type EXTI_SWIER1 = crate::Reg<exti_swier1::EXTI_SWIER1rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier1;
#[doc = "EXTI_RPR1 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rpr1`]
module"]
pub type EXTI_RPR1 = crate::Reg<exti_rpr1::EXTI_RPR1rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr1;
#[doc = "EXTI_FPR1 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_fpr1`]
module"]
pub type EXTI_FPR1 = crate::Reg<exti_fpr1::EXTI_FPR1rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr1;
#[doc = "EXTI_TZENR1 (rw) register accessor: This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_tzenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_tzenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_tzenr1`]
module"]
pub type EXTI_TZENR1 = crate::Reg<exti_tzenr1::EXTI_TZENR1rs>;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr1;
#[doc = "EXTI_RTSR2 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rtsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rtsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rtsr2`]
module"]
pub type EXTI_RTSR2 = crate::Reg<exti_rtsr2::EXTI_RTSR2rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr2;
#[doc = "EXTI_FTSR2 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_ftsr2`]
module"]
pub type EXTI_FTSR2 = crate::Reg<exti_ftsr2::EXTI_FTSR2rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr2;
#[doc = "EXTI_SWIER2 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_swier2`]
module"]
pub type EXTI_SWIER2 = crate::Reg<exti_swier2::EXTI_SWIER2rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier2;
#[doc = "EXTI_RPR2 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rpr2`]
module"]
pub type EXTI_RPR2 = crate::Reg<exti_rpr2::EXTI_RPR2rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr2;
#[doc = "EXTI_FPR2 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_fpr2`]
module"]
pub type EXTI_FPR2 = crate::Reg<exti_fpr2::EXTI_FPR2rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr2;
#[doc = "EXTI_TZENR2 (rw) register accessor: This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_tzenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_tzenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_tzenr2`]
module"]
pub type EXTI_TZENR2 = crate::Reg<exti_tzenr2::EXTI_TZENR2rs>;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr2;
#[doc = "EXTI_RTSR3 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rtsr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rtsr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rtsr3`]
module"]
pub type EXTI_RTSR3 = crate::Reg<exti_rtsr3::EXTI_RTSR3rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr3;
#[doc = "EXTI_FTSR3 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_ftsr3`]
module"]
pub type EXTI_FTSR3 = crate::Reg<exti_ftsr3::EXTI_FTSR3rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr3;
#[doc = "EXTI_SWIER3 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_swier3`]
module"]
pub type EXTI_SWIER3 = crate::Reg<exti_swier3::EXTI_SWIER3rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier3;
#[doc = "EXTI_RPR3 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_rpr3`]
module"]
pub type EXTI_RPR3 = crate::Reg<exti_rpr3::EXTI_RPR3rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr3;
#[doc = "EXTI_FPR3 (rw) register accessor: Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_fpr3`]
module"]
pub type EXTI_FPR3 = crate::Reg<exti_fpr3::EXTI_FPR3rs>;
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr3;
#[doc = "EXTI_TZENR3 (rw) register accessor: This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_tzenr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_tzenr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_tzenr3`]
module"]
pub type EXTI_TZENR3 = crate::Reg<exti_tzenr3::EXTI_TZENR3rs>;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr3;
#[doc = "EXTI_EXTICR1 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr1`]
module"]
pub type EXTI_EXTICR1 = crate::Reg<exti_exticr1::EXTI_EXTICR1rs>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr1;
#[doc = "EXTI_EXTICR2 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr2`]
module"]
pub type EXTI_EXTICR2 = crate::Reg<exti_exticr2::EXTI_EXTICR2rs>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr2;
#[doc = "EXTI_EXTICR3 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr3`]
module"]
pub type EXTI_EXTICR3 = crate::Reg<exti_exticr3::EXTI_EXTICR3rs>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr3;
#[doc = "EXTI_EXTICR4 (rw) register accessor: EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_exticr4`]
module"]
pub type EXTI_EXTICR4 = crate::Reg<exti_exticr4::EXTI_EXTICR4rs>;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr4;
#[doc = "EXTI_IMR1 (rw) register accessor: Contains register bits for configurable events and Direct events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_imr1`]
module"]
pub type EXTI_IMR1 = crate::Reg<exti_imr1::EXTI_IMR1rs>;
#[doc = "Contains register bits for configurable events and Direct events."]
pub mod exti_imr1;
#[doc = "EXTI_EMR1 (rw) register accessor: EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_emr1`]
module"]
pub type EXTI_EMR1 = crate::Reg<exti_emr1::EXTI_EMR1rs>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr1;
#[doc = "EXTI_IMR2 (rw) register accessor: Contains register bits for configurable events and direct events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_imr2`]
module"]
pub type EXTI_IMR2 = crate::Reg<exti_imr2::EXTI_IMR2rs>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_imr2;
#[doc = "EXTI_EMR2 (rw) register accessor: EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_emr2`]
module"]
pub type EXTI_EMR2 = crate::Reg<exti_emr2::EXTI_EMR2rs>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr2;
#[doc = "EXTI_IMR3 (rw) register accessor: Contains register bits for configurable events and direct events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_imr3`]
module"]
pub type EXTI_IMR3 = crate::Reg<exti_imr3::EXTI_IMR3rs>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_imr3;
#[doc = "EXTI_EMR3 (rw) register accessor: EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_emr3`]
module"]
pub type EXTI_EMR3 = crate::Reg<exti_emr3::EXTI_EMR3rs>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr3;
#[doc = "EXTI_C2IMR1 (rw) register accessor: Contains register bits for configurable events and Direct events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_c2imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_c2imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_c2imr1`]
module"]
pub type EXTI_C2IMR1 = crate::Reg<exti_c2imr1::EXTI_C2IMR1rs>;
#[doc = "Contains register bits for configurable events and Direct events."]
pub mod exti_c2imr1;
#[doc = "EXTI_C2EMR1 (rw) register accessor: EXTI CPU2 wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_c2emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_c2emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_c2emr1`]
module"]
pub type EXTI_C2EMR1 = crate::Reg<exti_c2emr1::EXTI_C2EMR1rs>;
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr1;
#[doc = "EXTI_C2IMR2 (rw) register accessor: Contains register bits for configurable events and direct events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_c2imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_c2imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_c2imr2`]
module"]
pub type EXTI_C2IMR2 = crate::Reg<exti_c2imr2::EXTI_C2IMR2rs>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_c2imr2;
#[doc = "EXTI_C2EMR2 (rw) register accessor: EXTI CPU2 wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_c2emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_c2emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_c2emr2`]
module"]
pub type EXTI_C2EMR2 = crate::Reg<exti_c2emr2::EXTI_C2EMR2rs>;
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr2;
#[doc = "EXTI_C2IMR3 (rw) register accessor: Contains register bits for configurable events and direct events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_c2imr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_c2imr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_c2imr3`]
module"]
pub type EXTI_C2IMR3 = crate::Reg<exti_c2imr3::EXTI_C2IMR3rs>;
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_c2imr3;
#[doc = "EXTI_C2EMR3 (rw) register accessor: EXTI CPU2 wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_c2emr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_c2emr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_c2emr3`]
module"]
pub type EXTI_C2EMR3 = crate::Reg<exti_c2emr3::EXTI_C2EMR3rs>;
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr3;
#[doc = "EXTI_HWCFGR13 (r) register accessor: EXTI hardware configuration register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr13`]
module"]
pub type EXTI_HWCFGR13 = crate::Reg<exti_hwcfgr13::EXTI_HWCFGR13rs>;
#[doc = "EXTI hardware configuration register 13"]
pub mod exti_hwcfgr13;
#[doc = "EXTI_HWCFGR12 (r) register accessor: EXTI hardware configuration register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr12`]
module"]
pub type EXTI_HWCFGR12 = crate::Reg<exti_hwcfgr12::EXTI_HWCFGR12rs>;
#[doc = "EXTI hardware configuration register 12"]
pub mod exti_hwcfgr12;
#[doc = "EXTI_HWCFGR11 (r) register accessor: EXTI hardware configuration register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr11`]
module"]
pub type EXTI_HWCFGR11 = crate::Reg<exti_hwcfgr11::EXTI_HWCFGR11rs>;
#[doc = "EXTI hardware configuration register 11"]
pub mod exti_hwcfgr11;
#[doc = "EXTI_HWCFGR10 (r) register accessor: EXTI hardware configuration register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr10`]
module"]
pub type EXTI_HWCFGR10 = crate::Reg<exti_hwcfgr10::EXTI_HWCFGR10rs>;
#[doc = "EXTI hardware configuration register 10"]
pub mod exti_hwcfgr10;
#[doc = "EXTI_HWCFGR9 (r) register accessor: EXTI hardware configuration register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr9`]
module"]
pub type EXTI_HWCFGR9 = crate::Reg<exti_hwcfgr9::EXTI_HWCFGR9rs>;
#[doc = "EXTI hardware configuration register 9"]
pub mod exti_hwcfgr9;
#[doc = "EXTI_HWCFGR8 (r) register accessor: EXTI hardware configuration register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr8`]
module"]
pub type EXTI_HWCFGR8 = crate::Reg<exti_hwcfgr8::EXTI_HWCFGR8rs>;
#[doc = "EXTI hardware configuration register 8"]
pub mod exti_hwcfgr8;
#[doc = "EXTI_HWCFGR7 (r) register accessor: EXTI hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr7`]
module"]
pub type EXTI_HWCFGR7 = crate::Reg<exti_hwcfgr7::EXTI_HWCFGR7rs>;
#[doc = "EXTI hardware configuration register 7"]
pub mod exti_hwcfgr7;
#[doc = "EXTI_HWCFGR6 (r) register accessor: EXTI hardware configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr6`]
module"]
pub type EXTI_HWCFGR6 = crate::Reg<exti_hwcfgr6::EXTI_HWCFGR6rs>;
#[doc = "EXTI hardware configuration register 6"]
pub mod exti_hwcfgr6;
#[doc = "EXTI_HWCFGR5 (r) register accessor: EXTI hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr5`]
module"]
pub type EXTI_HWCFGR5 = crate::Reg<exti_hwcfgr5::EXTI_HWCFGR5rs>;
#[doc = "EXTI hardware configuration register 5"]
pub mod exti_hwcfgr5;
#[doc = "EXTI_HWCFGR4 (r) register accessor: EXTI hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr4`]
module"]
pub type EXTI_HWCFGR4 = crate::Reg<exti_hwcfgr4::EXTI_HWCFGR4rs>;
#[doc = "EXTI hardware configuration register 4"]
pub mod exti_hwcfgr4;
#[doc = "EXTI_HWCFGR3 (r) register accessor: EXTI hardware configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr3`]
module"]
pub type EXTI_HWCFGR3 = crate::Reg<exti_hwcfgr3::EXTI_HWCFGR3rs>;
#[doc = "EXTI hardware configuration register 3"]
pub mod exti_hwcfgr3;
#[doc = "EXTI_HWCFGR2 (r) register accessor: EXTI hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr2`]
module"]
pub type EXTI_HWCFGR2 = crate::Reg<exti_hwcfgr2::EXTI_HWCFGR2rs>;
#[doc = "EXTI hardware configuration register 2"]
pub mod exti_hwcfgr2;
#[doc = "EXTI_HWCFGR1 (r) register accessor: EXTI hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_hwcfgr1`]
module"]
pub type EXTI_HWCFGR1 = crate::Reg<exti_hwcfgr1::EXTI_HWCFGR1rs>;
#[doc = "EXTI hardware configuration register 1"]
pub mod exti_hwcfgr1;
#[doc = "EXTI_VERR (r) register accessor: EXTI IP version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_verr`]
module"]
pub type EXTI_VERR = crate::Reg<exti_verr::EXTI_VERRrs>;
#[doc = "EXTI IP version register"]
pub mod exti_verr;
#[doc = "EXTI_IPIDR (r) register accessor: EXTI identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_ipidr`]
module"]
pub type EXTI_IPIDR = crate::Reg<exti_ipidr::EXTI_IPIDRrs>;
#[doc = "EXTI identification register"]
pub mod exti_ipidr;
#[doc = "EXTI_SIDR (r) register accessor: EXTI size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exti_sidr`]
module"]
pub type EXTI_SIDR = crate::Reg<exti_sidr::EXTI_SIDRrs>;
#[doc = "EXTI size ID register"]
pub mod exti_sidr;
