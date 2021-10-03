#[doc = "Register `TZC_REGION_ATTRIBUTE1` reader"]
pub struct R(crate::R<TZC_REGION_ATTRIBUTE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_ATTRIBUTE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_ATTRIBUTE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_ATTRIBUTE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_REGION_ATTRIBUTE1` writer"]
pub struct W(crate::W<TZC_REGION_ATTRIBUTE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_REGION_ATTRIBUTE1_SPEC>;
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
impl From<crate::W<TZC_REGION_ATTRIBUTE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_REGION_ATTRIBUTE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_EN` reader - FILTER_EN"]
pub struct FILTER_EN_R(crate::FieldReader<u8, u8>);
impl FILTER_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_EN` writer - FILTER_EN"]
pub struct FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `S_RD_EN` reader - S_RD_EN"]
pub struct S_RD_EN_R(crate::FieldReader<bool, bool>);
impl S_RD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_RD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_RD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_RD_EN` writer - S_RD_EN"]
pub struct S_RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RD_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `S_WR_EN` reader - S_WR_EN"]
pub struct S_WR_EN_R(crate::FieldReader<bool, bool>);
impl S_WR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_WR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_WR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_WR_EN` writer - S_WR_EN"]
pub struct S_WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_WR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    pub fn s_rd_en(&self) -> S_RD_EN_R {
        S_RD_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    pub fn s_wr_en(&self) -> S_WR_EN_R {
        S_WR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W {
        FILTER_EN_W { w: self }
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    pub fn s_rd_en(&mut self) -> S_RD_EN_W {
        S_RD_EN_W { w: self }
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    pub fn s_wr_en(&mut self) -> S_WR_EN_W {
        S_WR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute1](index.html) module"]
pub struct TZC_REGION_ATTRIBUTE1_SPEC;
impl crate::RegisterSpec for TZC_REGION_ATTRIBUTE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_attribute1::R](R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute1::W](W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZC_REGION_ATTRIBUTE1 to value 0"]
impl crate::Resettable for TZC_REGION_ATTRIBUTE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
