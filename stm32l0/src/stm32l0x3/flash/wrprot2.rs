#[doc = "Register `WRPROT2` reader"]
pub struct R(crate::R<WRPROT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPROT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPROT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPROT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRPROT2` reader - Write Protection"]
pub struct WRPROT2_R(crate::FieldReader<u16, u16>);
impl WRPROT2_R {
    pub(crate) fn new(bits: u16) -> Self {
        WRPROT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRPROT2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Write Protection"]
    #[inline(always)]
    pub fn wrprot2(&self) -> WRPROT2_R {
        WRPROT2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Write Protection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrprot2](index.html) module"]
pub struct WRPROT2_SPEC;
impl crate::RegisterSpec for WRPROT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrprot2::R](R) reader structure"]
impl crate::Readable for WRPROT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRPROT2 to value 0"]
impl crate::Resettable for WRPROT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
