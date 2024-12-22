///Register `SYSCFG_ITLINE30` reader
pub type R = crate::R<SYSCFG_ITLINE30rs>;
///Field `USART4` reader - USART4 interrupt request pending
pub type USART4_R = crate::BitReader;
///Field `LPUART3` reader - LPUART3 interrupt request pending (EXTI line 32)
pub type LPUART3_R = crate::BitReader;
impl R {
    ///Bit 0 - USART4 interrupt request pending
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPUART3 interrupt request pending (EXTI line 32)
    #[inline(always)]
    pub fn lpuart3(&self) -> LPUART3_R {
        LPUART3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE30")
            .field("usart4", &self.usart4())
            .field("lpuart3", &self.lpuart3())
            .finish()
    }
}
/**SYSCFG interrupt line 30 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:SYSCFG_ITLINE30)*/
pub struct SYSCFG_ITLINE30rs;
impl crate::RegisterSpec for SYSCFG_ITLINE30rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline30::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE30rs {}
///`reset()` method sets SYSCFG_ITLINE30 to value 0
impl crate::Resettable for SYSCFG_ITLINE30rs {
    const RESET_VALUE: u32 = 0;
}
