#[doc = "Register `RDFR` reader"]
pub struct R(crate::R<RDFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDF` reader - Read flags for MDIO registers 0 to 31"]
pub struct RDF_R(crate::FieldReader<u32, u32>);
impl RDF_R {
    pub(crate) fn new(bits: u32) -> Self {
        RDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Read flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "MDIOS read flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdfr](index.html) module"]
pub struct RDFR_SPEC;
impl crate::RegisterSpec for RDFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdfr::R](R) reader structure"]
impl crate::Readable for RDFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDFR to value 0"]
impl crate::Resettable for RDFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
