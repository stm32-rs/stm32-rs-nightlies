///Register `SYSCFG_ITLINE29` reader
pub type R = crate::R<SYSCFG_ITLINE29rs>;
///Field `USART3` reader - USART3 interrupt request pending
pub type USART3_R = crate::BitReader;
///Field `LPUART1` reader - LPUART1 interrupt request pending (EXTI line 30)
pub type LPUART1_R = crate::BitReader;
impl R {
    ///Bit 0 - USART3 interrupt request pending
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPUART1 interrupt request pending (EXTI line 30)
    #[inline(always)]
    pub fn lpuart1(&self) -> LPUART1_R {
        LPUART1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE29")
            .field("usart3", &self.usart3())
            .field("lpuart1", &self.lpuart1())
            .finish()
    }
}
/**SYSCFG interrupt line 29 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#SYSCFG:SYSCFG_ITLINE29)*/
pub struct SYSCFG_ITLINE29rs;
impl crate::RegisterSpec for SYSCFG_ITLINE29rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline29::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE29rs {}
///`reset()` method sets SYSCFG_ITLINE29 to value 0
impl crate::Resettable for SYSCFG_ITLINE29rs {
    const RESET_VALUE: u32 = 0;
}
