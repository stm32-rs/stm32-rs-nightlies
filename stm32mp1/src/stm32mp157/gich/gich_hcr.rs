#[doc = "Register `GICH_HCR` reader"]
pub struct R(crate::R<GICH_HCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_HCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_HCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_HCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICH_HCR` writer"]
pub struct W(crate::W<GICH_HCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICH_HCR_SPEC>;
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
impl From<crate::W<GICH_HCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICH_HCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EN"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `UIE` reader - UIE"]
pub struct UIE_R(crate::FieldReader<bool, bool>);
impl UIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIE` writer - UIE"]
pub struct UIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LRENPIE` reader - LRENPIE"]
pub struct LRENPIE_R(crate::FieldReader<bool, bool>);
impl LRENPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRENPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LRENPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRENPIE` writer - LRENPIE"]
pub struct LRENPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LRENPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `NPIE` reader - NPIE"]
pub struct NPIE_R(crate::FieldReader<bool, bool>);
impl NPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPIE` writer - NPIE"]
pub struct NPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `VGRP0EIE` reader - VGRP0EIE"]
pub struct VGRP0EIE_R(crate::FieldReader<bool, bool>);
impl VGRP0EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP0EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP0EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP0EIE` writer - VGRP0EIE"]
pub struct VGRP0EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP0EIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `VGRP0DIE` reader - VGRP0DIE"]
pub struct VGRP0DIE_R(crate::FieldReader<bool, bool>);
impl VGRP0DIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP0DIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP0DIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP0DIE` writer - VGRP0DIE"]
pub struct VGRP0DIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP0DIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `VGRP1EIE` reader - VGRP1EIE"]
pub struct VGRP1EIE_R(crate::FieldReader<bool, bool>);
impl VGRP1EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP1EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP1EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP1EIE` writer - VGRP1EIE"]
pub struct VGRP1EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP1EIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `VGRP1DIE` reader - VGRP1DIE"]
pub struct VGRP1DIE_R(crate::FieldReader<bool, bool>);
impl VGRP1DIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP1DIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP1DIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP1DIE` writer - VGRP1DIE"]
pub struct VGRP1DIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP1DIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `EOICOUNT` reader - EOICOUNT"]
pub struct EOICOUNT_R(crate::FieldReader<u8, u8>);
impl EOICOUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EOICOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOICOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOICOUNT` writer - EOICOUNT"]
pub struct EOICOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOICOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LRENPIE"]
    #[inline(always)]
    pub fn lrenpie(&self) -> LRENPIE_R {
        LRENPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NPIE"]
    #[inline(always)]
    pub fn npie(&self) -> NPIE_R {
        NPIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VGRP0EIE"]
    #[inline(always)]
    pub fn vgrp0eie(&self) -> VGRP0EIE_R {
        VGRP0EIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VGRP0DIE"]
    #[inline(always)]
    pub fn vgrp0die(&self) -> VGRP0DIE_R {
        VGRP0DIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VGRP1EIE"]
    #[inline(always)]
    pub fn vgrp1eie(&self) -> VGRP1EIE_R {
        VGRP1EIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VGRP1DIE"]
    #[inline(always)]
    pub fn vgrp1die(&self) -> VGRP1DIE_R {
        VGRP1DIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 27:31 - EOICOUNT"]
    #[inline(always)]
    pub fn eoicount(&self) -> EOICOUNT_R {
        EOICOUNT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W {
        UIE_W { w: self }
    }
    #[doc = "Bit 2 - LRENPIE"]
    #[inline(always)]
    pub fn lrenpie(&mut self) -> LRENPIE_W {
        LRENPIE_W { w: self }
    }
    #[doc = "Bit 3 - NPIE"]
    #[inline(always)]
    pub fn npie(&mut self) -> NPIE_W {
        NPIE_W { w: self }
    }
    #[doc = "Bit 4 - VGRP0EIE"]
    #[inline(always)]
    pub fn vgrp0eie(&mut self) -> VGRP0EIE_W {
        VGRP0EIE_W { w: self }
    }
    #[doc = "Bit 5 - VGRP0DIE"]
    #[inline(always)]
    pub fn vgrp0die(&mut self) -> VGRP0DIE_W {
        VGRP0DIE_W { w: self }
    }
    #[doc = "Bit 6 - VGRP1EIE"]
    #[inline(always)]
    pub fn vgrp1eie(&mut self) -> VGRP1EIE_W {
        VGRP1EIE_W { w: self }
    }
    #[doc = "Bit 7 - VGRP1DIE"]
    #[inline(always)]
    pub fn vgrp1die(&mut self) -> VGRP1DIE_W {
        VGRP1DIE_W { w: self }
    }
    #[doc = "Bits 27:31 - EOICOUNT"]
    #[inline(always)]
    pub fn eoicount(&mut self) -> EOICOUNT_W {
        EOICOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICH hypervisor control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_hcr](index.html) module"]
pub struct GICH_HCR_SPEC;
impl crate::RegisterSpec for GICH_HCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gich_hcr::R](R) reader structure"]
impl crate::Readable for GICH_HCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gich_hcr::W](W) writer structure"]
impl crate::Writable for GICH_HCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICH_HCR to value 0"]
impl crate::Resettable for GICH_HCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
