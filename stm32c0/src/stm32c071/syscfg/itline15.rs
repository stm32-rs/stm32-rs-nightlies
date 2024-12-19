///Register `ITLINE15` reader
pub type R = crate::R<ITLINE15rs>;
///Field `TIM2` reader - TIM2 interrupt request pending
pub type TIM2_R = crate::BitReader;
impl R {
    ///Bit 0 - TIM2 interrupt request pending
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE15")
            .field("tim2", &self.tim2())
            .finish()
    }
}
/**SYSCFG interrupt line 15 status register

You can [`read`](crate::Reg::read) this register and get [`itline15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#SYSCFG:ITLINE15)*/
pub struct ITLINE15rs;
impl crate::RegisterSpec for ITLINE15rs {
    type Ux = u32;
}
///`read()` method returns [`itline15::R`](R) reader structure
impl crate::Readable for ITLINE15rs {}
///`reset()` method sets ITLINE15 to value 0
impl crate::Resettable for ITLINE15rs {
    const RESET_VALUE: u32 = 0;
}
