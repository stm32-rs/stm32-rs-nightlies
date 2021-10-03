#[doc = "Register `RCC_MP_AHB4LPENCLRR` reader"]
pub struct R(crate::R<RCC_MP_AHB4LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB4LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB4LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB4LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_AHB4LPENCLRR` writer"]
pub struct W(crate::W<RCC_MP_AHB4LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB4LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB4LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB4LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOALPEN` reader - GPIOALPEN"]
pub struct GPIOALPEN_R(crate::FieldReader<bool, bool>);
impl GPIOALPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOALPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOALPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOALPEN` writer - GPIOALPEN"]
pub struct GPIOALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOALPEN_W<'a> {
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
#[doc = "Field `GPIOBLPEN` reader - GPIOBLPEN"]
pub struct GPIOBLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOBLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOBLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOBLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOBLPEN` writer - GPIOBLPEN"]
pub struct GPIOBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBLPEN_W<'a> {
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
#[doc = "Field `GPIOCLPEN` reader - GPIOCLPEN"]
pub struct GPIOCLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOCLPEN` writer - GPIOCLPEN"]
pub struct GPIOCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCLPEN_W<'a> {
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
#[doc = "Field `GPIODLPEN` reader - GPIODLPEN"]
pub struct GPIODLPEN_R(crate::FieldReader<bool, bool>);
impl GPIODLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIODLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIODLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIODLPEN` writer - GPIODLPEN"]
pub struct GPIODLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODLPEN_W<'a> {
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
#[doc = "Field `GPIOELPEN` reader - GPIOELPEN"]
pub struct GPIOELPEN_R(crate::FieldReader<bool, bool>);
impl GPIOELPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOELPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOELPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOELPEN` writer - GPIOELPEN"]
pub struct GPIOELPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOELPEN_W<'a> {
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
#[doc = "Field `GPIOFLPEN` reader - GPIOFLPEN"]
pub struct GPIOFLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOFLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOFLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOFLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOFLPEN` writer - GPIOFLPEN"]
pub struct GPIOFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFLPEN_W<'a> {
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
#[doc = "Field `GPIOGLPEN` reader - GPIOGLPEN"]
pub struct GPIOGLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOGLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOGLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOGLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOGLPEN` writer - GPIOGLPEN"]
pub struct GPIOGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGLPEN_W<'a> {
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
#[doc = "Field `GPIOHLPEN` reader - GPIOHLPEN"]
pub struct GPIOHLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOHLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOHLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOHLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOHLPEN` writer - GPIOHLPEN"]
pub struct GPIOHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHLPEN_W<'a> {
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
#[doc = "Field `GPIOILPEN` reader - GPIOILPEN"]
pub struct GPIOILPEN_R(crate::FieldReader<bool, bool>);
impl GPIOILPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOILPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOILPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOILPEN` writer - GPIOILPEN"]
pub struct GPIOILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOILPEN_W<'a> {
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
#[doc = "Field `GPIOJLPEN` reader - GPIOJLPEN"]
pub struct GPIOJLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOJLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOJLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOJLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOJLPEN` writer - GPIOJLPEN"]
pub struct GPIOJLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOJLPEN_W<'a> {
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
#[doc = "Field `GPIOKLPEN` reader - GPIOKLPEN"]
pub struct GPIOKLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOKLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOKLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOKLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOKLPEN` writer - GPIOKLPEN"]
pub struct GPIOKLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOKLPEN_W<'a> {
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
    #[doc = "Bit 0 - GPIOALPEN"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOBLPEN"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOCLPEN"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIODLPEN"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOELPEN"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOFLPEN"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOGLPEN"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOHLPEN"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIOILPEN"]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIOJLPEN"]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIOKLPEN"]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOALPEN"]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W {
        GPIOALPEN_W { w: self }
    }
    #[doc = "Bit 1 - GPIOBLPEN"]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W {
        GPIOBLPEN_W { w: self }
    }
    #[doc = "Bit 2 - GPIOCLPEN"]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W {
        GPIOCLPEN_W { w: self }
    }
    #[doc = "Bit 3 - GPIODLPEN"]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W {
        GPIODLPEN_W { w: self }
    }
    #[doc = "Bit 4 - GPIOELPEN"]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W {
        GPIOELPEN_W { w: self }
    }
    #[doc = "Bit 5 - GPIOFLPEN"]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W {
        GPIOFLPEN_W { w: self }
    }
    #[doc = "Bit 6 - GPIOGLPEN"]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W {
        GPIOGLPEN_W { w: self }
    }
    #[doc = "Bit 7 - GPIOHLPEN"]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W {
        GPIOHLPEN_W { w: self }
    }
    #[doc = "Bit 8 - GPIOILPEN"]
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W {
        GPIOILPEN_W { w: self }
    }
    #[doc = "Bit 9 - GPIOJLPEN"]
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W {
        GPIOJLPEN_W { w: self }
    }
    #[doc = "Bit 10 - GPIOKLPEN"]
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W {
        GPIOKLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb4lpenclrr](index.html) module"]
pub struct RCC_MP_AHB4LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB4LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_ahb4lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_AHB4LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb4lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_AHB4LPENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_AHB4LPENCLRR to value 0x07ff"]
impl crate::Resettable for RCC_MP_AHB4LPENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}
