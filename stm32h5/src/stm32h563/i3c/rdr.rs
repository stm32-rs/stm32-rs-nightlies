#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDRrs>;
#[doc = "Field `RDB0` reader - 8-bit received data on I3C bus."]
pub type RDB0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit received data on I3C bus."]
    #[inline(always)]
    pub fn rdb0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I3C receive data byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDRrs;
impl crate::RegisterSpec for RDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDRrs {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDRrs {
    const RESET_VALUE: u32 = 0;
}
