///Register `SR` reader
pub type R = crate::R<SRrs>;
/**PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: No operation in pgoress
    Idle = 0,
    ///1: Operation in progress
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.
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
    ///No operation in pgoress
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY::Idle
    }
    ///Operation in progress
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
/**PKA End of Operation flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDF {
    ///0: Operation in progress
    InProgress = 0,
    ///1: PKA operation is completed - set when BUSY is deasserted
    Completed = 1,
}
impl From<PROCENDF> for bool {
    #[inline(always)]
    fn from(variant: PROCENDF) -> Self {
        variant as u8 != 0
    }
}
///Field `PROCENDF` reader - PKA End of Operation flag
pub type PROCENDF_R = crate::BitReader<PROCENDF>;
impl PROCENDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PROCENDF {
        match self.bits {
            false => PROCENDF::InProgress,
            true => PROCENDF::Completed,
        }
    }
    ///Operation in progress
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == PROCENDF::InProgress
    }
    ///PKA operation is completed - set when BUSY is deasserted
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == PROCENDF::Completed
    }
}
/**PKA RAM error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRF {
    ///0: No error
    NoError = 0,
    ///1: An AHB access to the PKA RAM occurred while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress)
    Error = 1,
}
impl From<RAMERRF> for bool {
    #[inline(always)]
    fn from(variant: RAMERRF) -> Self {
        variant as u8 != 0
    }
}
///Field `RAMERRF` reader - PKA RAM error flag
pub type RAMERRF_R = crate::BitReader<RAMERRF>;
impl RAMERRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RAMERRF {
        match self.bits {
            false => RAMERRF::NoError,
            true => RAMERRF::Error,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RAMERRF::NoError
    }
    ///An AHB access to the PKA RAM occurred while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress)
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RAMERRF::Error
    }
}
/**Address error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRF {
    ///0: No error
    NoError = 0,
    ///1: Address access is out of range (unmapped address)
    Error = 1,
}
impl From<ADDRERRF> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRF) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRERRF` reader - Address error flag
pub type ADDRERRF_R = crate::BitReader<ADDRERRF>;
impl ADDRERRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDRERRF {
        match self.bits {
            false => ADDRERRF::NoError,
            true => ADDRERRF::Error,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ADDRERRF::NoError
    }
    ///Address access is out of range (unmapped address)
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ADDRERRF::Error
    }
}
impl R {
    ///Bit 16 - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PKA End of Operation flag
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - PKA RAM error flag
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error flag
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("addrerrf", &self.addrerrf())
            .field("ramerrf", &self.ramerrf())
            .field("procendf", &self.procendf())
            .field("busy", &self.busy())
            .finish()
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#PKA:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
