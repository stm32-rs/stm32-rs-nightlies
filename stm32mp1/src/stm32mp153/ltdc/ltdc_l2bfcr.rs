#[doc = "Register `LTDC_L2BFCR` reader"]
pub struct R(crate::R<LTDC_L2BFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2BFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2BFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2BFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2BFCR` writer"]
pub struct W(crate::W<LTDC_L2BFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2BFCR_SPEC>;
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
impl From<crate::W<LTDC_L2BFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2BFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BF2` reader - BF2"]
pub struct BF2_R(crate::FieldReader<u8, u8>);
impl BF2_R {
    pub(crate) fn new(bits: u8) -> Self {
        BF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BF2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BF2` writer - BF2"]
pub struct BF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `BF1` reader - BF1"]
pub struct BF1_R(crate::FieldReader<u8, u8>);
impl BF1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BF1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BF1` writer - BF1"]
pub struct BF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BF2"]
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - BF1"]
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BF2"]
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W {
        BF2_W { w: self }
    }
    #[doc = "Bits 8:10 - BF1"]
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W {
        BF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2bfcr](index.html) module"]
pub struct LTDC_L2BFCR_SPEC;
impl crate::RegisterSpec for LTDC_L2BFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2bfcr::R](R) reader structure"]
impl crate::Readable for LTDC_L2BFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2bfcr::W](W) writer structure"]
impl crate::Writable for LTDC_L2BFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L2BFCR to value 0x0607"]
impl crate::Resettable for LTDC_L2BFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0607
    }
}
