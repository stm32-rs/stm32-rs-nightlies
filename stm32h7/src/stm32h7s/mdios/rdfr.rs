///Register `RDFR` reader
pub type R = crate::R<RDFRrs>;
///Field `RDF` reader - read flags for MDIOS registers 0 to 31. Each bit is set by hardware when the MDIO master performs a read from the corresponding MDIOS register. An interrupt is generated if RDIE (in MDIOS_CR) is set. Each bit is cleared by software by writing 1 to the corresponding CRDF bit in the MDIOS_CRDFR register. For RDFx:
pub type RDF_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - read flags for MDIOS registers 0 to 31. Each bit is set by hardware when the MDIO master performs a read from the corresponding MDIOS register. An interrupt is generated if RDIE (in MDIOS_CR) is set. Each bit is cleared by software by writing 1 to the corresponding CRDF bit in the MDIOS_CRDFR register. For RDFx:
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDFR").field("rdf", &self.rdf()).finish()
    }
}
/**MDIOS read flag register

You can [`read`](crate::Reg::read) this register and get [`rdfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MDIOS:RDFR)*/
pub struct RDFRrs;
impl crate::RegisterSpec for RDFRrs {
    type Ux = u32;
}
///`read()` method returns [`rdfr::R`](R) reader structure
impl crate::Readable for RDFRrs {}
///`reset()` method sets RDFR to value 0
impl crate::Resettable for RDFRrs {}
