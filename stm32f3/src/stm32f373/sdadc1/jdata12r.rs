#[doc = "Register `JDATA12R` reader"]
pub struct R(crate::R<JDATA12R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDATA12R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDATA12R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDATA12R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA2` reader - Injected group conversion data for SDADC2"]
pub struct JDATA2_R(crate::FieldReader<u16, u16>);
impl JDATA2_R {
    pub(crate) fn new(bits: u16) -> Self {
        JDATA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDATA1` reader - Injected group conversion data for SDADC1"]
pub struct JDATA1_R(crate::FieldReader<u16, u16>);
impl JDATA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        JDATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - Injected group conversion data for SDADC2"]
    #[inline(always)]
    pub fn jdata2(&self) -> JDATA2_R {
        JDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Injected group conversion data for SDADC1"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SDADC1 and SDADC2 injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdata12r](index.html) module"]
pub struct JDATA12R_SPEC;
impl crate::RegisterSpec for JDATA12R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdata12r::R](R) reader structure"]
impl crate::Readable for JDATA12R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDATA12R to value 0"]
impl crate::Resettable for JDATA12R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
