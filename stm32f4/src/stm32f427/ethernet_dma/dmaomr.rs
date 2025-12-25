///Register `DMAOMR` reader
pub type R = crate::R<DMAOMRrs>;
///Register `DMAOMR` writer
pub type W = crate::W<DMAOMRrs>;
/**Start/stop receive

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR {
    ///0: Reception is stopped after transfer of the current frame
    Stopped = 0,
    ///1: Reception is placed in the Running state
    Started = 1,
}
impl From<SR> for bool {
    #[inline(always)]
    fn from(variant: SR) -> Self {
        variant as u8 != 0
    }
}
///Field `SR` reader - Start/stop receive
pub type SR_R = crate::BitReader<SR>;
impl SR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SR {
        match self.bits {
            false => SR::Stopped,
            true => SR::Started,
        }
    }
    ///Reception is stopped after transfer of the current frame
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == SR::Stopped
    }
    ///Reception is placed in the Running state
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SR::Started
    }
}
///Field `SR` writer - Start/stop receive
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG, SR>;
impl<'a, REG> SR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reception is stopped after transfer of the current frame
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(SR::Stopped)
    }
    ///Reception is placed in the Running state
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(SR::Started)
    }
}
///Field `OSF` reader - Operate on second frame
pub type OSF_R = crate::BitReader;
///Field `OSF` writer - Operate on second frame
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Receive threshold control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC {
    ///0: 64 bytes
    Rtc64 = 0,
    ///1: 32 bytes
    Rtc32 = 1,
    ///2: 96 bytes
    Rtc96 = 2,
    ///3: 128 bytes
    Rtc128 = 3,
}
impl From<RTC> for u8 {
    #[inline(always)]
    fn from(variant: RTC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTC {
    type Ux = u8;
}
impl crate::IsEnum for RTC {}
///Field `RTC` reader - Receive threshold control
pub type RTC_R = crate::FieldReader<RTC>;
impl RTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTC {
        match self.bits {
            0 => RTC::Rtc64,
            1 => RTC::Rtc32,
            2 => RTC::Rtc96,
            3 => RTC::Rtc128,
            _ => unreachable!(),
        }
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_rtc64(&self) -> bool {
        *self == RTC::Rtc64
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_rtc32(&self) -> bool {
        *self == RTC::Rtc32
    }
    ///96 bytes
    #[inline(always)]
    pub fn is_rtc96(&self) -> bool {
        *self == RTC::Rtc96
    }
    ///128 bytes
    #[inline(always)]
    pub fn is_rtc128(&self) -> bool {
        *self == RTC::Rtc128
    }
}
///Field `RTC` writer - Receive threshold control
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTC, crate::Safe>;
impl<'a, REG> RTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///64 bytes
    #[inline(always)]
    pub fn rtc64(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc64)
    }
    ///32 bytes
    #[inline(always)]
    pub fn rtc32(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc32)
    }
    ///96 bytes
    #[inline(always)]
    pub fn rtc96(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc96)
    }
    ///128 bytes
    #[inline(always)]
    pub fn rtc128(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc128)
    }
}
/**Forward undersized good frames

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUGF {
    ///0: Rx FIFO drops all frames of less than 64 bytes
    Drop = 0,
    ///1: Rx FIFO forwards undersized frames
    Forward = 1,
}
impl From<FUGF> for bool {
    #[inline(always)]
    fn from(variant: FUGF) -> Self {
        variant as u8 != 0
    }
}
///Field `FUGF` reader - Forward undersized good frames
pub type FUGF_R = crate::BitReader<FUGF>;
impl FUGF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FUGF {
        match self.bits {
            false => FUGF::Drop,
            true => FUGF::Forward,
        }
    }
    ///Rx FIFO drops all frames of less than 64 bytes
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FUGF::Drop
    }
    ///Rx FIFO forwards undersized frames
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FUGF::Forward
    }
}
///Field `FUGF` writer - Forward undersized good frames
pub type FUGF_W<'a, REG> = crate::BitWriter<'a, REG, FUGF>;
impl<'a, REG> FUGF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx FIFO drops all frames of less than 64 bytes
    #[inline(always)]
    pub fn drop(self) -> &'a mut crate::W<REG> {
        self.variant(FUGF::Drop)
    }
    ///Rx FIFO forwards undersized frames
    #[inline(always)]
    pub fn forward(self) -> &'a mut crate::W<REG> {
        self.variant(FUGF::Forward)
    }
}
/**Forward error frames

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF {
    ///0: Rx FIFO drops frames with error status
    Drop = 0,
    ///1: All frames except runt error frames are forwarded to the DMA
    Forward = 1,
}
impl From<FEF> for bool {
    #[inline(always)]
    fn from(variant: FEF) -> Self {
        variant as u8 != 0
    }
}
///Field `FEF` reader - Forward error frames
pub type FEF_R = crate::BitReader<FEF>;
impl FEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEF {
        match self.bits {
            false => FEF::Drop,
            true => FEF::Forward,
        }
    }
    ///Rx FIFO drops frames with error status
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FEF::Drop
    }
    ///All frames except runt error frames are forwarded to the DMA
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FEF::Forward
    }
}
///Field `FEF` writer - Forward error frames
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG, FEF>;
impl<'a, REG> FEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx FIFO drops frames with error status
    #[inline(always)]
    pub fn drop(self) -> &'a mut crate::W<REG> {
        self.variant(FEF::Drop)
    }
    ///All frames except runt error frames are forwarded to the DMA
    #[inline(always)]
    pub fn forward(self) -> &'a mut crate::W<REG> {
        self.variant(FEF::Forward)
    }
}
/**Start/stop transmission

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST {
    ///0: Transmission is placed in the Stopped state
    Stopped = 0,
    ///1: Transmission is placed in Running state
    Started = 1,
}
impl From<ST> for bool {
    #[inline(always)]
    fn from(variant: ST) -> Self {
        variant as u8 != 0
    }
}
///Field `ST` reader - Start/stop transmission
pub type ST_R = crate::BitReader<ST>;
impl ST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ST {
        match self.bits {
            false => ST::Stopped,
            true => ST::Started,
        }
    }
    ///Transmission is placed in the Stopped state
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == ST::Stopped
    }
    ///Transmission is placed in Running state
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == ST::Started
    }
}
///Field `ST` writer - Start/stop transmission
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG, ST>;
impl<'a, REG> ST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission is placed in the Stopped state
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(ST::Stopped)
    }
    ///Transmission is placed in Running state
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(ST::Started)
    }
}
/**Transmit threshold control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTC {
    ///0: 64 bytes
    Ttc64 = 0,
    ///1: 128 bytes
    Ttc128 = 1,
    ///2: 192 bytes
    Ttc192 = 2,
    ///3: 256 bytes
    Ttc256 = 3,
    ///4: 40 bytes
    Ttc40 = 4,
    ///5: 32 bytes
    Ttc32 = 5,
    ///6: 24 bytes
    Ttc24 = 6,
    ///7: 16 bytes
    Ttc16 = 7,
}
impl From<TTC> for u8 {
    #[inline(always)]
    fn from(variant: TTC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TTC {
    type Ux = u8;
}
impl crate::IsEnum for TTC {}
///Field `TTC` reader - Transmit threshold control
pub type TTC_R = crate::FieldReader<TTC>;
impl TTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TTC {
        match self.bits {
            0 => TTC::Ttc64,
            1 => TTC::Ttc128,
            2 => TTC::Ttc192,
            3 => TTC::Ttc256,
            4 => TTC::Ttc40,
            5 => TTC::Ttc32,
            6 => TTC::Ttc24,
            7 => TTC::Ttc16,
            _ => unreachable!(),
        }
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_ttc64(&self) -> bool {
        *self == TTC::Ttc64
    }
    ///128 bytes
    #[inline(always)]
    pub fn is_ttc128(&self) -> bool {
        *self == TTC::Ttc128
    }
    ///192 bytes
    #[inline(always)]
    pub fn is_ttc192(&self) -> bool {
        *self == TTC::Ttc192
    }
    ///256 bytes
    #[inline(always)]
    pub fn is_ttc256(&self) -> bool {
        *self == TTC::Ttc256
    }
    ///40 bytes
    #[inline(always)]
    pub fn is_ttc40(&self) -> bool {
        *self == TTC::Ttc40
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_ttc32(&self) -> bool {
        *self == TTC::Ttc32
    }
    ///24 bytes
    #[inline(always)]
    pub fn is_ttc24(&self) -> bool {
        *self == TTC::Ttc24
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_ttc16(&self) -> bool {
        *self == TTC::Ttc16
    }
}
///Field `TTC` writer - Transmit threshold control
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TTC, crate::Safe>;
impl<'a, REG> TTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///64 bytes
    #[inline(always)]
    pub fn ttc64(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc64)
    }
    ///128 bytes
    #[inline(always)]
    pub fn ttc128(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc128)
    }
    ///192 bytes
    #[inline(always)]
    pub fn ttc192(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc192)
    }
    ///256 bytes
    #[inline(always)]
    pub fn ttc256(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc256)
    }
    ///40 bytes
    #[inline(always)]
    pub fn ttc40(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc40)
    }
    ///32 bytes
    #[inline(always)]
    pub fn ttc32(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc32)
    }
    ///24 bytes
    #[inline(always)]
    pub fn ttc24(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc24)
    }
    ///16 bytes
    #[inline(always)]
    pub fn ttc16(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc16)
    }
}
/**Flush transmit FIFO

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF {
    ///1: Transmit FIFO controller logic is reset to its default values. Cleared automatically
    Flush = 1,
}
impl From<FTF> for bool {
    #[inline(always)]
    fn from(variant: FTF) -> Self {
        variant as u8 != 0
    }
}
///Field `FTF` reader - Flush transmit FIFO
pub type FTF_R = crate::BitReader<FTF>;
impl FTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FTF> {
        match self.bits {
            true => Some(FTF::Flush),
            _ => None,
        }
    }
    ///Transmit FIFO controller logic is reset to its default values. Cleared automatically
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FTF::Flush
    }
}
///Field `FTF` writer - Flush transmit FIFO
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG, FTF>;
impl<'a, REG> FTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit FIFO controller logic is reset to its default values. Cleared automatically
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FTF::Flush)
    }
}
/**Transmit store and forward

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSF {
    ///0: Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold
    CutThrough = 0,
    ///1: Transmission starts when a full frame is in the Tx FIFO
    StoreForward = 1,
}
impl From<TSF> for bool {
    #[inline(always)]
    fn from(variant: TSF) -> Self {
        variant as u8 != 0
    }
}
///Field `TSF` reader - Transmit store and forward
pub type TSF_R = crate::BitReader<TSF>;
impl TSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSF {
        match self.bits {
            false => TSF::CutThrough,
            true => TSF::StoreForward,
        }
    }
    ///Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == TSF::CutThrough
    }
    ///Transmission starts when a full frame is in the Tx FIFO
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == TSF::StoreForward
    }
}
///Field `TSF` writer - Transmit store and forward
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG, TSF>;
impl<'a, REG> TSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut crate::W<REG> {
        self.variant(TSF::CutThrough)
    }
    ///Transmission starts when a full frame is in the Tx FIFO
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut crate::W<REG> {
        self.variant(TSF::StoreForward)
    }
}
///Field `DFRF` reader - Disable flushing of received frames
pub type DFRF_R = crate::BitReader;
///Field `DFRF` writer - Disable flushing of received frames
pub type DFRF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Receive store and forward

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSF {
    ///0: Rx FIFO operates in cut-through mode, subject to RTC bits
    CutThrough = 0,
    ///1: Frames are read from Rx FIFO after complete frame has been written
    StoreForward = 1,
}
impl From<RSF> for bool {
    #[inline(always)]
    fn from(variant: RSF) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` reader - Receive store and forward
pub type RSF_R = crate::BitReader<RSF>;
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSF {
        match self.bits {
            false => RSF::CutThrough,
            true => RSF::StoreForward,
        }
    }
    ///Rx FIFO operates in cut-through mode, subject to RTC bits
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == RSF::CutThrough
    }
    ///Frames are read from Rx FIFO after complete frame has been written
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == RSF::StoreForward
    }
}
///Field `RSF` writer - Receive store and forward
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG, RSF>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx FIFO operates in cut-through mode, subject to RTC bits
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut crate::W<REG> {
        self.variant(RSF::CutThrough)
    }
    ///Frames are read from Rx FIFO after complete frame has been written
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut crate::W<REG> {
        self.variant(RSF::StoreForward)
    }
}
/**Dropping of TCP/IP checksum error frames disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCEFD {
    ///0: Drop frames with errors only in the receive checksum offload engine
    Enabled = 0,
    ///1: Do not drop frames that only have errors in the receive checksum offload engine
    Disabled = 1,
}
impl From<DTCEFD> for bool {
    #[inline(always)]
    fn from(variant: DTCEFD) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCEFD` reader - Dropping of TCP/IP checksum error frames disable
pub type DTCEFD_R = crate::BitReader<DTCEFD>;
impl DTCEFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTCEFD {
        match self.bits {
            false => DTCEFD::Enabled,
            true => DTCEFD::Disabled,
        }
    }
    ///Drop frames with errors only in the receive checksum offload engine
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTCEFD::Enabled
    }
    ///Do not drop frames that only have errors in the receive checksum offload engine
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTCEFD::Disabled
    }
}
///Field `DTCEFD` writer - Dropping of TCP/IP checksum error frames disable
pub type DTCEFD_W<'a, REG> = crate::BitWriter<'a, REG, DTCEFD>;
impl<'a, REG> DTCEFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Drop frames with errors only in the receive checksum offload engine
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTCEFD::Enabled)
    }
    ///Do not drop frames that only have errors in the receive checksum offload engine
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTCEFD::Disabled)
    }
}
impl R {
    ///Bit 1 - Start/stop receive
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Operate on second frame
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Receive threshold control
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 6 - Forward undersized good frames
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Forward error frames
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - Start/stop transmission
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:16 - Transmit threshold control
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bit 20 - Flush transmit FIFO
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Transmit store and forward
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Disable flushing of received frames
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Receive store and forward
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Dropping of TCP/IP checksum error frames disable
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAOMR")
            .field("sr", &self.sr())
            .field("osf", &self.osf())
            .field("rtc", &self.rtc())
            .field("fugf", &self.fugf())
            .field("fef", &self.fef())
            .field("st", &self.st())
            .field("ttc", &self.ttc())
            .field("ftf", &self.ftf())
            .field("tsf", &self.tsf())
            .field("dfrf", &self.dfrf())
            .field("rsf", &self.rsf())
            .field("dtcefd", &self.dtcefd())
            .finish()
    }
}
impl W {
    ///Bit 1 - Start/stop receive
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<'_, DMAOMRrs> {
        SR_W::new(self, 1)
    }
    ///Bit 2 - Operate on second frame
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<'_, DMAOMRrs> {
        OSF_W::new(self, 2)
    }
    ///Bits 3:4 - Receive threshold control
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<'_, DMAOMRrs> {
        RTC_W::new(self, 3)
    }
    ///Bit 6 - Forward undersized good frames
    #[inline(always)]
    pub fn fugf(&mut self) -> FUGF_W<'_, DMAOMRrs> {
        FUGF_W::new(self, 6)
    }
    ///Bit 7 - Forward error frames
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<'_, DMAOMRrs> {
        FEF_W::new(self, 7)
    }
    ///Bit 13 - Start/stop transmission
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<'_, DMAOMRrs> {
        ST_W::new(self, 13)
    }
    ///Bits 14:16 - Transmit threshold control
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<'_, DMAOMRrs> {
        TTC_W::new(self, 14)
    }
    ///Bit 20 - Flush transmit FIFO
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W<'_, DMAOMRrs> {
        FTF_W::new(self, 20)
    }
    ///Bit 21 - Transmit store and forward
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, DMAOMRrs> {
        TSF_W::new(self, 21)
    }
    ///Bit 24 - Disable flushing of received frames
    #[inline(always)]
    pub fn dfrf(&mut self) -> DFRF_W<'_, DMAOMRrs> {
        DFRF_W::new(self, 24)
    }
    ///Bit 25 - Receive store and forward
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, DMAOMRrs> {
        RSF_W::new(self, 25)
    }
    ///Bit 26 - Dropping of TCP/IP checksum error frames disable
    #[inline(always)]
    pub fn dtcefd(&mut self) -> DTCEFD_W<'_, DMAOMRrs> {
        DTCEFD_W::new(self, 26)
    }
}
/**Ethernet DMA operation mode register

You can [`read`](crate::Reg::read) this register and get [`dmaomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#Ethernet_DMA:DMAOMR)*/
pub struct DMAOMRrs;
impl crate::RegisterSpec for DMAOMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaomr::R`](R) reader structure
impl crate::Readable for DMAOMRrs {}
///`write(|w| ..)` method takes [`dmaomr::W`](W) writer structure
impl crate::Writable for DMAOMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAOMR to value 0
impl crate::Resettable for DMAOMRrs {}
