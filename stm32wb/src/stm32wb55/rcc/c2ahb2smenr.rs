#[doc = "Register `C2AHB2SMENR` reader"]
pub struct R(crate::R<C2AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB2SMENR` writer"]
pub struct W(crate::W<C2AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB2SMENR_SPEC>;
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
impl From<crate::W<C2AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES1SMEN` reader - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
pub struct AES1SMEN_R(crate::FieldReader<bool, bool>);
impl AES1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES1SMEN` writer - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
pub struct AES1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AES1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ADCFSSMEN` reader - CPU2 ADC clocks enable during Sleep and Stop modes"]
pub struct ADCFSSMEN_R(crate::FieldReader<bool, bool>);
impl ADCFSSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCFSSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCFSSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCFSSMEN` writer - CPU2 ADC clocks enable during Sleep and Stop modes"]
pub struct ADCFSSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCFSSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `GPIOHSMEN` reader - CPU2 IO port H clocks enable during Sleep and Stop modes"]
pub struct GPIOHSMEN_R(crate::FieldReader<bool, bool>);
impl GPIOHSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOHSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOHSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOHSMEN` writer - CPU2 IO port H clocks enable during Sleep and Stop modes"]
pub struct GPIOHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHSMEN_W<'a> {
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
#[doc = "Field `GPIOESMEN` reader - CPU2 IO port E clocks enable during Sleep and Stop modes"]
pub struct GPIOESMEN_R(crate::FieldReader<bool, bool>);
impl GPIOESMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOESMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOESMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOESMEN` writer - CPU2 IO port E clocks enable during Sleep and Stop modes"]
pub struct GPIOESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOESMEN_W<'a> {
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
#[doc = "Field `GPIODSMEN` reader - CPU2 IO port D clocks enable during Sleep and Stop modes"]
pub struct GPIODSMEN_R(crate::FieldReader<bool, bool>);
impl GPIODSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIODSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIODSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIODSMEN` writer - CPU2 IO port D clocks enable during Sleep and Stop modes"]
pub struct GPIODSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODSMEN_W<'a> {
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
#[doc = "Field `GPIOCSMEN` reader - CPU2 IO port C clocks enable during Sleep and Stop modes"]
pub struct GPIOCSMEN_R(crate::FieldReader<bool, bool>);
impl GPIOCSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOCSMEN` writer - CPU2 IO port C clocks enable during Sleep and Stop modes"]
pub struct GPIOCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCSMEN_W<'a> {
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
#[doc = "Field `GPIOBSMEN` reader - CPU2 IO port B clocks enable during Sleep and Stop modes"]
pub struct GPIOBSMEN_R(crate::FieldReader<bool, bool>);
impl GPIOBSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOBSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOBSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOBSMEN` writer - CPU2 IO port B clocks enable during Sleep and Stop modes"]
pub struct GPIOBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBSMEN_W<'a> {
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
#[doc = "Field `GPIOASMEN` reader - CPU2 IO port A clocks enable during Sleep and Stop modes"]
pub struct GPIOASMEN_R(crate::FieldReader<bool, bool>);
impl GPIOASMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOASMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOASMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOASMEN` writer - CPU2 IO port A clocks enable during Sleep and Stop modes"]
pub struct GPIOASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOASMEN_W<'a> {
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
impl R {
    #[doc = "Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aes1smen(&self) -> AES1SMEN_R {
        AES1SMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aes1smen(&mut self) -> AES1SMEN_W {
        AES1SMEN_W { w: self }
    }
    #[doc = "Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W {
        ADCFSSMEN_W { w: self }
    }
    #[doc = "Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W {
        GPIOHSMEN_W { w: self }
    }
    #[doc = "Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W {
        GPIOESMEN_W { w: self }
    }
    #[doc = "Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W {
        GPIODSMEN_W { w: self }
    }
    #[doc = "Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W {
        GPIOCSMEN_W { w: self }
    }
    #[doc = "Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W {
        GPIOBSMEN_W { w: self }
    }
    #[doc = "Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W {
        GPIOASMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb2smenr](index.html) module"]
pub struct C2AHB2SMENR_SPEC;
impl crate::RegisterSpec for C2AHB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb2smenr::R](R) reader structure"]
impl crate::Readable for C2AHB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb2smenr::W](W) writer structure"]
impl crate::Writable for C2AHB2SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2AHB2SMENR to value 0x0001_209f"]
impl crate::Resettable for C2AHB2SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_209f
    }
}
