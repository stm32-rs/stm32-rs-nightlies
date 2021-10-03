#[doc = "Register `COMP1_CSR` reader"]
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CSR` writer"]
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_EN` reader - Comparator enable"]
pub struct COMP1_EN_R(crate::FieldReader<bool, bool>);
impl COMP1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_EN` writer - Comparator enable"]
pub struct COMP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_EN_W<'a> {
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
#[doc = "Field `COMP1_PWRMODE` reader - Comparator power mode"]
pub struct COMP1_PWRMODE_R(crate::FieldReader<u8, u8>);
impl COMP1_PWRMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_PWRMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_PWRMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_PWRMODE` writer - Comparator power mode"]
pub struct COMP1_PWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_PWRMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP1_INMSEL` reader - Comparator input minus selection"]
pub struct COMP1_INMSEL_R(crate::FieldReader<u8, u8>);
impl COMP1_INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_INMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_INMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_INMSEL` writer - Comparator input minus selection"]
pub struct COMP1_INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP1_INPSEL` reader - Comparator input plus selection"]
pub struct COMP1_INPSEL_R(crate::FieldReader<u8, u8>);
impl COMP1_INPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_INPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_INPSEL` writer - Comparator input plus selection"]
pub struct COMP1_INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_INPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `COMP1_POLARITY` reader - Comparator output polarity"]
pub struct COMP1_POLARITY_R(crate::FieldReader<bool, bool>);
impl COMP1_POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_POLARITY` writer - Comparator output polarity"]
pub struct COMP1_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_POLARITY_W<'a> {
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
#[doc = "Field `COMP1_HYST` reader - Comparator hysteresis"]
pub struct COMP1_HYST_R(crate::FieldReader<u8, u8>);
impl COMP1_HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_HYST` writer - Comparator hysteresis"]
pub struct COMP1_HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `COMP1_BLANKING` reader - Comparator blanking source"]
pub struct COMP1_BLANKING_R(crate::FieldReader<u8, u8>);
impl COMP1_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_BLANKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_BLANKING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_BLANKING` writer - Comparator blanking source"]
pub struct COMP1_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `COMP1_BRGEN` reader - Comparator voltage scaler enable"]
pub struct COMP1_BRGEN_R(crate::FieldReader<bool, bool>);
impl COMP1_BRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_BRGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_BRGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_BRGEN` writer - Comparator voltage scaler enable"]
pub struct COMP1_BRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_BRGEN_W<'a> {
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
#[doc = "Field `COMP1_SCALEN` reader - Comparator scaler bridge enable"]
pub struct COMP1_SCALEN_R(crate::FieldReader<bool, bool>);
impl COMP1_SCALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_SCALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_SCALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_SCALEN` writer - Comparator scaler bridge enable"]
pub struct COMP1_SCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_SCALEN_W<'a> {
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
#[doc = "Field `COMP1_INMESEL` reader - Comparator input minus extended selection"]
pub struct COMP1_INMESEL_R(crate::FieldReader<u8, u8>);
impl COMP1_INMESEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_INMESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_INMESEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_INMESEL` writer - Comparator input minus extended selection"]
pub struct COMP1_INMESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_INMESEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `COMP1_VALUE` reader - Comparator output level"]
pub struct COMP1_VALUE_R(crate::FieldReader<bool, bool>);
impl COMP1_VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_LOCK` reader - Comparator lock"]
pub struct COMP1_LOCK_R(crate::FieldReader<bool, bool>);
impl COMP1_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_LOCK` writer - Comparator lock"]
pub struct COMP1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator power mode"]
    #[inline(always)]
    pub fn comp1_pwrmode(&self) -> COMP1_PWRMODE_R {
        COMP1_PWRMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator input minus selection"]
    #[inline(always)]
    pub fn comp1_inmsel(&self) -> COMP1_INMSEL_R {
        COMP1_INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:8 - Comparator input plus selection"]
    #[inline(always)]
    pub fn comp1_inpsel(&self) -> COMP1_INPSEL_R {
        COMP1_INPSEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Comparator output polarity"]
    #[inline(always)]
    pub fn comp1_polarity(&self) -> COMP1_POLARITY_R {
        COMP1_POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    pub fn comp1_hyst(&self) -> COMP1_HYST_R {
        COMP1_HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator blanking source"]
    #[inline(always)]
    pub fn comp1_blanking(&self) -> COMP1_BLANKING_R {
        COMP1_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 22 - Comparator voltage scaler enable"]
    #[inline(always)]
    pub fn comp1_brgen(&self) -> COMP1_BRGEN_R {
        COMP1_BRGEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Comparator scaler bridge enable"]
    #[inline(always)]
    pub fn comp1_scalen(&self) -> COMP1_SCALEN_R {
        COMP1_SCALEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Comparator input minus extended selection"]
    #[inline(always)]
    pub fn comp1_inmesel(&self) -> COMP1_INMESEL_R {
        COMP1_INMESEL_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Comparator output level"]
    #[inline(always)]
    pub fn comp1_value(&self) -> COMP1_VALUE_R {
        COMP1_VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator lock"]
    #[inline(always)]
    pub fn comp1_lock(&self) -> COMP1_LOCK_R {
        COMP1_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator enable"]
    #[inline(always)]
    pub fn comp1_en(&mut self) -> COMP1_EN_W {
        COMP1_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator power mode"]
    #[inline(always)]
    pub fn comp1_pwrmode(&mut self) -> COMP1_PWRMODE_W {
        COMP1_PWRMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator input minus selection"]
    #[inline(always)]
    pub fn comp1_inmsel(&mut self) -> COMP1_INMSEL_W {
        COMP1_INMSEL_W { w: self }
    }
    #[doc = "Bits 7:8 - Comparator input plus selection"]
    #[inline(always)]
    pub fn comp1_inpsel(&mut self) -> COMP1_INPSEL_W {
        COMP1_INPSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator output polarity"]
    #[inline(always)]
    pub fn comp1_polarity(&mut self) -> COMP1_POLARITY_W {
        COMP1_POLARITY_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    pub fn comp1_hyst(&mut self) -> COMP1_HYST_W {
        COMP1_HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator blanking source"]
    #[inline(always)]
    pub fn comp1_blanking(&mut self) -> COMP1_BLANKING_W {
        COMP1_BLANKING_W { w: self }
    }
    #[doc = "Bit 22 - Comparator voltage scaler enable"]
    #[inline(always)]
    pub fn comp1_brgen(&mut self) -> COMP1_BRGEN_W {
        COMP1_BRGEN_W { w: self }
    }
    #[doc = "Bit 23 - Comparator scaler bridge enable"]
    #[inline(always)]
    pub fn comp1_scalen(&mut self) -> COMP1_SCALEN_W {
        COMP1_SCALEN_W { w: self }
    }
    #[doc = "Bits 25:26 - Comparator input minus extended selection"]
    #[inline(always)]
    pub fn comp1_inmesel(&mut self) -> COMP1_INMESEL_W {
        COMP1_INMESEL_W { w: self }
    }
    #[doc = "Bit 31 - Comparator lock"]
    #[inline(always)]
    pub fn comp1_lock(&mut self) -> COMP1_LOCK_W {
        COMP1_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](index.html) module"]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_csr::R](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
