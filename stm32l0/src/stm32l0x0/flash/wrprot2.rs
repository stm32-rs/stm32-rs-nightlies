#[doc = "Register `WRPROT2` reader"]
pub type R = crate::R<WRPROT2rs>;
#[doc = "Field `WRPROT2` reader - Write protection"]
pub type WRPROT2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Write protection"]
    #[inline(always)]
    pub fn wrprot2(&self) -> WRPROT2_R {
        WRPROT2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrprot2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPROT2rs;
impl crate::RegisterSpec for WRPROT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrprot2::R`](R) reader structure"]
impl crate::Readable for WRPROT2rs {}
#[doc = "`reset()` method sets WRPROT2 to value 0"]
impl crate::Resettable for WRPROT2rs {
    const RESET_VALUE: u32 = 0;
}
