#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    #[doc = "0: No operation in pgoress"]
    Idle = 0,
    #[doc = "1: Operation in progress"]
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started."]
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::Idle,
            true => BUSY::Busy,
        }
    }
    #[doc = "No operation in pgoress"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY::Idle
    }
    #[doc = "Operation in progress"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
#[doc = "PKA End of Operation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDF {
    #[doc = "0: Operation in progress"]
    InProgress = 0,
    #[doc = "1: PKA operation is completed - set when BUSY is deasserted"]
    Completed = 1,
}
impl From<PROCENDF> for bool {
    #[inline(always)]
    fn from(variant: PROCENDF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROCENDF` reader - PKA End of Operation flag"]
pub type PROCENDF_R = crate::BitReader<PROCENDF>;
impl PROCENDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PROCENDF {
        match self.bits {
            false => PROCENDF::InProgress,
            true => PROCENDF::Completed,
        }
    }
    #[doc = "Operation in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == PROCENDF::InProgress
    }
    #[doc = "PKA operation is completed - set when BUSY is deasserted"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == PROCENDF::Completed
    }
}
#[doc = "PKA RAM error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRF {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: An AHB access to the PKA RAM occurred while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress)"]
    Error = 1,
}
impl From<RAMERRF> for bool {
    #[inline(always)]
    fn from(variant: RAMERRF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMERRF` reader - PKA RAM error flag"]
pub type RAMERRF_R = crate::BitReader<RAMERRF>;
impl RAMERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMERRF {
        match self.bits {
            false => RAMERRF::NoError,
            true => RAMERRF::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RAMERRF::NoError
    }
    #[doc = "An AHB access to the PKA RAM occurred while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RAMERRF::Error
    }
}
#[doc = "Address error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRF {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Address access is out of range (unmapped address)"]
    Error = 1,
}
impl From<ADDRERRF> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRERRF` reader - Address error flag"]
pub type ADDRERRF_R = crate::BitReader<ADDRERRF>;
impl ADDRERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRERRF {
        match self.bits {
            false => ADDRERRF::NoError,
            true => ADDRERRF::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ADDRERRF::NoError
    }
    #[doc = "Address access is out of range (unmapped address)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ADDRERRF::Error
    }
}
impl R {
    #[doc = "Bit 16 - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - PKA RAM error flag"]
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Address error flag"]
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
