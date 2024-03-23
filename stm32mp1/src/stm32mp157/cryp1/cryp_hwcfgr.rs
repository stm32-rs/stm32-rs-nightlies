#[doc = "Register `CRYP_HWCFGR` reader"]
pub type R = crate::R<CRYP_HWCFGRrs>;
#[doc = "Field `CFG1` reader - CFG1"]
pub type CFG1_R = crate::FieldReader;
#[doc = "Field `CFG2` reader - CFG2"]
pub type CFG2_R = crate::FieldReader;
#[doc = "Field `CFG3` reader - CFG3"]
pub type CFG3_R = crate::FieldReader;
#[doc = "Field `CFG4` reader - CFG4"]
pub type CFG4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "CRYP hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_HWCFGRrs;
impl crate::RegisterSpec for CRYP_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_hwcfgr::R`](R) reader structure"]
impl crate::Readable for CRYP_HWCFGRrs {}
#[doc = "`reset()` method sets CRYP_HWCFGR to value 0x0131"]
impl crate::Resettable for CRYP_HWCFGRrs {
    const RESET_VALUE: u32 = 0x0131;
}
