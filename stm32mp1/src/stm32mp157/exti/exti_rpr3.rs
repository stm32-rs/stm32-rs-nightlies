#[doc = "Register `EXTI_RPR3` reader"]
pub struct R(crate::R<EXTI_RPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_RPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_RPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_RPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_RPR3` writer"]
pub struct W(crate::W<EXTI_RPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_RPR3_SPEC>;
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
impl From<crate::W<EXTI_RPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_RPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF65` reader - RPIF65"]
pub struct RPIF65_R(crate::FieldReader<bool, bool>);
impl RPIF65_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF65` writer - RPIF65"]
pub struct RPIF65_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF65_W<'a> {
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
#[doc = "Field `RPIF66` reader - RPIF66"]
pub struct RPIF66_R(crate::FieldReader<bool, bool>);
impl RPIF66_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF66` writer - RPIF66"]
pub struct RPIF66_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF66_W<'a> {
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
#[doc = "Field `RPIF68` reader - RPIF68"]
pub struct RPIF68_R(crate::FieldReader<bool, bool>);
impl RPIF68_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF68` writer - RPIF68"]
pub struct RPIF68_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF68_W<'a> {
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
#[doc = "Field `RPIF73` reader - RPIF73"]
pub struct RPIF73_R(crate::FieldReader<bool, bool>);
impl RPIF73_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF73` writer - RPIF73"]
pub struct RPIF73_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF73_W<'a> {
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
#[doc = "Field `RPIF74` reader - RPIF74"]
pub struct RPIF74_R(crate::FieldReader<bool, bool>);
impl RPIF74_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF74` writer - RPIF74"]
pub struct RPIF74_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF74_W<'a> {
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
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    pub fn rpif65(&self) -> RPIF65_R {
        RPIF65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    pub fn rpif66(&self) -> RPIF66_R {
        RPIF66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    pub fn rpif68(&self) -> RPIF68_R {
        RPIF68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    pub fn rpif73(&self) -> RPIF73_R {
        RPIF73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    pub fn rpif74(&self) -> RPIF74_R {
        RPIF74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    pub fn rpif65(&mut self) -> RPIF65_W {
        RPIF65_W { w: self }
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    pub fn rpif66(&mut self) -> RPIF66_W {
        RPIF66_W { w: self }
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    pub fn rpif68(&mut self) -> RPIF68_W {
        RPIF68_W { w: self }
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    pub fn rpif73(&mut self) -> RPIF73_W {
        RPIF73_W { w: self }
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    pub fn rpif74(&mut self) -> RPIF74_W {
        RPIF74_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rpr3](index.html) module"]
pub struct EXTI_RPR3_SPEC;
impl crate::RegisterSpec for EXTI_RPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_rpr3::R](R) reader structure"]
impl crate::Readable for EXTI_RPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_rpr3::W](W) writer structure"]
impl crate::Writable for EXTI_RPR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_RPR3 to value 0"]
impl crate::Resettable for EXTI_RPR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
