#[doc = "Register `LTDC_CPSR` reader"]
pub type R = crate::R<LTDC_CPSRrs>;
#[doc = "Field `CYPOS` reader - current Y position These bits return the current Y position."]
pub type CYPOS_R = crate::FieldReader<u16>;
#[doc = "Field `CXPOS` reader - current X position These bits return the current X position."]
pub type CXPOS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - current Y position These bits return the current Y position."]
    #[inline(always)]
    pub fn cypos(&self) -> CYPOS_R {
        CYPOS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - current X position These bits return the current X position."]
    #[inline(always)]
    pub fn cxpos(&self) -> CXPOS_R {
        CXPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_cpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_CPSRrs;
impl crate::RegisterSpec for LTDC_CPSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_cpsr::R`](R) reader structure"]
impl crate::Readable for LTDC_CPSRrs {}
#[doc = "`reset()` method sets LTDC_CPSR to value 0"]
impl crate::Resettable for LTDC_CPSRrs {
    const RESET_VALUE: u32 = 0;
}
