#[doc = "Register `VLCCR` reader"]
pub type R = crate::R<VLCCRrs>;
#[doc = "Field `HLINE` reader - Horizontal Line duration"]
pub type HLINE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Horizontal Line duration"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "DSI Host Video Line Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VLCCRrs;
impl crate::RegisterSpec for VLCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlccr::R`](R) reader structure"]
impl crate::Readable for VLCCRrs {}
#[doc = "`reset()` method sets VLCCR to value 0"]
impl crate::Resettable for VLCCRrs {
    const RESET_VALUE: u32 = 0;
}
