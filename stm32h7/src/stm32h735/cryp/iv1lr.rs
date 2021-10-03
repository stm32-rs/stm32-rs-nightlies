#[doc = "Register `IV1LR` reader"]
pub struct R(crate::R<IV1LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV1LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV1LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV1LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV1LR` writer"]
pub struct W(crate::W<IV1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV1LR_SPEC>;
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
impl From<crate::W<IV1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV` reader - IV95"]
pub struct IV_R(crate::FieldReader<u32, u32>);
impl IV_R {
    pub(crate) fn new(bits: u32) -> Self {
        IV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV` writer - IV95"]
pub struct IV_W<'a> {
    w: &'a mut W,
}
impl<'a> IV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IV95"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV95"]
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W {
        IV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initialization vector register 1L\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv1lr](index.html) module"]
pub struct IV1LR_SPEC;
impl crate::RegisterSpec for IV1LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv1lr::R](R) reader structure"]
impl crate::Readable for IV1LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv1lr::W](W) writer structure"]
impl crate::Writable for IV1LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV1LR to value 0"]
impl crate::Resettable for IV1LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
