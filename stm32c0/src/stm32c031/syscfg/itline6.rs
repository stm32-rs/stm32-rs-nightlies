///Register `ITLINE6` reader
pub type R = crate::R<ITLINE6rs>;
///Field `EXTI2` reader - EXTI line 2 interrupt request pending
pub type EXTI2_R = crate::BitReader;
///Field `EXTI3` reader - EXTI line 3 interrupt request pending
pub type EXTI3_R = crate::BitReader;
impl R {
    ///Bit 0 - EXTI line 2 interrupt request pending
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EXTI line 3 interrupt request pending
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE6")
            .field("exti2", &self.exti2())
            .field("exti3", &self.exti3())
            .finish()
    }
}
/**SYSCFG interrupt line 6 status register

You can [`read`](crate::Reg::read) this register and get [`itline6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE6)*/
pub struct ITLINE6rs;
impl crate::RegisterSpec for ITLINE6rs {
    type Ux = u32;
}
///`read()` method returns [`itline6::R`](R) reader structure
impl crate::Readable for ITLINE6rs {}
///`reset()` method sets ITLINE6 to value 0
impl crate::Resettable for ITLINE6rs {
    const RESET_VALUE: u32 = 0;
}
