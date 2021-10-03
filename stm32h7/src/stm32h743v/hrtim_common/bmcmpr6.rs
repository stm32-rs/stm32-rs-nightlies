#[doc = "Register `BMCMPR6` reader"]
pub struct R(crate::R<BMCMPR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCMPR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCMPR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCMPR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMCMPR6` writer"]
pub struct W(crate::W<BMCMPR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCMPR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BMCMPR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCMPR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMCMP` reader - BMCMP"]
pub struct BMCMP_R(crate::FieldReader<u16, u16>);
impl BMCMP_R {
    pub(crate) fn new(bits: u16) -> Self {
        BMCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMCMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMCMP` writer - BMCMP"]
pub struct BMCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> BMCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&mut self) -> BMCMP_W {
        BMCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMCMPR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcmpr6](index.html) module"]
pub struct BMCMPR6_SPEC;
impl crate::RegisterSpec for BMCMPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmcmpr6::R](R) reader structure"]
impl crate::Readable for BMCMPR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmcmpr6::W](W) writer structure"]
impl crate::Writable for BMCMPR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMCMPR6 to value 0"]
impl crate::Resettable for BMCMPR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
