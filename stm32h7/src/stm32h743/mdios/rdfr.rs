#[doc = "Reader of register RDFR"]
pub type R = crate::R<u32, super::RDFR>;
#[doc = "Reader of field `RDF`"]
pub type RDF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
