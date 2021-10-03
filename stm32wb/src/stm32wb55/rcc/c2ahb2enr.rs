#[doc = "Register `C2AHB2ENR` reader"]
pub struct R(crate::R<C2AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB2ENR` writer"]
pub struct W(crate::W<C2AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB2ENR_SPEC>;
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
impl From<crate::W<C2AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES1EN` reader - CPU2 AES1 accelerator clock enable"]
pub struct AES1EN_R(crate::FieldReader<bool, bool>);
impl AES1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES1EN` writer - CPU2 AES1 accelerator clock enable"]
pub struct AES1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AES1EN_W<'a> {
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
#[doc = "Field `ADCEN` reader - CPU2 ADC clock enable"]
pub struct ADCEN_R(crate::FieldReader<bool, bool>);
impl ADCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCEN` writer - CPU2 ADC clock enable"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
#[doc = "Field `GPIOHEN` reader - CPU2 IO port H clock enable"]
pub struct GPIOHEN_R(crate::FieldReader<bool, bool>);
impl GPIOHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOHEN` writer - CPU2 IO port H clock enable"]
pub struct GPIOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHEN_W<'a> {
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
#[doc = "Field `GPIOEEN` reader - CPU2 IO port E clock enable"]
pub struct GPIOEEN_R(crate::FieldReader<bool, bool>);
impl GPIOEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOEEN` writer - CPU2 IO port E clock enable"]
pub struct GPIOEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOEEN_W<'a> {
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
#[doc = "Field `GPIODEN` reader - CPU2 IO port D clock enable"]
pub struct GPIODEN_R(crate::FieldReader<bool, bool>);
impl GPIODEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIODEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIODEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIODEN` writer - CPU2 IO port D clock enable"]
pub struct GPIODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODEN_W<'a> {
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
#[doc = "Field `GPIOCEN` reader - CPU2 IO port C clock enable"]
pub struct GPIOCEN_R(crate::FieldReader<bool, bool>);
impl GPIOCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOCEN` writer - CPU2 IO port C clock enable"]
pub struct GPIOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCEN_W<'a> {
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
#[doc = "Field `GPIOBEN` reader - CPU2 IO port B clock enable"]
pub struct GPIOBEN_R(crate::FieldReader<bool, bool>);
impl GPIOBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOBEN` writer - CPU2 IO port B clock enable"]
pub struct GPIOBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBEN_W<'a> {
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
#[doc = "Field `GPIOAEN` reader - CPU2 IO port A clock enable"]
pub struct GPIOAEN_R(crate::FieldReader<bool, bool>);
impl GPIOAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOAEN` writer - CPU2 IO port A clock enable"]
pub struct GPIOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOAEN_W<'a> {
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
    #[doc = "Bit 16 - CPU2 AES1 accelerator clock enable"]
    #[inline(always)]
    pub fn aes1en(&self) -> AES1EN_R {
        AES1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CPU2 ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPU2 IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CPU2 IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPU2 IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU2 IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CPU2 IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - CPU2 AES1 accelerator clock enable"]
    #[inline(always)]
    pub fn aes1en(&mut self) -> AES1EN_W {
        AES1EN_W { w: self }
    }
    #[doc = "Bit 13 - CPU2 ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 7 - CPU2 IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W {
        GPIOHEN_W { w: self }
    }
    #[doc = "Bit 4 - CPU2 IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W {
        GPIOEEN_W { w: self }
    }
    #[doc = "Bit 3 - CPU2 IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W {
        GPIODEN_W { w: self }
    }
    #[doc = "Bit 2 - CPU2 IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W {
        GPIOCEN_W { w: self }
    }
    #[doc = "Bit 1 - CPU2 IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W {
        GPIOBEN_W { w: self }
    }
    #[doc = "Bit 0 - CPU2 IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W {
        GPIOAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb2enr](index.html) module"]
pub struct C2AHB2ENR_SPEC;
impl crate::RegisterSpec for C2AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb2enr::R](R) reader structure"]
impl crate::Readable for C2AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb2enr::W](W) writer structure"]
impl crate::Writable for C2AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2AHB2ENR to value 0"]
impl crate::Resettable for C2AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
