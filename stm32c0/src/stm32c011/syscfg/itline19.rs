///Register `ITLINE19` reader
pub type R = crate::R<ITLINE19rs>;
///Field `TIM14` reader - Timer 14 interrupt request pending
pub type TIM14_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 14 interrupt request pending
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE19")
            .field("tim14", &self.tim14())
            .finish()
    }
}
/**SYSCFG interrupt line 19 status register

You can [`read`](crate::Reg::read) this register and get [`itline19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:ITLINE19)*/
pub struct ITLINE19rs;
impl crate::RegisterSpec for ITLINE19rs {
    type Ux = u32;
}
///`read()` method returns [`itline19::R`](R) reader structure
impl crate::Readable for ITLINE19rs {}
///`reset()` method sets ITLINE19 to value 0
impl crate::Resettable for ITLINE19rs {
    const RESET_VALUE: u32 = 0;
}
