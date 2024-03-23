#[doc = "Register `HWCFGR2` reader"]
pub type R = crate::R<HWCFGR2rs>;
#[doc = "Field `PTIONREG_OUT` reader - PTIONREG_OUT"]
pub type PTIONREG_OUT_R = crate::FieldReader;
#[doc = "Field `TRUST_ZONE` reader - TRUST_ZONE"]
pub type TRUST_ZONE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PTIONREG_OUT"]
    #[inline(always)]
    pub fn ptionreg_out(&self) -> PTIONREG_OUT_R {
        PTIONREG_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "TAMP hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr2::R`](R) reader structure"]
impl crate::Readable for HWCFGR2rs {}
#[doc = "`reset()` method sets HWCFGR2 to value 0"]
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
