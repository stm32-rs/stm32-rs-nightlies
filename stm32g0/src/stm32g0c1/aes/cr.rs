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
#[doc = "AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Disable"]
    B_0X0 = 0,
    #[doc = "1: Enable"]
    B_0X1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0X0,
            true => EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EN_A::B_0X1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EN_A::B_0X0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EN_A::B_0X1)
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
#[doc = "Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATATYPE_A {
    #[doc = "0: None"]
    B_0X0 = 0,
    #[doc = "1: Half-word (16-bit)"]
    B_0X1 = 1,
    #[doc = "2: Byte (8-bit)"]
    B_0X2 = 2,
    #[doc = "3: Bit"]
    B_0X3 = 3,
}
impl From<DATATYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATATYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATATYPE` reader - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct DATATYPE_R(crate::FieldReader<u8, DATATYPE_A>);
impl DATATYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATATYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATATYPE_A {
        match self.bits {
            0 => DATATYPE_A::B_0X0,
            1 => DATATYPE_A::B_0X1,
            2 => DATATYPE_A::B_0X2,
            3 => DATATYPE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DATATYPE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DATATYPE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == DATATYPE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == DATATYPE_A::B_0X3
    }
}
impl core::ops::Deref for DATATYPE_R {
    type Target = crate::FieldReader<u8, DATATYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATATYPE` writer - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct DATATYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATATYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DATATYPE_A::B_0X0)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DATATYPE_A::B_0X1)
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DATATYPE_A::B_0X2)
    }
    #[doc = "Bit"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DATATYPE_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Mode 1: encryption"]
    B_0X0 = 0,
    #[doc = "1: Mode 2: key derivation (or key preparation for ECB/CBC decryption)"]
    B_0X1 = 1,
    #[doc = "2: Mode 3: decryption"]
    B_0X2 = 2,
    #[doc = "3: Mode 4: key derivation then single decryption"]
    B_0X3 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::B_0X0,
            1 => MODE_A::B_0X1,
            2 => MODE_A::B_0X2,
            3 => MODE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MODE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MODE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == MODE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == MODE_A::B_0X3
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
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
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MODE_A::B_0X0)
    }
    #[doc = "Mode 2: key derivation (or key preparation for ECB/CBC decryption)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MODE_A::B_0X1)
    }
    #[doc = "Mode 3: decryption"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MODE_A::B_0X2)
    }
    #[doc = "Mode 4: key derivation then single decryption"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MODE_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `CHMOD1` reader - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct CHMOD1_R(crate::FieldReader<u8, u8>);
impl CHMOD1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMOD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMOD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMOD1` writer - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct CHMOD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMOD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCFC_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear CCF"]
    B_0X1 = 1,
}
impl From<CCFC_A> for bool {
    #[inline(always)]
    fn from(variant: CCFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCFC` reader - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
pub struct CCFC_R(crate::FieldReader<bool, CCFC_A>);
impl CCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCFC_A {
        match self.bits {
            false => CCFC_A::B_0X0,
            true => CCFC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CCFC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CCFC_A::B_0X1
    }
}
impl core::ops::Deref for CCFC_R {
    type Target = crate::FieldReader<bool, CCFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCFC` writer - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
pub struct CCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCFC_A::B_0X0)
    }
    #[doc = "Clear CCF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCFC_A::B_0X1)
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
#[doc = "Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRC_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Clear RDERR and WRERR flags"]
    B_0X1 = 1,
}
impl From<ERRC_A> for bool {
    #[inline(always)]
    fn from(variant: ERRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRC` reader - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
pub struct ERRC_R(crate::FieldReader<bool, ERRC_A>);
impl ERRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRC_A {
        match self.bits {
            false => ERRC_A::B_0X0,
            true => ERRC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ERRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ERRC_A::B_0X1
    }
}
impl core::ops::Deref for ERRC_R {
    type Target = crate::FieldReader<bool, ERRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRC` writer - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
pub struct ERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ERRC_A::B_0X0)
    }
    #[doc = "Clear RDERR and WRERR flags"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ERRC_A::B_0X1)
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
#[doc = "CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCFIE_A {
    #[doc = "0: Disable (mask)"]
    B_0X0 = 0,
    #[doc = "1: Enable"]
    B_0X1 = 1,
}
impl From<CCFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCFIE` reader - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
pub struct CCFIE_R(crate::FieldReader<bool, CCFIE_A>);
impl CCFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCFIE_A {
        match self.bits {
            false => CCFIE_A::B_0X0,
            true => CCFIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CCFIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CCFIE_A::B_0X1
    }
}
impl core::ops::Deref for CCFIE_R {
    type Target = crate::FieldReader<bool, CCFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCFIE` writer - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
pub struct CCFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable (mask)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCFIE_A::B_0X0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCFIE_A::B_0X1)
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
#[doc = "Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Disable (mask)"]
    B_0X0 = 0,
    #[doc = "1: Enable"]
    B_0X1 = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
pub struct ERRIE_R(crate::FieldReader<bool, ERRIE_A>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::B_0X0,
            true => ERRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ERRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ERRIE_A::B_0X1
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, ERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable (mask)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ERRIE_A::B_0X0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ERRIE_A::B_0X1)
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
#[doc = "DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINEN_A {
    #[doc = "0: Disable"]
    B_0X0 = 0,
    #[doc = "1: Enable"]
    B_0X1 = 1,
}
impl From<DMAINEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINEN` reader - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub struct DMAINEN_R(crate::FieldReader<bool, DMAINEN_A>);
impl DMAINEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAINEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINEN_A {
        match self.bits {
            false => DMAINEN_A::B_0X0,
            true => DMAINEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DMAINEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DMAINEN_A::B_0X1
    }
}
impl core::ops::Deref for DMAINEN_R {
    type Target = crate::FieldReader<bool, DMAINEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAINEN` writer - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub struct DMAINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAINEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAINEN_A::B_0X0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAINEN_A::B_0X1)
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
#[doc = "DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAOUTEN_A {
    #[doc = "0: Disable"]
    B_0X0 = 0,
    #[doc = "1: Enable"]
    B_0X1 = 1,
}
impl From<DMAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAOUTEN` reader - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub struct DMAOUTEN_R(crate::FieldReader<bool, DMAOUTEN_A>);
impl DMAOUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAOUTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAOUTEN_A {
        match self.bits {
            false => DMAOUTEN_A::B_0X0,
            true => DMAOUTEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DMAOUTEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DMAOUTEN_A::B_0X1
    }
}
impl core::ops::Deref for DMAOUTEN_R {
    type Target = crate::FieldReader<bool, DMAOUTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAOUTEN` writer - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
pub struct DMAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAOUTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAOUTEN_A::B_0X0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAOUTEN_A::B_0X1)
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
#[doc = "GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GCMPH_A {
    #[doc = "0: Init phase"]
    B_0X0 = 0,
    #[doc = "1: Header phase"]
    B_0X1 = 1,
    #[doc = "2: Payload phase"]
    B_0X2 = 2,
    #[doc = "3: Final phase"]
    B_0X3 = 3,
}
impl From<GCMPH_A> for u8 {
    #[inline(always)]
    fn from(variant: GCMPH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GCMPH` reader - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
pub struct GCMPH_R(crate::FieldReader<u8, GCMPH_A>);
impl GCMPH_R {
    pub(crate) fn new(bits: u8) -> Self {
        GCMPH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCMPH_A {
        match self.bits {
            0 => GCMPH_A::B_0X0,
            1 => GCMPH_A::B_0X1,
            2 => GCMPH_A::B_0X2,
            3 => GCMPH_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == GCMPH_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == GCMPH_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == GCMPH_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == GCMPH_A::B_0X3
    }
}
impl core::ops::Deref for GCMPH_R {
    type Target = crate::FieldReader<u8, GCMPH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCMPH` writer - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
pub struct GCMPH_W<'a> {
    w: &'a mut W,
}
impl<'a> GCMPH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCMPH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Init phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GCMPH_A::B_0X0)
    }
    #[doc = "Header phase"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GCMPH_A::B_0X1)
    }
    #[doc = "Payload phase"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(GCMPH_A::B_0X2)
    }
    #[doc = "Final phase"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(GCMPH_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `CHMOD2` reader - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct CHMOD2_R(crate::FieldReader<bool, bool>);
impl CHMOD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHMOD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMOD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMOD2` writer - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct CHMOD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMOD2_W<'a> {
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
#[doc = "Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSIZE_A {
    #[doc = "0: 128"]
    B_0X0 = 0,
    #[doc = "1: 256"]
    B_0X1 = 1,
}
impl From<KEYSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSIZE` reader - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct KEYSIZE_R(crate::FieldReader<bool, KEYSIZE_A>);
impl KEYSIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEYSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSIZE_A {
        match self.bits {
            false => KEYSIZE_A::B_0X0,
            true => KEYSIZE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == KEYSIZE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == KEYSIZE_A::B_0X1
    }
}
impl core::ops::Deref for KEYSIZE_R {
    type Target = crate::FieldReader<bool, KEYSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYSIZE` writer - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSIZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(KEYSIZE_A::B_0X0)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(KEYSIZE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NPBLB_A {
    #[doc = "0: All bytes are valid (no padding)"]
    B_0X0 = 0,
    #[doc = "1: Padding for one least-significant byte of last block"]
    B_0X1 = 1,
    #[doc = "15: Padding for 15 least-significant bytes of last block"]
    B_0XF = 15,
}
impl From<NPBLB_A> for u8 {
    #[inline(always)]
    fn from(variant: NPBLB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NPBLB` reader - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
pub struct NPBLB_R(crate::FieldReader<u8, NPBLB_A>);
impl NPBLB_R {
    pub(crate) fn new(bits: u8) -> Self {
        NPBLB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NPBLB_A> {
        match self.bits {
            0 => Some(NPBLB_A::B_0X0),
            1 => Some(NPBLB_A::B_0X1),
            15 => Some(NPBLB_A::B_0XF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == NPBLB_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == NPBLB_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        **self == NPBLB_A::B_0XF
    }
}
impl core::ops::Deref for NPBLB_R {
    type Target = crate::FieldReader<u8, NPBLB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPBLB` writer - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
pub struct NPBLB_W<'a> {
    w: &'a mut W,
}
impl<'a> NPBLB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NPBLB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All bytes are valid (no padding)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(NPBLB_A::B_0X0)
    }
    #[doc = "Padding for one least-significant byte of last block"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(NPBLB_A::B_0X1)
    }
    #[doc = "Padding for 15 least-significant bytes of last block"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(NPBLB_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod1(&self) -> CHMOD1_R {
        CHMOD1_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod2(&self) -> CHMOD2_R {
        CHMOD2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W {
        DATATYPE_W { w: self }
    }
    #[doc = "Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 5:6 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod1(&mut self) -> CHMOD1_W {
        CHMOD1_W { w: self }
    }
    #[doc = "Bit 7 - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W {
        CCFC_W { w: self }
    }
    #[doc = "Bit 8 - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero."]
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W {
        ERRC_W { w: self }
    }
    #[doc = "Bit 9 - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:"]
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W {
        CCFIE_W { w: self }
    }
    #[doc = "Bit 10 - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W {
        DMAINEN_W { w: self }
    }
    #[doc = "Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\\[1:0\\]
bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended."]
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W {
        DMAOUTEN_W { w: self }
    }
    #[doc = "Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield)."]
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W {
        GCMPH_W { w: self }
    }
    #[doc = "Bit 16 - Chaining mode selection, bit \\[2\\]
Refer to the bits \\[5:6\\]
of the register for the description of the CHMOD\\[2:0\\]
bitfield CHMOD\\[1:0\\]: Chaining mode selection, bits \\[1:0\\]
This bitfield, together with the bit CHMOD\\[2\\]
forming CHMOD\\[2:0\\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn chmod2(&mut self) -> CHMOD2_W {
        CHMOD2_W { w: self }
    }
    #[doc = "Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access."]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ..."]
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W {
        NPBLB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
