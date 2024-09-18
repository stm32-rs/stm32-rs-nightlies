///Register `I2C_IPIDR` reader
pub type R = crate::R<I2C_IPIDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_IPIDR").field("id", &self.id()).finish()
    }
}
/**I2C identification register

You can [`read`](crate::Reg::read) this register and get [`i2c_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_IPIDR)*/
pub struct I2C_IPIDRrs;
impl crate::RegisterSpec for I2C_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_ipidr::R`](R) reader structure
impl crate::Readable for I2C_IPIDRrs {}
///`reset()` method sets I2C_IPIDR to value 0x0013_0012
impl crate::Resettable for I2C_IPIDRrs {
    const RESET_VALUE: u32 = 0x0013_0012;
}
