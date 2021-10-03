#[doc = "Register `EXTI_FTSR3` reader"]
pub struct R(crate::R<EXTI_FTSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_FTSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_FTSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_FTSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_FTSR3` writer"]
pub struct W(crate::W<EXTI_FTSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_FTSR3_SPEC>;
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
impl From<crate::W<EXTI_FTSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_FTSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FT65` reader - FT65"]
pub struct FT65_R(crate::FieldReader<bool, bool>);
impl FT65_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT65` writer - FT65"]
pub struct FT65_W<'a> {
    w: &'a mut W,
}
impl<'a> FT65_W<'a> {
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
#[doc = "Field `FT66` reader - FT66"]
pub struct FT66_R(crate::FieldReader<bool, bool>);
impl FT66_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT66` writer - FT66"]
pub struct FT66_W<'a> {
    w: &'a mut W,
}
impl<'a> FT66_W<'a> {
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
#[doc = "Field `FT68` reader - FT68"]
pub struct FT68_R(crate::FieldReader<bool, bool>);
impl FT68_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT68` writer - FT68"]
pub struct FT68_W<'a> {
    w: &'a mut W,
}
impl<'a> FT68_W<'a> {
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
#[doc = "Field `FT73` reader - FT73"]
pub struct FT73_R(crate::FieldReader<bool, bool>);
impl FT73_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT73` writer - FT73"]
pub struct FT73_W<'a> {
    w: &'a mut W,
}
impl<'a> FT73_W<'a> {
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
#[doc = "Field `FT74` reader - FT74"]
pub struct FT74_R(crate::FieldReader<bool, bool>);
impl FT74_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT74` writer - FT74"]
pub struct FT74_W<'a> {
    w: &'a mut W,
}
impl<'a> FT74_W<'a> {
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
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    pub fn ft65(&self) -> FT65_R {
        FT65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    pub fn ft66(&self) -> FT66_R {
        FT66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    pub fn ft68(&self) -> FT68_R {
        FT68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    pub fn ft73(&self) -> FT73_R {
        FT73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    pub fn ft74(&self) -> FT74_R {
        FT74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    pub fn ft65(&mut self) -> FT65_W {
        FT65_W { w: self }
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    pub fn ft66(&mut self) -> FT66_W {
        FT66_W { w: self }
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    pub fn ft68(&mut self) -> FT68_W {
        FT68_W { w: self }
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    pub fn ft73(&mut self) -> FT73_W {
        FT73_W { w: self }
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    pub fn ft74(&mut self) -> FT74_W {
        FT74_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ftsr3](index.html) module"]
pub struct EXTI_FTSR3_SPEC;
impl crate::RegisterSpec for EXTI_FTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_ftsr3::R](R) reader structure"]
impl crate::Readable for EXTI_FTSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_ftsr3::W](W) writer structure"]
impl crate::Writable for EXTI_FTSR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_FTSR3 to value 0"]
impl crate::Resettable for EXTI_FTSR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
