#[doc = "Register `RDATA13R` reader"]
pub type R = crate::R<RDATA13Rrs>;
#[doc = "Field `RDATA1` reader - Regular conversion data for SDADC1"]
pub type RDATA1_R = crate::FieldReader<u16>;
#[doc = "Field `RDATA3` reader - Regular conversion data for SDADC3"]
pub type RDATA3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular conversion data for SDADC1"]
    #[inline(always)]
    pub fn rdata1(&self) -> RDATA1_R {
        RDATA1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Regular conversion data for SDADC3"]
    #[inline(always)]
    pub fn rdata3(&self) -> RDATA3_R {
        RDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SDADC1 and SDADC3 regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata13r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATA13Rrs;
impl crate::RegisterSpec for RDATA13Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata13r::R`](R) reader structure"]
impl crate::Readable for RDATA13Rrs {}
#[doc = "`reset()` method sets RDATA13R to value 0"]
impl crate::Resettable for RDATA13Rrs {
    const RESET_VALUE: u32 = 0;
}
