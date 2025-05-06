///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**Compare match

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMR {
    ///1: Compare match
    Set = 1,
}
impl From<CMPMR> for bool {
    #[inline(always)]
    fn from(variant: CMPMR) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPM` reader - Compare match
pub type CMPM_R = crate::BitReader<CMPMR>;
impl CMPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMPMR> {
        match self.bits {
            true => Some(CMPMR::Set),
            _ => None,
        }
    }
    ///Compare match
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPMR::Set
    }
}
/**Autoreload match

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMR {
    ///1: Autoreload match
    Set = 1,
}
impl From<ARRMR> for bool {
    #[inline(always)]
    fn from(variant: ARRMR) -> Self {
        variant as u8 != 0
    }
}
///Field `ARRM` reader - Autoreload match
pub type ARRM_R = crate::BitReader<ARRMR>;
impl ARRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARRMR> {
        match self.bits {
            true => Some(ARRMR::Set),
            _ => None,
        }
    }
    ///Autoreload match
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARRMR::Set
    }
}
/**External trigger edge event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGR {
    ///1: External trigger edge event
    Set = 1,
}
impl From<EXTTRIGR> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGR) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTTRIG` reader - External trigger edge event
pub type EXTTRIG_R = crate::BitReader<EXTTRIGR>;
impl EXTTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTTRIGR> {
        match self.bits {
            true => Some(EXTTRIGR::Set),
            _ => None,
        }
    }
    ///External trigger edge event
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EXTTRIGR::Set
    }
}
/**Compare register update OK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKR {
    ///1: Compare register update OK
    Set = 1,
}
impl From<CMPOKR> for bool {
    #[inline(always)]
    fn from(variant: CMPOKR) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOK` reader - Compare register update OK
pub type CMPOK_R = crate::BitReader<CMPOKR>;
impl CMPOK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMPOKR> {
        match self.bits {
            true => Some(CMPOKR::Set),
            _ => None,
        }
    }
    ///Compare register update OK
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPOKR::Set
    }
}
/**Autoreload register update OK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKR {
    ///1: Autoreload register update OK
    Set = 1,
}
impl From<ARROKR> for bool {
    #[inline(always)]
    fn from(variant: ARROKR) -> Self {
        variant as u8 != 0
    }
}
///Field `ARROK` reader - Autoreload register update OK
pub type ARROK_R = crate::BitReader<ARROKR>;
impl ARROK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARROKR> {
        match self.bits {
            true => Some(ARROKR::Set),
            _ => None,
        }
    }
    ///Autoreload register update OK
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARROKR::Set
    }
}
/**Counter direction change down to up

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPR {
    ///1: Counter direction change down to up
    Set = 1,
}
impl From<UPR> for bool {
    #[inline(always)]
    fn from(variant: UPR) -> Self {
        variant as u8 != 0
    }
}
///Field `UP` reader - Counter direction change down to up
pub type UP_R = crate::BitReader<UPR>;
impl UP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPR> {
        match self.bits {
            true => Some(UPR::Set),
            _ => None,
        }
    }
    ///Counter direction change down to up
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UPR::Set
    }
}
/**Counter direction change up to down

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNR {
    ///1: Counter direction change up to down
    Set = 1,
}
impl From<DOWNR> for bool {
    #[inline(always)]
    fn from(variant: DOWNR) -> Self {
        variant as u8 != 0
    }
}
///Field `DOWN` reader - Counter direction change up to down
pub type DOWN_R = crate::BitReader<DOWNR>;
impl DOWN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DOWNR> {
        match self.bits {
            true => Some(DOWNR::Set),
            _ => None,
        }
    }
    ///Counter direction change up to down
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DOWNR::Set
    }
}
/**LPTIM update event occurred

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UER {
    ///1: LPTIM update event occurred
    Set = 1,
}
impl From<UER> for bool {
    #[inline(always)]
    fn from(variant: UER) -> Self {
        variant as u8 != 0
    }
}
///Field `UE` reader - LPTIM update event occurred
pub type UE_R = crate::BitReader<UER>;
impl UE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UER> {
        match self.bits {
            true => Some(UER::Set),
            _ => None,
        }
    }
    ///LPTIM update event occurred
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UER::Set
    }
}
/**Repetition register update Ok

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPOKR {
    ///1: Repetition register update OK
    Set = 1,
}
impl From<REPOKR> for bool {
    #[inline(always)]
    fn from(variant: REPOKR) -> Self {
        variant as u8 != 0
    }
}
///Field `REPOK` reader - Repetition register update Ok
pub type REPOK_R = crate::BitReader<REPOKR>;
impl REPOK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<REPOKR> {
        match self.bits {
            true => Some(REPOKR::Set),
            _ => None,
        }
    }
    ///Repetition register update OK
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == REPOKR::Set
    }
}
impl R {
    ///Bit 0 - Compare match
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger edge event
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare register update OK
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Counter direction change down to up
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Counter direction change up to down
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LPTIM update event occurred
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update Ok
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("repok", &self.repok())
            .field("ue", &self.ue())
            .field("down", &self.down())
            .field("up", &self.up())
            .field("arrok", &self.arrok())
            .field("cmpok", &self.cmpok())
            .field("exttrig", &self.exttrig())
            .field("arrm", &self.arrm())
            .field("cmpm", &self.cmpm())
            .finish()
    }
}
/**interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#LPTIM1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
