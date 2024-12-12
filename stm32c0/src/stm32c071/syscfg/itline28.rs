///Register `ITLINE28` reader
pub type R = crate::R<ITLINE28rs>;
///Field `USART2` reader - USART2 interrupt request pending (EXTI line 26)
pub type USART2_R = crate::BitReader;
impl R {
    ///Bit 0 - USART2 interrupt request pending (EXTI line 26)
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE28")
            .field("usart2", &self.usart2())
            .finish()
    }
}
/**SYSCFG interrupt line 28 status register

You can [`read`](crate::Reg::read) this register and get [`itline28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#SYSCFG:ITLINE28)*/
pub struct ITLINE28rs;
impl crate::RegisterSpec for ITLINE28rs {
    type Ux = u32;
}
///`read()` method returns [`itline28::R`](R) reader structure
impl crate::Readable for ITLINE28rs {}
///`reset()` method sets ITLINE28 to value 0
impl crate::Resettable for ITLINE28rs {
    const RESET_VALUE: u32 = 0;
}
