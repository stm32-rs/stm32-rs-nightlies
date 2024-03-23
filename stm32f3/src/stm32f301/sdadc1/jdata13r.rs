#[doc = "Register `JDATA13R` reader"]
pub type R = crate::R<JDATA13Rrs>;
#[doc = "Field `JDATA1` reader - Injected group conversion data for SDADC1"]
pub type JDATA1_R = crate::FieldReader<u16>;
#[doc = "Field `JDATA3` reader - Injected group conversion data for SDADC3"]
pub type JDATA3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected group conversion data for SDADC1"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Injected group conversion data for SDADC3"]
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SDADC1 and SDADC3 injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdata13r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDATA13Rrs;
impl crate::RegisterSpec for JDATA13Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdata13r::R`](R) reader structure"]
impl crate::Readable for JDATA13Rrs {}
#[doc = "`reset()` method sets JDATA13R to value 0"]
impl crate::Resettable for JDATA13Rrs {
    const RESET_VALUE: u32 = 0;
}
