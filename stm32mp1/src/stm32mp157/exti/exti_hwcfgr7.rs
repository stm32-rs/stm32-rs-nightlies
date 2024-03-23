#[doc = "Register `EXTI_HWCFGR7` reader"]
pub type R = crate::R<EXTI_HWCFGR7rs>;
#[doc = "Field `CPUEVENT` reader - CPUEVENT"]
pub type CPUEVENT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CPUEVENT"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
#[doc = "EXTI hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_HWCFGR7rs;
impl crate::RegisterSpec for EXTI_HWCFGR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_hwcfgr7::R`](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR7rs {}
#[doc = "`reset()` method sets EXTI_HWCFGR7 to value 0x000e_ffff"]
impl crate::Resettable for EXTI_HWCFGR7rs {
    const RESET_VALUE: u32 = 0x000e_ffff;
}
