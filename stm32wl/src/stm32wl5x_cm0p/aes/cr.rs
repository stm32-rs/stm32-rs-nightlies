#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "AES enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Disable AES"]
    Disabled = 0,
    #[doc = "1: Enable AES"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - AES enable"]
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    #[doc = "Disable AES"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Enable AES"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - AES enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable AES"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Enable AES"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "Data type selection (for data in and data out to/from the cryptographic block)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATATYPE {
    #[doc = "0: Word"]
    None = 0,
    #[doc = "1: Half-word (16-bit)"]
    HalfWord = 1,
    #[doc = "2: Byte (8-bit)"]
    Byte = 2,
    #[doc = "3: Bit"]
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
#[doc = "Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DATATYPE_R = crate::FieldReader<DATATYPE>;
impl DATATYPE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Word"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DATATYPE::None
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DATATYPE::HalfWord
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DATATYPE::Byte
    }
    #[doc = "Bit"]
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == DATATYPE::Bit
    }
}
#[doc = "Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DATATYPE>;
impl<'a, REG> DATATYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Word"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::None)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::HalfWord)
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::Byte)
    }
    #[doc = "Bit"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE::Bit)
    }
}
#[doc = "AES operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Mode 1: encryption"]
    Mode1 = 0,
    #[doc = "1: Mode 2: key derivation (or key preparation for ECB/CBC decryption)"]
    Mode2 = 1,
    #[doc = "2: Mode 3: decryption"]
    Mode3 = 2,
    #[doc = "3: Mode 4: key derivation &amp; decrypt (UNDOCUMENTED in ref. manual, exists in CubeMX code)"]
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
#[doc = "Field `MODE` reader - AES operating mode"]
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Mode 1: encryption"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == MODE::Mode1
    }
    #[doc = "Mode 2: key derivation (or key preparation for ECB/CBC decryption)"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == MODE::Mode2
    }
    #[doc = "Mode 3: decryption"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == MODE::Mode3
    }
    #[doc = "Mode 4: key derivation &amp; decrypt (UNDOCUMENTED in ref. manual, exists in CubeMX code)"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == MODE::Mode4
    }
}
#[doc = "Field `MODE` writer - AES operating mode"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 1: encryption"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode1)
    }
    #[doc = "Mode 2: key derivation (or key preparation for ECB/CBC decryption)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode2)
    }
    #[doc = "Mode 3: decryption"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode3)
    }
    #[doc = "Mode 4: key derivation &amp; decrypt (UNDOCUMENTED in ref. manual, exists in CubeMX code)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Mode4)
    }
}
#[doc = "AES chaining mode Bit1 Bit0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMOD {
    #[doc = "0: Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1"]
    Ecb = 0,
    #[doc = "1: Cipher-block chaining (CBC)"]
    Cbc = 1,
    #[doc = "2: Counter mode (CTR)"]
    Ctr = 2,
    #[doc = "3: Galois counter mode (GCM) and Galois message authentication code (GMAC)"]
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
#[doc = "Field `CHMOD` reader - AES chaining mode Bit1 Bit0"]
pub type CHMOD_R = crate::FieldReader<CHMOD>;
impl CHMOD_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CHMOD::Ecb
    }
    #[doc = "Cipher-block chaining (CBC)"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CHMOD::Cbc
    }
    #[doc = "Counter mode (CTR)"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == CHMOD::Ctr
    }
    #[doc = "Galois counter mode (GCM) and Galois message authentication code (GMAC)"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == CHMOD::Gcm
    }
}
#[doc = "Field `CHMOD` writer - AES chaining mode Bit1 Bit0"]
pub type CHMOD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CHMOD>;
impl<'a, REG> CHMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Electronic codebook (ECB) / Counter with CBC-MAC (CCM) if CHMOD2 is 1"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ecb)
    }
    #[doc = "Cipher-block chaining (CBC)"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Cbc)
    }
    #[doc = "Counter mode (CTR)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ctr)
    }
    #[doc = "Galois counter mode (GCM) and Galois message authentication code (GMAC)"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Gcm)
    }
}
#[doc = "Computation Complete Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFCW {
    #[doc = "1: Clear computation complete flag"]
    Clear = 1,
}
impl From<CCFCW> for bool {
    #[inline(always)]
    fn from(variant: CCFCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCFC` reader - Computation Complete Flag Clear"]
pub type CCFC_R = crate::BitReader<CCFCW>;
impl CCFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCFCW> {
        match self.bits {
            true => Some(CCFCW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear computation complete flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCFCW::Clear
    }
}
#[doc = "Field `CCFC` writer - Computation Complete Flag Clear"]
pub type CCFC_W<'a, REG> = crate::BitWriter<'a, REG, CCFCW>;
impl<'a, REG> CCFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear computation complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCFCW::Clear)
    }
}
#[doc = "Error clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRCW {
    #[doc = "1: Clear RDERR and WRERR flags"]
    Clear = 1,
}
impl From<ERRCW> for bool {
    #[inline(always)]
    fn from(variant: ERRCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRC` reader - Error clear"]
pub type ERRC_R = crate::BitReader<ERRCW>;
impl ERRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERRCW> {
        match self.bits {
            true => Some(ERRCW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear RDERR and WRERR flags"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ERRCW::Clear
    }
}
#[doc = "Field `ERRC` writer - Error clear"]
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG, ERRCW>;
impl<'a, REG> ERRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear RDERR and WRERR flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRCW::Clear)
    }
}
#[doc = "CCF flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFIE {
    #[doc = "0: Disable (mask) CCF interrupt"]
    Disabled = 0,
    #[doc = "1: Enable CCF interrupt"]
    Enabled = 1,
}
impl From<CCFIE> for bool {
    #[inline(always)]
    fn from(variant: CCFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCFIE` reader - CCF flag interrupt enable"]
pub type CCFIE_R = crate::BitReader<CCFIE>;
impl CCFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCFIE {
        match self.bits {
            false => CCFIE::Disabled,
            true => CCFIE::Enabled,
        }
    }
    #[doc = "Disable (mask) CCF interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCFIE::Disabled
    }
    #[doc = "Enable CCF interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCFIE::Enabled
    }
}
#[doc = "Field `CCFIE` writer - CCF flag interrupt enable"]
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG, CCFIE>;
impl<'a, REG> CCFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (mask) CCF interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCFIE::Disabled)
    }
    #[doc = "Enable CCF interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCFIE::Enabled)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: Disable (mask) error interrupt"]
    Disabled = 0,
    #[doc = "1: Enable error interrupt"]
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    #[doc = "Disable (mask) error interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    #[doc = "Enable error interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (mask) error interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    #[doc = "Enable error interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
#[doc = "Enable DMA management of data input phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAINEN {
    #[doc = "0: Disable DMA Input"]
    Disabled = 0,
    #[doc = "1: Enable DMA Input"]
    Enabled = 1,
}
impl From<DMAINEN> for bool {
    #[inline(always)]
    fn from(variant: DMAINEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINEN` reader - Enable DMA management of data input phase"]
pub type DMAINEN_R = crate::BitReader<DMAINEN>;
impl DMAINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAINEN {
        match self.bits {
            false => DMAINEN::Disabled,
            true => DMAINEN::Enabled,
        }
    }
    #[doc = "Disable DMA Input"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAINEN::Disabled
    }
    #[doc = "Enable DMA Input"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAINEN::Enabled
    }
}
#[doc = "Field `DMAINEN` writer - Enable DMA management of data input phase"]
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAINEN>;
impl<'a, REG> DMAINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA Input"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINEN::Disabled)
    }
    #[doc = "Enable DMA Input"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINEN::Enabled)
    }
}
#[doc = "Enable DMA management of data output phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAOUTEN {
    #[doc = "0: Disable DMA Output"]
    Disabled = 0,
    #[doc = "1: Enabled DMA Output"]
    Enabled = 1,
}
impl From<DMAOUTEN> for bool {
    #[inline(always)]
    fn from(variant: DMAOUTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAOUTEN` reader - Enable DMA management of data output phase"]
pub type DMAOUTEN_R = crate::BitReader<DMAOUTEN>;
impl DMAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAOUTEN {
        match self.bits {
            false => DMAOUTEN::Disabled,
            true => DMAOUTEN::Enabled,
        }
    }
    #[doc = "Disable DMA Output"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAOUTEN::Disabled
    }
    #[doc = "Enabled DMA Output"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAOUTEN::Enabled
    }
}
#[doc = "Field `DMAOUTEN` writer - Enable DMA management of data output phase"]
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAOUTEN>;
impl<'a, REG> DMAOUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA Output"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAOUTEN::Disabled)
    }
    #[doc = "Enabled DMA Output"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAOUTEN::Enabled)
    }
}
#[doc = "Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GCMPH {
    #[doc = "0: Init phase"]
    Init = 0,
    #[doc = "1: Header phase"]
    Header = 1,
    #[doc = "2: Payload phase"]
    Payload = 2,
    #[doc = "3: Final phase"]
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
#[doc = "Field `GCMPH` reader - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
pub type GCMPH_R = crate::FieldReader<GCMPH>;
impl GCMPH_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Init phase"]
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        *self == GCMPH::Init
    }
    #[doc = "Header phase"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == GCMPH::Header
    }
    #[doc = "Payload phase"]
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        *self == GCMPH::Payload
    }
    #[doc = "Final phase"]
    #[inline(always)]
    pub fn is_final(&self) -> bool {
        *self == GCMPH::Final
    }
}
#[doc = "Field `GCMPH` writer - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
pub type GCMPH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, GCMPH>;
impl<'a, REG> GCMPH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Init phase"]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Init)
    }
    #[doc = "Header phase"]
    #[inline(always)]
    pub fn header(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Header)
    }
    #[doc = "Payload phase"]
    #[inline(always)]
    pub fn payload(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Payload)
    }
    #[doc = "Final phase"]
    #[inline(always)]
    pub fn final_(self) -> &'a mut crate::W<REG> {
        self.variant(GCMPH::Final)
    }
}
#[doc = "AES chaining mode Bit2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHMOD2 {
    #[doc = "0: Mode as per CHMOD (ECB, CBC, CTR, GCM)"]
    Chmod = 0,
    #[doc = "1: Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)"]
    Ccm = 1,
}
impl From<CHMOD2> for bool {
    #[inline(always)]
    fn from(variant: CHMOD2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHMOD2` reader - AES chaining mode Bit2"]
pub type CHMOD2_R = crate::BitReader<CHMOD2>;
impl CHMOD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHMOD2 {
        match self.bits {
            false => CHMOD2::Chmod,
            true => CHMOD2::Ccm,
        }
    }
    #[doc = "Mode as per CHMOD (ECB, CBC, CTR, GCM)"]
    #[inline(always)]
    pub fn is_chmod(&self) -> bool {
        *self == CHMOD2::Chmod
    }
    #[doc = "Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)"]
    #[inline(always)]
    pub fn is_ccm(&self) -> bool {
        *self == CHMOD2::Ccm
    }
}
#[doc = "Field `CHMOD2` writer - AES chaining mode Bit2"]
pub type CHMOD2_W<'a, REG> = crate::BitWriter<'a, REG, CHMOD2>;
impl<'a, REG> CHMOD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode as per CHMOD (ECB, CBC, CTR, GCM)"]
    #[inline(always)]
    pub fn chmod(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD2::Chmod)
    }
    #[doc = "Counter with CBC-MAC (CCM) - CHMOD must be 0 (ECB)"]
    #[inline(always)]
    pub fn ccm(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD2::Ccm)
    }
}
#[doc = "Key size selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYSIZE {
    #[doc = "0: 128 bits"]
    Bits128 = 0,
    #[doc = "1: 256 bits"]
    Bits256 = 1,
}
impl From<KEYSIZE> for bool {
    #[inline(always)]
    fn from(variant: KEYSIZE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSIZE` reader - Key size selection"]
pub type KEYSIZE_R = crate::BitReader<KEYSIZE>;
impl KEYSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KEYSIZE {
        match self.bits {
            false => KEYSIZE::Bits128,
            true => KEYSIZE::Bits256,
        }
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn is_bits128(&self) -> bool {
        *self == KEYSIZE::Bits128
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn is_bits256(&self) -> bool {
        *self == KEYSIZE::Bits256
    }
}
#[doc = "Field `KEYSIZE` writer - Key size selection"]
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG, KEYSIZE>;
impl<'a, REG> KEYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn bits128(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZE::Bits128)
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn bits256(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZE::Bits256)
    }
}
#[doc = "Field `NPBLB` reader - Number of padding bytes in last block of payload"]
pub type NPBLB_R = crate::FieldReader;
#[doc = "Field `NPBLB` writer - Number of padding bytes in last block of payload"]
pub type NPBLB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES chaining mode Bit1 Bit0"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - AES chaining mode Bit2"]
    #[inline(always)]
    pub fn chmod2(&self) -> CHMOD2_R {
        CHMOD2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Key size selection"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block of payload"]
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - AES chaining mode Bit1 Bit0"]
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccfc(&mut self) -> CCFC_W<CRrs> {
        CCFC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<CRrs> {
        ERRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CCFIE_W<CRrs> {
        CCFIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
    #[inline(always)]
    #[must_use]
    pub fn gcmph(&mut self) -> GCMPH_W<CRrs> {
        GCMPH_W::new(self, 13)
    }
    #[doc = "Bit 16 - AES chaining mode Bit2"]
    #[inline(always)]
    #[must_use]
    pub fn chmod2(&mut self) -> CHMOD2_W<CRrs> {
        CHMOD2_W::new(self, 16)
    }
    #[doc = "Bit 18 - Key size selection"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block of payload"]
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NPBLB_W<CRrs> {
        NPBLB_W::new(self, 20)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
