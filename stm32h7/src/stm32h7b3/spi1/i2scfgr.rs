#[doc = "Register `I2SCFGR` reader"]
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCFGR` writer"]
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKOE_A {
    #[doc = "0: Master clock output disabled"]
    DISABLED = 0,
    #[doc = "1: Master clock output enabled"]
    ENABLED = 1,
}
impl From<MCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub struct MCKOE_R(crate::FieldReader<bool, MCKOE_A>);
impl MCKOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCKOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKOE_A {
        match self.bits {
            false => MCKOE_A::DISABLED,
            true => MCKOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MCKOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MCKOE_A::ENABLED
    }
}
impl core::ops::Deref for MCKOE_R {
    type Target = crate::FieldReader<bool, MCKOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub struct MCKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master clock output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKOE_A::DISABLED)
    }
    #[doc = "Master clock output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Odd factor for the prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODD_A {
    #[doc = "0: Real divider value is I2SDIV*2"]
    EVEN = 0,
    #[doc = "1: Real divider value is I2SDIV*2 + 1"]
    ODD = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub struct ODD_R(crate::FieldReader<bool, ODD_A>);
impl ODD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::EVEN,
            true => ODD_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == ODD_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == ODD_A::ODD
    }
}
impl core::ops::Deref for ODD_R {
    type Target = crate::FieldReader<bool, ODD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub struct ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> ODD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Real divider value is I2SDIV*2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(ODD_A::EVEN)
    }
    #[doc = "Real divider value is I2SDIV*2 + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(ODD_A::ODD)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `I2SDIV` reader - I2S linear prescaler"]
pub struct I2SDIV_R(crate::FieldReader<u8, u8>);
impl I2SDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SDIV` writer - I2S linear prescaler"]
pub struct I2SDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Data format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATFMT_A {
    #[doc = "0: The data inside RXDR and TXDR are right aligned"]
    RIGHTALIGNED = 0,
    #[doc = "1: The data inside RXDR and TXDR are left aligned"]
    LEFTALIGNED = 1,
}
impl From<DATFMT_A> for bool {
    #[inline(always)]
    fn from(variant: DATFMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATFMT` reader - Data format"]
pub struct DATFMT_R(crate::FieldReader<bool, DATFMT_A>);
impl DATFMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATFMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATFMT_A {
        match self.bits {
            false => DATFMT_A::RIGHTALIGNED,
            true => DATFMT_A::LEFTALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHTALIGNED`"]
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        **self == DATFMT_A::RIGHTALIGNED
    }
    #[doc = "Checks if the value of the field is `LEFTALIGNED`"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        **self == DATFMT_A::LEFTALIGNED
    }
}
impl core::ops::Deref for DATFMT_R {
    type Target = crate::FieldReader<bool, DATFMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATFMT` writer - Data format"]
pub struct DATFMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATFMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATFMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The data inside RXDR and TXDR are right aligned"]
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut W {
        self.variant(DATFMT_A::RIGHTALIGNED)
    }
    #[doc = "The data inside RXDR and TXDR are left aligned"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut W {
        self.variant(DATFMT_A::LEFTALIGNED)
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
#[doc = "Fixed channel length in SLAVE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSINV_A {
    #[doc = "0: Word select inversion disabled"]
    DISABLED = 0,
    #[doc = "1: Word select inversion enabled"]
    ENABLED = 1,
}
impl From<WSINV_A> for bool {
    #[inline(always)]
    fn from(variant: WSINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSINV` reader - Fixed channel length in SLAVE"]
pub struct WSINV_R(crate::FieldReader<bool, WSINV_A>);
impl WSINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        WSINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSINV_A {
        match self.bits {
            false => WSINV_A::DISABLED,
            true => WSINV_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WSINV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WSINV_A::ENABLED
    }
}
impl core::ops::Deref for WSINV_R {
    type Target = crate::FieldReader<bool, WSINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WSINV` writer - Fixed channel length in SLAVE"]
pub struct WSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> WSINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Word select inversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WSINV_A::DISABLED)
    }
    #[doc = "Word select inversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WSINV_A::ENABLED)
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
#[doc = "Word select inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXCH_A {
    #[doc = "0: The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    NOTFIXED = 0,
    #[doc = "1: The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    FIXED = 1,
}
impl From<FIXCH_A> for bool {
    #[inline(always)]
    fn from(variant: FIXCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIXCH` reader - Word select inversion"]
pub struct FIXCH_R(crate::FieldReader<bool, FIXCH_A>);
impl FIXCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIXCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXCH_A {
        match self.bits {
            false => FIXCH_A::NOTFIXED,
            true => FIXCH_A::FIXED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFIXED`"]
    #[inline(always)]
    pub fn is_not_fixed(&self) -> bool {
        **self == FIXCH_A::NOTFIXED
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        **self == FIXCH_A::FIXED
    }
}
impl core::ops::Deref for FIXCH_R {
    type Target = crate::FieldReader<bool, FIXCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXCH` writer - Word select inversion"]
pub struct FIXCH_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIXCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    #[inline(always)]
    pub fn not_fixed(self) -> &'a mut W {
        self.variant(FIXCH_A::NOTFIXED)
    }
    #[doc = "The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(FIXCH_A::FIXED)
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
#[doc = "Serial audio clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPOL_A {
    #[doc = "0: Signals are sampled on rising and changed on falling clock edges"]
    SAMPLEONRISING = 0,
    #[doc = "1: Signals are sampled on falling and changed on rising clock edges"]
    SAMPLEONFALLING = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - Serial audio clock polarity"]
pub struct CKPOL_R(crate::FieldReader<bool, CKPOL_A>);
impl CKPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::SAMPLEONRISING,
            true => CKPOL_A::SAMPLEONFALLING,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLEONRISING`"]
    #[inline(always)]
    pub fn is_sample_on_rising(&self) -> bool {
        **self == CKPOL_A::SAMPLEONRISING
    }
    #[doc = "Checks if the value of the field is `SAMPLEONFALLING`"]
    #[inline(always)]
    pub fn is_sample_on_falling(&self) -> bool {
        **self == CKPOL_A::SAMPLEONFALLING
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<bool, CKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPOL` writer - Serial audio clock polarity"]
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Signals are sampled on rising and changed on falling clock edges"]
    #[inline(always)]
    pub fn sample_on_rising(self) -> &'a mut W {
        self.variant(CKPOL_A::SAMPLEONRISING)
    }
    #[doc = "Signals are sampled on falling and changed on rising clock edges"]
    #[inline(always)]
    pub fn sample_on_falling(self) -> &'a mut W {
        self.variant(CKPOL_A::SAMPLEONFALLING)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Channel length (number of bits per audio channel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLEN_A {
    #[doc = "0: 16 bit per channel"]
    BITS16 = 0,
    #[doc = "1: 32 bit per channel"]
    BITS32 = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub struct CHLEN_R(crate::FieldReader<bool, CHLEN_A>);
impl CHLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::BITS16,
            true => CHLEN_A::BITS32,
        }
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        **self == CHLEN_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        **self == CHLEN_A::BITS32
    }
}
impl core::ops::Deref for CHLEN_R {
    type Target = crate::FieldReader<bool, CHLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub struct CHLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "16 bit per channel"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(CHLEN_A::BITS16)
    }
    #[doc = "32 bit per channel"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(CHLEN_A::BITS32)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Data length to be transferred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATLEN_A {
    #[doc = "0: 16 bit data length"]
    BITS16 = 0,
    #[doc = "1: 24 bit data length"]
    BITS24 = 1,
    #[doc = "2: 32 bit data length"]
    BITS32 = 2,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATLEN` reader - Data length to be transferred"]
pub struct DATLEN_R(crate::FieldReader<u8, DATLEN_A>);
impl DATLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATLEN_A> {
        match self.bits {
            0 => Some(DATLEN_A::BITS16),
            1 => Some(DATLEN_A::BITS24),
            2 => Some(DATLEN_A::BITS32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        **self == DATLEN_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS24`"]
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        **self == DATLEN_A::BITS24
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        **self == DATLEN_A::BITS32
    }
}
impl core::ops::Deref for DATLEN_R {
    type Target = crate::FieldReader<u8, DATLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLEN` writer - Data length to be transferred"]
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16 bit data length"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(DATLEN_A::BITS16)
    }
    #[doc = "24 bit data length"]
    #[inline(always)]
    pub fn bits24(self) -> &'a mut W {
        self.variant(DATLEN_A::BITS24)
    }
    #[doc = "32 bit data length"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(DATLEN_A::BITS32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "PCM frame synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMSYNC_A {
    #[doc = "0: Short PCM frame synchronization"]
    SHORT = 0,
    #[doc = "1: Long PCM frame synchronization"]
    LONG = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub struct PCMSYNC_R(crate::FieldReader<bool, PCMSYNC_A>);
impl PCMSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCMSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::SHORT,
            true => PCMSYNC_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        **self == PCMSYNC_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        **self == PCMSYNC_A::LONG
    }
}
impl core::ops::Deref for PCMSYNC_R {
    type Target = crate::FieldReader<bool, PCMSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub struct PCMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCMSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Short PCM frame synchronization"]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNC_A::SHORT)
    }
    #[doc = "Long PCM frame synchronization"]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSYNC_A::LONG)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "I2S standard selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SSTD_A {
    #[doc = "0: I2S Philips standard"]
    PHILIPS = 0,
    #[doc = "1: MSB/left justified standard"]
    LEFTALIGNED = 1,
    #[doc = "2: LSB/right justified standard"]
    RIGHTALIGNED = 2,
    #[doc = "3: PCM standard"]
    PCM = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub struct I2SSTD_R(crate::FieldReader<u8, I2SSTD_A>);
impl I2SSTD_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SSTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::PHILIPS,
            1 => I2SSTD_A::LEFTALIGNED,
            2 => I2SSTD_A::RIGHTALIGNED,
            3 => I2SSTD_A::PCM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHILIPS`"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        **self == I2SSTD_A::PHILIPS
    }
    #[doc = "Checks if the value of the field is `LEFTALIGNED`"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        **self == I2SSTD_A::LEFTALIGNED
    }
    #[doc = "Checks if the value of the field is `RIGHTALIGNED`"]
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        **self == I2SSTD_A::RIGHTALIGNED
    }
    #[doc = "Checks if the value of the field is `PCM`"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        **self == I2SSTD_A::PCM
    }
}
impl core::ops::Deref for I2SSTD_R {
    type Target = crate::FieldReader<u8, I2SSTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub struct I2SSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SSTD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn philips(self) -> &'a mut W {
        self.variant(I2SSTD_A::PHILIPS)
    }
    #[doc = "MSB/left justified standard"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut W {
        self.variant(I2SSTD_A::LEFTALIGNED)
    }
    #[doc = "LSB/right justified standard"]
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut W {
        self.variant(I2SSTD_A::RIGHTALIGNED)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTD_A::PCM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "I2S configuration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SCFG_A {
    #[doc = "0: Slave, transmit"]
    SLAVETRANSMIT = 0,
    #[doc = "1: Slave, recteive"]
    SLAVERECEIVE = 1,
    #[doc = "2: Master, transmit"]
    MASTERTRANSMIT = 2,
    #[doc = "3: Master, receive"]
    MASTERRECEIVE = 3,
    #[doc = "4: Slave, full duplex"]
    SLAVEFULLDUPLEX = 4,
    #[doc = "5: Master, full duplex"]
    MASTERFULLDUPLEX = 5,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2SCFG` reader - I2S configuration mode"]
pub struct I2SCFG_R(crate::FieldReader<u8, I2SCFG_A>);
impl I2SCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2SCFG_A> {
        match self.bits {
            0 => Some(I2SCFG_A::SLAVETRANSMIT),
            1 => Some(I2SCFG_A::SLAVERECEIVE),
            2 => Some(I2SCFG_A::MASTERTRANSMIT),
            3 => Some(I2SCFG_A::MASTERRECEIVE),
            4 => Some(I2SCFG_A::SLAVEFULLDUPLEX),
            5 => Some(I2SCFG_A::MASTERFULLDUPLEX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVETRANSMIT`"]
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        **self == I2SCFG_A::SLAVETRANSMIT
    }
    #[doc = "Checks if the value of the field is `SLAVERECEIVE`"]
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        **self == I2SCFG_A::SLAVERECEIVE
    }
    #[doc = "Checks if the value of the field is `MASTERTRANSMIT`"]
    #[inline(always)]
    pub fn is_master_transmit(&self) -> bool {
        **self == I2SCFG_A::MASTERTRANSMIT
    }
    #[doc = "Checks if the value of the field is `MASTERRECEIVE`"]
    #[inline(always)]
    pub fn is_master_receive(&self) -> bool {
        **self == I2SCFG_A::MASTERRECEIVE
    }
    #[doc = "Checks if the value of the field is `SLAVEFULLDUPLEX`"]
    #[inline(always)]
    pub fn is_slave_full_duplex(&self) -> bool {
        **self == I2SCFG_A::SLAVEFULLDUPLEX
    }
    #[doc = "Checks if the value of the field is `MASTERFULLDUPLEX`"]
    #[inline(always)]
    pub fn is_master_full_duplex(&self) -> bool {
        **self == I2SCFG_A::MASTERFULLDUPLEX
    }
}
impl core::ops::Deref for I2SCFG_R {
    type Target = crate::FieldReader<u8, I2SCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SCFG` writer - I2S configuration mode"]
pub struct I2SCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slave, transmit"]
    #[inline(always)]
    pub fn slave_transmit(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVETRANSMIT)
    }
    #[doc = "Slave, recteive"]
    #[inline(always)]
    pub fn slave_receive(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVERECEIVE)
    }
    #[doc = "Master, transmit"]
    #[inline(always)]
    pub fn master_transmit(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTERTRANSMIT)
    }
    #[doc = "Master, receive"]
    #[inline(always)]
    pub fn master_receive(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTERRECEIVE)
    }
    #[doc = "Slave, full duplex"]
    #[inline(always)]
    pub fn slave_full_duplex(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVEFULLDUPLEX)
    }
    #[doc = "Master, full duplex"]
    #[inline(always)]
    pub fn master_full_duplex(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTERFULLDUPLEX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SMOD_A {
    #[doc = "0: SPI mode selected"]
    SPI = 0,
    #[doc = "1: I2S/PCM mode selected"]
    I2S = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub struct I2SMOD_R(crate::FieldReader<bool, I2SMOD_A>);
impl I2SMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::SPI,
            true => I2SMOD_A::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        **self == I2SMOD_A::SPI
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        **self == I2SMOD_A::I2S
    }
}
impl core::ops::Deref for I2SMOD_R {
    type Target = crate::FieldReader<bool, I2SMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub struct I2SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI mode selected"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(I2SMOD_A::SPI)
    }
    #[doc = "I2S/PCM mode selected"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(I2SMOD_A::I2S)
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
impl R {
    #[doc = "Bit 25 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 14 - Data format"]
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fixed channel length in SLAVE"]
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Word select inversion"]
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Serial audio clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:3 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W {
        MCKOE_W { w: self }
    }
    #[doc = "Bit 24 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W {
        ODD_W { w: self }
    }
    #[doc = "Bits 16:23 - I2S linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W {
        I2SDIV_W { w: self }
    }
    #[doc = "Bit 14 - Data format"]
    #[inline(always)]
    pub fn datfmt(&mut self) -> DATFMT_W {
        DATFMT_W { w: self }
    }
    #[doc = "Bit 13 - Fixed channel length in SLAVE"]
    #[inline(always)]
    pub fn wsinv(&mut self) -> WSINV_W {
        WSINV_W { w: self }
    }
    #[doc = "Bit 12 - Word select inversion"]
    #[inline(always)]
    pub fn fixch(&mut self) -> FIXCH_W {
        FIXCH_W { w: self }
    }
    #[doc = "Bit 11 - Serial audio clock polarity"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    #[doc = "Bit 10 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W {
        PCMSYNC_W { w: self }
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W { w: self }
    }
    #[doc = "Bits 1:3 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W {
        I2SCFG_W { w: self }
    }
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W {
        I2SMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scfgr](index.html) module"]
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2scfgr::R](R) reader structure"]
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2scfgr::W](W) writer structure"]
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
