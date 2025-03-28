///Register `ITLINE18` reader
pub type R = crate::R<ITLINE18rs>;
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
        f.debug_struct("ITLINE18")
            .field("tim7", &self.tim7())
            .field("lptim2", &self.lptim2())
            .finish()
    }
}
/**SYSCFG interrupt line 18 status register

You can [`read`](crate::Reg::read) this register and get [`itline18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#SYSCFG:ITLINE18)*/
pub struct ITLINE18rs;
impl crate::RegisterSpec for ITLINE18rs {
    type Ux = u32;
}
///`read()` method returns [`itline18::R`](R) reader structure
impl crate::Readable for ITLINE18rs {}
///`reset()` method sets ITLINE18 to value 0
impl crate::Resettable for ITLINE18rs {}
