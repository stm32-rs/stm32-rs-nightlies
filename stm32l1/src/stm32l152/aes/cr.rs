///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**AES enable

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
///Field `EN` reader - AES enable
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
///Field `EN` writer - AES enable
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
/**Data type selection

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
///Field `DATATYPE` reader - Data type selection
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
///Field `DATATYPE` writer - Data type selection
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
/**AES operating mode

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
///Field `MODE` reader - AES operating mode
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
///Field `MODE` writer - AES operating mode
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
/**AES chaining mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMOD {
    ///0: Electronic codebook (ECB)
    Ecb = 0,
    ///1: Cipher-Block Chaining (CBC)
    Cbc = 1,
    ///2: Counter Mode (CTR)
    Ctr = 2,
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
///Field `CHMOD` reader - AES chaining mode
pub type CHMOD_R = crate::FieldReader<CHMOD>;
impl CHMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHMOD> {
        match self.bits {
            0 => Some(CHMOD::Ecb),
            1 => Some(CHMOD::Cbc),
            2 => Some(CHMOD::Ctr),
            _ => None,
        }
    }
    ///Electronic codebook (ECB)
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CHMOD::Ecb
    }
    ///Cipher-Block Chaining (CBC)
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CHMOD::Cbc
    }
    ///Counter Mode (CTR)
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == CHMOD::Ctr
    }
}
///Field `CHMOD` writer - AES chaining mode
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CHMOD>;
impl<'a, REG> CHMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Electronic codebook (ECB)
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ecb)
    }
    ///Cipher-Block Chaining (CBC)
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Cbc)
    }
    ///Counter Mode (CTR)
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(CHMOD::Ctr)
    }
}
/**Computation Complete Flag Clear

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
///Field `CCFC` reader - Computation Complete Flag Clear
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
///Field `CCFC` writer - Computation Complete Flag Clear
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
/**Error clear

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
///Field `ERRC` reader - Error clear
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
///Field `ERRC` writer - Error clear
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
/**CCF flag interrupt enable

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
///Field `CCFIE` reader - CCF flag interrupt enable
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
///Field `CCFIE` writer - CCF flag interrupt enable
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
/**Error interrupt enable

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
///Field `ERRIE` reader - Error interrupt enable
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
///Field `ERRIE` writer - Error interrupt enable
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
/**Enable DMA management of data input phase

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
///Field `DMAINEN` reader - Enable DMA management of data input phase
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
///Field `DMAINEN` writer - Enable DMA management of data input phase
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
/**Enable DMA management of data output phase

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
///Field `DMAOUTEN` reader - Enable DMA management of data output phase
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
///Field `DMAOUTEN` writer - Enable DMA management of data output phase
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
impl R {
    ///Bit 0 - AES enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - AES chaining mode
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Computation Complete Flag Clear
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Error clear
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CCF flag interrupt enable
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dmaouten", &self.dmaouten())
            .field("dmainen", &self.dmainen())
            .field("errie", &self.errie())
            .field("ccfie", &self.ccfie())
            .field("errc", &self.errc())
            .field("ccfc", &self.ccfc())
            .field("chmod", &self.chmod())
            .field("mode", &self.mode())
            .field("datatype", &self.datatype())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type selection
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - AES chaining mode
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 7 - Computation Complete Flag Clear
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W<CRrs> {
        CCFC_W::new(self, 7)
    }
    ///Bit 8 - Error clear
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<CRrs> {
        ERRC_W::new(self, 8)
    }
    ///Bit 9 - CCF flag interrupt enable
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<CRrs> {
        CCFIE_W::new(self, 9)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 10)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#AES:CR)*/
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
