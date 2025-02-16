///Register `ITLINE5` reader
pub type R = crate::R<ITLINE5rs>;
///Field `EXTI0` reader - EXTI line 0 interrupt request pending
pub type EXTI0_R = crate::BitReader;
///Field `EXTI1` reader - EXTI line 1 interrupt request pending
pub type EXTI1_R = crate::BitReader;
impl R {
    ///Bit 0 - EXTI line 0 interrupt request pending
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EXTI line 1 interrupt request pending
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE5")
            .field("exti0", &self.exti0())
            .field("exti1", &self.exti1())
            .finish()
    }
}
/**SYSCFG interrupt line 5 status register

You can [`read`](crate::Reg::read) this register and get [`itline5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:ITLINE5)*/
pub struct ITLINE5rs;
impl crate::RegisterSpec for ITLINE5rs {
    type Ux = u32;
}
///`read()` method returns [`itline5::R`](R) reader structure
impl crate::Readable for ITLINE5rs {}
///`reset()` method sets ITLINE5 to value 0
impl crate::Resettable for ITLINE5rs {
    const RESET_VALUE: u32 = 0;
}
