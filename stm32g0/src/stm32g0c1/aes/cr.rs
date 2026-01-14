///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Disable AES
    Disabled = 0,
    ///1: Enable AES
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase.
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Disable AES
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Enable AES
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable AES
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Enable AES
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATATYPE {
    ///0: Word
    None = 0,
    ///1: Half-word (16-bit)
    HalfWord = 1,
    ///2: Byte (8-bit)
    Byte = 2,
    ///3: Bit
    Bit = 3,
}
impl From<DATATYPE> for u8 {
    #[inline(always)]
    fn from(variant: DATATYPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATATYPE {
    type Ux = u8;
}
impl crate::IsEnum for DATATYPE {}
///Field `DATATYPE` reader - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type DATATYPE_R = crate::FieldReader<DATATYPE>;
impl DATATYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATATYPE {
        match self.bits {
            0 => DATATYPE::None,
            1 => DATATYPE::HalfWord,
            2 => DATATYPE::Byte,
            3 => DATATYPE::Bit,
            _ => unreachable!(),
        }
    }
    ///Word
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DATATYPE::None
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DATATYPE::HalfWord
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DATATYPE::Byte
    }
    ///Bit
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == DATATYPE::Bit
    }
}
///Field `DATATYPE` writer - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATATYPE, crate::Safe>;
impl<'a, REG> DATATYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Word
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::None)
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::HalfWord)
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::Byte)
    }
    ///Bit
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::Bit)
    }
}
/**AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Mode 1: encryption
    Mode1 = 0,
    ///1: Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    Mode2 = 1,
    ///2: Mode 3: decryption
    Mode3 = 2,
    ///3: Mode 4: key derivation then single decryption
    Mode4 = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE` reader - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4.
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Mode1,
            1 => MODE::Mode2,
            2 => MODE::Mode3,
            3 => MODE::Mode4,
            _ => unreachable!(),
        }
    }
    ///Mode 1: encryption
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == MODE::Mode1
    }
    ///Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == MODE::Mode2
    }
    ///Mode 3: decryption
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == MODE::Mode3
    }
    ///Mode 4: key derivation then single decryption
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == MODE::Mode4
    }
}
///Field `MODE` writer - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4.
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Mode 1: encryption
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode1)
    }
    ///Mode 2: key derivation (or key preparation for ECB/CBC decryption)
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode2)
    }
    ///Mode 3: decryption
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode3)
    }
    ///Mode 4: key derivation then single decryption
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode4)
    }
}
/**Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMOD {
    ///0: Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    Ecb = 0,
    ///1: Cipher-block chaining (CBC)
    Cbc = 1,
    ///2: Counter mode (CTR)
    Ctr = 2,
    ///3: Galois counter mode (GCM) and Galois message authentication code (GMAC)
    Gcm = 3,
}
impl From<CHMOD> for u8 {
    #[inline(always)]
    fn from(variant: CHMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHMOD {
    type Ux = u8;
}
impl crate::IsEnum for CHMOD {}
///Field `CHMOD` reader - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD_R = crate::FieldReader<CHMOD>;
impl CHMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHMOD {
        match self.bits {
            0 => CHMOD::Ecb,
            1 => CHMOD::Cbc,
            2 => CHMOD::Ctr,
            3 => CHMOD::Gcm,
            _ => unreachable!(),
        }
    }
    ///Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CHMOD::Ecb
    }
    ///Cipher-block chaining (CBC)
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CHMOD::Cbc
    }
    ///Counter mode (CTR)
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == CHMOD::Ctr
    }
    ///Galois counter mode (GCM) and Galois message authentication code (GMAC)
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == CHMOD::Gcm
    }
}
///Field `CHMOD` writer - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CHMOD, crate::Safe>;
impl<'a, REG> CHMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ecb)
    }
    ///Cipher-block chaining (CBC)
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Cbc)
    }
    ///Counter mode (CTR)
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ctr)
    }
    ///Galois counter mode (GCM) and Galois message authentication code (GMAC)
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Gcm)
    }
}
/**Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFCW {
    ///1: Clear computation complete flag
    Clear = 1,
}
impl From<CCFCW> for bool {
    #[inline(always)]
    fn from(variant: CCFCW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCFC` reader - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero.
pub type CCFC_R = crate::BitReader<CCFCW>;
impl CCFC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCFCW> {
        match self.bits {
            true => Some(CCFCW::Clear),
            _ => None,
        }
    }
    ///Clear computation complete flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCFCW::Clear
    }
}
///Field `CCFC` writer - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero.
pub type CCFC_W<'a, REG> = crate::BitWriter<'a, REG, CCFCW>;
impl<'a, REG> CCFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear computation complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCFCW::Clear)
    }
}
/**Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRCW {
    ///1: Clear RDERR and WRERR flags
    Clear = 1,
}
impl From<ERRCW> for bool {
    #[inline(always)]
    fn from(variant: ERRCW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRC` reader - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero.
pub type ERRC_R = crate::BitReader<ERRCW>;
impl ERRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERRCW> {
        match self.bits {
            true => Some(ERRCW::Clear),
            _ => None,
        }
    }
    ///Clear RDERR and WRERR flags
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ERRCW::Clear
    }
}
///Field `ERRC` writer - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero.
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG, ERRCW>;
impl<'a, REG> ERRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear RDERR and WRERR flags
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRCW::Clear)
    }
}
/**CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFIE {
    ///0: Disable (mask) CCF interrupt
    Disabled = 0,
    ///1: Enable CCF interrupt
    Enabled = 1,
}
impl From<CCFIE> for bool {
    #[inline(always)]
    fn from(variant: CCFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCFIE` reader - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:
pub type CCFIE_R = crate::BitReader<CCFIE>;
impl CCFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCFIE {
        match self.bits {
            false => CCFIE::Disabled,
            true => CCFIE::Enabled,
        }
    }
    ///Disable (mask) CCF interrupt
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCFIE::Disabled
    }
    ///Enable CCF interrupt
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCFIE::Enabled
    }
}
///Field `CCFIE` writer - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG, CCFIE>;
impl<'a, REG> CCFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable (mask) CCF interrupt
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCFIE::Disabled)
    }
    ///Enable CCF interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCFIE::Enabled)
    }
}
/**Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: Disable (mask) error interrupt
    Disabled = 0,
    ///1: Enable error interrupt
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    ///Disable (mask) error interrupt
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    ///Enable error interrupt
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
///Field `ERRIE` writer - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable (mask) error interrupt
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    ///Enable error interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
/**DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAINEN {
    ///0: Disable DMA Input
    Disabled = 0,
    ///1: Enable DMA Input
    Enabled = 1,
}
impl From<DMAINEN> for bool {
    #[inline(always)]
    fn from(variant: DMAINEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAINEN` reader - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
pub type DMAINEN_R = crate::BitReader<DMAINEN>;
impl DMAINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAINEN {
        match self.bits {
            false => DMAINEN::Disabled,
            true => DMAINEN::Enabled,
        }
    }
    ///Disable DMA Input
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAINEN::Disabled
    }
    ///Enable DMA Input
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAINEN::Enabled
    }
}
///Field `DMAINEN` writer - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAINEN>;
impl<'a, REG> DMAINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DMA Input
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINEN::Disabled)
    }
    ///Enable DMA Input
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINEN::Enabled)
    }
}
/**DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAOUTEN {
    ///0: Disable DMA Output
    Disabled = 0,
    ///1: Enabled DMA Output
    Enabled = 1,
}
impl From<DMAOUTEN> for bool {
    #[inline(always)]
    fn from(variant: DMAOUTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAOUTEN` reader - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
pub type DMAOUTEN_R = crate::BitReader<DMAOUTEN>;
impl DMAOUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAOUTEN {
        match self.bits {
            false => DMAOUTEN::Disabled,
            true => DMAOUTEN::Enabled,
        }
    }
    ///Disable DMA Output
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAOUTEN::Disabled
    }
    ///Enabled DMA Output
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAOUTEN::Enabled
    }
}
///Field `DMAOUTEN` writer - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAOUTEN>;
impl<'a, REG> DMAOUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DMA Output
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAOUTEN::Disabled)
    }
    ///Enabled DMA Output
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAOUTEN::Enabled)
    }
}
/**GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GCMPH {
    ///0: Init phase
    Init = 0,
    ///1: Header phase
    Header = 1,
    ///2: Payload phase
    Payload = 2,
    ///3: Final Phase
    Final = 3,
}
impl From<GCMPH> for u8 {
    #[inline(always)]
    fn from(variant: GCMPH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GCMPH {
    type Ux = u8;
}
impl crate::IsEnum for GCMPH {}
///Field `GCMPH` reader - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
pub type GCMPH_R = crate::FieldReader<GCMPH>;
impl GCMPH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCMPH {
        match self.bits {
            0 => GCMPH::Init,
            1 => GCMPH::Header,
            2 => GCMPH::Payload,
            3 => GCMPH::Final,
            _ => unreachable!(),
        }
    }
    ///Init phase
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        *self == GCMPH::Init
    }
    ///Header phase
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == GCMPH::Header
    }
    ///Payload phase
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        *self == GCMPH::Payload
    }
    ///Final Phase
    #[inline(always)]
    pub fn is_final(&self) -> bool {
        *self == GCMPH::Final
    }
}
///Field `GCMPH` writer - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GCMPH, crate::Safe>;
impl<'a, REG> GCMPH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Init phase
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Init)
    }
    ///Header phase
    #[inline(always)]
    pub fn header(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Header)
    }
    ///Payload phase
    #[inline(always)]
    pub fn payload(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Payload)
    }
    ///Final Phase
    #[inline(always)]
    pub fn final_(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Final)
    }
}
/**Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHMOD_2 {
    ///0: Mode as per CHMOD (ECB, CBC, CTR, GCM)
    Chmod = 0,
    ///1: Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    Ccm = 1,
}
impl From<CHMOD_2> for bool {
    #[inline(always)]
    fn from(variant: CHMOD_2) -> Self {
        variant as u8 != 0
    }
}
///Field `CHMOD_2` reader - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD_2_R = crate::BitReader<CHMOD_2>;
impl CHMOD_2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHMOD_2 {
        match self.bits {
            false => CHMOD_2::Chmod,
            true => CHMOD_2::Ccm,
        }
    }
    ///Mode as per CHMOD (ECB, CBC, CTR, GCM)
    #[inline(always)]
    pub fn is_chmod(&self) -> bool {
        *self == CHMOD_2::Chmod
    }
    ///Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    #[inline(always)]
    pub fn is_ccm(&self) -> bool {
        *self == CHMOD_2::Ccm
    }
}
///Field `CHMOD_2` writer - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD_2_W<'a, REG> = crate::BitWriter<'a, REG, CHMOD_2>;
impl<'a, REG> CHMOD_2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mode as per CHMOD (ECB, CBC, CTR, GCM)
    #[inline(always)]
    pub fn chmod(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD_2::Chmod)
    }
    ///Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)
    #[inline(always)]
    pub fn ccm(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD_2::Ccm)
    }
}
/**Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYSIZE {
    ///0: 128
    Aes128 = 0,
    ///1: 256
    Aes256 = 1,
}
impl From<KEYSIZE> for bool {
    #[inline(always)]
    fn from(variant: KEYSIZE) -> Self {
        variant as u8 != 0
    }
}
///Field `KEYSIZE` reader - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_R = crate::BitReader<KEYSIZE>;
impl KEYSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KEYSIZE {
        match self.bits {
            false => KEYSIZE::Aes128,
            true => KEYSIZE::Aes256,
        }
    }
    ///128
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEYSIZE::Aes128
    }
    ///256
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEYSIZE::Aes256
    }
}
///Field `KEYSIZE` writer - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG, KEYSIZE>;
impl<'a, REG> KEYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///128
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZE::Aes128)
    }
    ///256
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZE::Aes256)
    }
}
///Field `NPBLB` reader - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero.
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero.
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod_2(&self) -> CHMOD_2_R {
        CHMOD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("chmod", &self.chmod())
            .field("ccfc", &self.ccfc())
            .field("errc", &self.errc())
            .field("ccfie", &self.ccfie())
            .field("errie", &self.errie())
            .field("dmainen", &self.dmainen())
            .field("dmaouten", &self.dmaouten())
            .field("gcmph", &self.gcmph())
            .field("chmod_2", &self.chmod_2())
            .field("keysize", &self.keysize())
            .field("npblb", &self.npblb())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access. Any attempt to selecting Mode 4 while either ECB or CBC chaining mode is not selected, defaults to effective selection of Mode 3. It is not possible to select a Mode 3 following a Mode 4.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<'_, CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 7 - Computation complete flag clear Upon written to 1, this bit clears the computation complete flag (CCF) in the AES_SR register: Reading the flag always returns zero.
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W<'_, CRrs> {
        CCFC_W::new(self, 7)
    }
    ///Bit 8 - Error flag clear Upon written to 1, this bit clears the RDERR and WRERR error flags in the AES_SR register: Reading the flag always returns zero.
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<'_, CRrs> {
        ERRC_W::new(self, 8)
    }
    ///Bit 9 - CCF interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set:
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<'_, CRrs> {
        CCFIE_W::new(self, 9)
    }
    ///Bit 10 - Error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RDERR and/or WRERR is set:
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CRrs> {
        ERRIE_W::new(self, 10)
    }
    ///Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<'_, CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation). Usage of DMA with Mode 4 (single decryption) is not recommended.
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<'_, CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<'_, CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - Chaining mode selection, bit \[2\] Refer to the bits \[5:6\] of the register for the description of the CHMOD\[2:0\] bitfield CHMOD\[1:0\]: Chaining mode selection, bits \[1:0\] This bitfield, together with the bit CHMOD\[2\] forming CHMOD\[2:0\], selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod_2(&mut self) -> CHMOD_2_W<'_, CRrs> {
        CHMOD_2_W::new(self, 16)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<'_, CRrs> {
        NPBLB_W::new(self, 20)
    }
}
/**AES control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#AES:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
