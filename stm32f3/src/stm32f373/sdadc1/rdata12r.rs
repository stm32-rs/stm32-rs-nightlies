#[doc = "Register `RDATA12R` reader"]
pub type R = crate::R<RDATA12Rrs>;
#[doc = "Field `RDATA1` reader - Regular conversion data for SDADC1"]
pub type RDATA1_R = crate::FieldReader<u16>;
#[doc = "Field `RDATA2` reader - Regular conversion data for SDADC2"]
pub type RDATA2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular conversion data for SDADC1"]
    #[inline(always)]
    pub fn rdata1(&self) -> RDATA1_R {
        RDATA1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Regular conversion data for SDADC2"]
    #[inline(always)]
    pub fn rdata2(&self) -> RDATA2_R {
        RDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SDADC1 and SDADC2 regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata12r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATA12Rrs;
impl crate::RegisterSpec for RDATA12Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata12r::R`](R) reader structure"]
impl crate::Readable for RDATA12Rrs {}
#[doc = "`reset()` method sets RDATA12R to value 0"]
impl crate::Resettable for RDATA12Rrs {
    const RESET_VALUE: u32 = 0;
}
