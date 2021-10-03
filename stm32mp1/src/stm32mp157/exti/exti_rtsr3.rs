#[doc = "Register `EXTI_RTSR3` reader"]
pub struct R(crate::R<EXTI_RTSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_RTSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_RTSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_RTSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_RTSR3` writer"]
pub struct W(crate::W<EXTI_RTSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_RTSR3_SPEC>;
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
impl From<crate::W<EXTI_RTSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_RTSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT65` reader - RT65"]
pub struct RT65_R(crate::FieldReader<bool, bool>);
impl RT65_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT65` writer - RT65"]
pub struct RT65_W<'a> {
    w: &'a mut W,
}
impl<'a> RT65_W<'a> {
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
#[doc = "Field `RT66` reader - RT66"]
pub struct RT66_R(crate::FieldReader<bool, bool>);
impl RT66_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT66` writer - RT66"]
pub struct RT66_W<'a> {
    w: &'a mut W,
}
impl<'a> RT66_W<'a> {
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
#[doc = "Field `RT68` reader - RT68"]
pub struct RT68_R(crate::FieldReader<bool, bool>);
impl RT68_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT68` writer - RT68"]
pub struct RT68_W<'a> {
    w: &'a mut W,
}
impl<'a> RT68_W<'a> {
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
#[doc = "Field `RT73` reader - RT73"]
pub struct RT73_R(crate::FieldReader<bool, bool>);
impl RT73_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT73` writer - RT73"]
pub struct RT73_W<'a> {
    w: &'a mut W,
}
impl<'a> RT73_W<'a> {
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
#[doc = "Field `RT74` reader - RT74"]
pub struct RT74_R(crate::FieldReader<bool, bool>);
impl RT74_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT74` writer - RT74"]
pub struct RT74_W<'a> {
    w: &'a mut W,
}
impl<'a> RT74_W<'a> {
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
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    pub fn rt65(&self) -> RT65_R {
        RT65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    pub fn rt66(&self) -> RT66_R {
        RT66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    pub fn rt68(&self) -> RT68_R {
        RT68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    pub fn rt73(&self) -> RT73_R {
        RT73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    pub fn rt74(&self) -> RT74_R {
        RT74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    pub fn rt65(&mut self) -> RT65_W {
        RT65_W { w: self }
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    pub fn rt66(&mut self) -> RT66_W {
        RT66_W { w: self }
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    pub fn rt68(&mut self) -> RT68_W {
        RT68_W { w: self }
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    pub fn rt73(&mut self) -> RT73_W {
        RT73_W { w: self }
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    pub fn rt74(&mut self) -> RT74_W {
        RT74_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rtsr3](index.html) module"]
pub struct EXTI_RTSR3_SPEC;
impl crate::RegisterSpec for EXTI_RTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_rtsr3::R](R) reader structure"]
impl crate::Readable for EXTI_RTSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_rtsr3::W](W) writer structure"]
impl crate::Writable for EXTI_RTSR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_RTSR3 to value 0"]
impl crate::Resettable for EXTI_RTSR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
