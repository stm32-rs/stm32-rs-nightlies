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
#[doc = "Enable DMA management of data output phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAOUTEN_A {
    #[doc = "0: Disable DMA Output"]
    DISABLED = 0,
    #[doc = "1: Enabled DMA Output"]
    ENABLED = 1,
}
impl From<DMAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAOUTEN` reader - Enable DMA management of data output phase"]
pub struct DMAOUTEN_R(crate::FieldReader<bool, DMAOUTEN_A>);
impl DMAOUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAOUTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAOUTEN_A {
        match self.bits {
            false => DMAOUTEN_A::DISABLED,
            true => DMAOUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAOUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAOUTEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAOUTEN_R {
    type Target = crate::FieldReader<bool, DMAOUTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAOUTEN` writer - Enable DMA management of data output phase"]
pub struct DMAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAOUTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable DMA Output"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAOUTEN_A::DISABLED)
    }
    #[doc = "Enabled DMA Output"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAOUTEN_A::ENABLED)
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
#[doc = "Enable DMA management of data input phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINEN_A {
    #[doc = "0: Disable DMA Input"]
    DISABLED = 0,
    #[doc = "1: Enable DMA Input"]
    ENABLED = 1,
}
impl From<DMAINEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINEN` reader - Enable DMA management of data input phase"]
pub struct DMAINEN_R(crate::FieldReader<bool, DMAINEN_A>);
impl DMAINEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAINEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINEN_A {
        match self.bits {
            false => DMAINEN_A::DISABLED,
            true => DMAINEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAINEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAINEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAINEN_R {
    type Target = crate::FieldReader<bool, DMAINEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAINEN` writer - Enable DMA management of data input phase"]
pub struct DMAINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAINEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable DMA Input"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAINEN_A::DISABLED)
    }
    #[doc = "Enable DMA Input"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAINEN_A::ENABLED)
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
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Disable (mask) error interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable error interrupt"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub struct ERRIE_R(crate::FieldReader<bool, ERRIE_A>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERRIE_A::ENABLED
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, ERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable (mask) error interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Enable error interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
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
#[doc = "CCF flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCFIE_A {
    #[doc = "0: Disable (mask) CCF interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable CCF interrupt"]
    ENABLED = 1,
}
impl From<CCFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCFIE` reader - CCF flag interrupt enable"]
pub struct CCFIE_R(crate::FieldReader<bool, CCFIE_A>);
impl CCFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCFIE_A {
        match self.bits {
            false => CCFIE_A::DISABLED,
            true => CCFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CCFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CCFIE_A::ENABLED
    }
}
impl core::ops::Deref for CCFIE_R {
    type Target = crate::FieldReader<bool, CCFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCFIE` writer - CCF flag interrupt enable"]
pub struct CCFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable (mask) CCF interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCFIE_A::DISABLED)
    }
    #[doc = "Enable CCF interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCFIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Error clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRC_A {
    #[doc = "1: Clear RDERR and WRERR flags"]
    CLEAR = 1,
}
impl From<ERRC_A> for bool {
    #[inline(always)]
    fn from(variant: ERRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRC` reader - Error clear"]
pub struct ERRC_R(crate::FieldReader<bool, ERRC_A>);
impl ERRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERRC_A> {
        match self.bits {
            true => Some(ERRC_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == ERRC_A::CLEAR
    }
}
impl core::ops::Deref for ERRC_R {
    type Target = crate::FieldReader<bool, ERRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRC` writer - Error clear"]
pub struct ERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear RDERR and WRERR flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Computation Complete Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCFC_A {
    #[doc = "1: Clear computation complete flag"]
    CLEAR = 1,
}
impl From<CCFC_A> for bool {
    #[inline(always)]
    fn from(variant: CCFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCFC` reader - Computation Complete Flag Clear"]
pub struct CCFC_R(crate::FieldReader<bool, CCFC_A>);
impl CCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCFC_A> {
        match self.bits {
            true => Some(CCFC_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CCFC_A::CLEAR
    }
}
impl core::ops::Deref for CCFC_R {
    type Target = crate::FieldReader<bool, CCFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCFC` writer - Computation Complete Flag Clear"]
pub struct CCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear computation complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCFC_A::CLEAR)
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
#[doc = "AES chaining mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHMOD_A {
    #[doc = "0: Electronic codebook (ECB)"]
    ECB = 0,
    #[doc = "1: Cipher-Block Chaining (CBC)"]
    CBC = 1,
    #[doc = "2: Counter Mode (CTR)"]
    CTR = 2,
}
impl From<CHMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHMOD` reader - AES chaining mode"]
pub struct CHMOD_R(crate::FieldReader<u8, CHMOD_A>);
impl CHMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHMOD_A> {
        match self.bits {
            0 => Some(CHMOD_A::ECB),
            1 => Some(CHMOD_A::CBC),
            2 => Some(CHMOD_A::CTR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        **self == CHMOD_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        **self == CHMOD_A::CBC
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        **self == CHMOD_A::CTR
    }
}
impl core::ops::Deref for CHMOD_R {
    type Target = crate::FieldReader<u8, CHMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMOD` writer - AES chaining mode"]
pub struct CHMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Electronic codebook (ECB)"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(CHMOD_A::ECB)
    }
    #[doc = "Cipher-Block Chaining (CBC)"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(CHMOD_A::CBC)
    }
    #[doc = "Counter Mode (CTR)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(CHMOD_A::CTR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "AES operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Mode 1: encryption"]
    MODE1 = 0,
    #[doc = "1: Mode 2: key derivation (or key preparation for ECB/CBC decryption)"]
    MODE2 = 1,
    #[doc = "2: Mode 3: decryption"]
    MODE3 = 2,
    #[doc = "3: Mode 4: key derivation then single decryption"]
    MODE4 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - AES operating mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::MODE1,
            1 => MODE_A::MODE2,
            2 => MODE_A::MODE3,
            3 => MODE_A::MODE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        **self == MODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        **self == MODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        **self == MODE_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        **self == MODE_A::MODE4
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - AES operating mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Mode 1: encryption"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(MODE_A::MODE1)
    }
    #[doc = "Mode 2: key derivation (or key preparation for ECB/CBC decryption)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(MODE_A::MODE2)
    }
    #[doc = "Mode 3: decryption"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(MODE_A::MODE3)
    }
    #[doc = "Mode 4: key derivation then single decryption"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(MODE_A::MODE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Data type selection (for data in and data out to/from the cryptographic block)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATATYPE_A {
    #[doc = "0: Word"]
    NONE = 0,
    #[doc = "1: Half-word (16-bit)"]
    HALFWORD = 1,
    #[doc = "2: Byte (8-bit)"]
    BYTE = 2,
    #[doc = "3: Bit"]
    BIT = 3,
}
impl From<DATATYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATATYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)"]
pub struct DATATYPE_R(crate::FieldReader<u8, DATATYPE_A>);
impl DATATYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATATYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATATYPE_A {
        match self.bits {
            0 => DATATYPE_A::NONE,
            1 => DATATYPE_A::HALFWORD,
            2 => DATATYPE_A::BYTE,
            3 => DATATYPE_A::BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == DATATYPE_A::NONE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        **self == DATATYPE_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == DATATYPE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `BIT`"]
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        **self == DATATYPE_A::BIT
    }
}
impl core::ops::Deref for DATATYPE_R {
    type Target = crate::FieldReader<u8, DATATYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)"]
pub struct DATATYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATATYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Word"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DATATYPE_A::NONE)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DATATYPE_A::HALFWORD)
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DATATYPE_A::BYTE)
    }
    #[doc = "Bit"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(DATATYPE_A::BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "AES enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Disable AES"]
    DISABLED = 0,
    #[doc = "1: Enable AES"]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - AES enable"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN_A::ENABLED
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - AES enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable AES"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Enable AES"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
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
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W {
        DMAOUTEN_W { w: self }
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W {
        DMAINEN_W { w: self }
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W {
        CCFIE_W { w: self }
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W {
        ERRC_W { w: self }
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W {
        CCFC_W { w: self }
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W {
        CHMOD_W { w: self }
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W {
        DATATYPE_W { w: self }
    }
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
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
