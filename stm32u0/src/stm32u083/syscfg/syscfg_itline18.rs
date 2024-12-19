///Register `SYSCFG_ITLINE18` reader
pub type R = crate::R<SYSCFG_ITLINE18rs>;
///Field `TIM7` reader - Timer 7 interrupt request pending
pub type TIM7_R = crate::BitReader;
///Field `LPTIM2` reader - Low-power timer 2 interrupt request pending (EXTI line 30)
pub type LPTIM2_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 7 interrupt request pending
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Low-power timer 2 interrupt request pending (EXTI line 30)
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE18")
            .field("tim7", &self.tim7())
            .field("lptim2", &self.lptim2())
            .finish()
    }
}
/**SYSCFG interrupt line 18 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:SYSCFG_ITLINE18)*/
pub struct SYSCFG_ITLINE18rs;
impl crate::RegisterSpec for SYSCFG_ITLINE18rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline18::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE18rs {}
///`reset()` method sets SYSCFG_ITLINE18 to value 0
impl crate::Resettable for SYSCFG_ITLINE18rs {
    const RESET_VALUE: u32 = 0;
}
