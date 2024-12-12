///Register `RXDR` reader
pub type R = crate::R<RXDRrs>;
///Field `RXDATA` reader - 8-bit receive data Data byte received from the I2C bus
pub type RXDATA_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - 8-bit receive data Data byte received from the I2C bus
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDR")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
/**I2C receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#I2C1:RXDR)*/
pub struct RXDRrs;
impl crate::RegisterSpec for RXDRrs {
    type Ux = u32;
}
///`read()` method returns [`rxdr::R`](R) reader structure
impl crate::Readable for RXDRrs {}
///`reset()` method sets RXDR to value 0
impl crate::Resettable for RXDRrs {
    const RESET_VALUE: u32 = 0;
}
