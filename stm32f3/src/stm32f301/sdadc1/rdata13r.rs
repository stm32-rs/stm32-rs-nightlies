#[doc = "Register `RDATA13R` reader"]
pub struct R(crate::R<RDATA13R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA13R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA13R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA13R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA3` reader - Regular conversion data for SDADC3"]
pub struct RDATA3_R(crate::FieldReader<u16, u16>);
impl RDATA3_R {
    pub(crate) fn new(bits: u16) -> Self {
        RDATA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATA3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDATA1` reader - Regular conversion data for SDADC1"]
pub struct RDATA1_R(crate::FieldReader<u16, u16>);
impl RDATA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        RDATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - Regular conversion data for SDADC3"]
    #[inline(always)]
    pub fn rdata3(&self) -> RDATA3_R {
        RDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Regular conversion data for SDADC1"]
    #[inline(always)]
    pub fn rdata1(&self) -> RDATA1_R {
        RDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SDADC1 and SDADC3 regular data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata13r](index.html) module"]
pub struct RDATA13R_SPEC;
impl crate::RegisterSpec for RDATA13R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdata13r::R](R) reader structure"]
impl crate::Readable for RDATA13R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDATA13R to value 0"]
impl crate::Resettable for RDATA13R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
