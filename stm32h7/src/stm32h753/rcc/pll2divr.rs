#[doc = "Register `PLL2DIVR` reader"]
pub struct R(crate::R<PLL2DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL2DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL2DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL2DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL2DIVR` writer"]
pub struct W(crate::W<PLL2DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL2DIVR_SPEC>;
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
impl From<crate::W<PLL2DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL2DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVN2` reader - Multiplication factor for PLL1 VCO"]
pub struct DIVN2_R(crate::FieldReader<u16, u16>);
impl DIVN2_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVN2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVN2` writer - Multiplication factor for PLL1 VCO"]
pub struct DIVN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `DIVP2` reader - PLL1 DIVP division factor"]
pub struct DIVP2_R(crate::FieldReader<u8, u8>);
impl DIVP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVP2` writer - PLL1 DIVP division factor"]
pub struct DIVP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `DIVQ2` reader - PLL1 DIVQ division factor"]
pub struct DIVQ2_R(crate::FieldReader<u8, u8>);
impl DIVQ2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVQ2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVQ2` writer - PLL1 DIVQ division factor"]
pub struct DIVQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `DIVR2` reader - PLL1 DIVR division factor"]
pub struct DIVR2_R(crate::FieldReader<u8, u8>);
impl DIVR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVR2` writer - PLL1 DIVR division factor"]
pub struct DIVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR2_W<'a> {
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
    pub fn divn2(&self) -> DIVN2_R {
        DIVN2_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&self) -> DIVP2_R {
        DIVP2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&self) -> DIVQ2_R {
        DIVQ2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&self) -> DIVR2_R {
        DIVR2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn2(&mut self) -> DIVN2_W {
        DIVN2_W { w: self }
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&mut self) -> DIVP2_W {
        DIVP2_W { w: self }
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&mut self) -> DIVQ2_W {
        DIVQ2_W { w: self }
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&mut self) -> DIVR2_W {
        DIVR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL2 Dividers Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2divr](index.html) module"]
pub struct PLL2DIVR_SPEC;
impl crate::RegisterSpec for PLL2DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll2divr::R](R) reader structure"]
impl crate::Readable for PLL2DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll2divr::W](W) writer structure"]
impl crate::Writable for PLL2DIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL2DIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_0280
    }
}
