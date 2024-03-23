#[doc = "Register `I2C_PECR` reader"]
pub type R = crate::R<I2C_PECRrs>;
#[doc = "Field `PEC` reader - PEC"]
pub type PEC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_pecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_PECRrs;
impl crate::RegisterSpec for I2C_PECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_pecr::R`](R) reader structure"]
impl crate::Readable for I2C_PECRrs {}
#[doc = "`reset()` method sets I2C_PECR to value 0"]
impl crate::Resettable for I2C_PECRrs {
    const RESET_VALUE: u32 = 0;
}
