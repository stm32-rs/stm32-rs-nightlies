///Register `SYSCFG_ITLINE15` reader
pub type R = crate::R<SYSCFG_ITLINE15rs>;
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
        f.debug_struct("SYSCFG_ITLINE15")
            .field("tim2", &self.tim2())
            .finish()
    }
}
/**SYSCFG interrupt line 15 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SYSCFG:SYSCFG_ITLINE15)*/
pub struct SYSCFG_ITLINE15rs;
impl crate::RegisterSpec for SYSCFG_ITLINE15rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline15::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE15rs {}
///`reset()` method sets SYSCFG_ITLINE15 to value 0
impl crate::Resettable for SYSCFG_ITLINE15rs {}
