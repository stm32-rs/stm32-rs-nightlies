#[doc = "Register `EXTI_FPR3` reader"]
pub struct R(crate::R<EXTI_FPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_FPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_FPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_FPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_FPR3` writer"]
pub struct W(crate::W<EXTI_FPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_FPR3_SPEC>;
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
impl From<crate::W<EXTI_FPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_FPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIF65` reader - FPIF65"]
pub struct FPIF65_R(crate::FieldReader<bool, bool>);
impl FPIF65_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF65` writer - FPIF65"]
pub struct FPIF65_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF65_W<'a> {
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
#[doc = "Field `FPIF66` reader - FPIF66"]
pub struct FPIF66_R(crate::FieldReader<bool, bool>);
impl FPIF66_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF66` writer - FPIF66"]
pub struct FPIF66_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF66_W<'a> {
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
#[doc = "Field `FPIF68` reader - FPIF68"]
pub struct FPIF68_R(crate::FieldReader<bool, bool>);
impl FPIF68_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF68` writer - FPIF68"]
pub struct FPIF68_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF68_W<'a> {
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
#[doc = "Field `FPIF73` reader - FPIF73"]
pub struct FPIF73_R(crate::FieldReader<bool, bool>);
impl FPIF73_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF73` writer - FPIF73"]
pub struct FPIF73_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF73_W<'a> {
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
#[doc = "Field `FPIF74` reader - FPIF74"]
pub struct FPIF74_R(crate::FieldReader<bool, bool>);
impl FPIF74_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF74` writer - FPIF74"]
pub struct FPIF74_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF74_W<'a> {
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
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    pub fn fpif65(&self) -> FPIF65_R {
        FPIF65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    pub fn fpif66(&self) -> FPIF66_R {
        FPIF66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    pub fn fpif68(&self) -> FPIF68_R {
        FPIF68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    pub fn fpif73(&self) -> FPIF73_R {
        FPIF73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    pub fn fpif74(&self) -> FPIF74_R {
        FPIF74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    pub fn fpif65(&mut self) -> FPIF65_W {
        FPIF65_W { w: self }
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    pub fn fpif66(&mut self) -> FPIF66_W {
        FPIF66_W { w: self }
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    pub fn fpif68(&mut self) -> FPIF68_W {
        FPIF68_W { w: self }
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    pub fn fpif73(&mut self) -> FPIF73_W {
        FPIF73_W { w: self }
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    pub fn fpif74(&mut self) -> FPIF74_W {
        FPIF74_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_fpr3](index.html) module"]
pub struct EXTI_FPR3_SPEC;
impl crate::RegisterSpec for EXTI_FPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_fpr3::R](R) reader structure"]
impl crate::Readable for EXTI_FPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_fpr3::W](W) writer structure"]
impl crate::Writable for EXTI_FPR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_FPR3 to value 0"]
impl crate::Resettable for EXTI_FPR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
