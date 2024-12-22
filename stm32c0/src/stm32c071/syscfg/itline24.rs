///Register `ITLINE24` reader
pub type R = crate::R<ITLINE24rs>;
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
        f.debug_struct("ITLINE24")
            .field("i2c2", &self.i2c2())
            .finish()
    }
}
/**SYSCFG interrupt line 24 status register

You can [`read`](crate::Reg::read) this register and get [`itline24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#SYSCFG:ITLINE24)*/
pub struct ITLINE24rs;
impl crate::RegisterSpec for ITLINE24rs {
    type Ux = u32;
}
///`read()` method returns [`itline24::R`](R) reader structure
impl crate::Readable for ITLINE24rs {}
///`reset()` method sets ITLINE24 to value 0
impl crate::Resettable for ITLINE24rs {
    const RESET_VALUE: u32 = 0;
}
