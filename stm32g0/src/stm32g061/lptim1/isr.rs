///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register's value.

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
///Field `CMPM` reader - Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register's value.
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
/**Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register's value reached the LPTIM_ARR register's value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.

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
///Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register's value reached the LPTIM_ARR register's value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
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
/**External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.

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
///Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
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
/**Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed.

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
///Field `CMPOK` reader - Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed.
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
/**Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.

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
///Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
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
/**Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .

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
///Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
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
/**Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .

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
///Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
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
impl R {
    ///Bit 0 - Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register's value.
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register's value reached the LPTIM_ARR register's value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed.
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("cmpm", &self.cmpm())
            .field("arrm", &self.arrm())
            .field("exttrig", &self.exttrig())
            .field("cmpok", &self.cmpok())
            .field("arrok", &self.arrok())
            .field("up", &self.up())
            .field("down", &self.down())
            .finish()
    }
}
/**Interrupt and Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
