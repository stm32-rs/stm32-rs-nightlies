#[doc = "Register `SYSCFG_ITLINE19` reader"]
pub type R = crate::R<SYSCFG_ITLINE19rs>;
#[doc = "Field `TIM14` reader - Timer 14 interrupt request pending"]
pub type TIM14_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 14 interrupt request pending"]
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 19 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline19::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE19rs;
impl crate::RegisterSpec for SYSCFG_ITLINE19rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline19::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE19rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE19 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE19rs {
    const RESET_VALUE: u32 = 0;
}
