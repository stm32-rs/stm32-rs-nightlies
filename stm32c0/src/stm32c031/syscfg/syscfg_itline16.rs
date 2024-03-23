#[doc = "Register `SYSCFG_ITLINE16` reader"]
pub type R = crate::R<SYSCFG_ITLINE16rs>;
#[doc = "Field `TIM3` reader - Timer 3 interrupt request pending"]
pub type TIM3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 3 interrupt request pending"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 16 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE16rs;
impl crate::RegisterSpec for SYSCFG_ITLINE16rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline16::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE16rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE16 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE16rs {
    const RESET_VALUE: u32 = 0;
}
