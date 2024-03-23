#[doc = "Register `MDIOS_RDFR` reader"]
pub type R = crate::R<MDIOS_RDFRrs>;
#[doc = "Field `RDF` reader - RDF"]
pub type RDF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RDF"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(self.bits)
    }
}
#[doc = "MDIOS read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_rdfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_RDFRrs;
impl crate::RegisterSpec for MDIOS_RDFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_rdfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_RDFRrs {}
#[doc = "`reset()` method sets MDIOS_RDFR to value 0"]
impl crate::Resettable for MDIOS_RDFRrs {
    const RESET_VALUE: u32 = 0;
}
