///Register `MDIOS_RDFR` reader
pub type R = crate::R<MDIOS_RDFRrs>;
///Field `RDF` reader - RDF
pub type RDF_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RDF
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIOS_RDFR")
            .field("rdf", &self.rdf())
            .finish()
    }
}
/**MDIOS read flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_rdfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_RDFR)*/
pub struct MDIOS_RDFRrs;
impl crate::RegisterSpec for MDIOS_RDFRrs {
    type Ux = u32;
}
///`read()` method returns [`mdios_rdfr::R`](R) reader structure
impl crate::Readable for MDIOS_RDFRrs {}
///`reset()` method sets MDIOS_RDFR to value 0
impl crate::Resettable for MDIOS_RDFRrs {
    const RESET_VALUE: u32 = 0;
}
