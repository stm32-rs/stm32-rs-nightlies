#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICH hypervisor control register"]
    pub gich_hcr: GICH_HCR,
    #[doc = "0x04 - GICH VGIC type register"]
    pub gich_vtr: GICH_VTR,
    #[doc = "0x08 - GICH virtual machine control register"]
    pub gich_vmcr: GICH_VMCR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - GICH maintenance interrupt status register"]
    pub gich_misr: GICH_MISR,
    _reserved4: [u8; 12usize],
    #[doc = "0x20 - GICH end of interrupt status register"]
    pub gich_eisr0: GICH_EISR0,
    _reserved5: [u8; 12usize],
    #[doc = "0x30 - GICH empty list status register"]
    pub gich_elsr0: GICH_ELSR0,
    _reserved6: [u8; 188usize],
    #[doc = "0xf0 - GICH active priority register"]
    pub gich_apr0: GICH_APR0,
    _reserved7: [u8; 12usize],
    #[doc = "0x100 - GICH list register 0"]
    pub gich_lr0: GICH_LR0,
    #[doc = "0x104 - GICH list register 1"]
    pub gich_lr1: GICH_LR1,
    #[doc = "0x108 - GICH list register 2"]
    pub gich_lr2: GICH_LR2,
    #[doc = "0x10c - GICH list register 3"]
    pub gich_lr3: GICH_LR3,
}
#[doc = "GICH hypervisor control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_hcr](gich_hcr) module"]
pub type GICH_HCR = crate::Reg<u32, _GICH_HCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_HCR;
#[doc = "`read()` method returns [gich_hcr::R](gich_hcr::R) reader structure"]
impl crate::Readable for GICH_HCR {}
#[doc = "`write(|w| ..)` method takes [gich_hcr::W](gich_hcr::W) writer structure"]
impl crate::Writable for GICH_HCR {}
#[doc = "GICH hypervisor control register"]
pub mod gich_hcr;
#[doc = "GICH VGIC type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_vtr](gich_vtr) module"]
pub type GICH_VTR = crate::Reg<u32, _GICH_VTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_VTR;
#[doc = "`read()` method returns [gich_vtr::R](gich_vtr::R) reader structure"]
impl crate::Readable for GICH_VTR {}
#[doc = "GICH VGIC type register"]
pub mod gich_vtr;
#[doc = "GICH virtual machine control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_vmcr](gich_vmcr) module"]
pub type GICH_VMCR = crate::Reg<u32, _GICH_VMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_VMCR;
#[doc = "`read()` method returns [gich_vmcr::R](gich_vmcr::R) reader structure"]
impl crate::Readable for GICH_VMCR {}
#[doc = "`write(|w| ..)` method takes [gich_vmcr::W](gich_vmcr::W) writer structure"]
impl crate::Writable for GICH_VMCR {}
#[doc = "GICH virtual machine control register"]
pub mod gich_vmcr;
#[doc = "GICH maintenance interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_misr](gich_misr) module"]
pub type GICH_MISR = crate::Reg<u32, _GICH_MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_MISR;
#[doc = "`read()` method returns [gich_misr::R](gich_misr::R) reader structure"]
impl crate::Readable for GICH_MISR {}
#[doc = "GICH maintenance interrupt status register"]
pub mod gich_misr;
#[doc = "GICH end of interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_eisr0](gich_eisr0) module"]
pub type GICH_EISR0 = crate::Reg<u32, _GICH_EISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_EISR0;
#[doc = "`read()` method returns [gich_eisr0::R](gich_eisr0::R) reader structure"]
impl crate::Readable for GICH_EISR0 {}
#[doc = "GICH end of interrupt status register"]
pub mod gich_eisr0;
#[doc = "GICH empty list status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_elsr0](gich_elsr0) module"]
pub type GICH_ELSR0 = crate::Reg<u32, _GICH_ELSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_ELSR0;
#[doc = "`read()` method returns [gich_elsr0::R](gich_elsr0::R) reader structure"]
impl crate::Readable for GICH_ELSR0 {}
#[doc = "GICH empty list status register"]
pub mod gich_elsr0;
#[doc = "GICH active priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_apr0](gich_apr0) module"]
pub type GICH_APR0 = crate::Reg<u32, _GICH_APR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_APR0;
#[doc = "`read()` method returns [gich_apr0::R](gich_apr0::R) reader structure"]
impl crate::Readable for GICH_APR0 {}
#[doc = "`write(|w| ..)` method takes [gich_apr0::W](gich_apr0::W) writer structure"]
impl crate::Writable for GICH_APR0 {}
#[doc = "GICH active priority register"]
pub mod gich_apr0;
#[doc = "GICH list register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_lr0](gich_lr0) module"]
pub type GICH_LR0 = crate::Reg<u32, _GICH_LR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_LR0;
#[doc = "`read()` method returns [gich_lr0::R](gich_lr0::R) reader structure"]
impl crate::Readable for GICH_LR0 {}
#[doc = "`write(|w| ..)` method takes [gich_lr0::W](gich_lr0::W) writer structure"]
impl crate::Writable for GICH_LR0 {}
#[doc = "GICH list register 0"]
pub mod gich_lr0;
#[doc = "GICH list register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_lr1](gich_lr1) module"]
pub type GICH_LR1 = crate::Reg<u32, _GICH_LR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_LR1;
#[doc = "`read()` method returns [gich_lr1::R](gich_lr1::R) reader structure"]
impl crate::Readable for GICH_LR1 {}
#[doc = "`write(|w| ..)` method takes [gich_lr1::W](gich_lr1::W) writer structure"]
impl crate::Writable for GICH_LR1 {}
#[doc = "GICH list register 1"]
pub mod gich_lr1;
#[doc = "GICH list register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_lr2](gich_lr2) module"]
pub type GICH_LR2 = crate::Reg<u32, _GICH_LR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_LR2;
#[doc = "`read()` method returns [gich_lr2::R](gich_lr2::R) reader structure"]
impl crate::Readable for GICH_LR2 {}
#[doc = "`write(|w| ..)` method takes [gich_lr2::W](gich_lr2::W) writer structure"]
impl crate::Writable for GICH_LR2 {}
#[doc = "GICH list register 2"]
pub mod gich_lr2;
#[doc = "GICH list register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_lr3](gich_lr3) module"]
pub type GICH_LR3 = crate::Reg<u32, _GICH_LR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICH_LR3;
#[doc = "`read()` method returns [gich_lr3::R](gich_lr3::R) reader structure"]
impl crate::Readable for GICH_LR3 {}
#[doc = "`write(|w| ..)` method takes [gich_lr3::W](gich_lr3::W) writer structure"]
impl crate::Writable for GICH_LR3 {}
#[doc = "GICH list register 3"]
pub mod gich_lr3;
