#[doc = "Register `SDSR` reader"]
pub type R = crate::R<SDSRrs>;
#[doc = "Refresh error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    #[doc = "0: No refresh error has been detected"]
    NoError = 0,
    #[doc = "1: A refresh error has been detected"]
    Error = 1,
}
impl From<RE> for bool {
    #[inline(always)]
    fn from(variant: RE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Refresh error flag"]
pub type RE_R = crate::BitReader<RE>;
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE {
        match self.bits {
            false => RE::NoError,
            true => RE::Error,
        }
    }
    #[doc = "No refresh error has been detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RE::NoError
    }
    #[doc = "A refresh error has been detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RE::Error
    }
}
#[doc = "Status Mode for Bank 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODES1 {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Self-refresh mode"]
    SelfRefresh = 1,
    #[doc = "2: Power-down mode"]
    PowerDown = 2,
}
impl From<MODES1> for u8 {
    #[inline(always)]
    fn from(variant: MODES1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODES1 {
    type Ux = u8;
}
#[doc = "Field `MODES1` reader - Status Mode for Bank 1"]
pub type MODES1_R = crate::FieldReader<MODES1>;
impl MODES1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODES1> {
        match self.bits {
            0 => Some(MODES1::Normal),
            1 => Some(MODES1::SelfRefresh),
            2 => Some(MODES1::PowerDown),
            _ => None,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODES1::Normal
    }
    #[doc = "Self-refresh mode"]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        *self == MODES1::SelfRefresh
    }
    #[doc = "Power-down mode"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == MODES1::PowerDown
    }
}
#[doc = "Field `MODES2` reader - Status Mode for Bank 2"]
pub use MODES1_R as MODES2_R;
#[doc = "Busy status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    #[doc = "0: SDRAM Controller is ready to accept a new request"]
    NotBusy = 0,
    #[doc = "1: SDRAM Controller is not ready to accept a new request"]
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy status"]
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::NotBusy,
            true => BUSY::Busy,
        }
    }
    #[doc = "SDRAM Controller is ready to accept a new request"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY::NotBusy
    }
    #[doc = "SDRAM Controller is not ready to accept a new request"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Refresh error flag"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1"]
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2"]
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "SDRAM Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDSRrs;
impl crate::RegisterSpec for SDSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdsr::R`](R) reader structure"]
impl crate::Readable for SDSRrs {}
#[doc = "`reset()` method sets SDSR to value 0"]
impl crate::Resettable for SDSRrs {
    const RESET_VALUE: u32 = 0;
}
