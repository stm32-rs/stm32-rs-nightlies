#[doc = "Register `RSSCMDR` reader"]
pub struct R(crate::R<RSSCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSSCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSSCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSSCMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSSCMDR` writer"]
pub struct W(crate::W<RSSCMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSSCMDR_SPEC>;
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
impl From<crate::W<RSSCMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSSCMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSCMD` reader - RSS commands"]
pub struct RSSCMD_R(crate::FieldReader<u8, u8>);
impl RSSCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSSCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSCMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSCMD` writer - RSS commands"]
pub struct RSSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RSS commands"]
    #[inline(always)]
    pub fn rsscmd(&self) -> RSSCMD_R {
        RSSCMD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RSS commands"]
    #[inline(always)]
    pub fn rsscmd(&mut self) -> RSSCMD_W {
        RSSCMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSSCMDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsscmdr](index.html) module"]
pub struct RSSCMDR_SPEC;
impl crate::RegisterSpec for RSSCMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsscmdr::R](R) reader structure"]
impl crate::Readable for RSSCMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsscmdr::W](W) writer structure"]
impl crate::Writable for RSSCMDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSSCMDR to value 0"]
impl crate::Resettable for RSSCMDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
