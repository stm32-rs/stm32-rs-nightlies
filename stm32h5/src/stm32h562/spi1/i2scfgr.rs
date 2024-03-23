#[doc = "Register `I2SCFGR` reader"]
pub type R = crate::R<I2SCFGRrs>;
#[doc = "Register `I2SCFGR` writer"]
pub type W = crate::W<I2SCFGRrs>;
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD {
    #[doc = "0: SPI mode selected"]
    Spi = 0,
    #[doc = "1: I2S/PCM mode selected"]
    I2s = 1,
}
impl From<I2SMOD> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2SMOD_R = crate::BitReader<I2SMOD>;
impl I2SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SMOD {
        match self.bits {
            false => I2SMOD::Spi,
            true => I2SMOD::I2s,
        }
    }
    #[doc = "SPI mode selected"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == I2SMOD::Spi
    }
    #[doc = "I2S/PCM mode selected"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2SMOD::I2s
    }
}
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG, I2SMOD>;
impl<'a, REG> I2SMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode selected"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::Spi)
    }
    #[doc = "I2S/PCM mode selected"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::I2s)
    }
}
#[doc = "I2S configuration mode others, not used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG {
    #[doc = "0: Slave, transmit"]
    SlaveTransmit = 0,
    #[doc = "1: Slave, recteive"]
    SlaveReceive = 1,
    #[doc = "2: Master, transmit"]
    MasterTransmit = 2,
    #[doc = "3: Master, receive"]
    MasterReceive = 3,
    #[doc = "4: Slave, full duplex"]
    SlaveFullDuplex = 4,
    #[doc = "5: Master, full duplex"]
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
#[doc = "Field `I2SCFG` reader - I2S configuration mode others, not used"]
pub type I2SCFG_R = crate::FieldReader<I2SCFG>;
impl I2SCFG_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Slave, transmit"]
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        *self == I2SCFG::SlaveTransmit
    }
    #[doc = "Slave, recteive"]
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        *self == I2SCFG::SlaveReceive
    }
    #[doc = "Master, transmit"]
    #[inline(always)]
    pub fn is_master_transmit(&self) -> bool {
        *self == I2SCFG::MasterTransmit
    }
    #[doc = "Master, receive"]
    #[inline(always)]
    pub fn is_master_receive(&self) -> bool {
        *self == I2SCFG::MasterReceive
    }
    #[doc = "Slave, full duplex"]
    #[inline(always)]
    pub fn is_slave_full_duplex(&self) -> bool {
        *self == I2SCFG::SlaveFullDuplex
    }
    #[doc = "Master, full duplex"]
    #[inline(always)]
    pub fn is_master_full_duplex(&self) -> bool {
        *self == I2SCFG::MasterFullDuplex
    }
}
#[doc = "Field `I2SCFG` writer - I2S configuration mode others, not used"]
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, I2SCFG>;
impl<'a, REG> I2SCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave, transmit"]
    #[inline(always)]
    pub fn slave_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveTransmit)
    }
    #[doc = "Slave, recteive"]
    #[inline(always)]
    pub fn slave_receive(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveReceive)
    }
    #[doc = "Master, transmit"]
    #[inline(always)]
    pub fn master_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterTransmit)
    }
    #[doc = "Master, receive"]
    #[inline(always)]
    pub fn master_receive(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterReceive)
    }
    #[doc = "Slave, full duplex"]
    #[inline(always)]
    pub fn slave_full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::SlaveFullDuplex)
    }
    #[doc = "Master, full duplex"]
    #[inline(always)]
    pub fn master_full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::MasterFullDuplex)
    }
}
#[doc = "I2S standard selection For more details on I2S standards, refer to\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD {
    #[doc = "0: I2S Philips standard"]
    Philips = 0,
    #[doc = "1: MSB/left justified standard"]
    LeftAligned = 1,
    #[doc = "2: LSB/right justified standard"]
    RightAligned = 2,
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
#[doc = "Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to"]
pub type I2SSTD_R = crate::FieldReader<I2SSTD>;
impl I2SSTD_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD::Philips
    }
    #[doc = "MSB/left justified standard"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == I2SSTD::LeftAligned
    }
    #[doc = "LSB/right justified standard"]
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        *self == I2SSTD::RightAligned
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD::Pcm
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to"]
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
    #[doc = "MSB/left justified standard"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::LeftAligned)
    }
    #[doc = "LSB/right justified standard"]
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::RightAligned)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::Pcm)
    }
}
#[doc = "PCM frame synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC {
    #[doc = "0: Short PCM frame synchronization"]
    Short = 0,
    #[doc = "1: Long PCM frame synchronization"]
    Long = 1,
}
impl From<PCMSYNC> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
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
    #[doc = "Short PCM frame synchronization"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNC::Short
    }
    #[doc = "Long PCM frame synchronization"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNC::Long
    }
}
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG, PCMSYNC>;
impl<'a, REG> PCMSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short PCM frame synchronization"]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Short)
    }
    #[doc = "Long PCM frame synchronization"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::Long)
    }
}
#[doc = "data length to be transferred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN {
    #[doc = "0: 16 bit data length"]
    Bits16 = 0,
    #[doc = "1: 24 bit data length"]
    Bits24 = 1,
    #[doc = "2: 32 bit data length"]
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
#[doc = "Field `DATLEN` reader - data length to be transferred"]
pub type DATLEN_R = crate::FieldReader<DATLEN>;
impl DATLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATLEN> {
        match self.bits {
            0 => Some(DATLEN::Bits16),
            1 => Some(DATLEN::Bits24),
            2 => Some(DATLEN::Bits32),
            _ => None,
        }
    }
    #[doc = "16 bit data length"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == DATLEN::Bits16
    }
    #[doc = "24 bit data length"]
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == DATLEN::Bits24
    }
    #[doc = "32 bit data length"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == DATLEN::Bits32
    }
}
#[doc = "Field `DATLEN` writer - data length to be transferred"]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATLEN>;
impl<'a, REG> DATLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 bit data length"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::Bits16)
    }
    #[doc = "24 bit data length"]
    #[inline(always)]
    pub fn bits24(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::Bits24)
    }
    #[doc = "32 bit data length"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::Bits32)
    }
}
#[doc = "channel length (number of bits per audio channel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN {
    #[doc = "0: 16 bit per channel"]
    Bits16 = 0,
    #[doc = "1: 32 bit per channel"]
    Bits32 = 1,
}
impl From<CHLEN> for bool {
    #[inline(always)]
    fn from(variant: CHLEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHLEN` reader - channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader<CHLEN>;
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHLEN {
        match self.bits {
            false => CHLEN::Bits16,
            true => CHLEN::Bits32,
        }
    }
    #[doc = "16 bit per channel"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == CHLEN::Bits16
    }
    #[doc = "32 bit per channel"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == CHLEN::Bits32
    }
}
#[doc = "Field `CHLEN` writer - channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG, CHLEN>;
impl<'a, REG> CHLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16 bit per channel"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::Bits16)
    }
    #[doc = "32 bit per channel"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::Bits32)
    }
}
#[doc = "serial audio clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL {
    #[doc = "0: Signals are sampled on rising and changed on falling clock edges"]
    SampleOnRising = 0,
    #[doc = "1: Signals are sampled on falling and changed on rising clock edges"]
    SampleOnFalling = 1,
}
impl From<CKPOL> for bool {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - serial audio clock polarity"]
pub type CKPOL_R = crate::BitReader<CKPOL>;
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL {
        match self.bits {
            false => CKPOL::SampleOnRising,
            true => CKPOL::SampleOnFalling,
        }
    }
    #[doc = "Signals are sampled on rising and changed on falling clock edges"]
    #[inline(always)]
    pub fn is_sample_on_rising(&self) -> bool {
        *self == CKPOL::SampleOnRising
    }
    #[doc = "Signals are sampled on falling and changed on rising clock edges"]
    #[inline(always)]
    pub fn is_sample_on_falling(&self) -> bool {
        *self == CKPOL::SampleOnFalling
    }
}
#[doc = "Field `CKPOL` writer - serial audio clock polarity"]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Signals are sampled on rising and changed on falling clock edges"]
    #[inline(always)]
    pub fn sample_on_rising(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::SampleOnRising)
    }
    #[doc = "Signals are sampled on falling and changed on rising clock edges"]
    #[inline(always)]
    pub fn sample_on_falling(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::SampleOnFalling)
    }
}
#[doc = "fixed channel length in slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXCH {
    #[doc = "0: The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    NotFixed = 0,
    #[doc = "1: The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    Fixed = 1,
}
impl From<FIXCH> for bool {
    #[inline(always)]
    fn from(variant: FIXCH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIXCH` reader - fixed channel length in slave"]
pub type FIXCH_R = crate::BitReader<FIXCH>;
impl FIXCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIXCH {
        match self.bits {
            false => FIXCH::NotFixed,
            true => FIXCH::Fixed,
        }
    }
    #[doc = "The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    #[inline(always)]
    pub fn is_not_fixed(&self) -> bool {
        *self == FIXCH::NotFixed
    }
    #[doc = "The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == FIXCH::Fixed
    }
}
#[doc = "Field `FIXCH` writer - fixed channel length in slave"]
pub type FIXCH_W<'a, REG> = crate::BitWriter<'a, REG, FIXCH>;
impl<'a, REG> FIXCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    #[inline(always)]
    pub fn not_fixed(self) -> &'a mut crate::W<REG> {
        self.variant(FIXCH::NotFixed)
    }
    #[doc = "The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(FIXCH::Fixed)
    }
}
#[doc = "word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSINV {
    #[doc = "0: Word select inversion disabled"]
    Disabled = 0,
    #[doc = "1: Word select inversion enabled"]
    Enabled = 1,
}
impl From<WSINV> for bool {
    #[inline(always)]
    fn from(variant: WSINV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSINV` reader - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
pub type WSINV_R = crate::BitReader<WSINV>;
impl WSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WSINV {
        match self.bits {
            false => WSINV::Disabled,
            true => WSINV::Enabled,
        }
    }
    #[doc = "Word select inversion disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WSINV::Disabled
    }
    #[doc = "Word select inversion enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WSINV::Enabled
    }
}
#[doc = "Field `WSINV` writer - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
pub type WSINV_W<'a, REG> = crate::BitWriter<'a, REG, WSINV>;
impl<'a, REG> WSINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word select inversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WSINV::Disabled)
    }
    #[doc = "Word select inversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WSINV::Enabled)
    }
}
#[doc = "data format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATFMT {
    #[doc = "0: The data inside RXDR and TXDR are right aligned"]
    RightAligned = 0,
    #[doc = "1: The data inside RXDR and TXDR are left aligned"]
    LeftAligned = 1,
}
impl From<DATFMT> for bool {
    #[inline(always)]
    fn from(variant: DATFMT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATFMT` reader - data format"]
pub type DATFMT_R = crate::BitReader<DATFMT>;
impl DATFMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATFMT {
        match self.bits {
            false => DATFMT::RightAligned,
            true => DATFMT::LeftAligned,
        }
    }
    #[doc = "The data inside RXDR and TXDR are right aligned"]
    #[inline(always)]
    pub fn is_right_aligned(&self) -> bool {
        *self == DATFMT::RightAligned
    }
    #[doc = "The data inside RXDR and TXDR are left aligned"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == DATFMT::LeftAligned
    }
}
#[doc = "Field `DATFMT` writer - data format"]
pub type DATFMT_W<'a, REG> = crate::BitWriter<'a, REG, DATFMT>;
impl<'a, REG> DATFMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data inside RXDR and TXDR are right aligned"]
    #[inline(always)]
    pub fn right_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(DATFMT::RightAligned)
    }
    #[doc = "The data inside RXDR and TXDR are left aligned"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(DATFMT::LeftAligned)
    }
}
#[doc = "Field `I2SDIV` reader - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
pub type I2SDIV_R = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
pub type I2SDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "odd factor for the prescaler Refer to for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD {
    #[doc = "0: Real divider value is I2SDIV*2"]
    Even = 0,
    #[doc = "1: Real divider value is I2SDIV*2 + 1"]
    Odd = 1,
}
impl From<ODD> for bool {
    #[inline(always)]
    fn from(variant: ODD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - odd factor for the prescaler Refer to for details"]
pub type ODD_R = crate::BitReader<ODD>;
impl ODD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODD {
        match self.bits {
            false => ODD::Even,
            true => ODD::Odd,
        }
    }
    #[doc = "Real divider value is I2SDIV*2"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ODD::Even
    }
    #[doc = "Real divider value is I2SDIV*2 + 1"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ODD::Odd
    }
}
#[doc = "Field `ODD` writer - odd factor for the prescaler Refer to for details"]
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real divider value is I2SDIV*2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Even)
    }
    #[doc = "Real divider value is I2SDIV*2 + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::Odd)
    }
}
#[doc = "master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE {
    #[doc = "0: Master clock output disabled"]
    Disabled = 0,
    #[doc = "1: Master clock output enabled"]
    Enabled = 1,
}
impl From<MCKOE> for bool {
    #[inline(always)]
    fn from(variant: MCKOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOE` reader - master clock output enable"]
pub type MCKOE_R = crate::BitReader<MCKOE>;
impl MCKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCKOE {
        match self.bits {
            false => MCKOE::Disabled,
            true => MCKOE::Enabled,
        }
    }
    #[doc = "Master clock output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOE::Disabled
    }
    #[doc = "Master clock output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOE::Enabled
    }
}
#[doc = "Field `MCKOE` writer - master clock output enable"]
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG, MCKOE>;
impl<'a, REG> MCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Disabled)
    }
    #[doc = "Master clock output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode others, not used"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - serial audio clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - fixed channel length in slave"]
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - data format"]
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - odd factor for the prescaler Refer to for details"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<I2SCFGRrs> {
        I2SMOD_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode others, not used"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<I2SCFGRrs> {
        I2SCFG_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - data length to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<I2SCFGRrs> {
        DATLEN_W::new(self, 8)
    }
    #[doc = "Bit 10 - channel length (number of bits per audio channel)"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<I2SCFGRrs> {
        CHLEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - serial audio clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<I2SCFGRrs> {
        CKPOL_W::new(self, 11)
    }
    #[doc = "Bit 12 - fixed channel length in slave"]
    #[inline(always)]
    #[must_use]
    pub fn fixch(&mut self) -> FIXCH_W<I2SCFGRrs> {
        FIXCH_W::new(self, 12)
    }
    #[doc = "Bit 13 - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
    #[inline(always)]
    #[must_use]
    pub fn wsinv(&mut self) -> WSINV_W<I2SCFGRrs> {
        WSINV_W::new(self, 13)
    }
    #[doc = "Bit 14 - data format"]
    #[inline(always)]
    #[must_use]
    pub fn datfmt(&mut self) -> DATFMT_W<I2SCFGRrs> {
        DATFMT_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<I2SCFGRrs> {
        I2SDIV_W::new(self, 16)
    }
    #[doc = "Bit 24 - odd factor for the prescaler Refer to for details"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<I2SCFGRrs> {
        ODD_W::new(self, 24)
    }
    #[doc = "Bit 25 - master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<I2SCFGRrs> {
        MCKOE_W::new(self, 25)
    }
}
#[doc = "SPI/I2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
