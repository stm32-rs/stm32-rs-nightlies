#[doc = "Register `DFSDM_IPIDR` reader"]
pub type R = crate::R<DFSDM_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "This register specifies the identification of DFSDM peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_IPIDRrs;
impl crate::RegisterSpec for DFSDM_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_ipidr::R`](R) reader structure"]
impl crate::Readable for DFSDM_IPIDRrs {}
#[doc = "`reset()` method sets DFSDM_IPIDR to value 0x0011_0031"]
impl crate::Resettable for DFSDM_IPIDRrs {
    const RESET_VALUE: u32 = 0x0011_0031;
}
