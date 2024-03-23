#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RXDRrs>;
#[doc = "Field `RXDATA` reader - 8-bit receive data Data byte received from the I2C bus"]
pub type RXDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit receive data Data byte received from the I2C bus"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDRrs;
impl crate::RegisterSpec for RXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RXDRrs {}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RXDRrs {
    const RESET_VALUE: u32 = 0;
}
