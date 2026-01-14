///Register `I2SCFGR` reader
pub type R = crate::R<I2SCFGRrs>;
///Register `I2SCFGR` writer
pub type W = crate::W<I2SCFGRrs>;
/**CHLEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN {
    ///0: 16-bit wide
    SixteenBit = 0,
    ///1: 32-bit wide
    ThirtyTwoBit = 1,
}
impl From<CHLEN> for bool {
    #[inline(always)]
    fn from(variant: CHLEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CHLEN` reader - CHLEN
pub type CHLEN_R = crate::BitReader<CHLEN>;
impl CHLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHLEN {
        match self.bits {
            false => CHLEN::SixteenBit,
            true => CHLEN::ThirtyTwoBit,
        }
    }
    ///16-bit wide
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CHLEN::SixteenBit
    }
    ///32-bit wide
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == CHLEN::ThirtyTwoBit
    }
}
///Field `CHLEN` writer - CHLEN
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG, CHLEN>;
impl<'a, REG> CHLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///16-bit wide
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::SixteenBit)
    }
    ///32-bit wide
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::ThirtyTwoBit)
    }
}
/**DATLEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN {
    ///0: 16-bit data length
    SixteenBit = 0,
    ///1: 24-bit data length
    TwentyFourBit = 1,
    ///2: 32-bit data length
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
impl crate::IsEnum for DATLEN {}
///Field `DATLEN` reader - DATLEN
pub type DATLEN_R = crate::FieldReader<DATLEN>;
impl DATLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATLEN> {
        match self.bits {
            0 => Some(DATLEN::SixteenBit),
            1 => Some(DATLEN::TwentyFourBit),
            2 => Some(DATLEN::ThirtyTwoBit),
            _ => None,
        }
    }
    ///16-bit data length
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DATLEN::SixteenBit
    }
    ///24-bit data length
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        *self == DATLEN::TwentyFourBit
    }
    ///32-bit data length
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == DATLEN::ThirtyTwoBit
    }
}
///Field `DATLEN` writer - DATLEN
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATLEN>;
impl<'a, REG> DATLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16-bit data length
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::SixteenBit)
    }
    ///24-bit data length
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::TwentyFourBit)
    }
    ///32-bit data length
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::ThirtyTwoBit)
    }
}
/**CKPOL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL {
    ///0: I2S clock inactive state is low level
    IdleLow = 0,
    ///1: I2S clock inactive state is high level
    IdleHigh = 1,
}
impl From<CKPOL> for bool {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - CKPOL
pub type CKPOL_R = crate::BitReader<CKPOL>;
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL {
        match self.bits {
            false => CKPOL::IdleLow,
            true => CKPOL::IdleHigh,
        }
    }
    ///I2S clock inactive state is low level
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPOL::IdleLow
    }
    ///I2S clock inactive state is high level
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPOL::IdleHigh
    }
}
///Field `CKPOL` writer - CKPOL
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2S clock inactive state is low level
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::IdleLow)
    }
    ///I2S clock inactive state is high level
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::IdleHigh)
    }
}
/**I2SSTD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD {
    ///0: I2S Philips standard
    Philips = 0,
    ///1: MSB justified standard
    Msb = 1,
    ///2: LSB justified standard
    Lsb = 2,
    ///3: PCM standard
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
impl crate::IsEnum for I2SSTD {}
///Field `I2SSTD` reader - I2SSTD
pub type I2SSTD_R = crate::FieldReader<I2SSTD>;
impl I2SSTD_R {
    ///Get enumerated values variant
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
    ///I2S Philips standard
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD::Philips
    }
    ///MSB justified standard
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTD::Msb
    }
    ///LSB justified standard
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTD::Lsb
    }
    ///PCM standard
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD::Pcm
    }
}
///Field `I2SSTD` writer - I2SSTD
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2SSTD, crate::Safe>;
impl<'a, REG> I2SSTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I2S Philips standard
    #[inline(always)]
    pub fn philips(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Philips)
    }
    ///MSB justified standard
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Msb)
    }
    ///LSB justified standard
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Lsb)
    }
    ///PCM standard
    #[inline(always)]
    pub fn pcm(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Pcm)
    }
}
/**PCMSYNC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC {
    ///0: Short frame synchronisation
    Short = 0,
    ///1: Long frame synchronisation
    Long = 1,
}
impl From<PCMSYNC> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `PCMSYNC` reader - PCMSYNC
pub type PCMSYNC_R = crate::BitReader<PCMSYNC>;
impl PCMSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCMSYNC {
        match self.bits {
            false => PCMSYNC::Short,
            true => PCMSYNC::Long,
        }
    }
    ///Short frame synchronisation
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNC::Short
    }
    ///Long frame synchronisation
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNC::Long
    }
}
///Field `PCMSYNC` writer - PCMSYNC
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG, PCMSYNC>;
impl<'a, REG> PCMSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Short frame synchronisation
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Short)
    }
    ///Long frame synchronisation
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Long)
    }
}
/**I2SCFG

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG {
    ///0: Slave - transmit
    SlaveTx = 0,
    ///1: Slave - receive
    SlaveRx = 1,
    ///2: Master - transmit
    MasterTx = 2,
    ///3: Master - receive
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
impl crate::IsEnum for I2SCFG {}
///Field `I2SCFG` reader - I2SCFG
pub type I2SCFG_R = crate::FieldReader<I2SCFG>;
impl I2SCFG_R {
    ///Get enumerated values variant
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
    ///Slave - transmit
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == I2SCFG::SlaveTx
    }
    ///Slave - receive
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == I2SCFG::SlaveRx
    }
    ///Master - transmit
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == I2SCFG::MasterTx
    }
    ///Master - receive
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == I2SCFG::MasterRx
    }
}
///Field `I2SCFG` writer - I2SCFG
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2SCFG, crate::Safe>;
impl<'a, REG> I2SCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Slave - transmit
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveTx)
    }
    ///Slave - receive
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveRx)
    }
    ///Master - transmit
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterTx)
    }
    ///Master - receive
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterRx)
    }
}
/**I2SE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SE {
    ///0: I2S peripheral is disabled
    Disabled = 0,
    ///1: I2S peripheral is enabled
    Enabled = 1,
}
impl From<I2SE> for bool {
    #[inline(always)]
    fn from(variant: I2SE) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SE` reader - I2SE
pub type I2SE_R = crate::BitReader<I2SE>;
impl I2SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SE {
        match self.bits {
            false => I2SE::Disabled,
            true => I2SE::Enabled,
        }
    }
    ///I2S peripheral is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2SE::Disabled
    }
    ///I2S peripheral is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2SE::Enabled
    }
}
///Field `I2SE` writer - I2SE
pub type I2SE_W<'a, REG> = crate::BitWriter<'a, REG, I2SE>;
impl<'a, REG> I2SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2S peripheral is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE::Disabled)
    }
    ///I2S peripheral is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE::Enabled)
    }
}
/**I2SMOD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD {
    ///0: SPI mode is selected
    Spimode = 0,
    ///1: I2S mode is selected
    I2smode = 1,
}
impl From<I2SMOD> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SMOD` reader - I2SMOD
pub type I2SMOD_R = crate::BitReader<I2SMOD>;
impl I2SMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SMOD {
        match self.bits {
            false => I2SMOD::Spimode,
            true => I2SMOD::I2smode,
        }
    }
    ///SPI mode is selected
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        *self == I2SMOD::Spimode
    }
    ///I2S mode is selected
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        *self == I2SMOD::I2smode
    }
}
///Field `I2SMOD` writer - I2SMOD
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG, I2SMOD>;
impl<'a, REG> I2SMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI mode is selected
    #[inline(always)]
    pub fn spimode(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::Spimode)
    }
    ///I2S mode is selected
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::I2smode)
    }
}
impl R {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCFGR")
            .field("chlen", &self.chlen())
            .field("datlen", &self.datlen())
            .field("ckpol", &self.ckpol())
            .field("i2sstd", &self.i2sstd())
            .field("pcmsync", &self.pcmsync())
            .field("i2scfg", &self.i2scfg())
            .field("i2se", &self.i2se())
            .field("i2smod", &self.i2smod())
            .finish()
    }
}
impl W {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W<'_, I2SCFGRrs> {
        CHLEN_W::new(self, 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W<'_, I2SCFGRrs> {
        DATLEN_W::new(self, 1)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<'_, I2SCFGRrs> {
        CKPOL_W::new(self, 3)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W<'_, I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<'_, I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W<'_, I2SCFGRrs> {
        I2SCFG_W::new(self, 8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W<'_, I2SCFGRrs> {
        I2SE_W::new(self, 10)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W<'_, I2SCFGRrs> {
        I2SMOD_W::new(self, 11)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#SPI1:I2SCFGR)*/
pub struct I2SCFGRrs;
impl crate::RegisterSpec for I2SCFGRrs {
    type Ux = u16;
}
///`read()` method returns [`i2scfgr::R`](R) reader structure
impl crate::Readable for I2SCFGRrs {}
///`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure
impl crate::Writable for I2SCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGRrs {}
