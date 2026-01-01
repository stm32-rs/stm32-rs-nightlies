///Register `I2C_RXDR` reader
pub type R = crate::R<I2C_RXDRrs>;
///Field `RXDATA` reader - 8-bit receive data Data byte received from the I<sup>2</sup>C bus
pub type RXDATA_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - 8-bit receive data Data byte received from the I<sup>2</sup>C bus
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_RXDR")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
/**I2C receive data register

You can [`read`](crate::Reg::read) this register and get [`i2c_rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#I2C1:I2C_RXDR)*/
pub struct I2C_RXDRrs;
impl crate::RegisterSpec for I2C_RXDRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_rxdr::R`](R) reader structure
impl crate::Readable for I2C_RXDRrs {}
///`reset()` method sets I2C_RXDR to value 0
impl crate::Resettable for I2C_RXDRrs {}
