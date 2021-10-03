#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2_EN` reader - Comparator 2 enable bit"]
pub struct COMP2_EN_R(crate::FieldReader<bool, bool>);
impl COMP2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_EN` writer - Comparator 2 enable bit"]
pub struct COMP2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_EN_W<'a> {
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
#[doc = "Field `COMP2_PWRMODE` reader - Power Mode of the comparator 2"]
pub struct COMP2_PWRMODE_R(crate::FieldReader<u8, u8>);
impl COMP2_PWRMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_PWRMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_PWRMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_PWRMODE` writer - Power Mode of the comparator 2"]
pub struct COMP2_PWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_PWRMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP2_INMSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2_INMSEL_R(crate::FieldReader<u8, u8>);
impl COMP2_INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_INMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_INMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_INMSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2_INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP2_INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2_INPSEL_R(crate::FieldReader<u8, u8>);
impl COMP2_INPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_INPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2_INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_INPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `COMP2_WINMODE` reader - Windows mode selection bit"]
pub struct COMP2_WINMODE_R(crate::FieldReader<bool, bool>);
impl COMP2_WINMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_WINMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_WINMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_WINMODE` writer - Windows mode selection bit"]
pub struct COMP2_WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_WINMODE_W<'a> {
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
#[doc = "Field `COMP2_POLARITY` reader - Comparator 2 polarity selection bit"]
pub struct COMP2_POLARITY_R(crate::FieldReader<bool, bool>);
impl COMP2_POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_POLARITY` writer - Comparator 2 polarity selection bit"]
pub struct COMP2_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `COMP2_HYST` reader - Comparator 2 hysteresis selection bits"]
pub struct COMP2_HYST_R(crate::FieldReader<u8, u8>);
impl COMP2_HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_HYST` writer - Comparator 2 hysteresis selection bits"]
pub struct COMP2_HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source selection bits"]
pub struct COMP2_BLANKING_R(crate::FieldReader<u8, u8>);
impl COMP2_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_BLANKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_BLANKING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source selection bits"]
pub struct COMP2_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `COMP2_BRGEN` reader - Scaler bridge enable"]
pub struct COMP2_BRGEN_R(crate::FieldReader<bool, bool>);
impl COMP2_BRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_BRGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_BRGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_BRGEN` writer - Scaler bridge enable"]
pub struct COMP2_BRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_BRGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `COMP2_SCALEN` reader - Voltage scaler enable bit"]
pub struct COMP2_SCALEN_R(crate::FieldReader<bool, bool>);
impl COMP2_SCALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_SCALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_SCALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_SCALEN` writer - Voltage scaler enable bit"]
pub struct COMP2_SCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_SCALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `COMP2_INMESEL` reader - comparator 2 input minus extended selection bits"]
pub struct COMP2_INMESEL_R(crate::FieldReader<u8, u8>);
impl COMP2_INMESEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_INMESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_INMESEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_INMESEL` writer - comparator 2 input minus extended selection bits"]
pub struct COMP2_INMESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_INMESEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `COMP2_VALUE` reader - Comparator 2 output status bit"]
pub struct COMP2_VALUE_R(crate::FieldReader<bool, bool>);
impl COMP2_VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_LOCK` writer - COMP2_CSR register lock bit"]
pub struct COMP2_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&self) -> COMP2_EN_R {
        COMP2_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn comp2_pwrmode(&self) -> COMP2_PWRMODE_R {
        COMP2_PWRMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inmsel(&self) -> COMP2_INMSEL_R {
        COMP2_INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:8 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inpsel(&self) -> COMP2_INPSEL_R {
        COMP2_INPSEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn comp2_winmode(&self) -> COMP2_WINMODE_R {
        COMP2_WINMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&self) -> COMP2_POLARITY_R {
        COMP2_POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp2_hyst(&self) -> COMP2_HYST_R {
        COMP2_HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp2_brgen(&self) -> COMP2_BRGEN_R {
        COMP2_BRGEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp2_scalen(&self) -> COMP2_SCALEN_R {
        COMP2_SCALEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - comparator 2 input minus extended selection bits"]
    #[inline(always)]
    pub fn comp2_inmesel(&self) -> COMP2_INMESEL_R {
        COMP2_INMESEL_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2_value(&self) -> COMP2_VALUE_R {
        COMP2_VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&mut self) -> COMP2_EN_W {
        COMP2_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn comp2_pwrmode(&mut self) -> COMP2_PWRMODE_W {
        COMP2_PWRMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inmsel(&mut self) -> COMP2_INMSEL_W {
        COMP2_INMSEL_W { w: self }
    }
    #[doc = "Bits 7:8 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inpsel(&mut self) -> COMP2_INPSEL_W {
        COMP2_INPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn comp2_winmode(&mut self) -> COMP2_WINMODE_W {
        COMP2_WINMODE_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&mut self) -> COMP2_POLARITY_W {
        COMP2_POLARITY_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp2_hyst(&mut self) -> COMP2_HYST_W {
        COMP2_HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W {
        COMP2_BLANKING_W { w: self }
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp2_brgen(&mut self) -> COMP2_BRGEN_W {
        COMP2_BRGEN_W { w: self }
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp2_scalen(&mut self) -> COMP2_SCALEN_W {
        COMP2_SCALEN_W { w: self }
    }
    #[doc = "Bits 25:26 - comparator 2 input minus extended selection bits"]
    #[inline(always)]
    pub fn comp2_inmesel(&mut self) -> COMP2_INMESEL_W {
        COMP2_INMESEL_W { w: self }
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2_lock(&mut self) -> COMP2_LOCK_W {
        COMP2_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
