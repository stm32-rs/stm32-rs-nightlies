#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: ACR,
    _reserved_1_acr_: [u8; 0x0200],
}
impl RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04..0x204 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub const fn bank(&self, n: usize) -> &BANK {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x204 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub fn bank_iter(&self) -> impl Iterator<Item = &BANK> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x04..0x104 - Cluster BANK1, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub const fn bank1(&self) -> &BANK {
        self.bank(0)
    }
    #[doc = "0x104..0x204 - Cluster BANK2, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub const fn bank2(&self) -> &BANK {
        self.bank(1)
    }
    #[doc = "0x08 - FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x18 - FLASH option control register"]
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur(&self) -> &OPTSR_CUR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg(&self) -> &OPTSR_PRG {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - FLASH option clear control register"]
    #[inline(always)]
    pub const fn optccr(&self) -> &OPTCCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x40 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub const fn boot7_curr(&self) -> &BOOT7_CURR {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub const fn boot7_prgr(&self) -> &BOOT7_PRGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub const fn boot4_curr(&self) -> &BOOT4_CURR {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub const fn boot4_prgr(&self) -> &BOOT4_PRGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x5c - FLASH CRC data register"]
    #[inline(always)]
    pub const fn crcdatar(&self) -> &CRCDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x100 - FLASH access control register"]
    #[inline(always)]
    pub const fn acr_(&self) -> &ACR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x108 - FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr_(&self) -> &OPTKEYR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x118 - FLASH option control register"]
    #[inline(always)]
    pub const fn optcr_(&self) -> &OPTCR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur_(&self) -> &OPTSR_CUR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x120 - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg_(&self) -> &OPTSR_PRG_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x124 - FLASH option clear control register"]
    #[inline(always)]
    pub const fn optccr_(&self) -> &OPTCCR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x140 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub const fn boot7_curr_(&self) -> &BOOT7_CURR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x144 - FLASH register boot address for Arm Cortex-M7 core"]
    #[inline(always)]
    pub const fn boot7_prgr_(&self) -> &BOOT7_PRGR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x148 - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub const fn boot4_curr_(&self) -> &BOOT4_CURR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x14c - FLASH register boot address for Arm Cortex-M4 core"]
    #[inline(always)]
    pub const fn boot4_prgr_(&self) -> &BOOT4_PRGR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(332).cast() }
    }
}
#[doc = "ACR (rw) register accessor: FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "FLASH access control register"]
pub mod acr;
#[doc = "Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
pub use self::bank::BANK;
#[doc = r"Cluster"]
#[doc = "Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
pub mod bank;
#[doc = "OPTKEYR (w) register accessor: FLASH option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPTCR (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr`]
module"]
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "OPTSR_CUR (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_cur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_cur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_cur`]
module"]
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CURrs>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_prg`]
module"]
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRGrs>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTCCR (rw) register accessor: FLASH option clear control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optccr`]
module"]
pub type OPTCCR = crate::Reg<optccr::OPTCCRrs>;
#[doc = "FLASH option clear control register"]
pub mod optccr;
#[doc = "BOOT7_CURR (rw) register accessor: FLASH register boot address for Arm Cortex-M7 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot7_curr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot7_curr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot7_curr`]
module"]
pub type BOOT7_CURR = crate::Reg<boot7_curr::BOOT7_CURRrs>;
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_curr;
#[doc = "BOOT7_PRGR (rw) register accessor: FLASH register boot address for Arm Cortex-M7 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot7_prgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot7_prgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot7_prgr`]
module"]
pub type BOOT7_PRGR = crate::Reg<boot7_prgr::BOOT7_PRGRrs>;
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_prgr;
#[doc = "BOOT4_CURR (rw) register accessor: FLASH register boot address for Arm Cortex-M4 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot4_curr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot4_curr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot4_curr`]
module"]
pub type BOOT4_CURR = crate::Reg<boot4_curr::BOOT4_CURRrs>;
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_curr;
#[doc = "BOOT4_PRGR (rw) register accessor: FLASH register boot address for Arm Cortex-M4 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot4_prgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot4_prgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot4_prgr`]
module"]
pub type BOOT4_PRGR = crate::Reg<boot4_prgr::BOOT4_PRGRrs>;
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_prgr;
#[doc = "CRCDATAR (rw) register accessor: FLASH CRC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdatar`]
module"]
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATARrs>;
#[doc = "FLASH CRC data register"]
pub mod crcdatar;
#[doc = "ACR_ (rw) register accessor: FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr_`]
module"]
pub type ACR_ = crate::Reg<acr_::ACR_rs>;
#[doc = "FLASH access control register"]
pub mod acr_;
#[doc = "OPTKEYR_ (w) register accessor: FLASH option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr_`]
module"]
pub type OPTKEYR_ = crate::Reg<optkeyr_::OPTKEYR_rs>;
#[doc = "FLASH option key register"]
pub mod optkeyr_;
#[doc = "OPTCR_ (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr_`]
module"]
pub type OPTCR_ = crate::Reg<optcr_::OPTCR_rs>;
#[doc = "FLASH option control register"]
pub mod optcr_;
#[doc = "OPTSR_CUR_ (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_cur_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_cur_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_cur_`]
module"]
pub type OPTSR_CUR_ = crate::Reg<optsr_cur_::OPTSR_CUR_rs>;
#[doc = "FLASH option status register"]
pub mod optsr_cur_;
#[doc = "OPTSR_PRG_ (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_prg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_prg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_prg_`]
module"]
pub type OPTSR_PRG_ = crate::Reg<optsr_prg_::OPTSR_PRG_rs>;
#[doc = "FLASH option status register"]
pub mod optsr_prg_;
#[doc = "OPTCCR_ (rw) register accessor: FLASH option clear control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optccr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optccr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optccr_`]
module"]
pub type OPTCCR_ = crate::Reg<optccr_::OPTCCR_rs>;
#[doc = "FLASH option clear control register"]
pub mod optccr_;
#[doc = "BOOT7_CURR_ (rw) register accessor: FLASH register boot address for Arm Cortex-M7 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot7_curr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot7_curr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot7_curr_`]
module"]
pub type BOOT7_CURR_ = crate::Reg<boot7_curr_::BOOT7_CURR_rs>;
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_curr_;
#[doc = "BOOT7_PRGR_ (rw) register accessor: FLASH register boot address for Arm Cortex-M7 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot7_prgr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot7_prgr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot7_prgr_`]
module"]
pub type BOOT7_PRGR_ = crate::Reg<boot7_prgr_::BOOT7_PRGR_rs>;
#[doc = "FLASH register boot address for Arm Cortex-M7 core"]
pub mod boot7_prgr_;
#[doc = "BOOT4_CURR_ (rw) register accessor: FLASH register boot address for Arm Cortex-M4 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot4_curr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot4_curr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot4_curr_`]
module"]
pub type BOOT4_CURR_ = crate::Reg<boot4_curr_::BOOT4_CURR_rs>;
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_curr_;
#[doc = "BOOT4_PRGR_ (rw) register accessor: FLASH register boot address for Arm Cortex-M4 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot4_prgr_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot4_prgr_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot4_prgr_`]
module"]
pub type BOOT4_PRGR_ = crate::Reg<boot4_prgr_::BOOT4_PRGR_rs>;
#[doc = "FLASH register boot address for Arm Cortex-M4 core"]
pub mod boot4_prgr_;
