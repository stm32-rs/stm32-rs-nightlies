///Register `I2SCFGR` reader
pub type R = crate::R<I2SCFGRrs>;
///Register `I2SCFGR` writer
pub type W = crate::W<I2SCFGRrs>;
/**I2S mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD {
    ///0: SPI mode selected
    Spi = 0,
    ///1: I2S/PCM mode selected
    I2s = 1,
}
impl From<I2SMOD> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SMOD` reader - I2S mode selection
pub type I2SMOD_R = crate::BitReader<I2SMOD>;
impl I2SMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SMOD {
        match self.bits {
            false => I2SMOD::Spi,
            true => I2SMOD::I2s,
        }
    }
    ///SPI mode selected
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == I2SMOD::Spi
    }
    ///I2S/PCM mode selected
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2SMOD::I2s
    }
}
///Field `I2SMOD` writer - I2S mode selection
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG, I2SMOD>;
impl<'a, REG> I2SMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI mode selected
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::Spi)
    }
    ///I2S/PCM mode selected
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::I2s)
    }
}
/**I2S configuration mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG {
    ///0: Slave, transmit
    SlaveTransmit = 0,
    ///1: Slave, recteive
    SlaveReceive = 1,
    ///2: Master, transmit
    MasterTransmit = 2,
    ///3: Master, receive
    MasterReceive = 3,
    ///4: Slave, full duplex
    SlaveFullDuplex = 4,
    ///5: Master, full duplex
    MasterFullDuplex = 5,
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
///Field `I2SCFG` reader - I2S configuration mode
pub type I2SCFG_R = crate::FieldReader<I2SCFG>;
impl I2SCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2SCFG> {
        match self.bits {
            0 => Some(I2SCFG::SlaveTransmit),
            1 => Some(I2SCFG::SlaveReceive),
            2 => Some(I2SCFG::MasterTransmit),
            3 => Some(I2SCFG::MasterReceive),
            4 => Some(I2SCFG::SlaveFullDuplex),
            5 => Some(I2SCFG::MasterFullDuplex),
            _ => None,
        }
    }
    ///Slave, transmit
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        *self == I2SCFG::SlaveTransmit
    }
    ///Slave, recteive
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        *self == I2SCFG::SlaveReceive
    }
    ///Master, transmit
    #[inline(always)]
    pub fn is_master_transmit(&self) -> bool {
        *self == I2SCFG::MasterTransmit
    }
    ///Master, receive
    #[inline(always)]
    pub fn is_master_receive(&self) -> bool {
        *self == I2SCFG::MasterReceive
    }
    ///Slave, full duplex
    #[inline(always)]
    pub fn is_slave_full_duplex(&self) -> bool {
        *self == I2SCFG::SlaveFullDuplex
    }
    ///Master, full duplex
    #[inline(always)]
    pub fn is_master_full_duplex(&self) -> bool {
        *self == I2SCFG::MasterFullDuplex
    }
}
///Field `I2SCFG` writer - I2S configuration mode
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, I2SCFG>;
impl<'a, REG> I2SCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Slave, transmit
    #[inline(always)]
    pub fn slave_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveTransmit)
    }
    ///Slave, recteive
    #[inline(always)]
    pub fn slave_receive(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveReceive)
    }
    ///Master, transmit
    #[inline(always)]
    pub fn master_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterTransmit)
    }
    ///Master, receive
    #[inline(always)]
    pub fn master_receive(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterReceive)
    }
    ///Slave, full duplex
    #[inline(always)]
    pub fn slave_full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveFullDuplex)
    }
    ///Master, full duplex
    #[inline(always)]
    pub fn master_full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterFullDuplex)
    }
}
/**I2S standard selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD {
    ///0: I2S Philips standard
    Philips = 0,
    ///1: MSB/left justified standard
    LeftAligned = 1,
    ///2: LSB/right justified standard
    RightAligned = 2,
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
///Field `I2SSTD` reader - I2S standard selection
pub type I2SSTD_R = crate::FieldReader<I2SSTD>;
impl I2SSTD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SSTD {
        match self.bits {
            0 => I2SSTD::Philips,
            1 => I2SSTD::LeftAligned,
            2 => I2SSTD::RightAligned,
            3 => I2SSTD::Pcm,
            _ => unreachable!(),
        }
    }
    ///I2S Philips standard
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD::Philips
    }
    ///MSB/left justified standard
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == I2SSTD::LeftAligned
    }
    ///LSB/right justified standard
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        *self == I2SSTD::RightAligned
    }
    ///PCM standard
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD::Pcm
    }
}
///Field `I2SSTD` writer - I2S standard selection
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
    ///MSB/left justified standard
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::LeftAligned)
    }
    ///LSB/right justified standard
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::RightAligned)
    }
    ///PCM standard
    #[inline(always)]
    pub fn pcm(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Pcm)
    }
}
/**PCM frame synchronization

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC {
    ///0: Short PCM frame synchronization
    Short = 0,
    ///1: Long PCM frame synchronization
    Long = 1,
}
impl From<PCMSYNC> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `PCMSYNC` reader - PCM frame synchronization
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
    ///Short PCM frame synchronization
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNC::Short
    }
    ///Long PCM frame synchronization
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNC::Long
    }
}
///Field `PCMSYNC` writer - PCM frame synchronization
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG, PCMSYNC>;
impl<'a, REG> PCMSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Short PCM frame synchronization
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Short)
    }
    ///Long PCM frame synchronization
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Long)
    }
}
/**Data length to be transferred

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN {
    ///0: 16 bit data length
    Bits16 = 0,
    ///1: 24 bit data length
    Bits24 = 1,
    ///2: 32 bit data length
    Bits32 = 2,
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
///Field `DATLEN` reader - Data length to be transferred
pub type DATLEN_R = crate::FieldReader<DATLEN>;
impl DATLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATLEN> {
        match self.bits {
            0 => Some(DATLEN::Bits16),
            1 => Some(DATLEN::Bits24),
            2 => Some(DATLEN::Bits32),
            _ => None,
        }
    }
    ///16 bit data length
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == DATLEN::Bits16
    }
    ///24 bit data length
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == DATLEN::Bits24
    }
    ///32 bit data length
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == DATLEN::Bits32
    }
}
///Field `DATLEN` writer - Data length to be transferred
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATLEN>;
impl<'a, REG> DATLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16 bit data length
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::Bits16)
    }
    ///24 bit data length
    #[inline(always)]
    pub fn bits24(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::Bits24)
    }
    ///32 bit data length
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::Bits32)
    }
}
/**Channel length (number of bits per audio channel)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN {
    ///0: 16 bit per channel
    Bits16 = 0,
    ///1: 32 bit per channel
    Bits32 = 1,
}
impl From<CHLEN> for bool {
    #[inline(always)]
    fn from(variant: CHLEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CHLEN` reader - Channel length (number of bits per audio channel)
pub type CHLEN_R = crate::BitReader<CHLEN>;
impl CHLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHLEN {
        match self.bits {
            false => CHLEN::Bits16,
            true => CHLEN::Bits32,
        }
    }
    ///16 bit per channel
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == CHLEN::Bits16
    }
    ///32 bit per channel
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == CHLEN::Bits32
    }
}
///Field `CHLEN` writer - Channel length (number of bits per audio channel)
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG, CHLEN>;
impl<'a, REG> CHLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///16 bit per channel
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::Bits16)
    }
    ///32 bit per channel
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::Bits32)
    }
}
/**Serial audio clock polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL {
    ///0: Signals are sampled on rising and changed on falling clock edges
    SampleOnRising = 0,
    ///1: Signals are sampled on falling and changed on rising clock edges
    SampleOnFalling = 1,
}
impl From<CKPOL> for bool {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - Serial audio clock polarity
pub type CKPOL_R = crate::BitReader<CKPOL>;
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL {
        match self.bits {
            false => CKPOL::SampleOnRising,
            true => CKPOL::SampleOnFalling,
        }
    }
    ///Signals are sampled on rising and changed on falling clock edges
    #[inline(always)]
    pub fn is_sample_on_rising(&self) -> bool {
        *self == CKPOL::SampleOnRising
    }
    ///Signals are sampled on falling and changed on rising clock edges
    #[inline(always)]
    pub fn is_sample_on_falling(&self) -> bool {
        *self == CKPOL::SampleOnFalling
    }
}
///Field `CKPOL` writer - Serial audio clock polarity
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Signals are sampled on rising and changed on falling clock edges
    #[inline(always)]
    pub fn sample_on_rising(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::SampleOnRising)
    }
    ///Signals are sampled on falling and changed on rising clock edges
    #[inline(always)]
    pub fn sample_on_falling(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::SampleOnFalling)
    }
}
/**Word select inversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXCH {
    ///0: The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)
    NotFixed = 0,
    ///1: The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)
    Fixed = 1,
}
impl From<FIXCH> for bool {
    #[inline(always)]
    fn from(variant: FIXCH) -> Self {
        variant as u8 != 0
    }
}
///Field `FIXCH` reader - Word select inversion
pub type FIXCH_R = crate::BitReader<FIXCH>;
impl FIXCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FIXCH {
        match self.bits {
            false => FIXCH::NotFixed,
            true => FIXCH::Fixed,
        }
    }
    ///The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)
    #[inline(always)]
    pub fn is_not_fixed(&self) -> bool {
        *self == FIXCH::NotFixed
    }
    ///The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == FIXCH::Fixed
    }
}
///Field `FIXCH` writer - Word select inversion
pub type FIXCH_W<'a, REG> = crate::BitWriter<'a, REG, FIXCH>;
impl<'a, REG> FIXCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)
    #[inline(always)]
    pub fn not_fixed(self) -> &'a mut crate::W<REG> {
        self.variant(FIXCH::NotFixed)
    }
    ///The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(FIXCH::Fixed)
    }
}
/**Fixed channel length in SLAVE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSINV {
    ///0: Word select inversion disabled
    Disabled = 0,
    ///1: Word select inversion enabled
    Enabled = 1,
}
impl From<WSINV> for bool {
    #[inline(always)]
    fn from(variant: WSINV) -> Self {
        variant as u8 != 0
    }
}
///Field `WSINV` reader - Fixed channel length in SLAVE
pub type WSINV_R = crate::BitReader<WSINV>;
impl WSINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WSINV {
        match self.bits {
            false => WSINV::Disabled,
            true => WSINV::Enabled,
        }
    }
    ///Word select inversion disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WSINV::Disabled
    }
    ///Word select inversion enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WSINV::Enabled
    }
}
///Field `WSINV` writer - Fixed channel length in SLAVE
pub type WSINV_W<'a, REG> = crate::BitWriter<'a, REG, WSINV>;
impl<'a, REG> WSINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Word select inversion disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WSINV::Disabled)
    }
    ///Word select inversion enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WSINV::Enabled)
    }
}
/**Data format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATFMT {
    ///0: The data inside RXDR and TXDR are right aligned
    RightAligned = 0,
    ///1: The data inside RXDR and TXDR are left aligned
    LeftAligned = 1,
}
impl From<DATFMT> for bool {
    #[inline(always)]
    fn from(variant: DATFMT) -> Self {
        variant as u8 != 0
    }
}
///Field `DATFMT` reader - Data format
pub type DATFMT_R = crate::BitReader<DATFMT>;
impl DATFMT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATFMT {
        match self.bits {
            false => DATFMT::RightAligned,
            true => DATFMT::LeftAligned,
        }
    }
    ///The data inside RXDR and TXDR are right aligned
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        *self == DATFMT::RightAligned
    }
    ///The data inside RXDR and TXDR are left aligned
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == DATFMT::LeftAligned
    }
}
///Field `DATFMT` writer - Data format
pub type DATFMT_W<'a, REG> = crate::BitWriter<'a, REG, DATFMT>;
impl<'a, REG> DATFMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The data inside RXDR and TXDR are right aligned
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(DATFMT::RightAligned)
    }
    ///The data inside RXDR and TXDR are left aligned
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(DATFMT::LeftAligned)
    }
}
///Field `I2SDIV` reader - I2S linear prescaler
pub type I2SDIV_R = crate::FieldReader;
///Field `I2SDIV` writer - I2S linear prescaler
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Odd factor for the prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD {
    ///0: Real divider value is I2SDIV*2
    Even = 0,
    ///1: Real divider value is I2SDIV*2 + 1
    Odd = 1,
}
impl From<ODD> for bool {
    #[inline(always)]
    fn from(variant: ODD) -> Self {
        variant as u8 != 0
    }
}
///Field `ODD` reader - Odd factor for the prescaler
pub type ODD_R = crate::BitReader<ODD>;
impl ODD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ODD {
        match self.bits {
            false => ODD::Even,
            true => ODD::Odd,
        }
    }
    ///Real divider value is I2SDIV*2
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ODD::Even
    }
    ///Real divider value is I2SDIV*2 + 1
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ODD::Odd
    }
}
///Field `ODD` writer - Odd factor for the prescaler
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Real divider value is I2SDIV*2
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Even)
    }
    ///Real divider value is I2SDIV*2 + 1
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Odd)
    }
}
/**Master clock output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE {
    ///0: Master clock output disabled
    Disabled = 0,
    ///1: Master clock output enabled
    Enabled = 1,
}
impl From<MCKOE> for bool {
    #[inline(always)]
    fn from(variant: MCKOE) -> Self {
        variant as u8 != 0
    }
}
///Field `MCKOE` reader - Master clock output enable
pub type MCKOE_R = crate::BitReader<MCKOE>;
impl MCKOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCKOE {
        match self.bits {
            false => MCKOE::Disabled,
            true => MCKOE::Enabled,
        }
    }
    ///Master clock output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOE::Disabled
    }
    ///Master clock output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOE::Enabled
    }
}
///Field `MCKOE` writer - Master clock output enable
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG, MCKOE>;
impl<'a, REG> MCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master clock output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Disabled)
    }
    ///Master clock output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Enabled)
    }
}
impl R {
    ///Bit 0 - I2S mode selection
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - I2S configuration mode
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:5 - I2S standard selection
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - PCM frame synchronization
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Data length to be transferred
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Channel length (number of bits per audio channel)
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Serial audio clock polarity
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word select inversion
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Fixed channel length in SLAVE
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Data format
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - I2S linear prescaler
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Odd factor for the prescaler
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Master clock output enable
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCFGR")
            .field("mckoe", &self.mckoe())
            .field("odd", &self.odd())
            .field("i2sdiv", &self.i2sdiv())
            .field("datfmt", &self.datfmt())
            .field("wsinv", &self.wsinv())
            .field("fixch", &self.fixch())
            .field("ckpol", &self.ckpol())
            .field("chlen", &self.chlen())
            .field("datlen", &self.datlen())
            .field("pcmsync", &self.pcmsync())
            .field("i2sstd", &self.i2sstd())
            .field("i2scfg", &self.i2scfg())
            .field("i2smod", &self.i2smod())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2S mode selection
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W<'_, I2SCFGRrs> {
        I2SMOD_W::new(self, 0)
    }
    ///Bits 1:3 - I2S configuration mode
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W<'_, I2SCFGRrs> {
        I2SCFG_W::new(self, 1)
    }
    ///Bits 4:5 - I2S standard selection
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W<'_, I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    ///Bit 7 - PCM frame synchronization
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<'_, I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    ///Bits 8:9 - Data length to be transferred
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W<'_, I2SCFGRrs> {
        DATLEN_W::new(self, 8)
    }
    ///Bit 10 - Channel length (number of bits per audio channel)
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W<'_, I2SCFGRrs> {
        CHLEN_W::new(self, 10)
    }
    ///Bit 11 - Serial audio clock polarity
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<'_, I2SCFGRrs> {
        CKPOL_W::new(self, 11)
    }
    ///Bit 12 - Word select inversion
    #[inline(always)]
    pub fn fixch(&mut self) -> FIXCH_W<'_, I2SCFGRrs> {
        FIXCH_W::new(self, 12)
    }
    ///Bit 13 - Fixed channel length in SLAVE
    #[inline(always)]
    pub fn wsinv(&mut self) -> WSINV_W<'_, I2SCFGRrs> {
        WSINV_W::new(self, 13)
    }
    ///Bit 14 - Data format
    #[inline(always)]
    pub fn datfmt(&mut self) -> DATFMT_W<'_, I2SCFGRrs> {
        DATFMT_W::new(self, 14)
    }
    ///Bits 16:23 - I2S linear prescaler
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<'_, I2SCFGRrs> {
        I2SDIV_W::new(self, 16)
    }
    ///Bit 24 - Odd factor for the prescaler
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W<'_, I2SCFGRrs> {
        ODD_W::new(self, 24)
    }
    ///Bit 25 - Master clock output enable
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W<'_, I2SCFGRrs> {
        MCKOE_W::new(self, 25)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#SPI1:I2SCFGR)*/
pub struct I2SCFGRrs;
impl crate::RegisterSpec for I2SCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`i2scfgr::R`](R) reader structure
impl crate::Readable for I2SCFGRrs {}
///`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure
impl crate::Writable for I2SCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGRrs {}
