#[doc = "Register `EXTI_HWCFGR4` reader"]
pub type R = crate::R<EXTI_HWCFGR4rs>;
#[doc = "Field `EVENT_TRG` reader - EVENT_TRG"]
pub type EVENT_TRG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - EVENT_TRG"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
#[doc = "EXTI hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_hwcfgr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_HWCFGR4rs;
impl crate::RegisterSpec for EXTI_HWCFGR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_hwcfgr4::R`](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR4rs {}
#[doc = "`reset()` method sets EXTI_HWCFGR4 to value 0x0001_ffff"]
impl crate::Resettable for EXTI_HWCFGR4rs {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
