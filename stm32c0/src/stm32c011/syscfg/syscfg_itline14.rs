#[doc = "Register `SYSCFG_ITLINE14` reader"]
pub type R = crate::R<SYSCFG_ITLINE14rs>;
#[doc = "Field `TIM1_CC` reader - Timer 1 capture compare interrupt request pending"]
pub type TIM1_CC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 1 capture compare interrupt request pending"]
    #[inline(always)]
    pub fn tim1_cc(&self) -> TIM1_CC_R {
        TIM1_CC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE14rs;
impl crate::RegisterSpec for SYSCFG_ITLINE14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline14::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE14rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE14 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE14rs {
    const RESET_VALUE: u32 = 0;
}
