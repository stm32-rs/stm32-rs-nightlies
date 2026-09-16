///Register `SYSCFG_ITLINE16` reader
pub type R = crate::R<SYSCFG_ITLINE16rs>;
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
        f.debug_struct("SYSCFG_ITLINE16")
            .field("tim3", &self.tim3())
            .finish()
    }
}
/**SYSCFG interrupt line 16 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SYSCFG:SYSCFG_ITLINE16)*/
pub struct SYSCFG_ITLINE16rs;
impl crate::RegisterSpec for SYSCFG_ITLINE16rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline16::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE16rs {}
///`reset()` method sets SYSCFG_ITLINE16 to value 0
impl crate::Resettable for SYSCFG_ITLINE16rs {}
