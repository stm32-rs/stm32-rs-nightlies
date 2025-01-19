///Register `ITLINE14` reader
pub type R = crate::R<ITLINE14rs>;
///Field `TIM1_CC` reader - Timer 1 capture compare interrupt request pending
pub type TIM1_CC_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 1 capture compare interrupt request pending
    #[inline(always)]
    pub fn tim1_cc(&self) -> TIM1_CC_R {
        TIM1_CC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE14")
            .field("tim1_cc", &self.tim1_cc())
            .finish()
    }
}
/**SYSCFG interrupt line 14 status register

You can [`read`](crate::Reg::read) this register and get [`itline14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:ITLINE14)*/
pub struct ITLINE14rs;
impl crate::RegisterSpec for ITLINE14rs {
    type Ux = u32;
}
///`read()` method returns [`itline14::R`](R) reader structure
impl crate::Readable for ITLINE14rs {}
///`reset()` method sets ITLINE14 to value 0
impl crate::Resettable for ITLINE14rs {
    const RESET_VALUE: u32 = 0;
}
