#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC Channel 1 calibration enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN1_A {
    #[doc = "0: DAC Channel X Normal operating mode"]
    NORMAL = 0,
    #[doc = "1: DAC Channel X calibration mode"]
    CALIBRATION = 1,
}
impl From<CEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN1` reader - DAC Channel 1 calibration enable"]
pub struct CEN1_R(crate::FieldReader<bool, CEN1_A>);
impl CEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN1_A {
        match self.bits {
            false => CEN1_A::NORMAL,
            true => CEN1_A::CALIBRATION,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == CEN1_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CALIBRATION`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        **self == CEN1_A::CALIBRATION
    }
}
impl core::ops::Deref for CEN1_R {
    type Target = crate::FieldReader<bool, CEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN1` writer - DAC Channel 1 calibration enable"]
pub struct CEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC Channel X Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CEN1_A::NORMAL)
    }
    #[doc = "DAC Channel X calibration mode"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(CEN1_A::CALIBRATION)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE1_A {
    #[doc = "0: DAC Channel X  DMA Underrun Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: DAC Channel X DMA Underrun Interrupt enabled"]
    ENABLED = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub struct DMAUDRIE1_R(crate::FieldReader<bool, DMAUDRIE1_A>);
impl DMAUDRIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDRIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::DISABLED,
            true => DMAUDRIE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAUDRIE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAUDRIE1_A::ENABLED
    }
}
impl core::ops::Deref for DMAUDRIE1_R {
    type Target = crate::FieldReader<bool, DMAUDRIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub struct DMAUDRIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDRIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::DISABLED)
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::ENABLED)
    }
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
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN1_A {
    #[doc = "0: DAC Channel X DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DAC Channel X DMA mode enabled"]
    ENABLED = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub struct DMAEN1_R(crate::FieldReader<bool, DMAEN1_A>);
impl DMAEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::DISABLED,
            true => DMAEN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN1_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN1_R {
    type Target = crate::FieldReader<bool, DMAEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub struct DMAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC Channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::DISABLED)
    }
    #[doc = "DAC Channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::ENABLED)
    }
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
#[doc = "DAC channel1 mask/amplitude selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAMP1_A {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    AMP1 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    AMP3 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    AMP7 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    AMP15 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    AMP31 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    AMP63 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    AMP127 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    AMP255 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    AMP511 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    AMP1023 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    AMP2047 = 10,
    #[doc = "11: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    AMP4095 = 11,
}
impl From<MAMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub struct MAMP1_R(crate::FieldReader<u8, MAMP1_A>);
impl MAMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAMP1_A> {
        match self.bits {
            0 => Some(MAMP1_A::AMP1),
            1 => Some(MAMP1_A::AMP3),
            2 => Some(MAMP1_A::AMP7),
            3 => Some(MAMP1_A::AMP15),
            4 => Some(MAMP1_A::AMP31),
            5 => Some(MAMP1_A::AMP63),
            6 => Some(MAMP1_A::AMP127),
            7 => Some(MAMP1_A::AMP255),
            8 => Some(MAMP1_A::AMP511),
            9 => Some(MAMP1_A::AMP1023),
            10 => Some(MAMP1_A::AMP2047),
            11 => Some(MAMP1_A::AMP4095),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AMP1`"]
    #[inline(always)]
    pub fn is_amp1(&self) -> bool {
        **self == MAMP1_A::AMP1
    }
    #[doc = "Checks if the value of the field is `AMP3`"]
    #[inline(always)]
    pub fn is_amp3(&self) -> bool {
        **self == MAMP1_A::AMP3
    }
    #[doc = "Checks if the value of the field is `AMP7`"]
    #[inline(always)]
    pub fn is_amp7(&self) -> bool {
        **self == MAMP1_A::AMP7
    }
    #[doc = "Checks if the value of the field is `AMP15`"]
    #[inline(always)]
    pub fn is_amp15(&self) -> bool {
        **self == MAMP1_A::AMP15
    }
    #[doc = "Checks if the value of the field is `AMP31`"]
    #[inline(always)]
    pub fn is_amp31(&self) -> bool {
        **self == MAMP1_A::AMP31
    }
    #[doc = "Checks if the value of the field is `AMP63`"]
    #[inline(always)]
    pub fn is_amp63(&self) -> bool {
        **self == MAMP1_A::AMP63
    }
    #[doc = "Checks if the value of the field is `AMP127`"]
    #[inline(always)]
    pub fn is_amp127(&self) -> bool {
        **self == MAMP1_A::AMP127
    }
    #[doc = "Checks if the value of the field is `AMP255`"]
    #[inline(always)]
    pub fn is_amp255(&self) -> bool {
        **self == MAMP1_A::AMP255
    }
    #[doc = "Checks if the value of the field is `AMP511`"]
    #[inline(always)]
    pub fn is_amp511(&self) -> bool {
        **self == MAMP1_A::AMP511
    }
    #[doc = "Checks if the value of the field is `AMP1023`"]
    #[inline(always)]
    pub fn is_amp1023(&self) -> bool {
        **self == MAMP1_A::AMP1023
    }
    #[doc = "Checks if the value of the field is `AMP2047`"]
    #[inline(always)]
    pub fn is_amp2047(&self) -> bool {
        **self == MAMP1_A::AMP2047
    }
    #[doc = "Checks if the value of the field is `AMP4095`"]
    #[inline(always)]
    pub fn is_amp4095(&self) -> bool {
        **self == MAMP1_A::AMP4095
    }
}
impl core::ops::Deref for MAMP1_R {
    type Target = crate::FieldReader<u8, MAMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub struct MAMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAMP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn amp1(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP1)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn amp3(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP3)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn amp7(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP7)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn amp15(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP15)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn amp31(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP31)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    #[inline(always)]
    pub fn amp63(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP63)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn amp127(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP127)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn amp255(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP255)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn amp511(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP511)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn amp1023(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP1023)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn amp2047(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP2047)
    }
    #[doc = "Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn amp4095(self) -> &'a mut W {
        self.variant(MAMP1_A::AMP4095)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVE1_A {
    #[doc = "0: Wave generation disabled"]
    DISABLED = 0,
    #[doc = "1: Noise wave generation enabled"]
    NOISE = 1,
    #[doc = "2: Triangle wave generation enabled"]
    TRIANGLE = 2,
}
impl From<WAVE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub struct WAVE1_R(crate::FieldReader<u8, WAVE1_A>);
impl WAVE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVE1_A> {
        match self.bits {
            0 => Some(WAVE1_A::DISABLED),
            1 => Some(WAVE1_A::NOISE),
            2 => Some(WAVE1_A::TRIANGLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAVE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        **self == WAVE1_A::NOISE
    }
    #[doc = "Checks if the value of the field is `TRIANGLE`"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        **self == WAVE1_A::TRIANGLE
    }
}
impl core::ops::Deref for WAVE1_R {
    type Target = crate::FieldReader<u8, WAVE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub struct WAVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE1_A::DISABLED)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE1_A::NOISE)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE1_A::TRIANGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN1_A {
    #[doc = "0: DAC Channel X trigger disabled"]
    DISABLED = 0,
    #[doc = "1: DAC Channel X trigger enabled"]
    ENABLED = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub struct TEN1_R(crate::FieldReader<bool, TEN1_A>);
impl TEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::DISABLED,
            true => TEN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TEN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TEN1_A::ENABLED
    }
}
impl core::ops::Deref for TEN1_R {
    type Target = crate::FieldReader<bool, TEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub struct TEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC Channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1_A::DISABLED)
    }
    #[doc = "DAC Channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1_A::ENABLED)
    }
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
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1_A {
    #[doc = "0: DAC Channel X disabled"]
    DISABLED = 0,
    #[doc = "1: DAC Channel X enabled"]
    ENABLED = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub struct EN1_R(crate::FieldReader<bool, EN1_A>);
impl EN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::DISABLED,
            true => EN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN1_A::ENABLED
    }
}
impl core::ops::Deref for EN1_R {
    type Target = crate::FieldReader<bool, EN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC Channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1_A::DISABLED)
    }
    #[doc = "DAC Channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1_A::ENABLED)
    }
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
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: SWTRIG1"]
    SWTRIG = 0,
    #[doc = "1: dac_chx_trg1"]
    TIM1_TRGO = 1,
    #[doc = "2: dac_chx_trg2"]
    TIM2_TRGO = 2,
    #[doc = "3: dac_chx_trg3"]
    TRG3 = 3,
    #[doc = "4: dac_chx_trg4"]
    TRG4 = 4,
    #[doc = "5: dac_chx_trg5"]
    TRG5 = 5,
    #[doc = "6: dac_chx_trg6"]
    TRG6 = 6,
    #[doc = "7: dac_chx_trg7"]
    TRG7 = 7,
    #[doc = "8: dac_chx_trg8"]
    TRG8 = 8,
    #[doc = "9: dac_chx_trg9"]
    TRG9 = 9,
    #[doc = "10: dac_chx_trg10"]
    TRG10 = 10,
    #[doc = "11: dac_chx_trg11"]
    LPTIM1_OUT = 11,
    #[doc = "12: dac_chx_trg12"]
    LPTIM2_OUT = 12,
    #[doc = "13: dac_chx_trg13"]
    LPTIM3_OUT = 13,
    #[doc = "14: dac_chx_trg14"]
    EXTI9 = 14,
    #[doc = "15: dac_chx_trg15"]
    TRG15 = 15,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub struct TSEL1_R(crate::FieldReader<u8, TSEL1_A>);
impl TSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEL1_A {
        match self.bits {
            0 => TSEL1_A::SWTRIG,
            1 => TSEL1_A::TIM1_TRGO,
            2 => TSEL1_A::TIM2_TRGO,
            3 => TSEL1_A::TRG3,
            4 => TSEL1_A::TRG4,
            5 => TSEL1_A::TRG5,
            6 => TSEL1_A::TRG6,
            7 => TSEL1_A::TRG7,
            8 => TSEL1_A::TRG8,
            9 => TSEL1_A::TRG9,
            10 => TSEL1_A::TRG10,
            11 => TSEL1_A::LPTIM1_OUT,
            12 => TSEL1_A::LPTIM2_OUT,
            13 => TSEL1_A::LPTIM3_OUT,
            14 => TSEL1_A::EXTI9,
            15 => TSEL1_A::TRG15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWTRIG`"]
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        **self == TSEL1_A::SWTRIG
    }
    #[doc = "Checks if the value of the field is `TIM1_TRGO`"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        **self == TSEL1_A::TIM1_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        **self == TSEL1_A::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `TRG3`"]
    #[inline(always)]
    pub fn is_trg3(&self) -> bool {
        **self == TSEL1_A::TRG3
    }
    #[doc = "Checks if the value of the field is `TRG4`"]
    #[inline(always)]
    pub fn is_trg4(&self) -> bool {
        **self == TSEL1_A::TRG4
    }
    #[doc = "Checks if the value of the field is `TRG5`"]
    #[inline(always)]
    pub fn is_trg5(&self) -> bool {
        **self == TSEL1_A::TRG5
    }
    #[doc = "Checks if the value of the field is `TRG6`"]
    #[inline(always)]
    pub fn is_trg6(&self) -> bool {
        **self == TSEL1_A::TRG6
    }
    #[doc = "Checks if the value of the field is `TRG7`"]
    #[inline(always)]
    pub fn is_trg7(&self) -> bool {
        **self == TSEL1_A::TRG7
    }
    #[doc = "Checks if the value of the field is `TRG8`"]
    #[inline(always)]
    pub fn is_trg8(&self) -> bool {
        **self == TSEL1_A::TRG8
    }
    #[doc = "Checks if the value of the field is `TRG9`"]
    #[inline(always)]
    pub fn is_trg9(&self) -> bool {
        **self == TSEL1_A::TRG9
    }
    #[doc = "Checks if the value of the field is `TRG10`"]
    #[inline(always)]
    pub fn is_trg10(&self) -> bool {
        **self == TSEL1_A::TRG10
    }
    #[doc = "Checks if the value of the field is `LPTIM1_OUT`"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        **self == TSEL1_A::LPTIM1_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM2_OUT`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        **self == TSEL1_A::LPTIM2_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM3_OUT`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        **self == TSEL1_A::LPTIM3_OUT
    }
    #[doc = "Checks if the value of the field is `EXTI9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        **self == TSEL1_A::EXTI9
    }
    #[doc = "Checks if the value of the field is `TRG15`"]
    #[inline(always)]
    pub fn is_trg15(&self) -> bool {
        **self == TSEL1_A::TRG15
    }
}
impl core::ops::Deref for TSEL1_R {
    type Target = crate::FieldReader<u8, TSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub struct TSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut W {
        self.variant(TSEL1_A::SWTRIG)
    }
    #[doc = "dac_chx_trg1"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM1_TRGO)
    }
    #[doc = "dac_chx_trg2"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM2_TRGO)
    }
    #[doc = "dac_chx_trg3"]
    #[inline(always)]
    pub fn trg3(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG3)
    }
    #[doc = "dac_chx_trg4"]
    #[inline(always)]
    pub fn trg4(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG4)
    }
    #[doc = "dac_chx_trg5"]
    #[inline(always)]
    pub fn trg5(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG5)
    }
    #[doc = "dac_chx_trg6"]
    #[inline(always)]
    pub fn trg6(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG6)
    }
    #[doc = "dac_chx_trg7"]
    #[inline(always)]
    pub fn trg7(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG7)
    }
    #[doc = "dac_chx_trg8"]
    #[inline(always)]
    pub fn trg8(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG8)
    }
    #[doc = "dac_chx_trg9"]
    #[inline(always)]
    pub fn trg9(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG9)
    }
    #[doc = "dac_chx_trg10"]
    #[inline(always)]
    pub fn trg10(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG10)
    }
    #[doc = "dac_chx_trg11"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(TSEL1_A::LPTIM1_OUT)
    }
    #[doc = "dac_chx_trg12"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(TSEL1_A::LPTIM2_OUT)
    }
    #[doc = "dac_chx_trg13"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(TSEL1_A::LPTIM3_OUT)
    }
    #[doc = "dac_chx_trg14"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1_A::EXTI9)
    }
    #[doc = "dac_chx_trg15"]
    #[inline(always)]
    pub fn trg15(self) -> &'a mut W {
        self.variant(TSEL1_A::TRG15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN1_W {
        CEN1_W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W {
        DMAUDRIE1_W { w: self }
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W {
        DMAEN1_W { w: self }
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W {
        MAMP1_W { w: self }
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W {
        WAVE1_W { w: self }
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W {
        TEN1_W { w: self }
    }
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W {
        TSEL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
