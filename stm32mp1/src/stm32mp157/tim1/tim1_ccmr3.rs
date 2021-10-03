#[doc = "Register `TIM1_CCMR3` reader"]
pub struct R(crate::R<TIM1_CCMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_CCMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_CCMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_CCMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_CCMR3` writer"]
pub struct W(crate::W<TIM1_CCMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_CCMR3_SPEC>;
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
impl From<crate::W<TIM1_CCMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_CCMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC5FE` reader - OC5FE"]
pub struct OC5FE_R(crate::FieldReader<bool, bool>);
impl OC5FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5FE` writer - OC5FE"]
pub struct OC5FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5FE_W<'a> {
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
#[doc = "Field `OC5PE` reader - OC5PE"]
pub struct OC5PE_R(crate::FieldReader<bool, bool>);
impl OC5PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5PE` writer - OC5PE"]
pub struct OC5PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5PE_W<'a> {
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
#[doc = "Field `OC5M` reader - OC5M"]
pub struct OC5M_R(crate::FieldReader<u8, u8>);
impl OC5M_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC5M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5M` writer - OC5M"]
pub struct OC5M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `OC5CE` reader - OC5CE"]
pub struct OC5CE_R(crate::FieldReader<bool, bool>);
impl OC5CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5CE` writer - OC5CE"]
pub struct OC5CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5CE_W<'a> {
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
#[doc = "Field `OC6FE` reader - OC6FE"]
pub struct OC6FE_R(crate::FieldReader<bool, bool>);
impl OC6FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6FE` writer - OC6FE"]
pub struct OC6FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6FE_W<'a> {
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
#[doc = "Field `OC6PE` reader - OC6PE"]
pub struct OC6PE_R(crate::FieldReader<bool, bool>);
impl OC6PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6PE` writer - OC6PE"]
pub struct OC6PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `OC6M` reader - OC6M"]
pub struct OC6M_R(crate::FieldReader<u8, u8>);
impl OC6M_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC6M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6M` writer - OC6M"]
pub struct OC6M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `OC6CE` reader - OC6CE"]
pub struct OC6CE_R(crate::FieldReader<bool, bool>);
impl OC6CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6CE` writer - OC6CE"]
pub struct OC6CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6CE_W<'a> {
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
#[doc = "Field `OC5M3` reader - OC5M3"]
pub struct OC5M3_R(crate::FieldReader<bool, bool>);
impl OC5M3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5M3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5M3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5M3` writer - OC5M3"]
pub struct OC5M3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `OC6M3` reader - OC6M3"]
pub struct OC6M3_R(crate::FieldReader<bool, bool>);
impl OC6M3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC6M3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC6M3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC6M3` writer - OC6M3"]
pub struct OC6M3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - OC5FE"]
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OC5PE"]
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - OC5M"]
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - OC5CE"]
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OC6FE"]
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - OC6PE"]
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - OC6M"]
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - OC6CE"]
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OC5M3"]
    #[inline(always)]
    pub fn oc5m3(&self) -> OC5M3_R {
        OC5M3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - OC6M3"]
    #[inline(always)]
    pub fn oc6m3(&self) -> OC6M3_R {
        OC6M3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - OC5FE"]
    #[inline(always)]
    pub fn oc5fe(&mut self) -> OC5FE_W {
        OC5FE_W { w: self }
    }
    #[doc = "Bit 3 - OC5PE"]
    #[inline(always)]
    pub fn oc5pe(&mut self) -> OC5PE_W {
        OC5PE_W { w: self }
    }
    #[doc = "Bits 4:6 - OC5M"]
    #[inline(always)]
    pub fn oc5m(&mut self) -> OC5M_W {
        OC5M_W { w: self }
    }
    #[doc = "Bit 7 - OC5CE"]
    #[inline(always)]
    pub fn oc5ce(&mut self) -> OC5CE_W {
        OC5CE_W { w: self }
    }
    #[doc = "Bit 10 - OC6FE"]
    #[inline(always)]
    pub fn oc6fe(&mut self) -> OC6FE_W {
        OC6FE_W { w: self }
    }
    #[doc = "Bit 11 - OC6PE"]
    #[inline(always)]
    pub fn oc6pe(&mut self) -> OC6PE_W {
        OC6PE_W { w: self }
    }
    #[doc = "Bits 12:14 - OC6M"]
    #[inline(always)]
    pub fn oc6m(&mut self) -> OC6M_W {
        OC6M_W { w: self }
    }
    #[doc = "Bit 15 - OC6CE"]
    #[inline(always)]
    pub fn oc6ce(&mut self) -> OC6CE_W {
        OC6CE_W { w: self }
    }
    #[doc = "Bit 16 - OC5M3"]
    #[inline(always)]
    pub fn oc5m3(&mut self) -> OC5M3_W {
        OC5M3_W { w: self }
    }
    #[doc = "Bit 24 - OC6M3"]
    #[inline(always)]
    pub fn oc6m3(&mut self) -> OC6M3_W {
        OC6M3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccmr3](index.html) module"]
pub struct TIM1_CCMR3_SPEC;
impl crate::RegisterSpec for TIM1_CCMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_ccmr3::R](R) reader structure"]
impl crate::Readable for TIM1_CCMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_ccmr3::W](W) writer structure"]
impl crate::Writable for TIM1_CCMR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM1_CCMR3 to value 0"]
impl crate::Resettable for TIM1_CCMR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
