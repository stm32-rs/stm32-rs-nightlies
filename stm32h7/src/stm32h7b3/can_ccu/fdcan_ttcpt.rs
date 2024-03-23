#[doc = "Register `FDCAN_TTCPT` reader"]
pub type R = crate::R<FDCAN_TTCPTrs>;
#[doc = "Field `CT` reader - Cycle Count Value"]
pub type CT_R = crate::FieldReader;
#[doc = "Field `SWV` reader - Stop Watch Value"]
pub type SWV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Value"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Stop Watch Value"]
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Capture Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttcpt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTCPTrs;
impl crate::RegisterSpec for FDCAN_TTCPTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttcpt::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTCPTrs {}
#[doc = "`reset()` method sets FDCAN_TTCPT to value 0"]
impl crate::Resettable for FDCAN_TTCPTrs {
    const RESET_VALUE: u32 = 0;
}
