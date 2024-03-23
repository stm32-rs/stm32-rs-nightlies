#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Field `SOF` reader - Synchronization overrun event flag"]
pub type SOF_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
