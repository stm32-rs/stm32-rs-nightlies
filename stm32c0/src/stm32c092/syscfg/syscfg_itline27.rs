///Register `SYSCFG_ITLINE27` reader
pub type R = crate::R<SYSCFG_ITLINE27rs>;
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
        f.debug_struct("SYSCFG_ITLINE27")
            .field("usart1", &self.usart1())
            .finish()
    }
}
/**SYSCFG interrupt line 27 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline27::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SYSCFG:SYSCFG_ITLINE27)*/
pub struct SYSCFG_ITLINE27rs;
impl crate::RegisterSpec for SYSCFG_ITLINE27rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline27::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE27rs {}
///`reset()` method sets SYSCFG_ITLINE27 to value 0
impl crate::Resettable for SYSCFG_ITLINE27rs {}
