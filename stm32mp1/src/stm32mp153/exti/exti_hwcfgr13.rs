#[doc = "Register `EXTI_HWCFGR13` reader"]
pub type R = crate::R<EXTI_HWCFGR13rs>;
#[doc = "Field `TZ` reader - TZ"]
pub type TZ_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TZ"]
    #[inline(always)]
    pub fn tz(&self) -> TZ_R {
        TZ_R::new(self.bits)
    }
}
#[doc = "EXTI hardware configuration register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_HWCFGR13rs;
impl crate::RegisterSpec for EXTI_HWCFGR13rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_hwcfgr13::R`](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR13rs {}
#[doc = "`reset()` method sets EXTI_HWCFGR13 to value 0x050e_ffff"]
impl crate::Resettable for EXTI_HWCFGR13rs {
    const RESET_VALUE: u32 = 0x050e_ffff;
}
