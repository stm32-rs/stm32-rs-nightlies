#[doc = "Register `C2IMR1` reader"]
pub struct R(crate::R<C2IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR1` writer"]
pub struct W(crate::W<C2IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR1_SPEC>;
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
impl From<crate::W<C2IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCSTAMP` reader - Peripheral RTCSTAMP interrupt mask to CPU2"]
pub struct RTCSTAMP_R(crate::FieldReader<bool, bool>);
impl RTCSTAMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCSTAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCSTAMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSTAMP` writer - Peripheral RTCSTAMP interrupt mask to CPU2"]
pub struct RTCSTAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSTAMP_W<'a> {
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
#[doc = "Field `RTCWKUP` reader - Peripheral RTCWKUP interrupt mask to CPU2"]
pub struct RTCWKUP_R(crate::FieldReader<bool, bool>);
impl RTCWKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCWKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCWKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCWKUP` writer - Peripheral RTCWKUP interrupt mask to CPU2"]
pub struct RTCWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCWKUP_W<'a> {
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
#[doc = "Field `RTCALARM` reader - Peripheral RTCALARM interrupt mask to CPU2"]
pub struct RTCALARM_R(crate::FieldReader<bool, bool>);
impl RTCALARM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCALARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCALARM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCALARM` writer - Peripheral RTCALARM interrupt mask to CPU2"]
pub struct RTCALARM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCALARM_W<'a> {
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
#[doc = "Field `RCC` reader - Peripheral RCC interrupt mask to CPU2"]
pub struct RCC_R(crate::FieldReader<bool, bool>);
impl RCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCC` writer - Peripheral RCC interrupt mask to CPU2"]
pub struct RCC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCC_W<'a> {
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
#[doc = "Field `FLASH` reader - Peripheral FLASH interrupt mask to CPU2"]
pub struct FLASH_R(crate::FieldReader<bool, bool>);
impl FLASH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH` writer - Peripheral FLASH interrupt mask to CPU2"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
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
#[doc = "Field `PKA` reader - Peripheral PKA interrupt mask to CPU2"]
pub struct PKA_R(crate::FieldReader<bool, bool>);
impl PKA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKA` writer - Peripheral PKA interrupt mask to CPU2"]
pub struct PKA_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_W<'a> {
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
#[doc = "Field `RNG` reader - Peripheral RNG interrupt mask to CPU2"]
pub struct RNG_R(crate::FieldReader<bool, bool>);
impl RNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG` writer - Peripheral RNG interrupt mask to CPU2"]
pub struct RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_W<'a> {
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
#[doc = "Field `AES1` reader - Peripheral AES1 interrupt mask to CPU2"]
pub struct AES1_R(crate::FieldReader<bool, bool>);
impl AES1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES1` writer - Peripheral AES1 interrupt mask to CPU2"]
pub struct AES1_W<'a> {
    w: &'a mut W,
}
impl<'a> AES1_W<'a> {
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
#[doc = "Field `COMP` reader - Peripheral COMP interrupt mask to CPU2"]
pub struct COMP_R(crate::FieldReader<bool, bool>);
impl COMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP` writer - Peripheral COMP interrupt mask to CPU2"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
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
#[doc = "Field `ADC` reader - Peripheral ADC interrupt mask to CPU2"]
pub struct ADC_R(crate::FieldReader<bool, bool>);
impl ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC` writer - Peripheral ADC interrupt mask to CPU2"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcstamp(&self) -> RTCSTAMP_R {
        RTCSTAMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcwkup(&self) -> RTCWKUP_R {
        RTCWKUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcalarm(&self) -> RTCALARM_R {
        RTCALARM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn aes1(&self) -> AES1_R {
        AES1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcstamp(&mut self) -> RTCSTAMP_W {
        RTCSTAMP_W { w: self }
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcwkup(&mut self) -> RTCWKUP_W {
        RTCWKUP_W { w: self }
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcalarm(&mut self) -> RTCALARM_W {
        RTCALARM_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rcc(&mut self) -> RCC_W {
        RCC_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pka(&mut self) -> PKA_W {
        PKA_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W {
        RNG_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn aes1(&mut self) -> AES1_W {
        AES1_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](index.html) module"]
pub struct C2IMR1_SPEC;
impl crate::RegisterSpec for C2IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr1::R](R) reader structure"]
impl crate::Readable for C2IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](W) writer structure"]
impl crate::Writable for C2IMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2IMR1 to value 0"]
impl crate::Resettable for C2IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
