#[doc = "Register `MACL3A21R` reader"]
pub struct R(crate::R<MACL3A21R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL3A21R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL3A21R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL3A21R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACL3A21R` writer"]
pub struct W(crate::W<MACL3A21R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL3A21R_SPEC>;
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
impl From<crate::W<MACL3A21R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL3A21R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3A21` reader - Layer 3 Address 2 Field"]
pub struct L3A21_R(crate::FieldReader<u32, u32>);
impl L3A21_R {
    pub(crate) fn new(bits: u32) -> Self {
        L3A21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3A21_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3A21` writer - Layer 3 Address 2 Field"]
pub struct L3A21_W<'a> {
    w: &'a mut W,
}
impl<'a> L3A21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field"]
    #[inline(always)]
    pub fn l3a21(&self) -> L3A21_R {
        L3A21_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field"]
    #[inline(always)]
    pub fn l3a21(&mut self) -> L3A21_W {
        L3A21_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer3 address 2 filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a21r](index.html) module"]
pub struct MACL3A21R_SPEC;
impl crate::RegisterSpec for MACL3A21R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macl3a21r::R](R) reader structure"]
impl crate::Readable for MACL3A21R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macl3a21r::W](W) writer structure"]
impl crate::Writable for MACL3A21R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACL3A21R to value 0"]
impl crate::Resettable for MACL3A21R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}