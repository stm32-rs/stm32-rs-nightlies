#[doc = "Register `S4M1AR` reader"]
pub struct R(crate::R<S4M1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S4M1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S4M1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S4M1AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S4M1AR` writer"]
pub struct W(crate::W<S4M1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S4M1AR_SPEC>;
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
impl From<crate::W<S4M1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S4M1AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M1A` reader - Memory 1 address (used in case of Double buffer mode)"]
pub struct M1A_R(crate::FieldReader<u32, u32>);
impl M1A_R {
    pub(crate) fn new(bits: u32) -> Self {
        M1A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M1A_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M1A` writer - Memory 1 address (used in case of Double buffer mode)"]
pub struct M1A_W<'a> {
    w: &'a mut W,
}
impl<'a> M1A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&mut self) -> M1A_W {
        M1A_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x memory 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s4m1ar](index.html) module"]
pub struct S4M1AR_SPEC;
impl crate::RegisterSpec for S4M1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s4m1ar::R](R) reader structure"]
impl crate::Readable for S4M1AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s4m1ar::W](W) writer structure"]
impl crate::Writable for S4M1AR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S4M1AR to value 0"]
impl crate::Resettable for S4M1AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
