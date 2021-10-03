#[doc = "Register `R1NONCER0` reader"]
pub struct R(crate::R<R1NONCER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R1NONCER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R1NONCER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R1NONCER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R1NONCER0` writer"]
pub struct W(crate::W<R1NONCER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R1NONCER0_SPEC>;
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
impl From<crate::W<R1NONCER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R1NONCER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_NONCE` reader - REGx_NONCE"]
pub struct REGX_NONCE_R(crate::FieldReader<u32, u32>);
impl REGX_NONCE_R {
    pub(crate) fn new(bits: u32) -> Self {
        REGX_NONCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGX_NONCE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGx_NONCE` writer - REGx_NONCE"]
pub struct REGX_NONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_NONCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - REGx_NONCE"]
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - REGx_NONCE"]
    #[inline(always)]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W {
        REGX_NONCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region x nonce register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1noncer0](index.html) module"]
pub struct R1NONCER0_SPEC;
impl crate::RegisterSpec for R1NONCER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r1noncer0::R](R) reader structure"]
impl crate::Readable for R1NONCER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r1noncer0::W](W) writer structure"]
impl crate::Writable for R1NONCER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R1NONCER0 to value 0"]
impl crate::Resettable for R1NONCER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
