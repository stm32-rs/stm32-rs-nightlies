#[doc = "Register `SYSCFG_ITLINE28` reader"]
pub type R = crate::R<SYSCFG_ITLINE28rs>;
#[doc = "Field `USART2` reader - USART2 interrupt request pending (EXTI line 26)"]
pub type USART2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART2 interrupt request pending (EXTI line 26)"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 28 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline28::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE28rs;
impl crate::RegisterSpec for SYSCFG_ITLINE28rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline28::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE28rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE28 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE28rs {
    const RESET_VALUE: u32 = 0;
}
