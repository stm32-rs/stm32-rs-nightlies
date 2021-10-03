#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSUSP` reader - Suspend Timer Mode"]
pub struct TSUSP_R(crate::FieldReader<bool, bool>);
impl TSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUSP` writer - Suspend Timer Mode"]
pub struct TSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUSP_W<'a> {
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
#[doc = "Field `CAIF` reader - Channel acquisition interrupt flag"]
pub struct CAIF_R(crate::FieldReader<bool, bool>);
impl CAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAIE` reader - Channel Acquisition Interrupt Enable / Clear"]
pub struct CAIE_R(crate::FieldReader<bool, bool>);
impl CAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAIE` writer - Channel Acquisition Interrupt Enable / Clear"]
pub struct CAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `RCH13` reader - Select GPIO port PC3 as re-routed ADC input channel CH13."]
pub struct RCH13_R(crate::FieldReader<bool, bool>);
impl RCH13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCH13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCH13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCH13` writer - Select GPIO port PC3 as re-routed ADC input channel CH13."]
pub struct RCH13_W<'a> {
    w: &'a mut W,
}
impl<'a> RCH13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `FCH8` reader - Select GPIO port PB0 as fast ADC input channel CH8."]
pub struct FCH8_R(crate::FieldReader<bool, bool>);
impl FCH8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCH8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCH8` writer - Select GPIO port PB0 as fast ADC input channel CH8."]
pub struct FCH8_W<'a> {
    w: &'a mut W,
}
impl<'a> FCH8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `FCH3` reader - Select GPIO port PA3 as fast ADC input channel CH3."]
pub struct FCH3_R(crate::FieldReader<bool, bool>);
impl FCH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCH3` writer - Select GPIO port PA3 as fast ADC input channel CH3."]
pub struct FCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> FCH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `OUTSEL` reader - Comparator 2 output selection"]
pub struct OUTSEL_R(crate::FieldReader<u8, u8>);
impl OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTSEL` writer - Comparator 2 output selection"]
pub struct OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `INSEL` reader - Inverted input selection"]
pub struct INSEL_R(crate::FieldReader<u8, u8>);
impl INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL` writer - Inverted input selection"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `WNDWE` reader - Window mode enable"]
pub struct WNDWE_R(crate::FieldReader<bool, bool>);
impl WNDWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WNDWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WNDWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WNDWE` writer - Window mode enable"]
pub struct WNDWE_W<'a> {
    w: &'a mut W,
}
impl<'a> WNDWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `VREFOUTEN` reader - VREFINT output enable"]
pub struct VREFOUTEN_R(crate::FieldReader<bool, bool>);
impl VREFOUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFOUTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFOUTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFOUTEN` writer - VREFINT output enable"]
pub struct VREFOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFOUTEN_W<'a> {
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
#[doc = "Field `CMP2OUT` reader - Comparator 2 output"]
pub struct CMP2OUT_R(crate::FieldReader<bool, bool>);
impl CMP2OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED` reader - Comparator 2 speed mode"]
pub struct SPEED_R(crate::FieldReader<bool, bool>);
impl SPEED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED` writer - Comparator 2 speed mode"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
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
#[doc = "Field `CMP1OUT` reader - Comparator 1 output"]
pub struct CMP1OUT_R(crate::FieldReader<bool, bool>);
impl CMP1OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW1` reader - SW1 analog switch enable"]
pub struct SW1_R(crate::FieldReader<bool, bool>);
impl SW1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW1` writer - SW1 analog switch enable"]
pub struct SW1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW1_W<'a> {
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
#[doc = "Field `CMP1EN` reader - Comparator 1 enable"]
pub struct CMP1EN_R(crate::FieldReader<bool, bool>);
impl CMP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1EN` writer - Comparator 1 enable"]
pub struct CMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1EN_W<'a> {
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
#[doc = "Field `PD400K` reader - 400 kO pull-down resistor"]
pub struct PD400K_R(crate::FieldReader<bool, bool>);
impl PD400K_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD400K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD400K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD400K` writer - 400 kO pull-down resistor"]
pub struct PD400K_W<'a> {
    w: &'a mut W,
}
impl<'a> PD400K_W<'a> {
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
#[doc = "Field `PD10K` reader - 10 kO pull-down resistor"]
pub struct PD10K_R(crate::FieldReader<bool, bool>);
impl PD10K_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD10K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD10K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD10K` writer - 10 kO pull-down resistor"]
pub struct PD10K_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10K_W<'a> {
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
#[doc = "Field `PU400K` reader - 400 kO pull-up resistor"]
pub struct PU400K_R(crate::FieldReader<bool, bool>);
impl PU400K_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU400K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU400K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU400K` writer - 400 kO pull-up resistor"]
pub struct PU400K_W<'a> {
    w: &'a mut W,
}
impl<'a> PU400K_W<'a> {
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
#[doc = "Field `PU10K` reader - 10 kO pull-up resistor"]
pub struct PU10K_R(crate::FieldReader<bool, bool>);
impl PU10K_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU10K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU10K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU10K` writer - 10 kO pull-up resistor"]
pub struct PU10K_W<'a> {
    w: &'a mut W,
}
impl<'a> PU10K_W<'a> {
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
    #[doc = "Bit 31 - Suspend Timer Mode"]
    #[inline(always)]
    pub fn tsusp(&self) -> TSUSP_R {
        TSUSP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel acquisition interrupt flag"]
    #[inline(always)]
    pub fn caif(&self) -> CAIF_R {
        CAIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel Acquisition Interrupt Enable / Clear"]
    #[inline(always)]
    pub fn caie(&self) -> CAIE_R {
        CAIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13."]
    #[inline(always)]
    pub fn rch13(&self) -> RCH13_R {
        RCH13_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8."]
    #[inline(always)]
    pub fn fch8(&self) -> FCH8_R {
        FCH8_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3."]
    #[inline(always)]
    pub fn fch3(&self) -> FCH3_R {
        FCH3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 21:23 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Inverted input selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 17 - Window mode enable"]
    #[inline(always)]
    pub fn wndwe(&self) -> WNDWE_R {
        WNDWE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - VREFINT output enable"]
    #[inline(always)]
    pub fn vrefouten(&self) -> VREFOUTEN_R {
        VREFOUTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparator 2 output"]
    #[inline(always)]
    pub fn cmp2out(&self) -> CMP2OUT_R {
        CMP2OUT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 2 speed mode"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 output"]
    #[inline(always)]
    pub fn cmp1out(&self) -> CMP1OUT_R {
        CMP1OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SW1 analog switch enable"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 400 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd400k(&self) -> PD400K_R {
        PD400K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 10 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd10k(&self) -> PD10K_R {
        PD10K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 400 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu400k(&self) -> PU400K_R {
        PU400K_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 10 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu10k(&self) -> PU10K_R {
        PU10K_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Suspend Timer Mode"]
    #[inline(always)]
    pub fn tsusp(&mut self) -> TSUSP_W {
        TSUSP_W { w: self }
    }
    #[doc = "Bit 29 - Channel Acquisition Interrupt Enable / Clear"]
    #[inline(always)]
    pub fn caie(&mut self) -> CAIE_W {
        CAIE_W { w: self }
    }
    #[doc = "Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13."]
    #[inline(always)]
    pub fn rch13(&mut self) -> RCH13_W {
        RCH13_W { w: self }
    }
    #[doc = "Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8."]
    #[inline(always)]
    pub fn fch8(&mut self) -> FCH8_W {
        FCH8_W { w: self }
    }
    #[doc = "Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3."]
    #[inline(always)]
    pub fn fch3(&mut self) -> FCH3_W {
        FCH3_W { w: self }
    }
    #[doc = "Bits 21:23 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W { w: self }
    }
    #[doc = "Bits 18:20 - Inverted input selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
    #[doc = "Bit 17 - Window mode enable"]
    #[inline(always)]
    pub fn wndwe(&mut self) -> WNDWE_W {
        WNDWE_W { w: self }
    }
    #[doc = "Bit 16 - VREFINT output enable"]
    #[inline(always)]
    pub fn vrefouten(&mut self) -> VREFOUTEN_W {
        VREFOUTEN_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 2 speed mode"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 5 - SW1 analog switch enable"]
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W {
        SW1_W { w: self }
    }
    #[doc = "Bit 4 - Comparator 1 enable"]
    #[inline(always)]
    pub fn cmp1en(&mut self) -> CMP1EN_W {
        CMP1EN_W { w: self }
    }
    #[doc = "Bit 3 - 400 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd400k(&mut self) -> PD400K_W {
        PD400K_W { w: self }
    }
    #[doc = "Bit 2 - 10 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd10k(&mut self) -> PD10K_W {
        PD10K_W { w: self }
    }
    #[doc = "Bit 1 - 400 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu400k(&mut self) -> PU400K_W {
        PU400K_W { w: self }
    }
    #[doc = "Bit 0 - 10 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu10k(&mut self) -> PU10K_W {
        PU10K_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "comparator control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
