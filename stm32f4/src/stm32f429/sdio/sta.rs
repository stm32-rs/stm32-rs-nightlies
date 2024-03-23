#[doc = "Register `STA` reader"]
pub type R = crate::R<STArs>;
#[doc = "Command response received (CRC check failed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRCFAIL {
    #[doc = "0: Command response received, crc check passed"]
    NotFailed = 0,
    #[doc = "1: Command response received, crc check failed"]
    Failed = 1,
}
impl From<CCRCFAIL> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAIL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub type CCRCFAIL_R = crate::BitReader<CCRCFAIL>;
impl CCRCFAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCRCFAIL {
        match self.bits {
            false => CCRCFAIL::NotFailed,
            true => CCRCFAIL::Failed,
        }
    }
    #[doc = "Command response received, crc check passed"]
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == CCRCFAIL::NotFailed
    }
    #[doc = "Command response received, crc check failed"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == CCRCFAIL::Failed
    }
}
#[doc = "Data block sent/received (CRC check failed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRCFAIL {
    #[doc = "0: No Data block sent/received crc check fail"]
    NotFailed = 0,
    #[doc = "1: Data block sent/received crc failed"]
    Failed = 1,
}
impl From<DCRCFAIL> for bool {
    #[inline(always)]
    fn from(variant: DCRCFAIL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub type DCRCFAIL_R = crate::BitReader<DCRCFAIL>;
impl DCRCFAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCRCFAIL {
        match self.bits {
            false => DCRCFAIL::NotFailed,
            true => DCRCFAIL::Failed,
        }
    }
    #[doc = "No Data block sent/received crc check fail"]
    #[inline(always)]
    pub fn is_not_failed(&self) -> bool {
        *self == DCRCFAIL::NotFailed
    }
    #[doc = "Data block sent/received crc failed"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == DCRCFAIL::Failed
    }
}
#[doc = "Command response timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIMEOUT {
    #[doc = "0: No Command timeout"]
    NoTimeout = 0,
    #[doc = "1: Command timeout"]
    Timeout = 1,
}
impl From<CTIMEOUT> for bool {
    #[inline(always)]
    fn from(variant: CTIMEOUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub type CTIMEOUT_R = crate::BitReader<CTIMEOUT>;
impl CTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTIMEOUT {
        match self.bits {
            false => CTIMEOUT::NoTimeout,
            true => CTIMEOUT::Timeout,
        }
    }
    #[doc = "No Command timeout"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == CTIMEOUT::NoTimeout
    }
    #[doc = "Command timeout"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == CTIMEOUT::Timeout
    }
}
#[doc = "Data timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTIMEOUT {
    #[doc = "0: No data timeout"]
    NoTimeout = 0,
    #[doc = "1: Data timeout"]
    Timeout = 1,
}
impl From<DTIMEOUT> for bool {
    #[inline(always)]
    fn from(variant: DTIMEOUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub type DTIMEOUT_R = crate::BitReader<DTIMEOUT>;
impl DTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTIMEOUT {
        match self.bits {
            false => DTIMEOUT::NoTimeout,
            true => DTIMEOUT::Timeout,
        }
    }
    #[doc = "No data timeout"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == DTIMEOUT::NoTimeout
    }
    #[doc = "Data timeout"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == DTIMEOUT::Timeout
    }
}
#[doc = "Transmit FIFO underrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXUNDERR {
    #[doc = "0: No transmit FIFO underrun error"]
    NoUnderrun = 0,
    #[doc = "1: Transmit FIFO underrun error"]
    Underrun = 1,
}
impl From<TXUNDERR> for bool {
    #[inline(always)]
    fn from(variant: TXUNDERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error"]
pub type TXUNDERR_R = crate::BitReader<TXUNDERR>;
impl TXUNDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXUNDERR {
        match self.bits {
            false => TXUNDERR::NoUnderrun,
            true => TXUNDERR::Underrun,
        }
    }
    #[doc = "No transmit FIFO underrun error"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TXUNDERR::NoUnderrun
    }
    #[doc = "Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TXUNDERR::Underrun
    }
}
#[doc = "Received FIFO overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOVERR {
    #[doc = "0: No FIFO overrun error"]
    NoOverrun = 0,
    #[doc = "1: Receive FIFO overrun error"]
    Overrun = 1,
}
impl From<RXOVERR> for bool {
    #[inline(always)]
    fn from(variant: RXOVERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error"]
pub type RXOVERR_R = crate::BitReader<RXOVERR>;
impl RXOVERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXOVERR {
        match self.bits {
            false => RXOVERR::NoOverrun,
            true => RXOVERR::Overrun,
        }
    }
    #[doc = "No FIFO overrun error"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == RXOVERR::NoOverrun
    }
    #[doc = "Receive FIFO overrun error"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == RXOVERR::Overrun
    }
}
#[doc = "Command response received (CRC check passed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDREND {
    #[doc = "0: Command not done"]
    NotDone = 0,
    #[doc = "1: Command response received (CRC check passed)"]
    Done = 1,
}
impl From<CMDREND> for bool {
    #[inline(always)]
    fn from(variant: CMDREND) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed)"]
pub type CMDREND_R = crate::BitReader<CMDREND>;
impl CMDREND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDREND {
        match self.bits {
            false => CMDREND::NotDone,
            true => CMDREND::Done,
        }
    }
    #[doc = "Command not done"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == CMDREND::NotDone
    }
    #[doc = "Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == CMDREND::Done
    }
}
#[doc = "Command sent (no response required)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDSENT {
    #[doc = "0: Command not sent"]
    NotSent = 0,
    #[doc = "1: Command sent (no response required)"]
    Sent = 1,
}
impl From<CMDSENT> for bool {
    #[inline(always)]
    fn from(variant: CMDSENT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub type CMDSENT_R = crate::BitReader<CMDSENT>;
impl CMDSENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDSENT {
        match self.bits {
            false => CMDSENT::NotSent,
            true => CMDSENT::Sent,
        }
    }
    #[doc = "Command not sent"]
    #[inline(always)]
    pub fn is_not_sent(&self) -> bool {
        *self == CMDSENT::NotSent
    }
    #[doc = "Command sent (no response required)"]
    #[inline(always)]
    pub fn is_sent(&self) -> bool {
        *self == CMDSENT::Sent
    }
}
#[doc = "Data end (data counter, SDIDCOUNT, is zero)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAEND {
    #[doc = "0: Not done"]
    NotDone = 0,
    #[doc = "1: Data end (DCOUNT, is zero)"]
    Done = 1,
}
impl From<DATAEND> for bool {
    #[inline(always)]
    fn from(variant: DATAEND) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)"]
pub type DATAEND_R = crate::BitReader<DATAEND>;
impl DATAEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATAEND {
        match self.bits {
            false => DATAEND::NotDone,
            true => DATAEND::Done,
        }
    }
    #[doc = "Not done"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == DATAEND::NotDone
    }
    #[doc = "Data end (DCOUNT, is zero)"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == DATAEND::Done
    }
}
#[doc = "Start bit not detected on all data signals in wide bus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBITERR {
    #[doc = "0: No start bit detected error"]
    Detected = 0,
    #[doc = "1: Start bit not detected error"]
    NotDetected = 1,
}
impl From<STBITERR> for bool {
    #[inline(always)]
    fn from(variant: STBITERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBITERR` reader - Start bit not detected on all data signals in wide bus mode"]
pub type STBITERR_R = crate::BitReader<STBITERR>;
impl STBITERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STBITERR {
        match self.bits {
            false => STBITERR::Detected,
            true => STBITERR::NotDetected,
        }
    }
    #[doc = "No start bit detected error"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == STBITERR::Detected
    }
    #[doc = "Start bit not detected error"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == STBITERR::NotDetected
    }
}
#[doc = "Data block sent/received (CRC check passed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBCKEND {
    #[doc = "0: Data block not sent/received (CRC check failed)"]
    NotTransferred = 0,
    #[doc = "1: Data block sent/received (CRC check passed)"]
    Transferred = 1,
}
impl From<DBCKEND> for bool {
    #[inline(always)]
    fn from(variant: DBCKEND) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBCKEND` reader - Data block sent/received (CRC check passed)"]
pub type DBCKEND_R = crate::BitReader<DBCKEND>;
impl DBCKEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBCKEND {
        match self.bits {
            false => DBCKEND::NotTransferred,
            true => DBCKEND::Transferred,
        }
    }
    #[doc = "Data block not sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn is_not_transferred(&self) -> bool {
        *self == DBCKEND::NotTransferred
    }
    #[doc = "Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn is_transferred(&self) -> bool {
        *self == DBCKEND::Transferred
    }
}
#[doc = "Command transfer in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDACT {
    #[doc = "0: Command transfer not in progress"]
    NotInProgress = 0,
    #[doc = "1: Command tranfer in progress"]
    InProgress = 1,
}
impl From<CMDACT> for bool {
    #[inline(always)]
    fn from(variant: CMDACT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDACT` reader - Command transfer in progress"]
pub type CMDACT_R = crate::BitReader<CMDACT>;
impl CMDACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDACT {
        match self.bits {
            false => CMDACT::NotInProgress,
            true => CMDACT::InProgress,
        }
    }
    #[doc = "Command transfer not in progress"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == CMDACT::NotInProgress
    }
    #[doc = "Command tranfer in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == CMDACT::InProgress
    }
}
#[doc = "Data transmit in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXACT {
    #[doc = "0: Data transmit is not in progress"]
    NotInProgress = 0,
    #[doc = "1: Data transmit in progress"]
    InProgress = 1,
}
impl From<TXACT> for bool {
    #[inline(always)]
    fn from(variant: TXACT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXACT` reader - Data transmit in progress"]
pub type TXACT_R = crate::BitReader<TXACT>;
impl TXACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXACT {
        match self.bits {
            false => TXACT::NotInProgress,
            true => TXACT::InProgress,
        }
    }
    #[doc = "Data transmit is not in progress"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == TXACT::NotInProgress
    }
    #[doc = "Data transmit in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == TXACT::InProgress
    }
}
#[doc = "Data receive in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXACT {
    #[doc = "0: Data receive not in progress"]
    NotInProgress = 0,
    #[doc = "1: Data receive in progress"]
    InProgress = 1,
}
impl From<RXACT> for bool {
    #[inline(always)]
    fn from(variant: RXACT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXACT` reader - Data receive in progress"]
pub type RXACT_R = crate::BitReader<RXACT>;
impl RXACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXACT {
        match self.bits {
            false => RXACT::NotInProgress,
            true => RXACT::InProgress,
        }
    }
    #[doc = "Data receive not in progress"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == RXACT::NotInProgress
    }
    #[doc = "Data receive in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RXACT::InProgress
    }
}
#[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOHE {
    #[doc = "0: Transmit FIFO not half empty"]
    NotHalfEmpty = 0,
    #[doc = "1: Transmit FIFO half empty. At least 8 words can be written into the FIFO"]
    HalfEmpty = 1,
}
impl From<TXFIFOHE> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOHE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
pub type TXFIFOHE_R = crate::BitReader<TXFIFOHE>;
impl TXFIFOHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFIFOHE {
        match self.bits {
            false => TXFIFOHE::NotHalfEmpty,
            true => TXFIFOHE::HalfEmpty,
        }
    }
    #[doc = "Transmit FIFO not half empty"]
    #[inline(always)]
    pub fn is_not_half_empty(&self) -> bool {
        *self == TXFIFOHE::NotHalfEmpty
    }
    #[doc = "Transmit FIFO half empty. At least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn is_half_empty(&self) -> bool {
        *self == TXFIFOHE::HalfEmpty
    }
}
#[doc = "Receive FIFO half full: there are at least 8 words in the FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOHF {
    #[doc = "0: Receive FIFO not half full"]
    NotHalfFull = 0,
    #[doc = "1: Receive FIFO half full. At least 8 words in the FIFO"]
    HalfFull = 1,
}
impl From<RXFIFOHF> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOHF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO"]
pub type RXFIFOHF_R = crate::BitReader<RXFIFOHF>;
impl RXFIFOHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFIFOHF {
        match self.bits {
            false => RXFIFOHF::NotHalfFull,
            true => RXFIFOHF::HalfFull,
        }
    }
    #[doc = "Receive FIFO not half full"]
    #[inline(always)]
    pub fn is_not_half_full(&self) -> bool {
        *self == RXFIFOHF::NotHalfFull
    }
    #[doc = "Receive FIFO half full. At least 8 words in the FIFO"]
    #[inline(always)]
    pub fn is_half_full(&self) -> bool {
        *self == RXFIFOHF::HalfFull
    }
}
#[doc = "Transmit FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOF {
    #[doc = "0: Transmit FIFO not full"]
    NotFull = 0,
    #[doc = "1: Transmit FIFO full"]
    Full = 1,
}
impl From<TXFIFOF> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub type TXFIFOF_R = crate::BitReader<TXFIFOF>;
impl TXFIFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFIFOF {
        match self.bits {
            false => TXFIFOF::NotFull,
            true => TXFIFOF::Full,
        }
    }
    #[doc = "Transmit FIFO not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXFIFOF::NotFull
    }
    #[doc = "Transmit FIFO full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXFIFOF::Full
    }
}
#[doc = "Receive FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOF {
    #[doc = "0: Transmit FIFO not full"]
    NotFull = 0,
    #[doc = "1: Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full."]
    Full = 1,
}
impl From<RXFIFOF> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub type RXFIFOF_R = crate::BitReader<RXFIFOF>;
impl RXFIFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFIFOF {
        match self.bits {
            false => RXFIFOF::NotFull,
            true => RXFIFOF::Full,
        }
    }
    #[doc = "Transmit FIFO not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFIFOF::NotFull
    }
    #[doc = "Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFIFOF::Full
    }
}
#[doc = "Transmit FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFOE {
    #[doc = "0: Transmit FIFO not empty"]
    NotEmpty = 0,
    #[doc = "1: Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words."]
    Empty = 1,
}
impl From<TXFIFOE> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub type TXFIFOE_R = crate::BitReader<TXFIFOE>;
impl TXFIFOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFIFOE {
        match self.bits {
            false => TXFIFOE::NotEmpty,
            true => TXFIFOE::Empty,
        }
    }
    #[doc = "Transmit FIFO not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFIFOE::NotEmpty
    }
    #[doc = "Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFIFOE::Empty
    }
}
#[doc = "Receive FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFOE {
    #[doc = "0: Receive FIFO not empty"]
    NotEmpty = 0,
    #[doc = "1: Receive FIFO empty"]
    Empty = 1,
}
impl From<RXFIFOE> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub type RXFIFOE_R = crate::BitReader<RXFIFOE>;
impl RXFIFOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFIFOE {
        match self.bits {
            false => RXFIFOE::NotEmpty,
            true => RXFIFOE::Empty,
        }
    }
    #[doc = "Receive FIFO not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXFIFOE::NotEmpty
    }
    #[doc = "Receive FIFO empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXFIFOE::Empty
    }
}
#[doc = "Data available in transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDAVL {
    #[doc = "0: Data not available in transmit FIFO"]
    NotAvailable = 0,
    #[doc = "1: Data available in transmit FIFO"]
    Available = 1,
}
impl From<TXDAVL> for bool {
    #[inline(always)]
    fn from(variant: TXDAVL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDAVL` reader - Data available in transmit FIFO"]
pub type TXDAVL_R = crate::BitReader<TXDAVL>;
impl TXDAVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDAVL {
        match self.bits {
            false => TXDAVL::NotAvailable,
            true => TXDAVL::Available,
        }
    }
    #[doc = "Data not available in transmit FIFO"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TXDAVL::NotAvailable
    }
    #[doc = "Data available in transmit FIFO"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == TXDAVL::Available
    }
}
#[doc = "Data available in receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDAVL {
    #[doc = "0: Data not available in receive FIFO"]
    NotAvailable = 0,
    #[doc = "1: Data available in receive FIFO"]
    Available = 1,
}
impl From<RXDAVL> for bool {
    #[inline(always)]
    fn from(variant: RXDAVL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDAVL` reader - Data available in receive FIFO"]
pub type RXDAVL_R = crate::BitReader<RXDAVL>;
impl RXDAVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDAVL {
        match self.bits {
            false => RXDAVL::NotAvailable,
            true => RXDAVL::Available,
        }
    }
    #[doc = "Data not available in receive FIFO"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == RXDAVL::NotAvailable
    }
    #[doc = "Data available in receive FIFO"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RXDAVL::Available
    }
}
#[doc = "SDIO interrupt received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOIT {
    #[doc = "0: SDIO interrupt not receieved"]
    NotReceived = 0,
    #[doc = "1: SDIO interrupt received"]
    Received = 1,
}
impl From<SDIOIT> for bool {
    #[inline(always)]
    fn from(variant: SDIOIT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub type SDIOIT_R = crate::BitReader<SDIOIT>;
impl SDIOIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDIOIT {
        match self.bits {
            false => SDIOIT::NotReceived,
            true => SDIOIT::Received,
        }
    }
    #[doc = "SDIO interrupt not receieved"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == SDIOIT::NotReceived
    }
    #[doc = "SDIO interrupt received"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == SDIOIT::Received
    }
}
#[doc = "CE-ATA command completion signal received for CMD61\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEATAEND {
    #[doc = "0: Completion signal not received"]
    NotReceived = 0,
    #[doc = "1: CE-ATA command completion signal received for CMD61"]
    Received = 1,
}
impl From<CEATAEND> for bool {
    #[inline(always)]
    fn from(variant: CEATAEND) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEATAEND` reader - CE-ATA command completion signal received for CMD61"]
pub type CEATAEND_R = crate::BitReader<CEATAEND>;
impl CEATAEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEATAEND {
        match self.bits {
            false => CEATAEND::NotReceived,
            true => CEATAEND::Received,
        }
    }
    #[doc = "Completion signal not received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == CEATAEND::NotReceived
    }
    #[doc = "CE-ATA command completion signal received for CMD61"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == CEATAEND::Received
    }
}
impl R {
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode"]
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO"]
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received for CMD61"]
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STArs;
impl crate::RegisterSpec for STArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for STArs {}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STArs {
    const RESET_VALUE: u32 = 0;
}
