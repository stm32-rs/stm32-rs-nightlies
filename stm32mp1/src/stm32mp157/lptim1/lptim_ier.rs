#[doc = "Register `LPTIM_IER` reader"]
pub struct R(crate::R<LPTIM_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_IER` writer"]
pub struct W(crate::W<LPTIM_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_IER_SPEC>;
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
impl From<crate::W<LPTIM_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPMIE` reader - CMPMIE"]
pub struct CMPMIE_R(crate::FieldReader<bool, bool>);
impl CMPMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMIE` writer - CMPMIE"]
pub struct CMPMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMIE_W<'a> {
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
#[doc = "Field `ARRMIE` reader - ARRMIE"]
pub struct ARRMIE_R(crate::FieldReader<bool, bool>);
impl ARRMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARRMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARRMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARRMIE` writer - ARRMIE"]
pub struct ARRMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRMIE_W<'a> {
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
#[doc = "Field `EXTTRIGIE` reader - EXTTRIGIE"]
pub struct EXTTRIGIE_R(crate::FieldReader<bool, bool>);
impl EXTTRIGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTTRIGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTTRIGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTTRIGIE` writer - EXTTRIGIE"]
pub struct EXTTRIGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTRIGIE_W<'a> {
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
#[doc = "Field `CMPOKIE` reader - CMPOKIE"]
pub struct CMPOKIE_R(crate::FieldReader<bool, bool>);
impl CMPOKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPOKIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPOKIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPOKIE` writer - CMPOKIE"]
pub struct CMPOKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOKIE_W<'a> {
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
#[doc = "Field `ARROKIE` reader - ARROKIE"]
pub struct ARROKIE_R(crate::FieldReader<bool, bool>);
impl ARROKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARROKIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARROKIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARROKIE` writer - ARROKIE"]
pub struct ARROKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARROKIE_W<'a> {
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
#[doc = "Field `UPIE` reader - UPIE"]
pub struct UPIE_R(crate::FieldReader<bool, bool>);
impl UPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPIE` writer - UPIE"]
pub struct UPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPIE_W<'a> {
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
#[doc = "Field `DOWNIE` reader - DOWNIE"]
pub struct DOWNIE_R(crate::FieldReader<bool, bool>);
impl DOWNIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOWNIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWNIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWNIE` writer - DOWNIE"]
pub struct DOWNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CMPMIE"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ARRMIE"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTTRIGIE"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMPOKIE"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ARROKIE"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UPIE"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DOWNIE"]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPMIE"]
    #[inline(always)]
    pub fn cmpmie(&mut self) -> CMPMIE_W {
        CMPMIE_W { w: self }
    }
    #[doc = "Bit 1 - ARRMIE"]
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W {
        ARRMIE_W { w: self }
    }
    #[doc = "Bit 2 - EXTTRIGIE"]
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W {
        EXTTRIGIE_W { w: self }
    }
    #[doc = "Bit 3 - CMPOKIE"]
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CMPOKIE_W {
        CMPOKIE_W { w: self }
    }
    #[doc = "Bit 4 - ARROKIE"]
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W {
        ARROKIE_W { w: self }
    }
    #[doc = "Bit 5 - UPIE"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W { w: self }
    }
    #[doc = "Bit 6 - DOWNIE"]
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W {
        DOWNIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_ier](index.html) module"]
pub struct LPTIM_IER_SPEC;
impl crate::RegisterSpec for LPTIM_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_ier::R](R) reader structure"]
impl crate::Readable for LPTIM_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_ier::W](W) writer structure"]
impl crate::Writable for LPTIM_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPTIM_IER to value 0"]
impl crate::Resettable for LPTIM_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
