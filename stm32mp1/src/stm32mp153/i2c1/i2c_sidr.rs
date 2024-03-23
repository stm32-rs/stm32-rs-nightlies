#[doc = "Register `I2C_SIDR` reader"]
pub type R = crate::R<I2C_SIDRrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "I2C size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_SIDRrs;
impl crate::RegisterSpec for I2C_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_sidr::R`](R) reader structure"]
impl crate::Readable for I2C_SIDRrs {}
#[doc = "`reset()` method sets I2C_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for I2C_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
