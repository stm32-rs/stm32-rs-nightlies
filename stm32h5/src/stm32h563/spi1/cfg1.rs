#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1rs>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1rs>;
#[doc = "Field `DSIZE` reader - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
pub type DSIZE_R = crate::FieldReader;
#[doc = "Field `DSIZE` writer - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
pub type DSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features\n\nValue on reset: 0"]
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
#[doc = "Field `FTHLV` reader - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
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
#[doc = "Field `FTHLV` writer - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
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
#[doc = "behavior of slave transmitter at underrun condition For more details see underrun condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRCFG {
    #[doc = "0: Slave sends a constant underrun pattern"]
    Constant = 0,
    #[doc = "1: Slave repeats last received data frame from master"]
    RepeatReceived = 1,
}
impl From<UDRCFG> for bool {
    #[inline(always)]
    fn from(variant: UDRCFG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDRCFG` reader - behavior of slave transmitter at underrun condition For more details see underrun condition."]
pub type UDRCFG_R = crate::BitReader<UDRCFG>;
impl UDRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDRCFG {
        match self.bits {
            false => UDRCFG::Constant,
            true => UDRCFG::RepeatReceived,
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
}
#[doc = "Field `UDRCFG` writer - behavior of slave transmitter at underrun condition For more details see underrun condition."]
pub type UDRCFG_W<'a, REG> = crate::BitWriter<'a, REG, UDRCFG>;
impl<'a, REG> UDRCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
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
#[doc = "Field `CRCSIZE` reader - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
pub type CRCSIZE_R = crate::FieldReader;
#[doc = "Field `CRCSIZE` writer - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
pub type CRCSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "hardware CRC computation enable\n\nValue on reset: 0"]
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
#[doc = "Field `CRCEN` reader - hardware CRC computation enable"]
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
#[doc = "Field `CRCEN` writer - hardware CRC computation enable"]
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
#[doc = "master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode).\n\nValue on reset: 0"]
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
#[doc = "Field `MBR` reader - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
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
#[doc = "Field `MBR` writer - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
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
#[doc = "bypass of the prescaler at master baud rate clock generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPASS {
    #[doc = "0: Bypass is disabled"]
    Disabled = 0,
    #[doc = "1: Bypass is enabled"]
    Enabled = 1,
}
impl From<BPASS> for bool {
    #[inline(always)]
    fn from(variant: BPASS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPASS` reader - bypass of the prescaler at master baud rate clock generator"]
pub type BPASS_R = crate::BitReader<BPASS>;
impl BPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BPASS {
        match self.bits {
            false => BPASS::Disabled,
            true => BPASS::Enabled,
        }
    }
    #[doc = "Bypass is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BPASS::Disabled
    }
    #[doc = "Bypass is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BPASS::Enabled
    }
}
#[doc = "Field `BPASS` writer - bypass of the prescaler at master baud rate clock generator"]
pub type BPASS_W<'a, REG> = crate::BitWriter<'a, REG, BPASS>;
impl<'a, REG> BPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BPASS::Disabled)
    }
    #[doc = "Bypass is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BPASS::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - behavior of slave transmitter at underrun condition For more details see underrun condition."]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bits 16:20 - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - bypass of the prescaler at master baud rate clock generator"]
    #[inline(always)]
    pub fn bpass(&self) -> BPASS_R {
        BPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\]
bits are reserved and must be kept at reset state. DSIZE\\[4:3\\]
bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<CFG1rs> {
        DSIZE_W::new(self, 0)
    }
    #[doc = "Bits 5:8 - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space. SPI interface is more efficient if configured packet sizes are aligned with data register access parallelism: If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bit, better to select FTHLV = 2, 4, 6. If SPI data register is accessed as a 32-bit register and DSIZE> 8 bit, better to select FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12. Note: FTHLV\\[3:2\\]
bits are reserved at instances with limited set of features"]
    #[inline(always)]
    #[must_use]
    pub fn fthlv(&mut self) -> FTHLV_W<CFG1rs> {
        FTHLV_W::new(self, 5)
    }
    #[doc = "Bit 9 - behavior of slave transmitter at underrun condition For more details see underrun condition."]
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<CFG1rs> {
        UDRCFG_W::new(self, 9)
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
    #[doc = "Bits 16:20 - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<CFG1rs> {
        CRCSIZE_W::new(self, 16)
    }
    #[doc = "Bit 22 - hardware CRC computation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<CFG1rs> {
        CRCEN_W::new(self, 22)
    }
    #[doc = "Bits 28:30 - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<CFG1rs> {
        MBR_W::new(self, 28)
    }
    #[doc = "Bit 31 - bypass of the prescaler at master baud rate clock generator"]
    #[inline(always)]
    #[must_use]
    pub fn bpass(&mut self) -> BPASS_W<CFG1rs> {
        BPASS_W::new(self, 31)
    }
}
#[doc = "SPI/I2S configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
