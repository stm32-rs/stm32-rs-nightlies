#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Compare match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMR {
    #[doc = "1: Compare match"]
    Set = 1,
}
impl From<CMPMR> for bool {
    #[inline(always)]
    fn from(variant: CMPMR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPM` reader - Compare match"]
pub type CMPM_R = crate::BitReader<CMPMR>;
impl CMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMPMR> {
        match self.bits {
            true => Some(CMPMR::Set),
            _ => None,
        }
    }
    #[doc = "Compare match"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPMR::Set
    }
}
#[doc = "Autoreload match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMR {
    #[doc = "1: Autoreload match"]
    Set = 1,
}
impl From<ARRMR> for bool {
    #[inline(always)]
    fn from(variant: ARRMR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRM` reader - Autoreload match"]
pub type ARRM_R = crate::BitReader<ARRMR>;
impl ARRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARRMR> {
        match self.bits {
            true => Some(ARRMR::Set),
            _ => None,
        }
    }
    #[doc = "Autoreload match"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARRMR::Set
    }
}
#[doc = "External trigger edge event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGR {
    #[doc = "1: External trigger edge event"]
    Set = 1,
}
impl From<EXTTRIGR> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIG` reader - External trigger edge event"]
pub type EXTTRIG_R = crate::BitReader<EXTTRIGR>;
impl EXTTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTTRIGR> {
        match self.bits {
            true => Some(EXTTRIGR::Set),
            _ => None,
        }
    }
    #[doc = "External trigger edge event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EXTTRIGR::Set
    }
}
#[doc = "Compare register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKR {
    #[doc = "1: Compare register update OK"]
    Set = 1,
}
impl From<CMPOKR> for bool {
    #[inline(always)]
    fn from(variant: CMPOKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOK` reader - Compare register update OK"]
pub type CMPOK_R = crate::BitReader<CMPOKR>;
impl CMPOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMPOKR> {
        match self.bits {
            true => Some(CMPOKR::Set),
            _ => None,
        }
    }
    #[doc = "Compare register update OK"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPOKR::Set
    }
}
#[doc = "Autoreload register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKR {
    #[doc = "1: Autoreload register update OK"]
    Set = 1,
}
impl From<ARROKR> for bool {
    #[inline(always)]
    fn from(variant: ARROKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROK` reader - Autoreload register update OK"]
pub type ARROK_R = crate::BitReader<ARROKR>;
impl ARROK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARROKR> {
        match self.bits {
            true => Some(ARROKR::Set),
            _ => None,
        }
    }
    #[doc = "Autoreload register update OK"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARROKR::Set
    }
}
#[doc = "Counter direction change down to up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPR {
    #[doc = "1: Counter direction change down to up"]
    Set = 1,
}
impl From<UPR> for bool {
    #[inline(always)]
    fn from(variant: UPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` reader - Counter direction change down to up"]
pub type UP_R = crate::BitReader<UPR>;
impl UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPR> {
        match self.bits {
            true => Some(UPR::Set),
            _ => None,
        }
    }
    #[doc = "Counter direction change down to up"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UPR::Set
    }
}
#[doc = "Counter direction change up to down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNR {
    #[doc = "1: Counter direction change up to down"]
    Set = 1,
}
impl From<DOWNR> for bool {
    #[inline(always)]
    fn from(variant: DOWNR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` reader - Counter direction change up to down"]
pub type DOWN_R = crate::BitReader<DOWNR>;
impl DOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DOWNR> {
        match self.bits {
            true => Some(DOWNR::Set),
            _ => None,
        }
    }
    #[doc = "Counter direction change up to down"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DOWNR::Set
    }
}
#[doc = "LPTIM update event occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UER {
    #[doc = "1: LPTIM update event occurred"]
    Set = 1,
}
impl From<UER> for bool {
    #[inline(always)]
    fn from(variant: UER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UE` reader - LPTIM update event occurred"]
pub type UE_R = crate::BitReader<UER>;
impl UE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UER> {
        match self.bits {
            true => Some(UER::Set),
            _ => None,
        }
    }
    #[doc = "LPTIM update event occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UER::Set
    }
}
#[doc = "Repetition register update Ok\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPOKR {
    #[doc = "1: Repetition register update OK"]
    Set = 1,
}
impl From<REPOKR> for bool {
    #[inline(always)]
    fn from(variant: REPOKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPOK` reader - Repetition register update Ok"]
pub type REPOK_R = crate::BitReader<REPOKR>;
impl REPOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REPOKR> {
        match self.bits {
            true => Some(REPOKR::Set),
            _ => None,
        }
    }
    #[doc = "Repetition register update OK"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == REPOKR::Set
    }
}
impl R {
    #[doc = "Bit 0 - Compare match"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match"]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK"]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPTIM update event occurred"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update Ok"]
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
