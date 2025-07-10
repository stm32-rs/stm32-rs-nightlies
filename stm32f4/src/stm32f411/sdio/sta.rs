///Register `STA` reader
pub type R = crate::R<STArs>;
/**Command response received (CRC check failed)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRCFAIL {
    ///0: Command response received, crc check passed
    NotFailed = 0,
    ///1: Command response received, crc check failed
    Failed = 1,
}
impl From<CCRCFAIL> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAIL) -> Self {
        variant as u8 != 0
    }
}
///Field `CCRCFAIL` reader - Command response received (CRC check failed)
pub type CCRCFAIL_R = crate::BitReader<CCRCFAIL>;
impl CCRCFAIL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCRCFAIL {
        match self.bits {
            false => CCRCFAIL::NotFailed,
            true => CCRCFAIL::Failed,
        }
    }
    ///Command response received, crc check passed
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == CCRCFAIL::NotFailed
    }
    ///Command response received, crc check failed
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == CCRCFAIL::Failed
    }
}
/**Data block sent/received (CRC check failed)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRCFAIL {
    ///0: No Data block sent/received crc check fail
    NotFailed = 0,
    ///1: Data block sent/received crc failed
    Failed = 1,
}
impl From<DCRCFAIL> for bool {
    #[inline(always)]
    fn from(variant: DCRCFAIL) -> Self {
        variant as u8 != 0
    }
}
///Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)
pub type DCRCFAIL_R = crate::BitReader<DCRCFAIL>;
impl DCRCFAIL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCRCFAIL {
        match self.bits {
            false => DCRCFAIL::NotFailed,
            true => DCRCFAIL::Failed,
        }
    }
    ///No Data block sent/received crc check fail
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == DCRCFAIL::NotFailed
    }
    ///Data block sent/received crc failed
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == DCRCFAIL::Failed
    }
}
/**Command response timeout

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMEOUT {
    ///0: No Command timeout
    NoTimeout = 0,
    ///1: Command timeout
    Timeout = 1,
}
impl From<CTIMEOUT> for bool {
    #[inline(always)]
    fn from(variant: CTIMEOUT) -> Self {
        variant as u8 != 0
    }
}
///Field `CTIMEOUT` reader - Command response timeout
pub type CTIMEOUT_R = crate::BitReader<CTIMEOUT>;
impl CTIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTIMEOUT {
        match self.bits {
            false => CTIMEOUT::NoTimeout,
            true => CTIMEOUT::Timeout,
        }
    }
    ///No Command timeout
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == CTIMEOUT::NoTimeout
    }
    ///Command timeout
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == CTIMEOUT::Timeout
    }
}
/**Data timeout

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTIMEOUT {
    ///0: No data timeout
    NoTimeout = 0,
    ///1: Data timeout
    Timeout = 1,
}
impl From<DTIMEOUT> for bool {
    #[inline(always)]
    fn from(variant: DTIMEOUT) -> Self {
        variant as u8 != 0
    }
}
///Field `DTIMEOUT` reader - Data timeout
pub type DTIMEOUT_R = crate::BitReader<DTIMEOUT>;
impl DTIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTIMEOUT {
        match self.bits {
            false => DTIMEOUT::NoTimeout,
            true => DTIMEOUT::Timeout,
        }
    }
    ///No data timeout
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == DTIMEOUT::NoTimeout
    }
    ///Data timeout
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == DTIMEOUT::Timeout
    }
}
/**Transmit FIFO underrun error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXUNDERR {
    ///0: No transmit FIFO underrun error
    NoUnderrun = 0,
    ///1: Transmit FIFO underrun error
    Underrun = 1,
}
impl From<TXUNDERR> for bool {
    #[inline(always)]
    fn from(variant: TXUNDERR) -> Self {
        variant as u8 != 0
    }
}
///Field `TXUNDERR` reader - Transmit FIFO underrun error
pub type TXUNDERR_R = crate::BitReader<TXUNDERR>;
impl TXUNDERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXUNDERR {
        match self.bits {
            false => TXUNDERR::NoUnderrun,
            true => TXUNDERR::Underrun,
        }
    }
    ///No transmit FIFO underrun error
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXUNDERR::NoUnderrun
    }
    ///Transmit FIFO underrun error
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXUNDERR::Underrun
    }
}
/**Received FIFO overrun error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOVERR {
    ///0: No FIFO overrun error
    NoOverrun = 0,
    ///1: Receive FIFO overrun error
    Overrun = 1,
}
impl From<RXOVERR> for bool {
    #[inline(always)]
    fn from(variant: RXOVERR) -> Self {
        variant as u8 != 0
    }
}
///Field `RXOVERR` reader - Received FIFO overrun error
pub type RXOVERR_R = crate::BitReader<RXOVERR>;
impl RXOVERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXOVERR {
        match self.bits {
            false => RXOVERR::NoOverrun,
            true => RXOVERR::Overrun,
        }
    }
    ///No FIFO overrun error
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == RXOVERR::NoOverrun
    }
    ///Receive FIFO overrun error
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == RXOVERR::Overrun
    }
}
/**Command response received (CRC check passed)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDREND {
    ///0: Command not done
    NotDone = 0,
    ///1: Command response received (CRC check passed)
    Done = 1,
}
impl From<CMDREND> for bool {
    #[inline(always)]
    fn from(variant: CMDREND) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDREND` reader - Command response received (CRC check passed)
pub type CMDREND_R = crate::BitReader<CMDREND>;
impl CMDREND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDREND {
        match self.bits {
            false => CMDREND::NotDone,
            true => CMDREND::Done,
        }
    }
    ///Command not done
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == CMDREND::NotDone
    }
    ///Command response received (CRC check passed)
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == CMDREND::Done
    }
}
/**Command sent (no response required)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDSENT {
    ///0: Command not sent
    NotSent = 0,
    ///1: Command sent (no response required)
    Sent = 1,
}
impl From<CMDSENT> for bool {
    #[inline(always)]
    fn from(variant: CMDSENT) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDSENT` reader - Command sent (no response required)
pub type CMDSENT_R = crate::BitReader<CMDSENT>;
impl CMDSENT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDSENT {
        match self.bits {
            false => CMDSENT::NotSent,
            true => CMDSENT::Sent,
        }
    }
    ///Command not sent
    #[inline(always)]
    pub fn is_not_sent(&self) -> bool {
        *self == CMDSENT::NotSent
    }
    ///Command sent (no response required)
    #[inline(always)]
    pub fn is_sent(&self) -> bool {
        *self == CMDSENT::Sent
    }
}
/**Data end (data counter, SDIDCOUNT, is zero)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAEND {
    ///0: Not done
    NotDone = 0,
    ///1: Data end (DCOUNT, is zero)
    Done = 1,
}
impl From<DATAEND> for bool {
    #[inline(always)]
    fn from(variant: DATAEND) -> Self {
        variant as u8 != 0
    }
}
///Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)
pub type DATAEND_R = crate::BitReader<DATAEND>;
impl DATAEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATAEND {
        match self.bits {
            false => DATAEND::NotDone,
            true => DATAEND::Done,
        }
    }
    ///Not done
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == DATAEND::NotDone
    }
    ///Data end (DCOUNT, is zero)
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == DATAEND::Done
    }
}
/**Start bit not detected on all data signals in wide bus mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBITERR {
    ///0: No start bit detected error
    Detected = 0,
    ///1: Start bit not detected error
    NotDetected = 1,
}
impl From<STBITERR> for bool {
    #[inline(always)]
    fn from(variant: STBITERR) -> Self {
        variant as u8 != 0
    }
}
///Field `STBITERR` reader - Start bit not detected on all data signals in wide bus mode
pub type STBITERR_R = crate::BitReader<STBITERR>;
impl STBITERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STBITERR {
        match self.bits {
            false => STBITERR::Detected,
            true => STBITERR::NotDetected,
        }
    }
    ///No start bit detected error
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == STBITERR::Detected
    }
    ///Start bit not detected error
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == STBITERR::NotDetected
    }
}
/**Data block sent/received (CRC check passed)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBCKEND {
    ///0: Data block not sent/received (CRC check failed)
    NotTransferred = 0,
    ///1: Data block sent/received (CRC check passed)
    Transferred = 1,
}
impl From<DBCKEND> for bool {
    #[inline(always)]
    fn from(variant: DBCKEND) -> Self {
        variant as u8 != 0
    }
}
///Field `DBCKEND` reader - Data block sent/received (CRC check passed)
pub type DBCKEND_R = crate::BitReader<DBCKEND>;
impl DBCKEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBCKEND {
        match self.bits {
            false => DBCKEND::NotTransferred,
            true => DBCKEND::Transferred,
        }
    }
    ///Data block not sent/received (CRC check failed)
    #[inline(always)]
    pub fn is_not_transferred(&self) -> bool {
        *self == DBCKEND::NotTransferred
    }
    ///Data block sent/received (CRC check passed)
    #[inline(always)]
    pub fn is_transferred(&self) -> bool {
        *self == DBCKEND::Transferred
    }
}
/**Command transfer in progress

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDACT {
    ///0: Command transfer not in progress
    NotInProgress = 0,
    ///1: Command tranfer in progress
    InProgress = 1,
}
impl From<CMDACT> for bool {
    #[inline(always)]
    fn from(variant: CMDACT) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDACT` reader - Command transfer in progress
pub type CMDACT_R = crate::BitReader<CMDACT>;
impl CMDACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDACT {
        match self.bits {
            false => CMDACT::NotInProgress,
            true => CMDACT::InProgress,
        }
    }
    ///Command transfer not in progress
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == CMDACT::NotInProgress
    }
    ///Command tranfer in progress
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == CMDACT::InProgress
    }
}
/**Data transmit in progress

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXACT {
    ///0: Data transmit is not in progress
    NotInProgress = 0,
    ///1: Data transmit in progress
    InProgress = 1,
}
impl From<TXACT> for bool {
    #[inline(always)]
    fn from(variant: TXACT) -> Self {
        variant as u8 != 0
    }
}
///Field `TXACT` reader - Data transmit in progress
pub type TXACT_R = crate::BitReader<TXACT>;
impl TXACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXACT {
        match self.bits {
            false => TXACT::NotInProgress,
            true => TXACT::InProgress,
        }
    }
    ///Data transmit is not in progress
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == TXACT::NotInProgress
    }
    ///Data transmit in progress
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == TXACT::InProgress
    }
}
/**Data receive in progress

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXACT {
    ///0: Data receive not in progress
    NotInProgress = 0,
    ///1: Data receive in progress
    InProgress = 1,
}
impl From<RXACT> for bool {
    #[inline(always)]
    fn from(variant: RXACT) -> Self {
        variant as u8 != 0
    }
}
///Field `RXACT` reader - Data receive in progress
pub type RXACT_R = crate::BitReader<RXACT>;
impl RXACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXACT {
        match self.bits {
            false => RXACT::NotInProgress,
            true => RXACT::InProgress,
        }
    }
    ///Data receive not in progress
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == RXACT::NotInProgress
    }
    ///Data receive in progress
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RXACT::InProgress
    }
}
/**Transmit FIFO half empty: at least 8 words can be written into the FIFO

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOHE {
    ///0: Transmit FIFO not half empty
    NotHalfEmpty = 0,
    ///1: Transmit FIFO half empty. At least 8 words can be written into the FIFO
    HalfEmpty = 1,
}
impl From<TXFIFOHE> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOHE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO
pub type TXFIFOHE_R = crate::BitReader<TXFIFOHE>;
impl TXFIFOHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFIFOHE {
        match self.bits {
            false => TXFIFOHE::NotHalfEmpty,
            true => TXFIFOHE::HalfEmpty,
        }
    }
    ///Transmit FIFO not half empty
    #[inline(always)]
    pub fn is_not_half_empty(&self) -> bool {
        *self == TXFIFOHE::NotHalfEmpty
    }
    ///Transmit FIFO half empty. At least 8 words can be written into the FIFO
    #[inline(always)]
    pub fn is_half_empty(&self) -> bool {
        *self == TXFIFOHE::HalfEmpty
    }
}
/**Receive FIFO half full: there are at least 8 words in the FIFO

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOHF {
    ///0: Receive FIFO not half full
    NotHalfFull = 0,
    ///1: Receive FIFO half full. At least 8 words in the FIFO
    HalfFull = 1,
}
impl From<RXFIFOHF> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOHF) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO
pub type RXFIFOHF_R = crate::BitReader<RXFIFOHF>;
impl RXFIFOHF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFIFOHF {
        match self.bits {
            false => RXFIFOHF::NotHalfFull,
            true => RXFIFOHF::HalfFull,
        }
    }
    ///Receive FIFO not half full
    #[inline(always)]
    pub fn is_not_half_full(&self) -> bool {
        *self == RXFIFOHF::NotHalfFull
    }
    ///Receive FIFO half full. At least 8 words in the FIFO
    #[inline(always)]
    pub fn is_half_full(&self) -> bool {
        *self == RXFIFOHF::HalfFull
    }
}
/**Transmit FIFO full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOF {
    ///0: Transmit FIFO not full
    NotFull = 0,
    ///1: Transmit FIFO full
    Full = 1,
}
impl From<TXFIFOF> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOF) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFIFOF` reader - Transmit FIFO full
pub type TXFIFOF_R = crate::BitReader<TXFIFOF>;
impl TXFIFOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFIFOF {
        match self.bits {
            false => TXFIFOF::NotFull,
            true => TXFIFOF::Full,
        }
    }
    ///Transmit FIFO not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXFIFOF::NotFull
    }
    ///Transmit FIFO full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXFIFOF::Full
    }
}
/**Receive FIFO full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOF {
    ///0: Transmit FIFO not full
    NotFull = 0,
    ///1: Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full.
    Full = 1,
}
impl From<RXFIFOF> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOF) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFIFOF` reader - Receive FIFO full
pub type RXFIFOF_R = crate::BitReader<RXFIFOF>;
impl RXFIFOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFIFOF {
        match self.bits {
            false => RXFIFOF::NotFull,
            true => RXFIFOF::Full,
        }
    }
    ///Transmit FIFO not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFIFOF::NotFull
    }
    ///Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFIFOF::Full
    }
}
/**Transmit FIFO empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOE {
    ///0: Transmit FIFO not empty
    NotEmpty = 0,
    ///1: Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words.
    Empty = 1,
}
impl From<TXFIFOE> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFIFOE` reader - Transmit FIFO empty
pub type TXFIFOE_R = crate::BitReader<TXFIFOE>;
impl TXFIFOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFIFOE {
        match self.bits {
            false => TXFIFOE::NotEmpty,
            true => TXFIFOE::Empty,
        }
    }
    ///Transmit FIFO not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFIFOE::NotEmpty
    }
    ///Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFIFOE::Empty
    }
}
/**Receive FIFO empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOE {
    ///0: Receive FIFO not empty
    NotEmpty = 0,
    ///1: Receive FIFO empty
    Empty = 1,
}
impl From<RXFIFOE> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFIFOE` reader - Receive FIFO empty
pub type RXFIFOE_R = crate::BitReader<RXFIFOE>;
impl RXFIFOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFIFOE {
        match self.bits {
            false => RXFIFOE::NotEmpty,
            true => RXFIFOE::Empty,
        }
    }
    ///Receive FIFO not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXFIFOE::NotEmpty
    }
    ///Receive FIFO empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXFIFOE::Empty
    }
}
/**Data available in transmit FIFO

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDAVL {
    ///0: Data not available in transmit FIFO
    NotAvailable = 0,
    ///1: Data available in transmit FIFO
    Available = 1,
}
impl From<TXDAVL> for bool {
    #[inline(always)]
    fn from(variant: TXDAVL) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDAVL` reader - Data available in transmit FIFO
pub type TXDAVL_R = crate::BitReader<TXDAVL>;
impl TXDAVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDAVL {
        match self.bits {
            false => TXDAVL::NotAvailable,
            true => TXDAVL::Available,
        }
    }
    ///Data not available in transmit FIFO
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TXDAVL::NotAvailable
    }
    ///Data available in transmit FIFO
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == TXDAVL::Available
    }
}
/**Data available in receive FIFO

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDAVL {
    ///0: Data not available in receive FIFO
    NotAvailable = 0,
    ///1: Data available in receive FIFO
    Available = 1,
}
impl From<RXDAVL> for bool {
    #[inline(always)]
    fn from(variant: RXDAVL) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDAVL` reader - Data available in receive FIFO
pub type RXDAVL_R = crate::BitReader<RXDAVL>;
impl RXDAVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDAVL {
        match self.bits {
            false => RXDAVL::NotAvailable,
            true => RXDAVL::Available,
        }
    }
    ///Data not available in receive FIFO
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == RXDAVL::NotAvailable
    }
    ///Data available in receive FIFO
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RXDAVL::Available
    }
}
/**SDIO interrupt received

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOIT {
    ///0: SDIO interrupt not receieved
    NotReceived = 0,
    ///1: SDIO interrupt received
    Received = 1,
}
impl From<SDIOIT> for bool {
    #[inline(always)]
    fn from(variant: SDIOIT) -> Self {
        variant as u8 != 0
    }
}
///Field `SDIOIT` reader - SDIO interrupt received
pub type SDIOIT_R = crate::BitReader<SDIOIT>;
impl SDIOIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDIOIT {
        match self.bits {
            false => SDIOIT::NotReceived,
            true => SDIOIT::Received,
        }
    }
    ///SDIO interrupt not receieved
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == SDIOIT::NotReceived
    }
    ///SDIO interrupt received
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == SDIOIT::Received
    }
}
/**CE-ATA command completion signal received for CMD61

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEATAEND {
    ///0: Completion signal not received
    NotReceived = 0,
    ///1: CE-ATA command completion signal received for CMD61
    Received = 1,
}
impl From<CEATAEND> for bool {
    #[inline(always)]
    fn from(variant: CEATAEND) -> Self {
        variant as u8 != 0
    }
}
///Field `CEATAEND` reader - CE-ATA command completion signal received for CMD61
pub type CEATAEND_R = crate::BitReader<CEATAEND>;
impl CEATAEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEATAEND {
        match self.bits {
            false => CEATAEND::NotReceived,
            true => CEATAEND::Received,
        }
    }
    ///Completion signal not received
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == CEATAEND::NotReceived
    }
    ///CE-ATA command completion signal received for CMD61
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == CEATAEND::Received
    }
}
impl R {
    ///Bit 0 - Command response received (CRC check failed)
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data block sent/received (CRC check failed)
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command response timeout
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data timeout
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit FIFO underrun error
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Received FIFO overrun error
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Command response received (CRC check passed)
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Command sent (no response required)
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Data end (data counter, SDIDCOUNT, is zero)
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start bit not detected on all data signals in wide bus mode
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data block sent/received (CRC check passed)
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Command transfer in progress
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data transmit in progress
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Data receive in progress
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Transmit FIFO full
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive FIFO full
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Transmit FIFO empty
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Receive FIFO empty
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Data available in transmit FIFO
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Data available in receive FIFO
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIO interrupt received
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CE-ATA command completion signal received for CMD61
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STA")
            .field("ceataend", &self.ceataend())
            .field("sdioit", &self.sdioit())
            .field("rxdavl", &self.rxdavl())
            .field("txdavl", &self.txdavl())
            .field("rxfifoe", &self.rxfifoe())
            .field("txfifoe", &self.txfifoe())
            .field("rxfifof", &self.rxfifof())
            .field("txfifof", &self.txfifof())
            .field("rxfifohf", &self.rxfifohf())
            .field("txfifohe", &self.txfifohe())
            .field("rxact", &self.rxact())
            .field("txact", &self.txact())
            .field("cmdact", &self.cmdact())
            .field("dbckend", &self.dbckend())
            .field("stbiterr", &self.stbiterr())
            .field("dataend", &self.dataend())
            .field("cmdsent", &self.cmdsent())
            .field("cmdrend", &self.cmdrend())
            .field("rxoverr", &self.rxoverr())
            .field("txunderr", &self.txunderr())
            .field("dtimeout", &self.dtimeout())
            .field("ctimeout", &self.ctimeout())
            .field("dcrcfail", &self.dcrcfail())
            .field("ccrcfail", &self.ccrcfail())
            .finish()
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F411.html#SDIO:STA)*/
pub struct STArs;
impl crate::RegisterSpec for STArs {
    type Ux = u32;
}
///`read()` method returns [`sta::R`](R) reader structure
impl crate::Readable for STArs {}
///`reset()` method sets STA to value 0
impl crate::Resettable for STArs {}
