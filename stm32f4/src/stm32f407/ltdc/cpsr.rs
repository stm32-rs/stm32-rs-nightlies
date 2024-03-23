#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CPSRrs>;
#[doc = "Field `CYPOS` reader - Current Y Position"]
pub type CYPOS_R = crate::FieldReader<u16>;
#[doc = "Field `CXPOS` reader - Current X Position"]
pub type CXPOS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Current Y Position"]
    #[inline(always)]
    pub fn cypos(&self) -> CYPOS_R {
        CYPOS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current X Position"]
    #[inline(always)]
    pub fn cxpos(&self) -> CXPOS_R {
        CXPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Current Position Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPSRrs;
impl crate::RegisterSpec for CPSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CPSRrs {}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CPSRrs {
    const RESET_VALUE: u32 = 0;
}
