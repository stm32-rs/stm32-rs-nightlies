#[doc = "Register `EXTI_TZENR2` reader"]
pub struct R(crate::R<EXTI_TZENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_TZENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_TZENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_TZENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_TZENR2` writer"]
pub struct W(crate::W<EXTI_TZENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_TZENR2_SPEC>;
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
impl From<crate::W<EXTI_TZENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_TZENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZEN41` reader - TZEN41"]
pub struct TZEN41_R(crate::FieldReader<bool, bool>);
impl TZEN41_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN41` writer - TZEN41"]
pub struct TZEN41_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN41_W<'a> {
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
#[doc = "Field `TZEN54` reader - TZEN54"]
pub struct TZEN54_R(crate::FieldReader<bool, bool>);
impl TZEN54_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN54_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN54` writer - TZEN54"]
pub struct TZEN54_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN54_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TZEN55` reader - TZEN55"]
pub struct TZEN55_R(crate::FieldReader<bool, bool>);
impl TZEN55_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN55_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN55` writer - TZEN55"]
pub struct TZEN55_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN55_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TZEN56` reader - TZEN56"]
pub struct TZEN56_R(crate::FieldReader<bool, bool>);
impl TZEN56_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN56_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN56` writer - TZEN56"]
pub struct TZEN56_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN56_W<'a> {
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
#[doc = "Field `TZEN57` reader - TZEN57"]
pub struct TZEN57_R(crate::FieldReader<bool, bool>);
impl TZEN57_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN57_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN57` writer - TZEN57"]
pub struct TZEN57_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN57_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `TZEN58` reader - TZEN58"]
pub struct TZEN58_R(crate::FieldReader<bool, bool>);
impl TZEN58_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN58_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN58` writer - TZEN58"]
pub struct TZEN58_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN58_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TZEN59` reader - TZEN59"]
pub struct TZEN59_R(crate::FieldReader<bool, bool>);
impl TZEN59_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN59_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN59` writer - TZEN59"]
pub struct TZEN59_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN59_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `TZEN60` reader - TZEN60"]
pub struct TZEN60_R(crate::FieldReader<bool, bool>);
impl TZEN60_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN60_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN60` writer - TZEN60"]
pub struct TZEN60_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN60_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    pub fn tzen41(&self) -> TZEN41_R {
        TZEN41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    pub fn tzen54(&self) -> TZEN54_R {
        TZEN54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    pub fn tzen55(&self) -> TZEN55_R {
        TZEN55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    pub fn tzen56(&self) -> TZEN56_R {
        TZEN56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    pub fn tzen57(&self) -> TZEN57_R {
        TZEN57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    pub fn tzen58(&self) -> TZEN58_R {
        TZEN58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    pub fn tzen59(&self) -> TZEN59_R {
        TZEN59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    pub fn tzen60(&self) -> TZEN60_R {
        TZEN60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    pub fn tzen41(&mut self) -> TZEN41_W {
        TZEN41_W { w: self }
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    pub fn tzen54(&mut self) -> TZEN54_W {
        TZEN54_W { w: self }
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    pub fn tzen55(&mut self) -> TZEN55_W {
        TZEN55_W { w: self }
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    pub fn tzen56(&mut self) -> TZEN56_W {
        TZEN56_W { w: self }
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    pub fn tzen57(&mut self) -> TZEN57_W {
        TZEN57_W { w: self }
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    pub fn tzen58(&mut self) -> TZEN58_W {
        TZEN58_W { w: self }
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    pub fn tzen59(&mut self) -> TZEN59_W {
        TZEN59_W { w: self }
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    pub fn tzen60(&mut self) -> TZEN60_W {
        TZEN60_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr2](index.html) module"]
pub struct EXTI_TZENR2_SPEC;
impl crate::RegisterSpec for EXTI_TZENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_tzenr2::R](R) reader structure"]
impl crate::Readable for EXTI_TZENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_tzenr2::W](W) writer structure"]
impl crate::Writable for EXTI_TZENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_TZENR2 to value 0"]
impl crate::Resettable for EXTI_TZENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
