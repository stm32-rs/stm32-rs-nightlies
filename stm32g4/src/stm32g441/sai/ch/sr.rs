///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Overrun / underrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRUDRR {
    ///0: No overrun/underrun error
    NoError = 0,
    ///1: Overrun/underrun error detection
    Overrun = 1,
}
impl From<OVRUDRR> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRUDR` reader - Overrun / underrun
pub type OVRUDR_R = crate::BitReader<OVRUDRR>;
impl OVRUDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRUDRR {
        match self.bits {
            false => OVRUDRR::NoError,
            true => OVRUDRR::Overrun,
        }
    }
    ///No overrun/underrun error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OVRUDRR::NoError
    }
    ///Overrun/underrun error detection
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRUDRR::Overrun
    }
}
/**Mute detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEDETR {
    ///0: No MUTE detection on the SD input line
    NoMute = 0,
    ///1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame
    Mute = 1,
}
impl From<MUTEDETR> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETR) -> Self {
        variant as u8 != 0
    }
}
///Field `MUTEDET` reader - Mute detection
pub type MUTEDET_R = crate::BitReader<MUTEDETR>;
impl MUTEDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MUTEDETR {
        match self.bits {
            false => MUTEDETR::NoMute,
            true => MUTEDETR::Mute,
        }
    }
    ///No MUTE detection on the SD input line
    #[inline(always)]
    pub fn is_no_mute(&self) -> bool {
        *self == MUTEDETR::NoMute
    }
    ///MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == MUTEDETR::Mute
    }
}
/**Wrong clock configuration flag. This bit is read only

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCKCFGR {
    ///0: Clock configuration is correct
    Correct = 0,
    ///1: Clock configuration does not respect the rule concerning the frame length specification
    Wrong = 1,
}
impl From<WCKCFGR> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGR) -> Self {
        variant as u8 != 0
    }
}
///Field `WCKCFG` reader - Wrong clock configuration flag. This bit is read only
pub type WCKCFG_R = crate::BitReader<WCKCFGR>;
impl WCKCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WCKCFGR {
        match self.bits {
            false => WCKCFGR::Correct,
            true => WCKCFGR::Wrong,
        }
    }
    ///Clock configuration is correct
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == WCKCFGR::Correct
    }
    ///Clock configuration does not respect the rule concerning the frame length specification
    #[inline(always)]
    pub fn is_wrong(&self) -> bool {
        *self == WCKCFGR::Wrong
    }
}
/**FIFO request

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQR {
    ///0: No FIFO request
    NoRequest = 0,
    ///1: FIFO request to read or to write the SAI_xDR
    Request = 1,
}
impl From<FREQR> for bool {
    #[inline(always)]
    fn from(variant: FREQR) -> Self {
        variant as u8 != 0
    }
}
///Field `FREQ` reader - FIFO request
pub type FREQ_R = crate::BitReader<FREQR>;
impl FREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FREQR {
        match self.bits {
            false => FREQR::NoRequest,
            true => FREQR::Request,
        }
    }
    ///No FIFO request
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == FREQR::NoRequest
    }
    ///FIFO request to read or to write the SAI_xDR
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == FREQR::Request
    }
}
/**Codec not ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNRDYR {
    ///0: External AC’97 Codec is ready
    Ready = 0,
    ///1: External AC’97 Codec is not ready
    NotReady = 1,
}
impl From<CNRDYR> for bool {
    #[inline(always)]
    fn from(variant: CNRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `CNRDY` reader - Codec not ready
pub type CNRDY_R = crate::BitReader<CNRDYR>;
impl CNRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CNRDYR {
        match self.bits {
            false => CNRDYR::Ready,
            true => CNRDYR::NotReady,
        }
    }
    ///External AC’97 Codec is ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CNRDYR::Ready
    }
    ///External AC’97 Codec is not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == CNRDYR::NotReady
    }
}
/**Anticipated frame synchronization detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFSDETR {
    ///0: No error
    NoError = 0,
    ///1: Frame synchronization signal is detected earlier than expected
    EarlySync = 1,
}
impl From<AFSDETR> for bool {
    #[inline(always)]
    fn from(variant: AFSDETR) -> Self {
        variant as u8 != 0
    }
}
///Field `AFSDET` reader - Anticipated frame synchronization detection
pub type AFSDET_R = crate::BitReader<AFSDETR>;
impl AFSDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSDETR {
        match self.bits {
            false => AFSDETR::NoError,
            true => AFSDETR::EarlySync,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AFSDETR::NoError
    }
    ///Frame synchronization signal is detected earlier than expected
    #[inline(always)]
    pub fn is_early_sync(&self) -> bool {
        *self == AFSDETR::EarlySync
    }
}
/**Late frame synchronization detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSDETR {
    ///0: No error
    NoError = 0,
    ///1: Frame synchronization signal is not present at the right time
    NoSync = 1,
}
impl From<LFSDETR> for bool {
    #[inline(always)]
    fn from(variant: LFSDETR) -> Self {
        variant as u8 != 0
    }
}
///Field `LFSDET` reader - Late frame synchronization detection
pub type LFSDET_R = crate::BitReader<LFSDETR>;
impl LFSDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LFSDETR {
        match self.bits {
            false => LFSDETR::NoError,
            true => LFSDETR::NoSync,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LFSDETR::NoError
    }
    ///Frame synchronization signal is not present at the right time
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == LFSDETR::NoSync
    }
}
/**FIFO level threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLVLR {
    ///0: FIFO empty
    Empty = 0,
    ///1: FIFO <= 1⁄4 but not empty
    Quarter1 = 1,
    ///2: 1⁄4 < FIFO <= 1⁄2
    Quarter2 = 2,
    ///3: 1⁄2 < FIFO <= 3⁄4
    Quarter3 = 3,
    ///4: 3⁄4 < FIFO but not full
    Quarter4 = 4,
    ///5: FIFO full
    Full = 5,
}
impl From<FLVLR> for u8 {
    #[inline(always)]
    fn from(variant: FLVLR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLVLR {
    type Ux = u8;
}
impl crate::IsEnum for FLVLR {}
///Field `FLVL` reader - FIFO level threshold
pub type FLVL_R = crate::FieldReader<FLVLR>;
impl FLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FLVLR> {
        match self.bits {
            0 => Some(FLVLR::Empty),
            1 => Some(FLVLR::Quarter1),
            2 => Some(FLVLR::Quarter2),
            3 => Some(FLVLR::Quarter3),
            4 => Some(FLVLR::Quarter4),
            5 => Some(FLVLR::Full),
            _ => None,
        }
    }
    ///FIFO empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FLVLR::Empty
    }
    ///FIFO <= 1⁄4 but not empty
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FLVLR::Quarter1
    }
    ///1⁄4 < FIFO <= 1⁄2
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FLVLR::Quarter2
    }
    ///1⁄2 < FIFO <= 3⁄4
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FLVLR::Quarter3
    }
    ///3⁄4 < FIFO but not full
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FLVLR::Quarter4
    }
    ///FIFO full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FLVLR::Full
    }
}
impl R {
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wrong clock configuration flag. This bit is read only
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("flvl", &self.flvl())
            .field("lfsdet", &self.lfsdet())
            .field("afsdet", &self.afsdet())
            .field("cnrdy", &self.cnrdy())
            .field("freq", &self.freq())
            .field("wckcfg", &self.wckcfg())
            .field("mutedet", &self.mutedet())
            .field("ovrudr", &self.ovrudr())
            .finish()
    }
}
/**AStatus register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x08
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x08;
}
