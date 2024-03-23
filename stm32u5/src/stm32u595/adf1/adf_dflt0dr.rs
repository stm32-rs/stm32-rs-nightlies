#[doc = "Register `ADF_DFLT0DR` reader"]
pub type R = crate::R<ADF_DFLT0DRrs>;
#[doc = "Field `DR` reader - DR"]
pub type DR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 8:31 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "ADF digital filter data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_dflt0dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_DFLT0DRrs;
impl crate::RegisterSpec for ADF_DFLT0DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_dflt0dr::R`](R) reader structure"]
impl crate::Readable for ADF_DFLT0DRrs {}
#[doc = "`reset()` method sets ADF_DFLT0DR to value 0"]
impl crate::Resettable for ADF_DFLT0DRrs {
    const RESET_VALUE: u32 = 0;
}
