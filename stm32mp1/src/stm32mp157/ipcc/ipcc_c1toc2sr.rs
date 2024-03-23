#[doc = "Register `IPCC_C1TOC2SR` reader"]
pub type R = crate::R<IPCC_C1TOC2SRrs>;
#[doc = "Field `CHxF` reader - CHxF"]
pub type CHX_F_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - CHxF"]
    #[inline(always)]
    pub fn chx_f(&self) -> CHX_F_R {
        CHX_F_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "IPCC processor 1 to processor 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcc_c1toc2sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPCC_C1TOC2SRrs;
impl crate::RegisterSpec for IPCC_C1TOC2SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcc_c1toc2sr::R`](R) reader structure"]
impl crate::Readable for IPCC_C1TOC2SRrs {}
#[doc = "`reset()` method sets IPCC_C1TOC2SR to value 0"]
impl crate::Resettable for IPCC_C1TOC2SRrs {
    const RESET_VALUE: u32 = 0;
}
