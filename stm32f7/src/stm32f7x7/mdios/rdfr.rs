#[doc = "Register `RDFR` reader"]
pub type R = crate::R<RDFRrs>;
#[doc = "Field `RDF` reader - Read flags for MDIO registers 0 to 31"]
pub type RDF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(self.bits)
    }
}
#[doc = "MDIOS read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDFRrs;
impl crate::RegisterSpec for RDFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdfr::R`](R) reader structure"]
impl crate::Readable for RDFRrs {}
#[doc = "`reset()` method sets RDFR to value 0"]
impl crate::Resettable for RDFRrs {
    const RESET_VALUE: u32 = 0;
}
