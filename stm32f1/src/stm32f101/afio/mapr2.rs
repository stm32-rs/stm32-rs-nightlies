#[doc = "Register `MAPR2` reader"]
pub struct R(crate::R<MAPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPR2` writer"]
pub struct W(crate::W<MAPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPR2_SPEC>;
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
impl From<crate::W<MAPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM9_REMAP` reader - TIM9 remapping"]
pub struct TIM9_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM9_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM9_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM9_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM9_REMAP` writer - TIM9 remapping"]
pub struct TIM9_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9_REMAP_W<'a> {
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
#[doc = "Field `TIM10_REMAP` reader - TIM10 remapping"]
pub struct TIM10_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM10_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM10_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM10_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM10_REMAP` writer - TIM10 remapping"]
pub struct TIM10_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM10_REMAP_W<'a> {
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
#[doc = "Field `TIM11_REMAP` reader - TIM11 remapping"]
pub struct TIM11_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM11_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM11_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM11_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM11_REMAP` writer - TIM11 remapping"]
pub struct TIM11_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM11_REMAP_W<'a> {
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
#[doc = "Field `TIM13_REMAP` reader - TIM13 remapping"]
pub struct TIM13_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM13_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM13_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM13_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM13_REMAP` writer - TIM13 remapping"]
pub struct TIM13_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TIM14_REMAP` reader - TIM14 remapping"]
pub struct TIM14_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM14_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM14_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM14_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM14_REMAP` writer - TIM14 remapping"]
pub struct TIM14_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FSMC_NADV` reader - NADV connect/disconnect"]
pub struct FSMC_NADV_R(crate::FieldReader<bool, bool>);
impl FSMC_NADV_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSMC_NADV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSMC_NADV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSMC_NADV` writer - NADV connect/disconnect"]
pub struct FSMC_NADV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_NADV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&self) -> TIM9_REMAP_R {
        TIM9_REMAP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&self) -> TIM10_REMAP_R {
        TIM10_REMAP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    pub fn tim11_remap(&self) -> TIM11_REMAP_R {
        TIM11_REMAP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&mut self) -> TIM9_REMAP_W {
        TIM9_REMAP_W { w: self }
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&mut self) -> TIM10_REMAP_W {
        TIM10_REMAP_W { w: self }
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    pub fn tim11_remap(&mut self) -> TIM11_REMAP_W {
        TIM11_REMAP_W { w: self }
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W {
        TIM13_REMAP_W { w: self }
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W {
        TIM14_REMAP_W { w: self }
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W {
        FSMC_NADV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AF remap and debug I/O configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapr2](index.html) module"]
pub struct MAPR2_SPEC;
impl crate::RegisterSpec for MAPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapr2::R](R) reader structure"]
impl crate::Readable for MAPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapr2::W](W) writer structure"]
impl crate::Writable for MAPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAPR2 to value 0"]
impl crate::Resettable for MAPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
