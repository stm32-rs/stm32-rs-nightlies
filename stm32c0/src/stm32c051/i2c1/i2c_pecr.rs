///Register `I2C_PECR` reader
pub type R = crate::R<I2C_PECRrs>;
///Field `PEC` reader - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE = 0.
pub type PEC_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_PECR")
            .field("pec", &self.pec())
            .finish()
    }
}
/**I2C PEC register

You can [`read`](crate::Reg::read) this register and get [`i2c_pecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#I2C1:I2C_PECR)*/
pub struct I2C_PECRrs;
impl crate::RegisterSpec for I2C_PECRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_pecr::R`](R) reader structure
impl crate::Readable for I2C_PECRrs {}
///`reset()` method sets I2C_PECR to value 0
impl crate::Resettable for I2C_PECRrs {}
