#[doc = "Register `SIDR` reader"]
pub type R = crate::R<SIDRrs>;
#[doc = "Field `ID` reader - Size Identification code"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Size Identification code"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "AES size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIDRrs;
impl crate::RegisterSpec for SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidr::R`](R) reader structure"]
impl crate::Readable for SIDRrs {}
#[doc = "`reset()` method sets SIDR to value 0x0017_0023"]
impl crate::Resettable for SIDRrs {
    const RESET_VALUE: u32 = 0x0017_0023;
}
