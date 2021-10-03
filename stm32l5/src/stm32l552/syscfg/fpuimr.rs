#[doc = "Register `FPUIMR` reader"]
pub struct R(crate::R<FPUIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPUIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPUIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPUIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPUIMR` writer"]
pub struct W(crate::W<FPUIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPUIMR_SPEC>;
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
impl From<crate::W<FPUIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPUIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPU_IE` reader - Floating point unit interrupts enable bits"]
pub struct FPU_IE_R(crate::FieldReader<u8, u8>);
impl FPU_IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FPU_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPU_IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_IE` writer - Floating point unit interrupts enable bits"]
pub struct FPU_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&mut self) -> FPU_IE_W {
        FPU_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FPU interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpuimr](index.html) module"]
pub struct FPUIMR_SPEC;
impl crate::RegisterSpec for FPUIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpuimr::R](R) reader structure"]
impl crate::Readable for FPUIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpuimr::W](W) writer structure"]
impl crate::Writable for FPUIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPUIMR to value 0x1f"]
impl crate::Resettable for FPUIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
