#[doc = "Register `I2C_IPIDR` reader"]
pub type R = crate::R<I2C_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "I2C identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_IPIDRrs;
impl crate::RegisterSpec for I2C_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_ipidr::R`](R) reader structure"]
impl crate::Readable for I2C_IPIDRrs {}
#[doc = "`reset()` method sets I2C_IPIDR to value 0x0013_0012"]
impl crate::Resettable for I2C_IPIDRrs {
    const RESET_VALUE: u32 = 0x0013_0012;
}
