#[doc = "Register `EXTI_SWIER3` reader"]
pub struct R(crate::R<EXTI_SWIER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_SWIER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_SWIER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_SWIER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_SWIER3` writer"]
pub struct W(crate::W<EXTI_SWIER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_SWIER3_SPEC>;
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
impl From<crate::W<EXTI_SWIER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_SWIER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI65` reader - SWI65"]
pub struct SWI65_R(crate::FieldReader<bool, bool>);
impl SWI65_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI65` writer - SWI65"]
pub struct SWI65_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI65_W<'a> {
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
#[doc = "Field `SWI66` reader - SWI66"]
pub struct SWI66_R(crate::FieldReader<bool, bool>);
impl SWI66_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI66` writer - SWI66"]
pub struct SWI66_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI66_W<'a> {
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
#[doc = "Field `SWI68` reader - SWI68"]
pub struct SWI68_R(crate::FieldReader<bool, bool>);
impl SWI68_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI68` writer - SWI68"]
pub struct SWI68_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI68_W<'a> {
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
#[doc = "Field `SWI73` reader - SWI73"]
pub struct SWI73_R(crate::FieldReader<bool, bool>);
impl SWI73_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI73` writer - SWI73"]
pub struct SWI73_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI73_W<'a> {
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
#[doc = "Field `SWI74` reader - SWI74"]
pub struct SWI74_R(crate::FieldReader<bool, bool>);
impl SWI74_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI74` writer - SWI74"]
pub struct SWI74_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI74_W<'a> {
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
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    pub fn swi65(&self) -> SWI65_R {
        SWI65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    pub fn swi66(&self) -> SWI66_R {
        SWI66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    pub fn swi68(&self) -> SWI68_R {
        SWI68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    pub fn swi73(&self) -> SWI73_R {
        SWI73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    pub fn swi74(&self) -> SWI74_R {
        SWI74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    pub fn swi65(&mut self) -> SWI65_W {
        SWI65_W { w: self }
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    pub fn swi66(&mut self) -> SWI66_W {
        SWI66_W { w: self }
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    pub fn swi68(&mut self) -> SWI68_W {
        SWI68_W { w: self }
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    pub fn swi73(&mut self) -> SWI73_W {
        SWI73_W { w: self }
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    pub fn swi74(&mut self) -> SWI74_W {
        SWI74_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swier3](index.html) module"]
pub struct EXTI_SWIER3_SPEC;
impl crate::RegisterSpec for EXTI_SWIER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_swier3::R](R) reader structure"]
impl crate::Readable for EXTI_SWIER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_swier3::W](W) writer structure"]
impl crate::Writable for EXTI_SWIER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_SWIER3 to value 0"]
impl crate::Resettable for EXTI_SWIER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
