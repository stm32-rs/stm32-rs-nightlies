#[doc = "Register `SYSCFG_ITLINE27` reader"]
pub type R = crate::R<SYSCFG_ITLINE27rs>;
#[doc = "Field `USART1` reader - USART1 interrupt request pending, combined with EXTI line 25"]
pub type USART1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART1 interrupt request pending, combined with EXTI line 25"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 27 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline27::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE27rs;
impl crate::RegisterSpec for SYSCFG_ITLINE27rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline27::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE27rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE27 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE27rs {
    const RESET_VALUE: u32 = 0;
}
