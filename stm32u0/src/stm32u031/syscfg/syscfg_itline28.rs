///Register `SYSCFG_ITLINE28` reader
pub type R = crate::R<SYSCFG_ITLINE28rs>;
///Field `USART2` reader - USART2 interrupt request pending (EXTI line 35)
pub type USART2_R = crate::BitReader;
///Field `LPUART2` reader - LPUART2 interrupt request pending (EXTI line 31)
pub type LPUART2_R = crate::BitReader;
impl R {
    ///Bit 0 - USART2 interrupt request pending (EXTI line 35)
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPUART2 interrupt request pending (EXTI line 31)
    #[inline(always)]
    pub fn lpuart2(&self) -> LPUART2_R {
        LPUART2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE28")
            .field("usart2", &self.usart2())
            .field("lpuart2", &self.lpuart2())
            .finish()
    }
}
/**SYSCFG interrupt line 28 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#SYSCFG:SYSCFG_ITLINE28)*/
pub struct SYSCFG_ITLINE28rs;
impl crate::RegisterSpec for SYSCFG_ITLINE28rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline28::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE28rs {}
///`reset()` method sets SYSCFG_ITLINE28 to value 0
impl crate::Resettable for SYSCFG_ITLINE28rs {
    const RESET_VALUE: u32 = 0;
}
