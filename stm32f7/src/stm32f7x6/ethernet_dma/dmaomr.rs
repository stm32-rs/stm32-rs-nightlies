#[doc = "Register `DMAOMR` reader"]
pub type R = crate::R<DMAOMRrs>;
#[doc = "Register `DMAOMR` writer"]
pub type W = crate::W<DMAOMRrs>;
#[doc = "Start/stop receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR {
    #[doc = "0: Reception is stopped after transfer of the current frame"]
    Stopped = 0,
    #[doc = "1: Reception is placed in the Running state"]
    Started = 1,
}
impl From<SR> for bool {
    #[inline(always)]
    fn from(variant: SR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Start/stop receive"]
pub type SR_R = crate::BitReader<SR>;
impl SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SR {
        match self.bits {
            false => SR::Stopped,
            true => SR::Started,
        }
    }
    #[doc = "Reception is stopped after transfer of the current frame"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == SR::Stopped
    }
    #[doc = "Reception is placed in the Running state"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SR::Started
    }
}
#[doc = "Field `SR` writer - Start/stop receive"]
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG, SR>;
impl<'a, REG> SR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reception is stopped after transfer of the current frame"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(SR::Stopped)
    }
    #[doc = "Reception is placed in the Running state"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(SR::Started)
    }
}
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive threshold control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTC {
    #[doc = "0: 64 bytes"]
    Rtc64 = 0,
    #[doc = "1: 32 bytes"]
    Rtc32 = 1,
    #[doc = "2: 96 bytes"]
    Rtc96 = 2,
    #[doc = "3: 128 bytes"]
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
#[doc = "Field `RTC` reader - Receive threshold control"]
pub type RTC_R = crate::FieldReader<RTC>;
impl RTC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_rtc64(&self) -> bool {
        *self == RTC::Rtc64
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_rtc32(&self) -> bool {
        *self == RTC::Rtc32
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn is_rtc96(&self) -> bool {
        *self == RTC::Rtc96
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_rtc128(&self) -> bool {
        *self == RTC::Rtc128
    }
}
#[doc = "Field `RTC` writer - Receive threshold control"]
pub type RTC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RTC>;
impl<'a, REG> RTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn rtc64(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc64)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn rtc32(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc32)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn rtc96(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc96)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn rtc128(self) -> &'a mut crate::W<REG> {
        self.variant(RTC::Rtc128)
    }
}
#[doc = "Forward undersized good frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUGF {
    #[doc = "0: Rx FIFO drops all frames of less than 64 bytes"]
    Drop = 0,
    #[doc = "1: Rx FIFO forwards undersized frames"]
    Forward = 1,
}
impl From<FUGF> for bool {
    #[inline(always)]
    fn from(variant: FUGF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUGF` reader - Forward undersized good frames"]
pub type FUGF_R = crate::BitReader<FUGF>;
impl FUGF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FUGF {
        match self.bits {
            false => FUGF::Drop,
            true => FUGF::Forward,
        }
    }
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FUGF::Drop
    }
    #[doc = "Rx FIFO forwards undersized frames"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FUGF::Forward
    }
}
#[doc = "Field `FUGF` writer - Forward undersized good frames"]
pub type FUGF_W<'a, REG> = crate::BitWriter<'a, REG, FUGF>;
impl<'a, REG> FUGF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut crate::W<REG> {
        self.variant(FUGF::Drop)
    }
    #[doc = "Rx FIFO forwards undersized frames"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut crate::W<REG> {
        self.variant(FUGF::Forward)
    }
}
#[doc = "Forward error frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF {
    #[doc = "0: Rx FIFO drops frames with error status"]
    Drop = 0,
    #[doc = "1: All frames except runt error frames are forwarded to the DMA"]
    Forward = 1,
}
impl From<FEF> for bool {
    #[inline(always)]
    fn from(variant: FEF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - Forward error frames"]
pub type FEF_R = crate::BitReader<FEF>;
impl FEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEF {
        match self.bits {
            false => FEF::Drop,
            true => FEF::Forward,
        }
    }
    #[doc = "Rx FIFO drops frames with error status"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FEF::Drop
    }
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FEF::Forward
    }
}
#[doc = "Field `FEF` writer - Forward error frames"]
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG, FEF>;
impl<'a, REG> FEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO drops frames with error status"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut crate::W<REG> {
        self.variant(FEF::Drop)
    }
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut crate::W<REG> {
        self.variant(FEF::Forward)
    }
}
#[doc = "Start/stop transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST {
    #[doc = "0: Transmission is placed in the Stopped state"]
    Stopped = 0,
    #[doc = "1: Transmission is placed in Running state"]
    Started = 1,
}
impl From<ST> for bool {
    #[inline(always)]
    fn from(variant: ST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST` reader - Start/stop transmission"]
pub type ST_R = crate::BitReader<ST>;
impl ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ST {
        match self.bits {
            false => ST::Stopped,
            true => ST::Started,
        }
    }
    #[doc = "Transmission is placed in the Stopped state"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == ST::Stopped
    }
    #[doc = "Transmission is placed in Running state"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == ST::Started
    }
}
#[doc = "Field `ST` writer - Start/stop transmission"]
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG, ST>;
impl<'a, REG> ST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission is placed in the Stopped state"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(ST::Stopped)
    }
    #[doc = "Transmission is placed in Running state"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(ST::Started)
    }
}
#[doc = "Transmit threshold control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTC {
    #[doc = "0: 64 bytes"]
    Ttc64 = 0,
    #[doc = "1: 128 bytes"]
    Ttc128 = 1,
    #[doc = "2: 192 bytes"]
    Ttc192 = 2,
    #[doc = "3: 256 bytes"]
    Ttc256 = 3,
    #[doc = "4: 40 bytes"]
    Ttc40 = 4,
    #[doc = "5: 32 bytes"]
    Ttc32 = 5,
    #[doc = "6: 24 bytes"]
    Ttc24 = 6,
    #[doc = "7: 16 bytes"]
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
#[doc = "Field `TTC` reader - Transmit threshold control"]
pub type TTC_R = crate::FieldReader<TTC>;
impl TTC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_ttc64(&self) -> bool {
        *self == TTC::Ttc64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_ttc128(&self) -> bool {
        *self == TTC::Ttc128
    }
    #[doc = "192 bytes"]
    #[inline(always)]
    pub fn is_ttc192(&self) -> bool {
        *self == TTC::Ttc192
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_ttc256(&self) -> bool {
        *self == TTC::Ttc256
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn is_ttc40(&self) -> bool {
        *self == TTC::Ttc40
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_ttc32(&self) -> bool {
        *self == TTC::Ttc32
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn is_ttc24(&self) -> bool {
        *self == TTC::Ttc24
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_ttc16(&self) -> bool {
        *self == TTC::Ttc16
    }
}
#[doc = "Field `TTC` writer - Transmit threshold control"]
pub type TTC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TTC>;
impl<'a, REG> TTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn ttc64(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn ttc128(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc128)
    }
    #[doc = "192 bytes"]
    #[inline(always)]
    pub fn ttc192(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc192)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn ttc256(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc256)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn ttc40(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc40)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn ttc32(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc32)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn ttc24(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc24)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn ttc16(self) -> &'a mut crate::W<REG> {
        self.variant(TTC::Ttc16)
    }
}
#[doc = "Flush transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF {
    #[doc = "1: Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    Flush = 1,
}
impl From<FTF> for bool {
    #[inline(always)]
    fn from(variant: FTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FTF_R = crate::BitReader<FTF>;
impl FTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FTF> {
        match self.bits {
            true => Some(FTF::Flush),
            _ => None,
        }
    }
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FTF::Flush
    }
}
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG, FTF>;
impl<'a, REG> FTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FTF::Flush)
    }
}
#[doc = "Transmit store and forward\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSF {
    #[doc = "0: Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    CutThrough = 0,
    #[doc = "1: Transmission starts when a full frame is in the Tx FIFO"]
    StoreForward = 1,
}
impl From<TSF> for bool {
    #[inline(always)]
    fn from(variant: TSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Transmit store and forward"]
pub type TSF_R = crate::BitReader<TSF>;
impl TSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSF {
        match self.bits {
            false => TSF::CutThrough,
            true => TSF::StoreForward,
        }
    }
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == TSF::CutThrough
    }
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == TSF::StoreForward
    }
}
#[doc = "Field `TSF` writer - Transmit store and forward"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG, TSF>;
impl<'a, REG> TSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut crate::W<REG> {
        self.variant(TSF::CutThrough)
    }
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut crate::W<REG> {
        self.variant(TSF::StoreForward)
    }
}
#[doc = "Field `DFRF` reader - Disable flushing of received frames"]
pub type DFRF_R = crate::BitReader;
#[doc = "Field `DFRF` writer - Disable flushing of received frames"]
pub type DFRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive store and forward\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSF {
    #[doc = "0: Rx FIFO operates in cut-through mode, subject to RTC bits"]
    CutThrough = 0,
    #[doc = "1: Frames are read from Rx FIFO after complete frame has been written"]
    StoreForward = 1,
}
impl From<RSF> for bool {
    #[inline(always)]
    fn from(variant: RSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Receive store and forward"]
pub type RSF_R = crate::BitReader<RSF>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSF {
        match self.bits {
            false => RSF::CutThrough,
            true => RSF::StoreForward,
        }
    }
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == RSF::CutThrough
    }
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == RSF::StoreForward
    }
}
#[doc = "Field `RSF` writer - Receive store and forward"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG, RSF>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut crate::W<REG> {
        self.variant(RSF::CutThrough)
    }
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut crate::W<REG> {
        self.variant(RSF::StoreForward)
    }
}
#[doc = "Dropping of TCP/IP checksum error frames disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCEFD {
    #[doc = "0: Drop frames with errors only in the receive checksum offload engine"]
    Enabled = 0,
    #[doc = "1: Do not drop frames that only have errors in the receive checksum offload engine"]
    Disabled = 1,
}
impl From<DTCEFD> for bool {
    #[inline(always)]
    fn from(variant: DTCEFD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCEFD` reader - Dropping of TCP/IP checksum error frames disable"]
pub type DTCEFD_R = crate::BitReader<DTCEFD>;
impl DTCEFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTCEFD {
        match self.bits {
            false => DTCEFD::Enabled,
            true => DTCEFD::Disabled,
        }
    }
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTCEFD::Enabled
    }
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTCEFD::Disabled
    }
}
#[doc = "Field `DTCEFD` writer - Dropping of TCP/IP checksum error frames disable"]
pub type DTCEFD_W<'a, REG> = crate::BitWriter<'a, REG, DTCEFD>;
impl<'a, REG> DTCEFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTCEFD::Enabled)
    }
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTCEFD::Disabled)
    }
}
impl R {
    #[doc = "Bit 1 - Start/stop receive"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/stop receive"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<DMAOMRrs> {
        SR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMAOMRrs> {
        OSF_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<DMAOMRrs> {
        RTC_W::new(self, 3)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    #[must_use]
    pub fn fugf(&mut self) -> FUGF_W<DMAOMRrs> {
        FUGF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<DMAOMRrs> {
        FEF_W::new(self, 7)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<DMAOMRrs> {
        ST_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<DMAOMRrs> {
        TTC_W::new(self, 14)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<DMAOMRrs> {
        FTF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<DMAOMRrs> {
        TSF_W::new(self, 21)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    #[must_use]
    pub fn dfrf(&mut self) -> DFRF_W<DMAOMRrs> {
        DFRF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<DMAOMRrs> {
        RSF_W::new(self, 25)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtcefd(&mut self) -> DTCEFD_W<DMAOMRrs> {
        DTCEFD_W::new(self, 26)
    }
}
#[doc = "Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAOMRrs;
impl crate::RegisterSpec for DMAOMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaomr::R`](R) reader structure"]
impl crate::Readable for DMAOMRrs {}
#[doc = "`write(|w| ..)` method takes [`dmaomr::W`](W) writer structure"]
impl crate::Writable for DMAOMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAOMR to value 0"]
impl crate::Resettable for DMAOMRrs {
    const RESET_VALUE: u32 = 0;
}
