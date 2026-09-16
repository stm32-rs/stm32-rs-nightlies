///Register `CRL` reader
pub type R = crate::R<CRLrs>;
///Register `CRL` writer
pub type W = crate::W<CRLrs>;
/**Second Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECFR {
    ///0: Second flag condition not met
    NoPrescalerOverflow = 0,
    ///1: Second flag condition met
    PrescalerOverflow = 1,
}
impl From<SECFR> for bool {
    #[inline(always)]
    fn from(variant: SECFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SECF` reader - Second Flag
pub type SECF_R = crate::BitReader<SECFR>;
impl SECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SECFR {
        match self.bits {
            false => SECFR::NoPrescalerOverflow,
            true => SECFR::PrescalerOverflow,
        }
    }
    ///Second flag condition not met
    #[inline(always)]
    pub fn is_no_prescaler_overflow(&self) -> bool {
        *self == SECFR::NoPrescalerOverflow
    }
    ///Second flag condition met
    #[inline(always)]
    pub fn is_prescaler_overflow(&self) -> bool {
        *self == SECFR::PrescalerOverflow
    }
}
/**Second Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLEAR {
    ///0: Clear flag
    Clear = 0,
}
impl From<CLEAR> for bool {
    #[inline(always)]
    fn from(variant: CLEAR) -> Self {
        variant as u8 != 0
    }
}
///Field `SECF` writer - Second Flag
pub type SECF_W<'a, REG> = crate::BitWriter0C<'a, REG, CLEAR>;
impl<'a, REG> SECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLEAR::Clear)
    }
}
/**Alarm Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRFR {
    ///0: Alarm not detected
    NoAlarm = 0,
    ///1: Alarm detected
    Alarm = 1,
}
impl From<ALRFR> for bool {
    #[inline(always)]
    fn from(variant: ALRFR) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRF` reader - Alarm Flag
pub type ALRF_R = crate::BitReader<ALRFR>;
impl ALRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRFR {
        match self.bits {
            false => ALRFR::NoAlarm,
            true => ALRFR::Alarm,
        }
    }
    ///Alarm not detected
    #[inline(always)]
    pub fn is_no_alarm(&self) -> bool {
        *self == ALRFR::NoAlarm
    }
    ///Alarm detected
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == ALRFR::Alarm
    }
}
///Field `ALRF` writer - Alarm Flag
pub use SECF_W as ALRF_W;
/**Overflow Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWFR {
    ///0: Overflow not detected
    NoOverflow = 0,
    ///1: 32-bit programmable counter overflow occurred
    Overflow = 1,
}
impl From<OWFR> for bool {
    #[inline(always)]
    fn from(variant: OWFR) -> Self {
        variant as u8 != 0
    }
}
///Field `OWF` reader - Overflow Flag
pub type OWF_R = crate::BitReader<OWFR>;
impl OWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OWFR {
        match self.bits {
            false => OWFR::NoOverflow,
            true => OWFR::Overflow,
        }
    }
    ///Overflow not detected
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == OWFR::NoOverflow
    }
    ///32-bit programmable counter overflow occurred
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OWFR::Overflow
    }
}
///Field `OWF` writer - Overflow Flag
pub use SECF_W as OWF_W;
/**Registers Synchronized Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR {
    ///0: Registers not yet synchronized
    NotSynchronized = 0,
    ///1: Registers synchronized
    Synchronized = 1,
}
impl From<RSFR> for bool {
    #[inline(always)]
    fn from(variant: RSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` reader - Registers Synchronized Flag
pub type RSF_R = crate::BitReader<RSFR>;
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSFR {
        match self.bits {
            false => RSFR::NotSynchronized,
            true => RSFR::Synchronized,
        }
    }
    ///Registers not yet synchronized
    #[inline(always)]
    pub fn is_not_synchronized(&self) -> bool {
        *self == RSFR::NotSynchronized
    }
    ///Registers synchronized
    #[inline(always)]
    pub fn is_synchronized(&self) -> bool {
        *self == RSFR::Synchronized
    }
}
///Field `RSF` writer - Registers Synchronized Flag
pub use SECF_W as RSF_W;
/**Configuration Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNF {
    ///0: Exit configuration mode (start update of RTC registers)
    Exit = 0,
    ///1: Enter configuration mode
    Enter = 1,
}
impl From<CNF> for bool {
    #[inline(always)]
    fn from(variant: CNF) -> Self {
        variant as u8 != 0
    }
}
///Field `CNF` reader - Configuration Flag
pub type CNF_R = crate::BitReader<CNF>;
impl CNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CNF {
        match self.bits {
            false => CNF::Exit,
            true => CNF::Enter,
        }
    }
    ///Exit configuration mode (start update of RTC registers)
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        *self == CNF::Exit
    }
    ///Enter configuration mode
    #[inline(always)]
    pub fn is_enter(&self) -> bool {
        *self == CNF::Enter
    }
}
///Field `CNF` writer - Configuration Flag
pub type CNF_W<'a, REG> = crate::BitWriter<'a, REG, CNF>;
impl<'a, REG> CNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Exit configuration mode (start update of RTC registers)
    #[inline(always)]
    pub fn exit(self) -> &'a mut crate::W<REG> {
        self.variant(CNF::Exit)
    }
    ///Enter configuration mode
    #[inline(always)]
    pub fn enter(self) -> &'a mut crate::W<REG> {
        self.variant(CNF::Enter)
    }
}
/**RTC operation OFF

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOFF {
    ///0: Last write operation on RTC registers is still ongoing
    Enabled = 0,
    ///1: Last write operation on RTC registers terminated
    Disabled = 1,
}
impl From<RTOFF> for bool {
    #[inline(always)]
    fn from(variant: RTOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOFF` reader - RTC operation OFF
pub type RTOFF_R = crate::BitReader<RTOFF>;
impl RTOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOFF {
        match self.bits {
            false => RTOFF::Enabled,
            true => RTOFF::Disabled,
        }
    }
    ///Last write operation on RTC registers is still ongoing
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOFF::Enabled
    }
    ///Last write operation on RTC registers terminated
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOFF::Disabled
    }
}
impl R {
    ///Bit 0 - Second Flag
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm Flag
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow Flag
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Registers Synchronized Flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configuration Flag
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RTC operation OFF
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRL")
            .field("secf", &self.secf())
            .field("alrf", &self.alrf())
            .field("owf", &self.owf())
            .field("rsf", &self.rsf())
            .field("cnf", &self.cnf())
            .field("rtoff", &self.rtoff())
            .finish()
    }
}
impl W {
    ///Bit 0 - Second Flag
    #[inline(always)]
    pub fn secf(&mut self) -> SECF_W<'_, CRLrs> {
        SECF_W::new(self, 0)
    }
    ///Bit 1 - Alarm Flag
    #[inline(always)]
    pub fn alrf(&mut self) -> ALRF_W<'_, CRLrs> {
        ALRF_W::new(self, 1)
    }
    ///Bit 2 - Overflow Flag
    #[inline(always)]
    pub fn owf(&mut self) -> OWF_W<'_, CRLrs> {
        OWF_W::new(self, 2)
    }
    ///Bit 3 - Registers Synchronized Flag
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, CRLrs> {
        RSF_W::new(self, 3)
    }
    ///Bit 4 - Configuration Flag
    #[inline(always)]
    pub fn cnf(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 4)
    }
}
/**RTC Control Register Low

You can [`read`](crate::Reg::read) this register and get [`crl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#RTC:CRL)*/
pub struct CRLrs;
impl crate::RegisterSpec for CRLrs {
    type Ux = u32;
}
///`read()` method returns [`crl::R`](R) reader structure
impl crate::Readable for CRLrs {}
///`write(|w| ..)` method takes [`crl::W`](W) writer structure
impl crate::Writable for CRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
///`reset()` method sets CRL to value 0x20
impl crate::Resettable for CRLrs {
    const RESET_VALUE: u32 = 0x20;
}
