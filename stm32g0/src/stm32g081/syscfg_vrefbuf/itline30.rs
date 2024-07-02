///Register `ITLINE30` reader
pub type R = crate::R<ITLINE30rs>;
///Field `USART2` reader - CEC
pub type USART2_R = crate::BitReader;
impl R {
    ///Bit 0 - CEC
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE30")
            .field("usart2", &self.usart2())
            .finish()
    }
}
/**interrupt line 30 status register

You can [`read`](crate::Reg::read) this register and get [`itline30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#SYSCFG_VREFBUF:ITLINE30)*/
pub struct ITLINE30rs;
impl crate::RegisterSpec for ITLINE30rs {
    type Ux = u32;
}
///`read()` method returns [`itline30::R`](R) reader structure
impl crate::Readable for ITLINE30rs {}
///`reset()` method sets ITLINE30 to value 0
impl crate::Resettable for ITLINE30rs {
    const RESET_VALUE: u32 = 0;
}
