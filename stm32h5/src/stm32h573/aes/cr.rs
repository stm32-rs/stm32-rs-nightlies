///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0. Note: With KMOD\[1:0\] other than 00, use the IPRST bit rather than the bit EN.

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
///Field `EN` reader - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0. Note: With KMOD\[1:0\] other than 00, use the IPRST bit rather than the bit EN.
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
///Field `EN` writer - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0. Note: With KMOD\[1:0\] other than 00, use the IPRST bit rather than the bit EN.
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
/**Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

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
///Field `DATATYPE` reader - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
///Field `DATATYPE` writer - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
/**AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

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
///Field `MODE` reader - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
///Field `MODE` writer - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
/**Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

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
///Field `CHMOD` reader - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
///Field `CHMOD` writer - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
/**DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).

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
///Field `DMAINEN` reader - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
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
///Field `DMAINEN` writer - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
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
/**DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).

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
///Field `DMAOUTEN` reader - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
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
///Field `DMAOUTEN` writer - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
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
/**Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

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
///Field `CHMOD_2` reader - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
///Field `CHMOD_2` writer - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
/**Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

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
///Field `KEYSIZE` reader - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
///Field `KEYSIZE` writer - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
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
/**Key mode selection The bitfield defines how the AES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to AES_DIN and AES_DOUT registers. With selection of shared key from SAES co-processor, the AES peripheral automatically loads its key registers with the data stored in the key registers of the SAES peripheral. The key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register. The bitfield must be set only when KEYSIZE is correct, and when a shared key decryption sequence has been successfully completed in SAES co-processor. N/AAttempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KMOD {
    ///0: Normal key mode. Key registers are freely usable
    NormalKey = 0,
    ///2: Shared key mode. If shared key mode is properly initialized in SAES peripheral, the AES peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in AES key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register
    SharedKey = 2,
}
impl From<KMOD> for u8 {
    #[inline(always)]
    fn from(variant: KMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KMOD {
    type Ux = u8;
}
impl crate::IsEnum for KMOD {}
///Field `KMOD` reader - Key mode selection The bitfield defines how the AES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to AES_DIN and AES_DOUT registers. With selection of shared key from SAES co-processor, the AES peripheral automatically loads its key registers with the data stored in the key registers of the SAES peripheral. The key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register. The bitfield must be set only when KEYSIZE is correct, and when a shared key decryption sequence has been successfully completed in SAES co-processor. N/AAttempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type KMOD_R = crate::FieldReader<KMOD>;
impl KMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<KMOD> {
        match self.bits {
            0 => Some(KMOD::NormalKey),
            2 => Some(KMOD::SharedKey),
            _ => None,
        }
    }
    ///Normal key mode. Key registers are freely usable
    #[inline(always)]
    pub fn is_normal_key(&self) -> bool {
        *self == KMOD::NormalKey
    }
    ///Shared key mode. If shared key mode is properly initialized in SAES peripheral, the AES peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in AES key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register
    #[inline(always)]
    pub fn is_shared_key(&self) -> bool {
        *self == KMOD::SharedKey
    }
}
///Field `KMOD` writer - Key mode selection The bitfield defines how the AES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to AES_DIN and AES_DOUT registers. With selection of shared key from SAES co-processor, the AES peripheral automatically loads its key registers with the data stored in the key registers of the SAES peripheral. The key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register. The bitfield must be set only when KEYSIZE is correct, and when a shared key decryption sequence has been successfully completed in SAES co-processor. N/AAttempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, KMOD>;
impl<'a, REG> KMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal key mode. Key registers are freely usable
    #[inline(always)]
    pub fn normal_key(self) -> &'a mut crate::W<REG> {
        self.variant(KMOD::NormalKey)
    }
    ///Shared key mode. If shared key mode is properly initialized in SAES peripheral, the AES peripheral automatically loads its key registers with the data stored in the SAES key registers. The key value is available in AES key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register
    #[inline(always)]
    pub fn shared_key(self) -> &'a mut crate::W<REG> {
        self.variant(KMOD::SharedKey)
    }
}
///Field `IPRST` reader - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be low while writing any configuration registers.
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be low while writing any configuration registers.
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0. Note: With KMOD\[1:0\] other than 00, use the IPRST bit rather than the bit EN.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod_2(&self) -> CHMOD_2_R {
        CHMOD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25 - Key mode selection The bitfield defines how the AES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to AES_DIN and AES_DOUT registers. With selection of shared key from SAES co-processor, the AES peripheral automatically loads its key registers with the data stored in the key registers of the SAES peripheral. The key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register. The bitfield must be set only when KEYSIZE is correct, and when a shared key decryption sequence has been successfully completed in SAES co-processor. N/AAttempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 31 - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be low while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("chmod", &self.chmod())
            .field("dmainen", &self.dmainen())
            .field("dmaouten", &self.dmaouten())
            .field("gcmph", &self.gcmph())
            .field("chmod_2", &self.chmod_2())
            .field("keysize", &self.keysize())
            .field("npblb", &self.npblb())
            .field("kmod", &self.kmod())
            .field("iprst", &self.iprst())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES enable This bit enables/disables the AES peripheral: At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0. Note: With KMOD\[1:0\] other than 00, use the IPRST bit rather than the bit EN.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - AES operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<'_, CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by AES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<'_, CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by AES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\] bitfield. It is not effective for Mode 2 (key derivation).
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<'_, CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<'_, CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod_2(&mut self) -> CHMOD_2_W<'_, CRrs> {
        CHMOD_2_W::new(self, 16)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the AES cryptographic core, in bits: Attempts to write the bit are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<'_, CRrs> {
        NPBLB_W::new(self, 20)
    }
    ///Bits 24:25 - Key mode selection The bitfield defines how the AES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to AES_DIN and AES_DOUT registers. With selection of shared key from SAES co-processor, the AES peripheral automatically loads its key registers with the data stored in the key registers of the SAES peripheral. The key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the AES_SR register. Key error flag KEIF is set otherwise in the AES_ISR register. The bitfield must be set only when KEYSIZE is correct, and when a shared key decryption sequence has been successfully completed in SAES co-processor. N/AAttempts to write the bitfield are ignored when the BUSY flag of AES_SR register is set, as well as when the EN bit of the AES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn kmod(&mut self) -> KMOD_W<'_, CRrs> {
        KMOD_W::new(self, 24)
    }
    ///Bit 31 - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be low while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<'_, CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**AES control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#AES:CR)*/
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
