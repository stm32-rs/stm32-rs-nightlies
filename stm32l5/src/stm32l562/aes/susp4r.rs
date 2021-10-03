#[doc = "Register `SUSP4R` reader"]
pub struct R(crate::R<SUSP4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSP4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSP4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSP4R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUSP4R` writer"]
pub struct W(crate::W<SUSP4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP4R_SPEC>;
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
impl From<crate::W<SUSP4R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP4R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_SUSP4R` reader - AES suspend register 4"]
pub struct AES_SUSP4R_R(crate::FieldReader<u32, u32>);
impl AES_SUSP4R_R {
    pub(crate) fn new(bits: u32) -> Self {
        AES_SUSP4R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_SUSP4R_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_SUSP4R` writer - AES suspend register 4"]
pub struct AES_SUSP4R_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_SUSP4R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES suspend register 4"]
    #[inline(always)]
    pub fn aes_susp4r(&self) -> AES_SUSP4R_R {
        AES_SUSP4R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 4"]
    #[inline(always)]
    pub fn aes_susp4r(&mut self) -> AES_SUSP4R_W {
        AES_SUSP4R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES suspend register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp4r](index.html) module"]
pub struct SUSP4R_SPEC;
impl crate::RegisterSpec for SUSP4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [susp4r::R](R) reader structure"]
impl crate::Readable for SUSP4R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [susp4r::W](W) writer structure"]
impl crate::Writable for SUSP4R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUSP4R to value 0"]
impl crate::Resettable for SUSP4R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
