#[doc = "Register `TTCPT` reader"]
pub type R = crate::R<TTCPTrs>;
#[doc = "Field `CCV` reader - Cycle Count Value"]
pub type CCV_R = crate::FieldReader;
#[doc = "Field `SWV` reader - Stop Watch Value"]
pub type SWV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Stop Watch Value"]
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Capture Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcpt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTCPTrs;
impl crate::RegisterSpec for TTCPTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttcpt::R`](R) reader structure"]
impl crate::Readable for TTCPTrs {}
#[doc = "`reset()` method sets TTCPT to value 0"]
impl crate::Resettable for TTCPTrs {
    const RESET_VALUE: u32 = 0;
}
