#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRDYIE` reader - Channel Configuration Ready Interrupt enable"]
pub struct CCRDYIE_R(crate::FieldReader<bool, bool>);
impl CCRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRDYIE` writer - Channel Configuration Ready Interrupt enable"]
pub struct CCRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRDYIE_W<'a> {
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
#[doc = "Field `EOCALIE` reader - End of calibration interrupt enable"]
pub struct EOCALIE_R(crate::FieldReader<bool, bool>);
impl EOCALIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCALIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOCALIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCALIE` writer - End of calibration interrupt enable"]
pub struct EOCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCALIE_W<'a> {
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
#[doc = "Field `AWD3IE` reader - ADC analog watchdog 3 interrupt"]
pub struct AWD3IE_R(crate::FieldReader<bool, bool>);
impl AWD3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD3IE` writer - ADC analog watchdog 3 interrupt"]
pub struct AWD3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3IE_W<'a> {
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
#[doc = "Field `AWD2IE` reader - ADC analog watchdog 2 interrupt"]
pub struct AWD2IE_R(crate::FieldReader<bool, bool>);
impl AWD2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD2IE` writer - ADC analog watchdog 2 interrupt"]
pub struct AWD2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2IE_W<'a> {
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
#[doc = "Field `AWD1IE` reader - ADC analog watchdog 1 interrupt"]
pub struct AWD1IE_R(crate::FieldReader<bool, bool>);
impl AWD1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD1IE` writer - ADC analog watchdog 1 interrupt"]
pub struct AWD1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1IE_W<'a> {
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
#[doc = "Field `OVRIE` reader - ADC group regular overrun interrupt"]
pub struct OVRIE_R(crate::FieldReader<bool, bool>);
impl OVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRIE` writer - ADC group regular overrun interrupt"]
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
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
#[doc = "Field `EOSIE` reader - ADC group regular end of sequence conversions interrupt"]
pub struct EOSIE_R(crate::FieldReader<bool, bool>);
impl EOSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOSIE` writer - ADC group regular end of sequence conversions interrupt"]
pub struct EOSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSIE_W<'a> {
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
#[doc = "Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt"]
pub struct EOCIE_R(crate::FieldReader<bool, bool>);
impl EOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt"]
pub struct EOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCIE_W<'a> {
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
#[doc = "Field `EOSMPIE` reader - ADC group regular end of sampling interrupt"]
pub struct EOSMPIE_R(crate::FieldReader<bool, bool>);
impl EOSMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOSMPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOSMPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOSMPIE` writer - ADC group regular end of sampling interrupt"]
pub struct EOSMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSMPIE_W<'a> {
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
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt"]
pub struct ADRDYIE_R(crate::FieldReader<bool, bool>);
impl ADRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt"]
pub struct ADRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRDYIE_W<'a> {
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
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable"]
    #[inline(always)]
    pub fn ccrdyie(&self) -> CCRDYIE_R {
        CCRDYIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End of calibration interrupt enable"]
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable"]
    #[inline(always)]
    pub fn ccrdyie(&mut self) -> CCRDYIE_W {
        CCRDYIE_W { w: self }
    }
    #[doc = "Bit 11 - End of calibration interrupt enable"]
    #[inline(always)]
    pub fn eocalie(&mut self) -> EOCALIE_W {
        EOCALIE_W { w: self }
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWD3IE_W {
        AWD3IE_W { w: self }
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWD2IE_W {
        AWD2IE_W { w: self }
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWD1IE_W {
        AWD1IE_W { w: self }
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W {
        EOSIE_W { w: self }
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W { w: self }
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W {
        EOSMPIE_W { w: self }
    }
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W {
        ADRDYIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
