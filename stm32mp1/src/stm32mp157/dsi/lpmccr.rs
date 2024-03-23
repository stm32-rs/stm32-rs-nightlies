#[doc = "Register `LPMCCR` reader"]
pub type R = crate::R<LPMCCRrs>;
#[doc = "Field `VLPSIZE` reader - VLPSIZE"]
pub type VLPSIZE_R = crate::FieldReader;
#[doc = "Field `LPSIZE` reader - LPSIZE"]
pub type LPSIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCCRrs;
impl crate::RegisterSpec for LPMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmccr::R`](R) reader structure"]
impl crate::Readable for LPMCCRrs {}
#[doc = "`reset()` method sets LPMCCR to value 0"]
impl crate::Resettable for LPMCCRrs {
    const RESET_VALUE: u32 = 0;
}
