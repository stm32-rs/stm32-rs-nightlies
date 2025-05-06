///Register `ITLINE20` reader
pub type R = crate::R<ITLINE20rs>;
///Field `TIM16` reader - Timer 16 interrupt request pending
pub type TIM16_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 16 interrupt request pending
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE20")
            .field("tim16", &self.tim16())
            .finish()
    }
}
/**SYSCFG interrupt line 20 status register

You can [`read`](crate::Reg::read) this register and get [`itline20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#SYSCFG:ITLINE20)*/
pub struct ITLINE20rs;
impl crate::RegisterSpec for ITLINE20rs {
    type Ux = u32;
}
///`read()` method returns [`itline20::R`](R) reader structure
impl crate::Readable for ITLINE20rs {}
///`reset()` method sets ITLINE20 to value 0
impl crate::Resettable for ITLINE20rs {}
