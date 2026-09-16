///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**PE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    ///0: No parity error
    NoError = 0,
    ///1: Parity error
    Error = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - PE
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::NoError,
            true => PE::Error,
        }
    }
    ///No parity error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PE::NoError
    }
    ///Parity error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PE::Error
    }
}
/**FE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE {
    ///0: No Framing error is detected
    NoError = 0,
    ///1: Framing error or break character is detected
    Error = 1,
}
impl From<FE> for bool {
    #[inline(always)]
    fn from(variant: FE) -> Self {
        variant as u8 != 0
    }
}
///Field `FE` reader - FE
pub type FE_R = crate::BitReader<FE>;
impl FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FE {
        match self.bits {
            false => FE::NoError,
            true => FE::Error,
        }
    }
    ///No Framing error is detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FE::NoError
    }
    ///Framing error or break character is detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FE::Error
    }
}
/**NF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NF {
    ///0: No noise is detected
    NoNoise = 0,
    ///1: Noise is detected
    Noise = 1,
}
impl From<NF> for bool {
    #[inline(always)]
    fn from(variant: NF) -> Self {
        variant as u8 != 0
    }
}
///Field `NF` reader - NF
pub type NF_R = crate::BitReader<NF>;
impl NF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NF {
        match self.bits {
            false => NF::NoNoise,
            true => NF::Noise,
        }
    }
    ///No noise is detected
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == NF::NoNoise
    }
    ///Noise is detected
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NF::Noise
    }
}
/**ORE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORE {
    ///0: No Overrun error
    NoOverrun = 0,
    ///1: Overrun error is detected
    Overrun = 1,
}
impl From<ORE> for bool {
    #[inline(always)]
    fn from(variant: ORE) -> Self {
        variant as u8 != 0
    }
}
///Field `ORE` reader - ORE
pub type ORE_R = crate::BitReader<ORE>;
impl ORE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORE {
        match self.bits {
            false => ORE::NoOverrun,
            true => ORE::Overrun,
        }
    }
    ///No Overrun error
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == ORE::NoOverrun
    }
    ///Overrun error is detected
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == ORE::Overrun
    }
}
/**IDLE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE {
    ///0: No Idle Line is detected
    NoIdle = 0,
    ///1: Idle Line is detected
    Idle = 1,
}
impl From<IDLE> for bool {
    #[inline(always)]
    fn from(variant: IDLE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLE` reader - IDLE
pub type IDLE_R = crate::BitReader<IDLE>;
impl IDLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLE {
        match self.bits {
            false => IDLE::NoIdle,
            true => IDLE::Idle,
        }
    }
    ///No Idle Line is detected
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == IDLE::NoIdle
    }
    ///Idle Line is detected
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLE::Idle
    }
}
/**RXNE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    ///0: Data is not received
    NoData = 0,
    ///1: Received data is ready to be read
    DataReady = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - RXNE
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::NoData,
            true => RXNE::DataReady,
        }
    }
    ///Data is not received
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == RXNE::NoData
    }
    ///Received data is ready to be read
    #[inline(always)]
    pub fn is_data_ready(&self) -> bool {
        *self == RXNE::DataReady
    }
}
/**TC

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC {
    ///0: Transmission is not complete
    TxNotComplete = 0,
    ///1: Transmission is complete
    TxComplete = 1,
}
impl From<TC> for bool {
    #[inline(always)]
    fn from(variant: TC) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - TC
pub type TC_R = crate::BitReader<TC>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TC {
        match self.bits {
            false => TC::TxNotComplete,
            true => TC::TxComplete,
        }
    }
    ///Transmission is not complete
    #[inline(always)]
    pub fn is_tx_not_complete(&self) -> bool {
        *self == TC::TxNotComplete
    }
    ///Transmission is complete
    #[inline(always)]
    pub fn is_tx_complete(&self) -> bool {
        *self == TC::TxComplete
    }
}
/**TXE

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE {
    ///0: Transmit FIFO is full
    Full = 0,
    ///1: Transmit FIFO is not full
    NotFull = 1,
}
impl From<TXE> for bool {
    #[inline(always)]
    fn from(variant: TXE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - TXE
pub type TXE_R = crate::BitReader<TXE>;
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXE {
        match self.bits {
            false => TXE::Full,
            true => TXE::NotFull,
        }
    }
    ///Transmit FIFO is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXE::Full
    }
    ///Transmit FIFO is not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXE::NotFull
    }
}
/**LBDF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDF {
    ///0: LIN break not detected
    NotDetected = 0,
    ///1: LIN break detected
    Detected = 1,
}
impl From<LBDF> for bool {
    #[inline(always)]
    fn from(variant: LBDF) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDF` reader - LBDF
pub type LBDF_R = crate::BitReader<LBDF>;
impl LBDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDF {
        match self.bits {
            false => LBDF::NotDetected,
            true => LBDF::Detected,
        }
    }
    ///LIN break not detected
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LBDF::NotDetected
    }
    ///LIN break detected
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LBDF::Detected
    }
}
/**CTSIF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIF {
    ///0: No change occurred on the CTS status line
    NotChanged = 0,
    ///1: A change occurred on the CTS status line
    Changed = 1,
}
impl From<CTSIF> for bool {
    #[inline(always)]
    fn from(variant: CTSIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIF` reader - CTSIF
pub type CTSIF_R = crate::BitReader<CTSIF>;
impl CTSIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSIF {
        match self.bits {
            false => CTSIF::NotChanged,
            true => CTSIF::Changed,
        }
    }
    ///No change occurred on the CTS status line
    #[inline(always)]
    pub fn is_not_changed(&self) -> bool {
        *self == CTSIF::NotChanged
    }
    ///A change occurred on the CTS status line
    #[inline(always)]
    pub fn is_changed(&self) -> bool {
        *self == CTSIF::Changed
    }
}
/**CTS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTS {
    ///0: CTS line set
    Set = 0,
    ///1: CTS line reset
    Reset = 1,
}
impl From<CTS> for bool {
    #[inline(always)]
    fn from(variant: CTS) -> Self {
        variant as u8 != 0
    }
}
///Field `CTS` reader - CTS
pub type CTS_R = crate::BitReader<CTS>;
impl CTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTS {
        match self.bits {
            false => CTS::Set,
            true => CTS::Reset,
        }
    }
    ///CTS line set
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CTS::Set
    }
    ///CTS line reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CTS::Reset
    }
}
/**RTOF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOF {
    ///0: Timeout value not reached
    NotReached = 0,
    ///1: Timeout value reached without any data reception
    Reached = 1,
}
impl From<RTOF> for bool {
    #[inline(always)]
    fn from(variant: RTOF) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOF` reader - RTOF
pub type RTOF_R = crate::BitReader<RTOF>;
impl RTOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOF {
        match self.bits {
            false => RTOF::NotReached,
            true => RTOF::Reached,
        }
    }
    ///Timeout value not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == RTOF::NotReached
    }
    ///Timeout value reached without any data reception
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == RTOF::Reached
    }
}
/**EOBF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBF {
    ///0: End of Block not reached
    NotReached = 0,
    ///1: End of Block (number of characters) reached
    Reached = 1,
}
impl From<EOBF> for bool {
    #[inline(always)]
    fn from(variant: EOBF) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBF` reader - EOBF
pub type EOBF_R = crate::BitReader<EOBF>;
impl EOBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOBF {
        match self.bits {
            false => EOBF::NotReached,
            true => EOBF::Reached,
        }
    }
    ///End of Block not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == EOBF::NotReached
    }
    ///End of Block (number of characters) reached
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == EOBF::Reached
    }
}
///Field `ABRE` reader - ABRE
pub type ABRE_R = crate::BitReader;
///Field `ABRF` reader - ABRF
pub type ABRF_R = crate::BitReader;
/**BUSY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: USART is idle (no reception)
    Idle = 0,
    ///1: Reception on going
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::Idle,
            true => BUSY::Busy,
        }
    }
    ///USART is idle (no reception)
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY::Idle
    }
    ///Reception on going
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
/**CMF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMF {
    ///0: No Character match detected
    NoMatch = 0,
    ///1: Character match detected
    Match = 1,
}
impl From<CMF> for bool {
    #[inline(always)]
    fn from(variant: CMF) -> Self {
        variant as u8 != 0
    }
}
///Field `CMF` reader - CMF
pub type CMF_R = crate::BitReader<CMF>;
impl CMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMF {
        match self.bits {
            false => CMF::NoMatch,
            true => CMF::Match,
        }
    }
    ///No Character match detected
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CMF::NoMatch
    }
    ///Character match detected
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CMF::Match
    }
}
/**SBKF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKF {
    ///0: No break character transmitted
    NoBreak = 0,
    ///1: Break character transmitted
    Break = 1,
}
impl From<SBKF> for bool {
    #[inline(always)]
    fn from(variant: SBKF) -> Self {
        variant as u8 != 0
    }
}
///Field `SBKF` reader - SBKF
pub type SBKF_R = crate::BitReader<SBKF>;
impl SBKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBKF {
        match self.bits {
            false => SBKF::NoBreak,
            true => SBKF::Break,
        }
    }
    ///No break character transmitted
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == SBKF::NoBreak
    }
    ///Break character transmitted
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBKF::Break
    }
}
/**RWU

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU {
    ///0: Receiver in Active mode
    Active = 0,
    ///1: Receiver in Mute mode
    Mute = 1,
}
impl From<RWU> for bool {
    #[inline(always)]
    fn from(variant: RWU) -> Self {
        variant as u8 != 0
    }
}
///Field `RWU` reader - RWU
pub type RWU_R = crate::BitReader<RWU>;
impl RWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWU {
        match self.bits {
            false => RWU::Active,
            true => RWU::Mute,
        }
    }
    ///Receiver in Active mode
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RWU::Active
    }
    ///Receiver in Mute mode
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == RWU::Mute
    }
}
///Field `WUF` reader - WUF
pub type WUF_R = crate::BitReader;
///Field `TEACK` reader - TEACK
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - REACK
pub type REACK_R = crate::BitReader;
impl R {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FE
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NF
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ORE
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXE
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LBDF
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTSIF
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RTOF
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EOBF
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - ABRE
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ABRF
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMF
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SBKF
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RWU
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - WUF
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TEACK
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - REACK
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("reack", &self.reack())
            .field("teack", &self.teack())
            .field("wuf", &self.wuf())
            .field("rwu", &self.rwu())
            .field("sbkf", &self.sbkf())
            .field("cmf", &self.cmf())
            .field("busy", &self.busy())
            .field("abrf", &self.abrf())
            .field("abre", &self.abre())
            .field("eobf", &self.eobf())
            .field("rtof", &self.rtof())
            .field("cts", &self.cts())
            .field("ctsif", &self.ctsif())
            .field("lbdf", &self.lbdf())
            .field("txe", &self.txe())
            .field("tc", &self.tc())
            .field("rxne", &self.rxne())
            .field("idle", &self.idle())
            .field("ore", &self.ore())
            .field("nf", &self.nf())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .finish()
    }
}
/**Interrupt & status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#USART1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0xc0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0xc0;
}
