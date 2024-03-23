#[doc = "Register `SYSCFG_ITLINE22` reader"]
pub type R = crate::R<SYSCFG_ITLINE22rs>;
#[doc = "Field `TIM17` reader - Timer 17 interrupt request pending"]
pub type TIM17_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 17 interrupt request pending"]
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 22 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline22::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE22rs;
impl crate::RegisterSpec for SYSCFG_ITLINE22rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline22::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE22rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE22 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE22rs {
    const RESET_VALUE: u32 = 0;
}
