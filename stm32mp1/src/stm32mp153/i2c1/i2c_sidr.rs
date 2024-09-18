///Register `I2C_SIDR` reader
pub type R = crate::R<I2C_SIDRrs>;
///Field `SID` reader - SID
pub type SID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SIDR")
            .field("sid", &self.sid())
            .finish()
    }
}
/**I2C size identification register

You can [`read`](crate::Reg::read) this register and get [`i2c_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_SIDR)*/
pub struct I2C_SIDRrs;
impl crate::RegisterSpec for I2C_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_sidr::R`](R) reader structure
impl crate::Readable for I2C_SIDRrs {}
///`reset()` method sets I2C_SIDR to value 0xa3c5_dd01
impl crate::Resettable for I2C_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
