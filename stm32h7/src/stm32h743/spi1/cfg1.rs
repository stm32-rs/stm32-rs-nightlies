#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1rs>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1rs>;
#[doc = "Field `DSIZE` reader - Number of bits in at single SPI data frame"]
pub type DSIZE_R = crate::FieldReader;
#[doc = "Field `DSIZE` writer - Number of bits in at single SPI data frame"]
pub type DSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "threshold level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTHLV {
    #[doc = "0: 1 frame"]
    OneFrame = 0,
    #[doc = "1: 2 frames"]
    TwoFrames = 1,
    #[doc = "2: 3 frames"]
    ThreeFrames = 2,
    #[doc = "3: 4 frames"]
    FourFrames = 3,
    #[doc = "4: 5 frames"]
    FiveFrames = 4,
    #[doc = "5: 6 frames"]
    SixFrames = 5,
    #[doc = "6: 7 frames"]
    SevenFrames = 6,
    #[doc = "7: 8 frames"]
    EightFrames = 7,
    #[doc = "8: 9 frames"]
    NineFrames = 8,
    #[doc = "9: 10 frames"]
    TenFrames = 9,
    #[doc = "10: 11 frames"]
    ElevenFrames = 10,
    #[doc = "11: 12 frames"]
    TwelveFrames = 11,
    #[doc = "12: 13 frames"]
    ThirteenFrames = 12,
    #[doc = "13: 14 frames"]
    FourteenFrames = 13,
    #[doc = "14: 15 frames"]
    FifteenFrames = 14,
    #[doc = "15: 16 frames"]
    SixteenFrames = 15,
}
impl From<FTHLV> for u8 {
    #[inline(always)]
    fn from(variant: FTHLV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTHLV {
    type Ux = u8;
}
#[doc = "Field `FTHLV` reader - threshold level"]
pub type FTHLV_R = crate::FieldReader<FTHLV>;
impl FTHLV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FTHLV {
        match self.bits {
            0 => FTHLV::OneFrame,
            1 => FTHLV::TwoFrames,
            2 => FTHLV::ThreeFrames,
            3 => FTHLV::FourFrames,
            4 => FTHLV::FiveFrames,
            5 => FTHLV::SixFrames,
            6 => FTHLV::SevenFrames,
            7 => FTHLV::EightFrames,
            8 => FTHLV::NineFrames,
            9 => FTHLV::TenFrames,
            10 => FTHLV::ElevenFrames,
            11 => FTHLV::TwelveFrames,
            12 => FTHLV::ThirteenFrames,
            13 => FTHLV::FourteenFrames,
            14 => FTHLV::FifteenFrames,
            15 => FTHLV::SixteenFrames,
            _ => unreachable!(),
        }
    }
    #[doc = "1 frame"]
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == FTHLV::OneFrame
    }
    #[doc = "2 frames"]
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == FTHLV::TwoFrames
    }
    #[doc = "3 frames"]
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == FTHLV::ThreeFrames
    }
    #[doc = "4 frames"]
    #[inline(always)]
    pub fn is_four_frames(&self) -> bool {
        *self == FTHLV::FourFrames
    }
    #[doc = "5 frames"]
    #[inline(always)]
    pub fn is_five_frames(&self) -> bool {
        *self == FTHLV::FiveFrames
    }
    #[doc = "6 frames"]
    #[inline(always)]
    pub fn is_six_frames(&self) -> bool {
        *self == FTHLV::SixFrames
    }
    #[doc = "7 frames"]
    #[inline(always)]
    pub fn is_seven_frames(&self) -> bool {
        *self == FTHLV::SevenFrames
    }
    #[doc = "8 frames"]
    #[inline(always)]
    pub fn is_eight_frames(&self) -> bool {
        *self == FTHLV::EightFrames
    }
    #[doc = "9 frames"]
    #[inline(always)]
    pub fn is_nine_frames(&self) -> bool {
        *self == FTHLV::NineFrames
    }
    #[doc = "10 frames"]
    #[inline(always)]
    pub fn is_ten_frames(&self) -> bool {
        *self == FTHLV::TenFrames
    }
    #[doc = "11 frames"]
    #[inline(always)]
    pub fn is_eleven_frames(&self) -> bool {
        *self == FTHLV::ElevenFrames
    }
    #[doc = "12 frames"]
    #[inline(always)]
    pub fn is_twelve_frames(&self) -> bool {
        *self == FTHLV::TwelveFrames
    }
    #[doc = "13 frames"]
    #[inline(always)]
    pub fn is_thirteen_frames(&self) -> bool {
        *self == FTHLV::ThirteenFrames
    }
    #[doc = "14 frames"]
    #[inline(always)]
    pub fn is_fourteen_frames(&self) -> bool {
        *self == FTHLV::FourteenFrames
    }
    #[doc = "15 frames"]
    #[inline(always)]
    pub fn is_fifteen_frames(&self) -> bool {
        *self == FTHLV::FifteenFrames
    }
    #[doc = "16 frames"]
    #[inline(always)]
    pub fn is_sixteen_frames(&self) -> bool {
        *self == FTHLV::SixteenFrames
    }
}
#[doc = "Field `FTHLV` writer - threshold level"]
pub type FTHLV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, FTHLV>;
impl<'a, REG> FTHLV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 frame"]
    #[inline(always)]
    pub fn one_frame(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::OneFrame)
    }
    #[doc = "2 frames"]
    #[inline(always)]
    pub fn two_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::TwoFrames)
    }
    #[doc = "3 frames"]
    #[inline(always)]
    pub fn three_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::ThreeFrames)
    }
    #[doc = "4 frames"]
    #[inline(always)]
    pub fn four_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FourFrames)
    }
    #[doc = "5 frames"]
    #[inline(always)]
    pub fn five_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FiveFrames)
    }
    #[doc = "6 frames"]
    #[inline(always)]
    pub fn six_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::SixFrames)
    }
    #[doc = "7 frames"]
    #[inline(always)]
    pub fn seven_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::SevenFrames)
    }
    #[doc = "8 frames"]
    #[inline(always)]
    pub fn eight_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::EightFrames)
    }
    #[doc = "9 frames"]
    #[inline(always)]
    pub fn nine_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::NineFrames)
    }
    #[doc = "10 frames"]
    #[inline(always)]
    pub fn ten_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::TenFrames)
    }
    #[doc = "11 frames"]
    #[inline(always)]
    pub fn eleven_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::ElevenFrames)
    }
    #[doc = "12 frames"]
    #[inline(always)]
    pub fn twelve_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::TwelveFrames)
    }
    #[doc = "13 frames"]
    #[inline(always)]
    pub fn thirteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::ThirteenFrames)
    }
    #[doc = "14 frames"]
    #[inline(always)]
    pub fn fourteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FourteenFrames)
    }
    #[doc = "15 frames"]
    #[inline(always)]
    pub fn fifteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::FifteenFrames)
    }
    #[doc = "16 frames"]
    #[inline(always)]
    pub fn sixteen_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV::SixteenFrames)
    }
}
#[doc = "Behavior of slave transmitter at underrun condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDRCFG {
    #[doc = "0: Slave sends a constant underrun pattern"]
    Constant = 0,
    #[doc = "1: Slave repeats last received data frame from master"]
    RepeatReceived = 1,
    #[doc = "2: Slave repeats last transmitted data frame"]
    RepeatTransmitted = 2,
}
impl From<UDRCFG> for u8 {
    #[inline(always)]
    fn from(variant: UDRCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UDRCFG {
    type Ux = u8;
}
#[doc = "Field `UDRCFG` reader - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_R = crate::FieldReader<UDRCFG>;
impl UDRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UDRCFG> {
        match self.bits {
            0 => Some(UDRCFG::Constant),
            1 => Some(UDRCFG::RepeatReceived),
            2 => Some(UDRCFG::RepeatTransmitted),
            _ => None,
        }
    }
    #[doc = "Slave sends a constant underrun pattern"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == UDRCFG::Constant
    }
    #[doc = "Slave repeats last received data frame from master"]
    #[inline(always)]
    pub fn is_repeat_received(&self) -> bool {
        *self == UDRCFG::RepeatReceived
    }
    #[doc = "Slave repeats last transmitted data frame"]
    #[inline(always)]
    pub fn is_repeat_transmitted(&self) -> bool {
        *self == UDRCFG::RepeatTransmitted
    }
}
#[doc = "Field `UDRCFG` writer - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UDRCFG>;
impl<'a, REG> UDRCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave sends a constant underrun pattern"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(UDRCFG::Constant)
    }
    #[doc = "Slave repeats last received data frame from master"]
    #[inline(always)]
    pub fn repeat_received(self) -> &'a mut crate::W<REG> {
        self.variant(UDRCFG::RepeatReceived)
    }
    #[doc = "Slave repeats last transmitted data frame"]
    #[inline(always)]
    pub fn repeat_transmitted(self) -> &'a mut crate::W<REG> {
        self.variant(UDRCFG::RepeatTransmitted)
    }
}
#[doc = "Detection of underrun condition at slave transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDRDET {
    #[doc = "0: Underrun is detected at begin of data frame"]
    StartOfFrame = 0,
    #[doc = "1: Underrun is detected at end of last data frame"]
    EndOfFrame = 1,
    #[doc = "2: Underrun is detected at begin of active SS signal"]
    StartOfSlaveSelect = 2,
}
impl From<UDRDET> for u8 {
    #[inline(always)]
    fn from(variant: UDRDET) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UDRDET {
    type Ux = u8;
}
#[doc = "Field `UDRDET` reader - Detection of underrun condition at slave transmitter"]
pub type UDRDET_R = crate::FieldReader<UDRDET>;
impl UDRDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UDRDET> {
        match self.bits {
            0 => Some(UDRDET::StartOfFrame),
            1 => Some(UDRDET::EndOfFrame),
            2 => Some(UDRDET::StartOfSlaveSelect),
            _ => None,
        }
    }
    #[doc = "Underrun is detected at begin of data frame"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == UDRDET::StartOfFrame
    }
    #[doc = "Underrun is detected at end of last data frame"]
    #[inline(always)]
    pub fn is_end_of_frame(&self) -> bool {
        *self == UDRDET::EndOfFrame
    }
    #[doc = "Underrun is detected at begin of active SS signal"]
    #[inline(always)]
    pub fn is_start_of_slave_select(&self) -> bool {
        *self == UDRDET::StartOfSlaveSelect
    }
}
#[doc = "Field `UDRDET` writer - Detection of underrun condition at slave transmitter"]
pub type UDRDET_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UDRDET>;
impl<'a, REG> UDRDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Underrun is detected at begin of data frame"]
    #[inline(always)]
    pub fn start_of_frame(self) -> &'a mut crate::W<REG> {
        self.variant(UDRDET::StartOfFrame)
    }
    #[doc = "Underrun is detected at end of last data frame"]
    #[inline(always)]
    pub fn end_of_frame(self) -> &'a mut crate::W<REG> {
        self.variant(UDRDET::EndOfFrame)
    }
    #[doc = "Underrun is detected at begin of active SS signal"]
    #[inline(always)]
    pub fn start_of_slave_select(self) -> &'a mut crate::W<REG> {
        self.variant(UDRDET::StartOfSlaveSelect)
    }
}
#[doc = "Rx DMA stream enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    #[doc = "0: Rx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx DMA stream enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
#[doc = "Field `RXDMAEN` writer - Rx DMA stream enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
#[doc = "Tx DMA stream enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    #[doc = "0: Tx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx DMA stream enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
#[doc = "Field `TXDMAEN` writer - Tx DMA stream enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
#[doc = "Field `CRCSIZE` reader - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_R = crate::FieldReader;
#[doc = "Field `CRCSIZE` writer - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Hardware CRC computation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN {
    #[doc = "0: CRC calculation disabled"]
    Disabled = 0,
    #[doc = "1: CRC calculation enabled"]
    Enabled = 1,
}
impl From<CRCEN> for bool {
    #[inline(always)]
    fn from(variant: CRCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - Hardware CRC computation enable"]
pub type CRCEN_R = crate::BitReader<CRCEN>;
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN {
        match self.bits {
            false => CRCEN::Disabled,
            true => CRCEN::Enabled,
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN::Disabled
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN::Enabled
    }
}
#[doc = "Field `CRCEN` writer - Hardware CRC computation enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Disabled)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Enabled)
    }
}
#[doc = "Master baud rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBR {
    #[doc = "0: f_spi_ker_ck / 2"]
    Div2 = 0,
    #[doc = "1: f_spi_ker_ck / 4"]
    Div4 = 1,
    #[doc = "2: f_spi_ker_ck / 8"]
    Div8 = 2,
    #[doc = "3: f_spi_ker_ck / 16"]
    Div16 = 3,
    #[doc = "4: f_spi_ker_ck / 32"]
    Div32 = 4,
    #[doc = "5: f_spi_ker_ck / 64"]
    Div64 = 5,
    #[doc = "6: f_spi_ker_ck / 128"]
    Div128 = 6,
    #[doc = "7: f_spi_ker_ck / 256"]
    Div256 = 7,
}
impl From<MBR> for u8 {
    #[inline(always)]
    fn from(variant: MBR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MBR {
    type Ux = u8;
}
#[doc = "Field `MBR` reader - Master baud rate"]
pub type MBR_R = crate::FieldReader<MBR>;
impl MBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBR {
        match self.bits {
            0 => MBR::Div2,
            1 => MBR::Div4,
            2 => MBR::Div8,
            3 => MBR::Div16,
            4 => MBR::Div32,
            5 => MBR::Div64,
            6 => MBR::Div128,
            7 => MBR::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "f_spi_ker_ck / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MBR::Div2
    }
    #[doc = "f_spi_ker_ck / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MBR::Div4
    }
    #[doc = "f_spi_ker_ck / 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MBR::Div8
    }
    #[doc = "f_spi_ker_ck / 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MBR::Div16
    }
    #[doc = "f_spi_ker_ck / 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MBR::Div32
    }
    #[doc = "f_spi_ker_ck / 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MBR::Div64
    }
    #[doc = "f_spi_ker_ck / 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MBR::Div128
    }
    #[doc = "f_spi_ker_ck / 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == MBR::Div256
    }
}
#[doc = "Field `MBR` writer - Master baud rate"]
pub type MBR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, MBR>;
impl<'a, REG> MBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f_spi_ker_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div2)
    }
    #[doc = "f_spi_ker_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div4)
    }
    #[doc = "f_spi_ker_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div8)
    }
    #[doc = "f_spi_ker_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div16)
    }
    #[doc = "f_spi_ker_ck / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div32)
    }
    #[doc = "f_spi_ker_ck / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div64)
    }
    #[doc = "f_spi_ker_ck / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div128)
    }
    #[doc = "f_spi_ker_ck / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(MBR::Div256)
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<CFG1rs> {
        DSIZE_W::new(self, 0)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn fthlv(&mut self) -> FTHLV_W<CFG1rs> {
        FTHLV_W::new(self, 5)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<CFG1rs> {
        UDRCFG_W::new(self, 9)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn udrdet(&mut self) -> UDRDET_W<CFG1rs> {
        UDRDET_W::new(self, 11)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFG1rs> {
        RXDMAEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFG1rs> {
        TXDMAEN_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<CFG1rs> {
        CRCSIZE_W::new(self, 16)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<CFG1rs> {
        CRCEN_W::new(self, 22)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<CFG1rs> {
        MBR_W::new(self, 28)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1rs;
impl crate::RegisterSpec for CFG1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1rs {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0x0007_0007"]
impl crate::Resettable for CFG1rs {
    const RESET_VALUE: u32 = 0x0007_0007;
}
