#[doc = "Register `KEYR6` reader"]
pub struct R(crate::R<KEYR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR6` writer"]
pub struct W(crate::W<KEYR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR6_SPEC>;
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
impl From<crate::W<KEYR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_KEYR6` reader - AES key register (MSB key \\[223:192\\])"]
pub struct AES_KEYR6_R(crate::FieldReader<u32, u32>);
impl AES_KEYR6_R {
    pub(crate) fn new(bits: u32) -> Self {
        AES_KEYR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_KEYR6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_KEYR6` writer - AES key register (MSB key \\[223:192\\])"]
pub struct AES_KEYR6_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEYR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[223:192\\])"]
    #[inline(always)]
    pub fn aes_keyr6(&self) -> AES_KEYR6_R {
        AES_KEYR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[223:192\\])"]
    #[inline(always)]
    pub fn aes_keyr6(&mut self) -> AES_KEYR6_W {
        AES_KEYR6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr6](index.html) module"]
pub struct KEYR6_SPEC;
impl crate::RegisterSpec for KEYR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr6::R](R) reader structure"]
impl crate::Readable for KEYR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr6::W](W) writer structure"]
impl crate::Writable for KEYR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR6 to value 0"]
impl crate::Resettable for KEYR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
