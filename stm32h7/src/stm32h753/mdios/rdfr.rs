///Register `RDFR` reader
pub type R = crate::R<RDFRrs>;
///Field `RDF` reader - Read flags for MDIO registers 0 to 31
pub type RDF_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Read flags for MDIO registers 0 to 31
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#MDIOS:RDFR)*/
pub struct RDFRrs;
impl crate::RegisterSpec for RDFRrs {
    type Ux = u32;
}
///`read()` method returns [`rdfr::R`](R) reader structure
impl crate::Readable for RDFRrs {}
///`reset()` method sets RDFR to value 0
impl crate::Resettable for RDFRrs {}
