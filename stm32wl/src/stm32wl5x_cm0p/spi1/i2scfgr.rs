#[doc = "Register `I2SCFGR` reader"]
pub type R = crate::R<I2SCFGRrs>;
#[doc = "Register `I2SCFGR` writer"]
pub type W = crate::W<I2SCFGRrs>;
#[doc = "CHLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN {
    #[doc = "0: 16-bit wide"]
    SixteenBit = 0,
    #[doc = "1: 32-bit wide"]
    ThirtyTwoBit = 1,
}
impl From<CHLEN> for bool {
    #[inline(always)]
    fn from(variant: CHLEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHLEN` reader - CHLEN"]
pub type CHLEN_R = crate::BitReader<CHLEN>;
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHLEN {
        match self.bits {
            false => CHLEN::SixteenBit,
            true => CHLEN::ThirtyTwoBit,
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CHLEN::SixteenBit
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == CHLEN::ThirtyTwoBit
    }
}
#[doc = "Field `CHLEN` writer - CHLEN"]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG, CHLEN>;
impl<'a, REG> CHLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::SixteenBit)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::ThirtyTwoBit)
    }
}
#[doc = "DATLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN {
    #[doc = "0: 16-bit data length"]
    SixteenBit = 0,
    #[doc = "1: 24-bit data length"]
    TwentyFourBit = 1,
    #[doc = "2: 32-bit data length"]
    ThirtyTwoBit = 2,
}
impl From<DATLEN> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATLEN {
    type Ux = u8;
}
#[doc = "Field `DATLEN` reader - DATLEN"]
pub type DATLEN_R = crate::FieldReader<DATLEN>;
impl DATLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATLEN> {
        match self.bits {
            0 => Some(DATLEN::SixteenBit),
            1 => Some(DATLEN::TwentyFourBit),
            2 => Some(DATLEN::ThirtyTwoBit),
            _ => None,
        }
    }
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DATLEN::SixteenBit
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        *self == DATLEN::TwentyFourBit
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == DATLEN::ThirtyTwoBit
    }
}
#[doc = "Field `DATLEN` writer - DATLEN"]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATLEN>;
impl<'a, REG> DATLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::SixteenBit)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::TwentyFourBit)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::ThirtyTwoBit)
    }
}
#[doc = "CKPOL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL {
    #[doc = "0: I2S clock inactive state is low level"]
    IdleLow = 0,
    #[doc = "1: I2S clock inactive state is high level"]
    IdleHigh = 1,
}
impl From<CKPOL> for bool {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - CKPOL"]
pub type CKPOL_R = crate::BitReader<CKPOL>;
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL {
        match self.bits {
            false => CKPOL::IdleLow,
            true => CKPOL::IdleHigh,
        }
    }
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPOL::IdleLow
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPOL::IdleHigh
    }
}
#[doc = "Field `CKPOL` writer - CKPOL"]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::IdleLow)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::IdleHigh)
    }
}
#[doc = "I2SSTD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD {
    #[doc = "0: I2S Philips standard"]
    Philips = 0,
    #[doc = "1: MSB justified standard"]
    Msb = 1,
    #[doc = "2: LSB justified standard"]
    Lsb = 2,
    #[doc = "3: PCM standard"]
    Pcm = 3,
}
impl From<I2SSTD> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SSTD {
    type Ux = u8;
}
#[doc = "Field `I2SSTD` reader - I2SSTD"]
pub type I2SSTD_R = crate::FieldReader<I2SSTD>;
impl I2SSTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SSTD {
        match self.bits {
            0 => I2SSTD::Philips,
            1 => I2SSTD::Msb,
            2 => I2SSTD::Lsb,
            3 => I2SSTD::Pcm,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD::Philips
    }
    #[doc = "MSB justified standard"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTD::Msb
    }
    #[doc = "LSB justified standard"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTD::Lsb
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD::Pcm
    }
}
#[doc = "Field `I2SSTD` writer - I2SSTD"]
pub type I2SSTD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2SSTD>;
impl<'a, REG> I2SSTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn philips(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Philips)
    }
    #[doc = "MSB justified standard"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Msb)
    }
    #[doc = "LSB justified standard"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Lsb)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Pcm)
    }
}
#[doc = "PCMSYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC {
    #[doc = "0: Short frame synchronisation"]
    Short = 0,
    #[doc = "1: Long frame synchronisation"]
    Long = 1,
}
impl From<PCMSYNC> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMSYNC` reader - PCMSYNC"]
pub type PCMSYNC_R = crate::BitReader<PCMSYNC>;
impl PCMSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCMSYNC {
        match self.bits {
            false => PCMSYNC::Short,
            true => PCMSYNC::Long,
        }
    }
    #[doc = "Short frame synchronisation"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNC::Short
    }
    #[doc = "Long frame synchronisation"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNC::Long
    }
}
#[doc = "Field `PCMSYNC` writer - PCMSYNC"]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG, PCMSYNC>;
impl<'a, REG> PCMSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short frame synchronisation"]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Short)
    }
    #[doc = "Long frame synchronisation"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Long)
    }
}
#[doc = "I2SCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG {
    #[doc = "0: Slave - transmit"]
    SlaveTx = 0,
    #[doc = "1: Slave - receive"]
    SlaveRx = 1,
    #[doc = "2: Master - transmit"]
    MasterTx = 2,
    #[doc = "3: Master - receive"]
    MasterRx = 3,
}
impl From<I2SCFG> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SCFG {
    type Ux = u8;
}
#[doc = "Field `I2SCFG` reader - I2SCFG"]
pub type I2SCFG_R = crate::FieldReader<I2SCFG>;
impl I2SCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SCFG {
        match self.bits {
            0 => I2SCFG::SlaveTx,
            1 => I2SCFG::SlaveRx,
            2 => I2SCFG::MasterTx,
            3 => I2SCFG::MasterRx,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == I2SCFG::SlaveTx
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == I2SCFG::SlaveRx
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == I2SCFG::MasterTx
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == I2SCFG::MasterRx
    }
}
#[doc = "Field `I2SCFG` writer - I2SCFG"]
pub type I2SCFG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2SCFG>;
impl<'a, REG> I2SCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveTx)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveRx)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterTx)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterRx)
    }
}
#[doc = "I2SE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SE {
    #[doc = "0: I2S peripheral is disabled"]
    Disabled = 0,
    #[doc = "1: I2S peripheral is enabled"]
    Enabled = 1,
}
impl From<I2SE> for bool {
    #[inline(always)]
    fn from(variant: I2SE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SE` reader - I2SE"]
pub type I2SE_R = crate::BitReader<I2SE>;
impl I2SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SE {
        match self.bits {
            false => I2SE::Disabled,
            true => I2SE::Enabled,
        }
    }
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2SE::Disabled
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2SE::Enabled
    }
}
#[doc = "Field `I2SE` writer - I2SE"]
pub type I2SE_W<'a, REG> = crate::BitWriter<'a, REG, I2SE>;
impl<'a, REG> I2SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE::Disabled)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE::Enabled)
    }
}
#[doc = "I2SMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD {
    #[doc = "0: SPI mode is selected"]
    Spimode = 0,
    #[doc = "1: I2S mode is selected"]
    I2smode = 1,
}
impl From<I2SMOD> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SMOD` reader - I2SMOD"]
pub type I2SMOD_R = crate::BitReader<I2SMOD>;
impl I2SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SMOD {
        match self.bits {
            false => I2SMOD::Spimode,
            true => I2SMOD::I2smode,
        }
    }
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        *self == I2SMOD::Spimode
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        *self == I2SMOD::I2smode
    }
}
#[doc = "Field `I2SMOD` writer - I2SMOD"]
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG, I2SMOD>;
impl<'a, REG> I2SMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn spimode(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::Spimode)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::I2smode)
    }
}
#[doc = "Field `ASTRTEN` reader - ASTRTEN"]
pub type ASTRTEN_R = crate::BitReader;
#[doc = "Field `ASTRTEN` writer - ASTRTEN"]
pub type ASTRTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CHLEN"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - DATLEN"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2SE"]
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2SMOD"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ASTRTEN"]
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHLEN"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<I2SCFGRrs> {
        CHLEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - DATLEN"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<I2SCFGRrs> {
        DATLEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - CKPOL"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<I2SCFGRrs> {
        CKPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2SCFG"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<I2SCFGRrs> {
        I2SCFG_W::new(self, 8)
    }
    #[doc = "Bit 10 - I2SE"]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<I2SCFGRrs> {
        I2SE_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2SMOD"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<I2SCFGRrs> {
        I2SMOD_W::new(self, 11)
    }
    #[doc = "Bit 12 - ASTRTEN"]
    #[inline(always)]
    #[must_use]
    pub fn astrten(&mut self) -> ASTRTEN_W<I2SCFGRrs> {
        ASTRTEN_W::new(self, 12)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCFGRrs;
impl crate::RegisterSpec for I2SCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2scfgr::R`](R) reader structure"]
impl crate::Readable for I2SCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure"]
impl crate::Writable for I2SCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGRrs {
    const RESET_VALUE: u32 = 0;
}
