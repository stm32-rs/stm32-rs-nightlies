///Register `SYSCFG_ITLINE24` reader
pub type R = crate::R<SYSCFG_ITLINE24rs>;
///Field `I2C2` reader - I2C2 interrupt request pending
pub type I2C2_R = crate::BitReader;
impl R {
    ///Bit 0 - I2C2 interrupt request pending
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE24")
            .field("i2c2", &self.i2c2())
            .finish()
    }
}
/**SYSCFG interrupt line 24 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SYSCFG:SYSCFG_ITLINE24)*/
pub struct SYSCFG_ITLINE24rs;
impl crate::RegisterSpec for SYSCFG_ITLINE24rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline24::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE24rs {}
///`reset()` method sets SYSCFG_ITLINE24 to value 0
impl crate::Resettable for SYSCFG_ITLINE24rs {}
