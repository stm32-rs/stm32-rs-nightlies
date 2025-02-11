///Register `ITLINE27` reader
pub type R = crate::R<ITLINE27rs>;
///Field `USART1` reader - USART1 interrupt request pending, combined with EXTI line 25
pub type USART1_R = crate::BitReader;
impl R {
    ///Bit 0 - USART1 interrupt request pending, combined with EXTI line 25
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE27")
            .field("usart1", &self.usart1())
            .finish()
    }
}
/**SYSCFG interrupt line 27 status register

You can [`read`](crate::Reg::read) this register and get [`itline27::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:ITLINE27)*/
pub struct ITLINE27rs;
impl crate::RegisterSpec for ITLINE27rs {
    type Ux = u32;
}
///`read()` method returns [`itline27::R`](R) reader structure
impl crate::Readable for ITLINE27rs {}
///`reset()` method sets ITLINE27 to value 0
impl crate::Resettable for ITLINE27rs {
    const RESET_VALUE: u32 = 0;
}
