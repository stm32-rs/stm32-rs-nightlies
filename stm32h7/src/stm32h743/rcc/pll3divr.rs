#[doc = "Register `PLL3DIVR` reader"]
pub struct R(crate::R<PLL3DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL3DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL3DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL3DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL3DIVR` writer"]
pub struct W(crate::W<PLL3DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL3DIVR_SPEC>;
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
impl From<crate::W<PLL3DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL3DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVN3` reader - Multiplication factor for PLL1 VCO"]
pub struct DIVN3_R(crate::FieldReader<u16, u16>);
impl DIVN3_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVN3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVN3` writer - Multiplication factor for PLL1 VCO"]
pub struct DIVN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `DIVP3` reader - PLL DIVP division factor"]
pub struct DIVP3_R(crate::FieldReader<u8, u8>);
impl DIVP3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVP3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVP3` writer - PLL DIVP division factor"]
pub struct DIVP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `DIVQ3` reader - PLL DIVQ division factor"]
pub struct DIVQ3_R(crate::FieldReader<u8, u8>);
impl DIVQ3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVQ3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVQ3` writer - PLL DIVQ division factor"]
pub struct DIVQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `DIVR3` reader - PLL DIVR division factor"]
pub struct DIVR3_R(crate::FieldReader<u8, u8>);
impl DIVR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVR3` writer - PLL DIVR division factor"]
pub struct DIVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn3(&self) -> DIVN3_R {
        DIVN3_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    pub fn divp3(&self) -> DIVP3_R {
        DIVP3_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    pub fn divq3(&self) -> DIVQ3_R {
        DIVQ3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    pub fn divr3(&self) -> DIVR3_R {
        DIVR3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn3(&mut self) -> DIVN3_W {
        DIVN3_W { w: self }
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    pub fn divp3(&mut self) -> DIVP3_W {
        DIVP3_W { w: self }
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    pub fn divq3(&mut self) -> DIVQ3_W {
        DIVQ3_W { w: self }
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    pub fn divr3(&mut self) -> DIVR3_W {
        DIVR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL3 Dividers Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll3divr](index.html) module"]
pub struct PLL3DIVR_SPEC;
impl crate::RegisterSpec for PLL3DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll3divr::R](R) reader structure"]
impl crate::Readable for PLL3DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll3divr::W](W) writer structure"]
impl crate::Writable for PLL3DIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL3DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL3DIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_0280
    }
}
