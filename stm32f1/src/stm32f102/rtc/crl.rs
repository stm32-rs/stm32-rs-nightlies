#[doc = "Register `CRL` reader"]
pub type R = crate::R<CRLrs>;
#[doc = "Register `CRL` writer"]
pub type W = crate::W<CRLrs>;
#[doc = "Second Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECFR {
    #[doc = "0: Second flag condition not met"]
    NoPrescalerOverflow = 0,
    #[doc = "1: Second flag condition met"]
    PrescalerOverflow = 1,
}
impl From<SECFR> for bool {
    #[inline(always)]
    fn from(variant: SECFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECF` reader - Second Flag"]
pub type SECF_R = crate::BitReader<SECFR>;
impl SECF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECFR {
        match self.bits {
            false => SECFR::NoPrescalerOverflow,
            true => SECFR::PrescalerOverflow,
        }
    }
    #[doc = "Second flag condition not met"]
    #[inline(always)]
    pub fn is_no_prescaler_overflow(&self) -> bool {
        *self == SECFR::NoPrescalerOverflow
    }
    #[doc = "Second flag condition met"]
    #[inline(always)]
    pub fn is_prescaler_overflow(&self) -> bool {
        *self == SECFR::PrescalerOverflow
    }
}
#[doc = "Second Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SECFW> for bool {
    #[inline(always)]
    fn from(variant: SECFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECF` writer - Second Flag"]
pub type SECF_W<'a, REG> = crate::BitWriter0C<'a, REG, SECFW>;
impl<'a, REG> SECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SECFW::Clear)
    }
}
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRFR {
    #[doc = "0: Alarm not detected"]
    NoAlarm = 0,
    #[doc = "1: Alarm detected"]
    Alarm = 1,
}
impl From<ALRFR> for bool {
    #[inline(always)]
    fn from(variant: ALRFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRF` reader - Alarm Flag"]
pub type ALRF_R = crate::BitReader<ALRFR>;
impl ALRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRFR {
        match self.bits {
            false => ALRFR::NoAlarm,
            true => ALRFR::Alarm,
        }
    }
    #[doc = "Alarm not detected"]
    #[inline(always)]
    pub fn is_no_alarm(&self) -> bool {
        *self == ALRFR::NoAlarm
    }
    #[doc = "Alarm detected"]
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == ALRFR::Alarm
    }
}
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<ALRFW> for bool {
    #[inline(always)]
    fn from(variant: ALRFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRF` writer - Alarm Flag"]
pub type ALRF_W<'a, REG> = crate::BitWriter0C<'a, REG, ALRFW>;
impl<'a, REG> ALRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ALRFW::Clear)
    }
}
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWFR {
    #[doc = "0: Overflow not detected"]
    NoOverflow = 0,
    #[doc = "1: 32-bit programmable counter overflow occurred"]
    Overflow = 1,
}
impl From<OWFR> for bool {
    #[inline(always)]
    fn from(variant: OWFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWF` reader - Overflow Flag"]
pub type OWF_R = crate::BitReader<OWFR>;
impl OWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OWFR {
        match self.bits {
            false => OWFR::NoOverflow,
            true => OWFR::Overflow,
        }
    }
    #[doc = "Overflow not detected"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == OWFR::NoOverflow
    }
    #[doc = "32-bit programmable counter overflow occurred"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OWFR::Overflow
    }
}
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<OWFW> for bool {
    #[inline(always)]
    fn from(variant: OWFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWF` writer - Overflow Flag"]
pub type OWF_W<'a, REG> = crate::BitWriter0C<'a, REG, OWFW>;
impl<'a, REG> OWF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OWFW::Clear)
    }
}
#[doc = "Registers Synchronized Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR {
    #[doc = "0: Registers not yet synchronized"]
    NotSynchronized = 0,
    #[doc = "1: Registers synchronized"]
    Synchronized = 1,
}
impl From<RSFR> for bool {
    #[inline(always)]
    fn from(variant: RSFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers Synchronized Flag"]
pub type RSF_R = crate::BitReader<RSFR>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSFR {
        match self.bits {
            false => RSFR::NotSynchronized,
            true => RSFR::Synchronized,
        }
    }
    #[doc = "Registers not yet synchronized"]
    #[inline(always)]
    pub fn is_not_synchronized(&self) -> bool {
        *self == RSFR::NotSynchronized
    }
    #[doc = "Registers synchronized"]
    #[inline(always)]
    pub fn is_synchronized(&self) -> bool {
        *self == RSFR::Synchronized
    }
}
#[doc = "Registers Synchronized Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<RSFW> for bool {
    #[inline(always)]
    fn from(variant: RSFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers Synchronized Flag"]
pub type RSF_W<'a, REG> = crate::BitWriter0C<'a, REG, RSFW>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RSFW::Clear)
    }
}
#[doc = "Configuration Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNF {
    #[doc = "0: Exit configuration mode (start update of RTC registers)"]
    Exit = 0,
    #[doc = "1: Enter configuration mode"]
    Enter = 1,
}
impl From<CNF> for bool {
    #[inline(always)]
    fn from(variant: CNF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNF` reader - Configuration Flag"]
pub type CNF_R = crate::BitReader<CNF>;
impl CNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNF {
        match self.bits {
            false => CNF::Exit,
            true => CNF::Enter,
        }
    }
    #[doc = "Exit configuration mode (start update of RTC registers)"]
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        *self == CNF::Exit
    }
    #[doc = "Enter configuration mode"]
    #[inline(always)]
    pub fn is_enter(&self) -> bool {
        *self == CNF::Enter
    }
}
#[doc = "Field `CNF` writer - Configuration Flag"]
pub type CNF_W<'a, REG> = crate::BitWriter<'a, REG, CNF>;
impl<'a, REG> CNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exit configuration mode (start update of RTC registers)"]
    #[inline(always)]
    pub fn exit(self) -> &'a mut crate::W<REG> {
        self.variant(CNF::Exit)
    }
    #[doc = "Enter configuration mode"]
    #[inline(always)]
    pub fn enter(self) -> &'a mut crate::W<REG> {
        self.variant(CNF::Enter)
    }
}
#[doc = "RTC operation OFF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOFF {
    #[doc = "0: Last write operation on RTC registers is still ongoing"]
    Enabled = 0,
    #[doc = "1: Last write operation on RTC registers terminated"]
    Disabled = 1,
}
impl From<RTOFF> for bool {
    #[inline(always)]
    fn from(variant: RTOFF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOFF` reader - RTC operation OFF"]
pub type RTOFF_R = crate::BitReader<RTOFF>;
impl RTOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTOFF {
        match self.bits {
            false => RTOFF::Enabled,
            true => RTOFF::Disabled,
        }
    }
    #[doc = "Last write operation on RTC registers is still ongoing"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOFF::Enabled
    }
    #[doc = "Last write operation on RTC registers terminated"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOFF::Disabled
    }
}
impl R {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    #[must_use]
    pub fn secf(&mut self) -> SECF_W<CRLrs> {
        SECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrf(&mut self) -> ALRF_W<CRLrs> {
        ALRF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn owf(&mut self) -> OWF_W<CRLrs> {
        OWF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<CRLrs> {
        RSF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnf(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 4)
    }
}
#[doc = "RTC Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRLrs;
impl crate::RegisterSpec for CRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crl::R`](R) reader structure"]
impl crate::Readable for CRLrs {}
#[doc = "`write(|w| ..)` method takes [`crl::W`](W) writer structure"]
impl crate::Writable for CRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRL to value 0x20"]
impl crate::Resettable for CRLrs {
    const RESET_VALUE: u32 = 0x20;
}
