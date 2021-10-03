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
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SMOD_A {
    #[doc = "0: SPI mode is selected"]
    SPIMODE = 0,
    #[doc = "1: I2S mode is selected"]
    I2SMODE = 1,
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
            false => I2SMOD_A::SPIMODE,
            true => I2SMOD_A::I2SMODE,
        }
    }
    #[doc = "Checks if the value of the field is `SPIMODE`"]
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        **self == I2SMOD_A::SPIMODE
    }
    #[doc = "Checks if the value of the field is `I2SMODE`"]
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        **self == I2SMOD_A::I2SMODE
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
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn spimode(self) -> &'a mut W {
        self.variant(I2SMOD_A::SPIMODE)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut W {
        self.variant(I2SMOD_A::I2SMODE)
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
#[doc = "I2S Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SE_A {
    #[doc = "0: I2S peripheral is disabled"]
    DISABLED = 0,
    #[doc = "1: I2S peripheral is enabled"]
    ENABLED = 1,
}
impl From<I2SE_A> for bool {
    #[inline(always)]
    fn from(variant: I2SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SE` reader - I2S Enable"]
pub struct I2SE_R(crate::FieldReader<bool, I2SE_A>);
impl I2SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SE_A {
        match self.bits {
            false => I2SE_A::DISABLED,
            true => I2SE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2SE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2SE_A::ENABLED
    }
}
impl core::ops::Deref for I2SE_R {
    type Target = crate::FieldReader<bool, I2SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SE` writer - I2S Enable"]
pub struct I2SE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2SE_A::DISABLED)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2SE_A::ENABLED)
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
#[doc = "I2S configuration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SCFG_A {
    #[doc = "0: Slave - transmit"]
    SLAVETX = 0,
    #[doc = "1: Slave - receive"]
    SLAVERX = 1,
    #[doc = "2: Master - transmit"]
    MASTERTX = 2,
    #[doc = "3: Master - receive"]
    MASTERRX = 3,
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
    pub fn variant(&self) -> I2SCFG_A {
        match self.bits {
            0 => I2SCFG_A::SLAVETX,
            1 => I2SCFG_A::SLAVERX,
            2 => I2SCFG_A::MASTERTX,
            3 => I2SCFG_A::MASTERRX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVETX`"]
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        **self == I2SCFG_A::SLAVETX
    }
    #[doc = "Checks if the value of the field is `SLAVERX`"]
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        **self == I2SCFG_A::SLAVERX
    }
    #[doc = "Checks if the value of the field is `MASTERTX`"]
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        **self == I2SCFG_A::MASTERTX
    }
    #[doc = "Checks if the value of the field is `MASTERRX`"]
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        **self == I2SCFG_A::MASTERRX
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
        self.bits(variant.into())
    }
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVETX)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVERX)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTERTX)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTERRX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "PCM frame synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMSYNC_A {
    #[doc = "0: Short frame synchronisation"]
    SHORT = 0,
    #[doc = "1: Long frame synchronisation"]
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
    #[doc = "Short frame synchronisation"]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNC_A::SHORT)
    }
    #[doc = "Long frame synchronisation"]
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
    #[doc = "1: MSB justified standard"]
    MSB = 1,
    #[doc = "2: LSB justified standard"]
    LSB = 2,
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
            1 => I2SSTD_A::MSB,
            2 => I2SSTD_A::LSB,
            3 => I2SSTD_A::PCM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHILIPS`"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        **self == I2SSTD_A::PHILIPS
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        **self == I2SSTD_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        **self == I2SSTD_A::LSB
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
    #[doc = "MSB justified standard"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(I2SSTD_A::MSB)
    }
    #[doc = "LSB justified standard"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(I2SSTD_A::LSB)
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
#[doc = "Steady state clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPOL_A {
    #[doc = "0: I2S clock inactive state is low level"]
    IDLELOW = 0,
    #[doc = "1: I2S clock inactive state is high level"]
    IDLEHIGH = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - Steady state clock polarity"]
pub struct CKPOL_R(crate::FieldReader<bool, CKPOL_A>);
impl CKPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::IDLELOW,
            true => CKPOL_A::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        **self == CKPOL_A::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        **self == CKPOL_A::IDLEHIGH
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<bool, CKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPOL` writer - Steady state clock polarity"]
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPOL_A::IDLELOW)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPOL_A::IDLEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Data length to be transferred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATLEN_A {
    #[doc = "0: 16-bit data length"]
    SIXTEENBIT = 0,
    #[doc = "1: 24-bit data length"]
    TWENTYFOURBIT = 1,
    #[doc = "2: 32-bit data length"]
    THIRTYTWOBIT = 2,
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
            0 => Some(DATLEN_A::SIXTEENBIT),
            1 => Some(DATLEN_A::TWENTYFOURBIT),
            2 => Some(DATLEN_A::THIRTYTWOBIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        **self == DATLEN_A::SIXTEENBIT
    }
    #[doc = "Checks if the value of the field is `TWENTYFOURBIT`"]
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        **self == DATLEN_A::TWENTYFOURBIT
    }
    #[doc = "Checks if the value of the field is `THIRTYTWOBIT`"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        **self == DATLEN_A::THIRTYTWOBIT
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
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::SIXTEENBIT)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::TWENTYFOURBIT)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::THIRTYTWOBIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Channel length (number of bits per audio channel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLEN_A {
    #[doc = "0: 16-bit wide"]
    SIXTEENBIT = 0,
    #[doc = "1: 32-bit wide"]
    THIRTYTWOBIT = 1,
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
            false => CHLEN_A::SIXTEENBIT,
            true => CHLEN_A::THIRTYTWOBIT,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        **self == CHLEN_A::SIXTEENBIT
    }
    #[doc = "Checks if the value of the field is `THIRTYTWOBIT`"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        **self == CHLEN_A::THIRTYTWOBIT
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
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::SIXTEENBIT)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::THIRTYTWOBIT)
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
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 0x03) as u8)
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
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W {
        I2SMOD_W { w: self }
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W {
        I2SE_W { w: self }
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W {
        I2SCFG_W { w: self }
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
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scfgr](index.html) module"]
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
