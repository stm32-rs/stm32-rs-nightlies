///Register `SYSCFG_ITLINE19` reader
pub type R = crate::R<SYSCFG_ITLINE19rs>;
///Field `TIM15` reader - Timer 15 interrupt request pending
pub type TIM15_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 15 interrupt request pending
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE19")
            .field("tim15", &self.tim15())
            .finish()
    }
}
/**SYSCFG interrupt line 19 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#SYSCFG:SYSCFG_ITLINE19)*/
pub struct SYSCFG_ITLINE19rs;
impl crate::RegisterSpec for SYSCFG_ITLINE19rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline19::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE19rs {}
///`reset()` method sets SYSCFG_ITLINE19 to value 0
impl crate::Resettable for SYSCFG_ITLINE19rs {
    const RESET_VALUE: u32 = 0;
}
