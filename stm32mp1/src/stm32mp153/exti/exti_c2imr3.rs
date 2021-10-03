#[doc = "Register `EXTI_C2IMR3` reader"]
pub struct R(crate::R<EXTI_C2IMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_C2IMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_C2IMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_C2IMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_C2IMR3` writer"]
pub struct W(crate::W<EXTI_C2IMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_C2IMR3_SPEC>;
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
impl From<crate::W<EXTI_C2IMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_C2IMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM64` reader - IM64"]
pub struct IM64_R(crate::FieldReader<bool, bool>);
impl IM64_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM64_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM64` writer - IM64"]
pub struct IM64_W<'a> {
    w: &'a mut W,
}
impl<'a> IM64_W<'a> {
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
#[doc = "Field `IM65` reader - IM65"]
pub struct IM65_R(crate::FieldReader<bool, bool>);
impl IM65_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM65` writer - IM65"]
pub struct IM65_W<'a> {
    w: &'a mut W,
}
impl<'a> IM65_W<'a> {
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
#[doc = "Field `IM66` reader - IM66"]
pub struct IM66_R(crate::FieldReader<bool, bool>);
impl IM66_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM66` writer - IM66"]
pub struct IM66_W<'a> {
    w: &'a mut W,
}
impl<'a> IM66_W<'a> {
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
#[doc = "Field `IM67` reader - IM67"]
pub struct IM67_R(crate::FieldReader<bool, bool>);
impl IM67_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM67_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM67` writer - IM67"]
pub struct IM67_W<'a> {
    w: &'a mut W,
}
impl<'a> IM67_W<'a> {
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
#[doc = "Field `IM68` reader - IM68"]
pub struct IM68_R(crate::FieldReader<bool, bool>);
impl IM68_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM68` writer - IM68"]
pub struct IM68_W<'a> {
    w: &'a mut W,
}
impl<'a> IM68_W<'a> {
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
#[doc = "Field `IM69` reader - IM69"]
pub struct IM69_R(crate::FieldReader<bool, bool>);
impl IM69_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM69_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM69_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM69` writer - IM69"]
pub struct IM69_W<'a> {
    w: &'a mut W,
}
impl<'a> IM69_W<'a> {
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
#[doc = "Field `IM70` reader - IM70"]
pub struct IM70_R(crate::FieldReader<bool, bool>);
impl IM70_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM70_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM70_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM70` writer - IM70"]
pub struct IM70_W<'a> {
    w: &'a mut W,
}
impl<'a> IM70_W<'a> {
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
#[doc = "Field `IM71` reader - IM71"]
pub struct IM71_R(crate::FieldReader<bool, bool>);
impl IM71_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM71_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM71_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM71` writer - IM71"]
pub struct IM71_W<'a> {
    w: &'a mut W,
}
impl<'a> IM71_W<'a> {
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
#[doc = "Field `IM72` reader - IM72"]
pub struct IM72_R(crate::FieldReader<bool, bool>);
impl IM72_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM72_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM72_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM72` writer - IM72"]
pub struct IM72_W<'a> {
    w: &'a mut W,
}
impl<'a> IM72_W<'a> {
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
#[doc = "Field `IM73` reader - IM73"]
pub struct IM73_R(crate::FieldReader<bool, bool>);
impl IM73_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM73` writer - IM73"]
pub struct IM73_W<'a> {
    w: &'a mut W,
}
impl<'a> IM73_W<'a> {
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
#[doc = "Field `IM74` reader - IM74"]
pub struct IM74_R(crate::FieldReader<bool, bool>);
impl IM74_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM74` writer - IM74"]
pub struct IM74_W<'a> {
    w: &'a mut W,
}
impl<'a> IM74_W<'a> {
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
#[doc = "Field `IM75` reader - IM75"]
pub struct IM75_R(crate::FieldReader<bool, bool>);
impl IM75_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM75_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM75_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM75` writer - IM75"]
pub struct IM75_W<'a> {
    w: &'a mut W,
}
impl<'a> IM75_W<'a> {
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
impl R {
    #[doc = "Bit 0 - IM64"]
    #[inline(always)]
    pub fn im64(&self) -> IM64_R {
        IM64_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IM65"]
    #[inline(always)]
    pub fn im65(&self) -> IM65_R {
        IM65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IM66"]
    #[inline(always)]
    pub fn im66(&self) -> IM66_R {
        IM66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IM67"]
    #[inline(always)]
    pub fn im67(&self) -> IM67_R {
        IM67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IM68"]
    #[inline(always)]
    pub fn im68(&self) -> IM68_R {
        IM68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IM69"]
    #[inline(always)]
    pub fn im69(&self) -> IM69_R {
        IM69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IM70"]
    #[inline(always)]
    pub fn im70(&self) -> IM70_R {
        IM70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IM71"]
    #[inline(always)]
    pub fn im71(&self) -> IM71_R {
        IM71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IM72"]
    #[inline(always)]
    pub fn im72(&self) -> IM72_R {
        IM72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IM73"]
    #[inline(always)]
    pub fn im73(&self) -> IM73_R {
        IM73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IM74"]
    #[inline(always)]
    pub fn im74(&self) -> IM74_R {
        IM74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IM75"]
    #[inline(always)]
    pub fn im75(&self) -> IM75_R {
        IM75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IM64"]
    #[inline(always)]
    pub fn im64(&mut self) -> IM64_W {
        IM64_W { w: self }
    }
    #[doc = "Bit 1 - IM65"]
    #[inline(always)]
    pub fn im65(&mut self) -> IM65_W {
        IM65_W { w: self }
    }
    #[doc = "Bit 2 - IM66"]
    #[inline(always)]
    pub fn im66(&mut self) -> IM66_W {
        IM66_W { w: self }
    }
    #[doc = "Bit 3 - IM67"]
    #[inline(always)]
    pub fn im67(&mut self) -> IM67_W {
        IM67_W { w: self }
    }
    #[doc = "Bit 4 - IM68"]
    #[inline(always)]
    pub fn im68(&mut self) -> IM68_W {
        IM68_W { w: self }
    }
    #[doc = "Bit 5 - IM69"]
    #[inline(always)]
    pub fn im69(&mut self) -> IM69_W {
        IM69_W { w: self }
    }
    #[doc = "Bit 6 - IM70"]
    #[inline(always)]
    pub fn im70(&mut self) -> IM70_W {
        IM70_W { w: self }
    }
    #[doc = "Bit 7 - IM71"]
    #[inline(always)]
    pub fn im71(&mut self) -> IM71_W {
        IM71_W { w: self }
    }
    #[doc = "Bit 8 - IM72"]
    #[inline(always)]
    pub fn im72(&mut self) -> IM72_W {
        IM72_W { w: self }
    }
    #[doc = "Bit 9 - IM73"]
    #[inline(always)]
    pub fn im73(&mut self) -> IM73_W {
        IM73_W { w: self }
    }
    #[doc = "Bit 10 - IM74"]
    #[inline(always)]
    pub fn im74(&mut self) -> IM74_W {
        IM74_W { w: self }
    }
    #[doc = "Bit 11 - IM75"]
    #[inline(always)]
    pub fn im75(&mut self) -> IM75_W {
        IM75_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains register bits for configurable events and direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2imr3](index.html) module"]
pub struct EXTI_C2IMR3_SPEC;
impl crate::RegisterSpec for EXTI_C2IMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_c2imr3::R](R) reader structure"]
impl crate::Readable for EXTI_C2IMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_c2imr3::W](W) writer structure"]
impl crate::Writable for EXTI_C2IMR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_C2IMR3 to value 0x0de9"]
impl crate::Resettable for EXTI_C2IMR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0de9
    }
}
