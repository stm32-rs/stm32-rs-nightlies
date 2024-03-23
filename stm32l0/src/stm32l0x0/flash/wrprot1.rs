#[doc = "Register `WRPROT1` reader"]
pub type R = crate::R<WRPROT1rs>;
#[doc = "Field `WRPROT1` reader - Write protection"]
pub type WRPROT1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrprot1(&self) -> WRPROT1_R {
        WRPROT1_R::new(self.bits)
    }
}
#[doc = "Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrprot1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPROT1rs;
impl crate::RegisterSpec for WRPROT1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrprot1::R`](R) reader structure"]
impl crate::Readable for WRPROT1rs {}
#[doc = "`reset()` method sets WRPROT1 to value 0"]
impl crate::Resettable for WRPROT1rs {
    const RESET_VALUE: u32 = 0;
}
