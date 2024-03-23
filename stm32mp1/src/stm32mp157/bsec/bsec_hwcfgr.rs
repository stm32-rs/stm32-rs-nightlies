#[doc = "Register `BSEC_HWCFGR` reader"]
pub type R = crate::R<BSEC_HWCFGRrs>;
#[doc = "Field `SIZE` reader - SIZE"]
pub type SIZE_R = crate::FieldReader;
#[doc = "Field `ECC_USE` reader - ECC_USE"]
pub type ECC_USE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ECC_USE"]
    #[inline(always)]
    pub fn ecc_use(&self) -> ECC_USE_R {
        ECC_USE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "BSEC hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_HWCFGRrs;
impl crate::RegisterSpec for BSEC_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_hwcfgr::R`](R) reader structure"]
impl crate::Readable for BSEC_HWCFGRrs {}
#[doc = "`reset()` method sets BSEC_HWCFGR to value 0x14"]
impl crate::Resettable for BSEC_HWCFGRrs {
    const RESET_VALUE: u32 = 0x14;
}
