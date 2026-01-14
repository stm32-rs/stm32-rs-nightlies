///Register `SYSCFG_ITLINE23` reader
pub type R = crate::R<SYSCFG_ITLINE23rs>;
///Field `I2C1` reader - I2C1 interrupt request pending, combined with EXTI line 23
pub type I2C1_R = crate::BitReader;
impl R {
    ///Bit 0 - I2C1 interrupt request pending, combined with EXTI line 23
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE23")
            .field("i2c1", &self.i2c1())
            .finish()
    }
}
/**SYSCFG interrupt line 23 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE23)*/
pub struct SYSCFG_ITLINE23rs;
impl crate::RegisterSpec for SYSCFG_ITLINE23rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline23::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE23rs {}
///`reset()` method sets SYSCFG_ITLINE23 to value 0
impl crate::Resettable for SYSCFG_ITLINE23rs {}
