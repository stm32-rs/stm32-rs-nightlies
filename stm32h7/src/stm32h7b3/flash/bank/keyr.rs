#[doc = "Register `KEYR` reader"]
pub struct R(crate::R<KEYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR` writer"]
pub struct W(crate::W<KEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR_SPEC>;
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
impl From<crate::W<KEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYR` reader - Bank 1 access configuration unlock key"]
pub struct KEYR_R(crate::FieldReader<u32, u32>);
impl KEYR_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEYR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYR` writer - Bank 1 access configuration unlock key"]
pub struct KEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr(&self) -> KEYR_R {
        KEYR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr(&mut self) -> KEYR_W {
        KEYR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH key register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr](index.html) module"]
pub struct KEYR_SPEC;
impl crate::RegisterSpec for KEYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr::R](R) reader structure"]
impl crate::Readable for KEYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr::W](W) writer structure"]
impl crate::Writable for KEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR to value 0"]
impl crate::Resettable for KEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
