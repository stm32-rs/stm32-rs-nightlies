#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RDATArs>;
#[doc = "Field `RDATA` reader - Read data When a read access to this register occurs, the read data are the contents of the Y output buffer at the address offset indicated by the READ pointer. The pointer address is automatically incremented after each read access."]
pub type RDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Read data When a read access to this register occurs, the read data are the contents of the Y output buffer at the address offset indicated by the READ pointer. The pointer address is automatically incremented after each read access."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FMAC read data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATArs;
impl crate::RegisterSpec for RDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata::R`](R) reader structure"]
impl crate::Readable for RDATArs {}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATArs {
    const RESET_VALUE: u32 = 0;
}
