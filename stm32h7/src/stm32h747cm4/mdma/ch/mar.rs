#[doc = "Register `MAR` reader"]
pub struct R(crate::R<MAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAR` writer"]
pub struct W(crate::W<MAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAR_SPEC>;
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
impl From<crate::W<MAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAR` reader - Mask address"]
pub struct MAR_R(crate::FieldReader<u32, u32>);
impl MAR_R {
    pub(crate) fn new(bits: u32) -> Self {
        MAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAR` writer - Mask address"]
pub struct MAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    pub fn mar(&self) -> MAR_R {
        MAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    pub fn mar(&mut self) -> MAR_W {
        MAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mar](index.html) module"]
pub struct MAR_SPEC;
impl crate::RegisterSpec for MAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mar::R](R) reader structure"]
impl crate::Readable for MAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mar::W](W) writer structure"]
impl crate::Writable for MAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAR to value 0"]
impl crate::Resettable for MAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
