///Register `ITLINE16` reader
pub type R = crate::R<ITLINE16rs>;
///Field `TIM3` reader - Timer 3 interrupt request pending
pub type TIM3_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 3 interrupt request pending
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE16")
            .field("tim3", &self.tim3())
            .finish()
    }
}
/**SYSCFG interrupt line 16 status register

You can [`read`](crate::Reg::read) this register and get [`itline16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:ITLINE16)*/
pub struct ITLINE16rs;
impl crate::RegisterSpec for ITLINE16rs {
    type Ux = u32;
}
///`read()` method returns [`itline16::R`](R) reader structure
impl crate::Readable for ITLINE16rs {}
///`reset()` method sets ITLINE16 to value 0
impl crate::Resettable for ITLINE16rs {
    const RESET_VALUE: u32 = 0;
}
