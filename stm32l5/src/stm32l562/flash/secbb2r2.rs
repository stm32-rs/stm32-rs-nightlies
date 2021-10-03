#[doc = "Register `SECBB2R2` reader"]
pub struct R(crate::R<SECBB2R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECBB2R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECBB2R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECBB2R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECBB2R2` writer"]
pub struct W(crate::W<SECBB2R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECBB2R2_SPEC>;
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
impl From<crate::W<SECBB2R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECBB2R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECBB2` reader - SECBB2"]
pub struct SECBB2_R(crate::FieldReader<u32, u32>);
impl SECBB2_R {
    pub(crate) fn new(bits: u32) -> Self {
        SECBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECBB2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECBB2` writer - SECBB2"]
pub struct SECBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SECBB2"]
    #[inline(always)]
    pub fn secbb2(&self) -> SECBB2_R {
        SECBB2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SECBB2"]
    #[inline(always)]
    pub fn secbb2(&mut self) -> SECBB2_W {
        SECBB2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure block based bank 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb2r2](index.html) module"]
pub struct SECBB2R2_SPEC;
impl crate::RegisterSpec for SECBB2R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secbb2r2::R](R) reader structure"]
impl crate::Readable for SECBB2R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secbb2r2::W](W) writer structure"]
impl crate::Writable for SECBB2R2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECBB2R2 to value 0"]
impl crate::Resettable for SECBB2R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
