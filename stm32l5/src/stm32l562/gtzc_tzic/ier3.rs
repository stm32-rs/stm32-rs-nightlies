#[doc = "Register `IER3` reader"]
pub struct R(crate::R<IER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER3` writer"]
pub struct W(crate::W<IER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER3_SPEC>;
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
impl From<crate::W<IER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZSCIE` reader - TZSCIE"]
pub struct TZSCIE_R(crate::FieldReader<bool, bool>);
impl TZSCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZSCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZSCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZSCIE` writer - TZSCIE"]
pub struct TZSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCIE_W<'a> {
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
#[doc = "Field `TZICIE` reader - TZICIE"]
pub struct TZICIE_R(crate::FieldReader<bool, bool>);
impl TZICIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZICIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZICIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZICIE` writer - TZICIE"]
pub struct TZICIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICIE_W<'a> {
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
#[doc = "Field `MPCWM1IE` reader - MPCWM1IE"]
pub struct MPCWM1IE_R(crate::FieldReader<bool, bool>);
impl MPCWM1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCWM1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCWM1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCWM1IE` writer - MPCWM1IE"]
pub struct MPCWM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM1IE_W<'a> {
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
#[doc = "Field `MPCWM2IE` reader - MPCWM2IE"]
pub struct MPCWM2IE_R(crate::FieldReader<bool, bool>);
impl MPCWM2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCWM2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCWM2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCWM2IE` writer - MPCWM2IE"]
pub struct MPCWM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM2IE_W<'a> {
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
#[doc = "Field `MPCBB1IE` reader - MPCBB1IE"]
pub struct MPCBB1IE_R(crate::FieldReader<bool, bool>);
impl MPCBB1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB1IE` writer - MPCBB1IE"]
pub struct MPCBB1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1IE_W<'a> {
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
#[doc = "Field `MPCBB1_REGIE` reader - MPCBB1_REGIE"]
pub struct MPCBB1_REGIE_R(crate::FieldReader<bool, bool>);
impl MPCBB1_REGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB1_REGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB1_REGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB1_REGIE` writer - MPCBB1_REGIE"]
pub struct MPCBB1_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1_REGIE_W<'a> {
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
#[doc = "Field `MPCBB2IE` reader - MPCBB2IE"]
pub struct MPCBB2IE_R(crate::FieldReader<bool, bool>);
impl MPCBB2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB2IE` writer - MPCBB2IE"]
pub struct MPCBB2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2IE_W<'a> {
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
#[doc = "Field `MPCBB2_REGIE` reader - MPCBB2_REGIE"]
pub struct MPCBB2_REGIE_R(crate::FieldReader<bool, bool>);
impl MPCBB2_REGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB2_REGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB2_REGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB2_REGIE` writer - MPCBB2_REGIE"]
pub struct MPCBB2_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2_REGIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    pub fn mpcwm1ie(&self) -> MPCWM1IE_R {
        MPCWM1IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    pub fn mpcwm2ie(&self) -> MPCWM2IE_R {
        MPCWM2IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    pub fn mpcbb1ie(&self) -> MPCBB1IE_R {
        MPCBB1IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    pub fn mpcbb2ie(&self) -> MPCBB2IE_R {
        MPCBB2IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&mut self) -> TZSCIE_W {
        TZSCIE_W { w: self }
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&mut self) -> TZICIE_W {
        TZICIE_W { w: self }
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    pub fn mpcwm1ie(&mut self) -> MPCWM1IE_W {
        MPCWM1IE_W { w: self }
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    pub fn mpcwm2ie(&mut self) -> MPCWM2IE_W {
        MPCWM2IE_W { w: self }
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    pub fn mpcbb1ie(&mut self) -> MPCBB1IE_W {
        MPCBB1IE_W { w: self }
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W {
        MPCBB1_REGIE_W { w: self }
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    pub fn mpcbb2ie(&mut self) -> MPCBB2IE_W {
        MPCBB2IE_W { w: self }
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W {
        MPCBB2_REGIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier3](index.html) module"]
pub struct IER3_SPEC;
impl crate::RegisterSpec for IER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier3::R](R) reader structure"]
impl crate::Readable for IER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier3::W](W) writer structure"]
impl crate::Writable for IER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER3 to value 0"]
impl crate::Resettable for IER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
