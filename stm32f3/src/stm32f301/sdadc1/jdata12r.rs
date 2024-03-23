#[doc = "Register `JDATA12R` reader"]
pub type R = crate::R<JDATA12Rrs>;
#[doc = "Field `JDATA1` reader - Injected group conversion data for SDADC1"]
pub type JDATA1_R = crate::FieldReader<u16>;
#[doc = "Field `JDATA2` reader - Injected group conversion data for SDADC2"]
pub type JDATA2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected group conversion data for SDADC1"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Injected group conversion data for SDADC2"]
    #[inline(always)]
    pub fn jdata2(&self) -> JDATA2_R {
        JDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SDADC1 and SDADC2 injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdata12r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDATA12Rrs;
impl crate::RegisterSpec for JDATA12Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdata12r::R`](R) reader structure"]
impl crate::Readable for JDATA12Rrs {}
#[doc = "`reset()` method sets JDATA12R to value 0"]
impl crate::Resettable for JDATA12Rrs {
    const RESET_VALUE: u32 = 0;
}
