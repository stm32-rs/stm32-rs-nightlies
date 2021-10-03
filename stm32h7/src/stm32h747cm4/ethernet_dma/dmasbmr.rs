#[doc = "Register `DMASBMR` reader"]
pub struct R(crate::R<DMASBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASBMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASBMR` writer"]
pub struct W(crate::W<DMASBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASBMR_SPEC>;
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
impl From<crate::W<DMASBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASBMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB` reader - Rebuild INCRx Burst"]
pub struct RB_R(crate::FieldReader<bool, bool>);
impl RB_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB` writer - Rebuild INCRx Burst"]
pub struct RB_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MB` reader - Mixed Burst"]
pub struct MB_R(crate::FieldReader<bool, bool>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB` writer - Mixed Burst"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub struct AAL_R(crate::FieldReader<bool, bool>);
impl AAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub struct AAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FB` reader - Fixed Burst Length"]
pub struct FB_R(crate::FieldReader<bool, bool>);
impl FB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB` writer - Fixed Burst Length"]
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
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
impl R {
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W {
        RB_W { w: self }
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W {
        AAL_W { w: self }
    }
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasbmr](index.html) module"]
pub struct DMASBMR_SPEC;
impl crate::RegisterSpec for DMASBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasbmr::R](R) reader structure"]
impl crate::Readable for DMASBMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasbmr::W](W) writer structure"]
impl crate::Writable for DMASBMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASBMR to value 0x0101_0000"]
impl crate::Resettable for DMASBMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_0000
    }
}
