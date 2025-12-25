///Register `ITLINE19` reader
pub type R = crate::R<ITLINE19rs>;
///Field `TIM15` reader - Timer 15 interrupt request pending
pub type TIM15_R = crate::BitReader;
///Field `LPTIM3` reader - Low-power timer 3 interrupt request pending
pub type LPTIM3_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 15 interrupt request pending
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Low-power timer 3 interrupt request pending
    #[inline(always)]
    pub fn lptim3(&self) -> LPTIM3_R {
        LPTIM3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE19")
            .field("tim15", &self.tim15())
            .field("lptim3", &self.lptim3())
            .finish()
    }
}
/**SYSCFG interrupt line 19 status register

You can [`read`](crate::Reg::read) this register and get [`itline19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:ITLINE19)*/
pub struct ITLINE19rs;
impl crate::RegisterSpec for ITLINE19rs {
    type Ux = u32;
}
///`read()` method returns [`itline19::R`](R) reader structure
impl crate::Readable for ITLINE19rs {}
///`reset()` method sets ITLINE19 to value 0
impl crate::Resettable for ITLINE19rs {}
