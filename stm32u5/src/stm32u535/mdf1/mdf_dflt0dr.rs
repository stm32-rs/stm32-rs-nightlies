#[doc = "Register `MDF_DFLT0DR` reader"]
pub type R = crate::R<MDF_DFLT0DRrs>;
#[doc = "Field `DR` reader - Data processed by digital filter."]
pub type DR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 8:31 - Data processed by digital filter."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "This register is used to read the data processed by each digital filter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_dflt0dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_DFLT0DRrs;
impl crate::RegisterSpec for MDF_DFLT0DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_dflt0dr::R`](R) reader structure"]
impl crate::Readable for MDF_DFLT0DRrs {}
#[doc = "`reset()` method sets MDF_DFLT0DR to value 0"]
impl crate::Resettable for MDF_DFLT0DRrs {
    const RESET_VALUE: u32 = 0;
}
