///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Transfer error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEF {
    ///0: `0`
    NoError = 0,
    ///1: `1`
    Error = 1,
}
impl From<TEF> for bool {
    #[inline(always)]
    fn from(variant: TEF) -> Self {
        variant as u8 != 0
    }
}
///Field `TEF` reader - Transfer error flag
pub type TEF_R = crate::BitReader<TEF>;
impl TEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEF {
        match self.bits {
            false => TEF::NoError,
            true => TEF::Error,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEF::NoError
    }
    ///`1`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEF::Error
    }
}
/**Transfer complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF {
    ///0: `0`
    NotComplete = 0,
    ///1: `1`
    Complete = 1,
}
impl From<TCF> for bool {
    #[inline(always)]
    fn from(variant: TCF) -> Self {
        variant as u8 != 0
    }
}
///Field `TCF` reader - Transfer complete flag
pub type TCF_R = crate::BitReader<TCF>;
impl TCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCF {
        match self.bits {
            false => TCF::NotComplete,
            true => TCF::Complete,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCF::NotComplete
    }
    ///`1`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCF::Complete
    }
}
/**FIFO threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF {
    ///0: `0`
    NotReached = 0,
    ///1: `1`
    Reached = 1,
}
impl From<FTF> for bool {
    #[inline(always)]
    fn from(variant: FTF) -> Self {
        variant as u8 != 0
    }
}
///Field `FTF` reader - FIFO threshold flag
pub type FTF_R = crate::BitReader<FTF>;
impl FTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FTF {
        match self.bits {
            false => FTF::NotReached,
            true => FTF::Reached,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == FTF::NotReached
    }
    ///`1`
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == FTF::Reached
    }
}
/**Status match flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMF {
    ///0: `0`
    NotMatched = 0,
    ///1: `1`
    Matched = 1,
}
impl From<SMF> for bool {
    #[inline(always)]
    fn from(variant: SMF) -> Self {
        variant as u8 != 0
    }
}
///Field `SMF` reader - Status match flag
pub type SMF_R = crate::BitReader<SMF>;
impl SMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMF {
        match self.bits {
            false => SMF::NotMatched,
            true => SMF::Matched,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_not_matched(&self) -> bool {
        *self == SMF::NotMatched
    }
    ///`1`
    #[inline(always)]
    pub fn is_matched(&self) -> bool {
        *self == SMF::Matched
    }
}
/**Timeout flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF {
    ///0: `0`
    NotTimeout = 0,
    ///1: `1`
    Timeout = 1,
}
impl From<TOF> for bool {
    #[inline(always)]
    fn from(variant: TOF) -> Self {
        variant as u8 != 0
    }
}
///Field `TOF` reader - Timeout flag
pub type TOF_R = crate::BitReader<TOF>;
impl TOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOF {
        match self.bits {
            false => TOF::NotTimeout,
            true => TOF::Timeout,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_not_timeout(&self) -> bool {
        *self == TOF::NotTimeout
    }
    ///`1`
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TOF::Timeout
    }
}
/**Busy

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: `0`
    NotBusy = 0,
    ///1: `1`
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - Busy
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::NotBusy,
            true => BUSY::Busy,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY::NotBusy
    }
    ///`1`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
///Field `FLEVEL` reader - FIFO level
pub type FLEVEL_R = crate::FieldReader;
impl R {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Busy
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:14 - FIFO level
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("flevel", &self.flevel())
            .field("busy", &self.busy())
            .field("tof", &self.tof())
            .field("smf", &self.smf())
            .field("ftf", &self.ftf())
            .field("tcf", &self.tcf())
            .field("tef", &self.tef())
            .finish()
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#QUADSPI:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
