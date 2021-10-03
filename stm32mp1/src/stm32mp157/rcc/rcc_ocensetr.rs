#[doc = "Register `RCC_OCENSETR` reader"]
pub struct R(crate::R<RCC_OCENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_OCENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_OCENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_OCENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_OCENSETR` writer"]
pub struct W(crate::W<RCC_OCENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_OCENSETR_SPEC>;
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
impl From<crate::W<RCC_OCENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_OCENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSION` reader - HSION"]
pub struct HSION_R(crate::FieldReader<bool, bool>);
impl HSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSION` writer - HSION"]
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
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
#[doc = "Field `HSIKERON` reader - HSIKERON"]
pub struct HSIKERON_R(crate::FieldReader<bool, bool>);
impl HSIKERON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIKERON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIKERON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIKERON` writer - HSIKERON"]
pub struct HSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIKERON_W<'a> {
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
#[doc = "Field `CSION` reader - CSION"]
pub struct CSION_R(crate::FieldReader<bool, bool>);
impl CSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSION` writer - CSION"]
pub struct CSION_W<'a> {
    w: &'a mut W,
}
impl<'a> CSION_W<'a> {
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
#[doc = "Field `CSIKERON` reader - CSIKERON"]
pub struct CSIKERON_R(crate::FieldReader<bool, bool>);
impl CSIKERON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSIKERON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSIKERON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSIKERON` writer - CSIKERON"]
pub struct CSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIKERON_W<'a> {
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
#[doc = "Field `DIGBYP` reader - DIGBYP"]
pub struct DIGBYP_R(crate::FieldReader<bool, bool>);
impl DIGBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIGBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIGBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIGBYP` writer - DIGBYP"]
pub struct DIGBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGBYP_W<'a> {
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
#[doc = "Field `HSEON` reader - HSEON"]
pub struct HSEON_R(crate::FieldReader<bool, bool>);
impl HSEON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEON` writer - HSEON"]
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
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
#[doc = "Field `HSEKERON` reader - HSEKERON"]
pub struct HSEKERON_R(crate::FieldReader<bool, bool>);
impl HSEKERON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEKERON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEKERON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEKERON` writer - HSEKERON"]
pub struct HSEKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEKERON_W<'a> {
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
#[doc = "Field `HSEBYP` reader - HSEBYP"]
pub struct HSEBYP_R(crate::FieldReader<bool, bool>);
impl HSEBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEBYP` writer - HSEBYP"]
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
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
#[doc = "Field `HSECSSON` reader - HSECSSON"]
pub struct HSECSSON_R(crate::FieldReader<bool, bool>);
impl HSECSSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSECSSON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSECSSON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSECSSON` writer - HSECSSON"]
pub struct HSECSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSECSSON_W<'a> {
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
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    pub fn hsekeron(&self) -> HSEKERON_R {
        HSEKERON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSECSSON"]
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W {
        HSIKERON_W { w: self }
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    pub fn csion(&mut self) -> CSION_W {
        CSION_W { w: self }
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    pub fn csikeron(&mut self) -> CSIKERON_W {
        CSIKERON_W { w: self }
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&mut self) -> DIGBYP_W {
        DIGBYP_W { w: self }
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    pub fn hsekeron(&mut self) -> HSEKERON_W {
        HSEKERON_W { w: self }
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    #[doc = "Bit 11 - HSECSSON"]
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HSECSSON_W {
        HSECSSON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ocensetr](index.html) module"]
pub struct RCC_OCENSETR_SPEC;
impl crate::RegisterSpec for RCC_OCENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ocensetr::R](R) reader structure"]
impl crate::Readable for RCC_OCENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ocensetr::W](W) writer structure"]
impl crate::Writable for RCC_OCENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_OCENSETR to value 0x01"]
impl crate::Resettable for RCC_OCENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
