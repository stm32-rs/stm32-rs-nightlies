#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFR {
    #[doc = "0: No wakeup event occurred"]
    NoWakeupEvent = 0,
    #[doc = "1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)"]
    WakeupEvent = 1,
}
impl From<WUFR> for bool {
    #[inline(always)]
    fn from(variant: WUFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader<WUFR>;
impl WUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUFR {
        match self.bits {
            false => WUFR::NoWakeupEvent,
            true => WUFR::WakeupEvent,
        }
    }
    #[doc = "No wakeup event occurred"]
    #[inline(always)]
    pub fn is_no_wakeup_event(&self) -> bool {
        *self == WUFR::NoWakeupEvent
    }
    #[doc = "A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)"]
    #[inline(always)]
    pub fn is_wakeup_event(&self) -> bool {
        *self == WUFR::WakeupEvent
    }
}
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBFR {
    #[doc = "0: Device has not been in Standby mode"]
    NoStandbyEvent = 0,
    #[doc = "1: Device has been in Standby mode"]
    StandbyEvent = 1,
}
impl From<SBFR> for bool {
    #[inline(always)]
    fn from(variant: SBFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader<SBFR>;
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBFR {
        match self.bits {
            false => SBFR::NoStandbyEvent,
            true => SBFR::StandbyEvent,
        }
    }
    #[doc = "Device has not been in Standby mode"]
    #[inline(always)]
    pub fn is_no_standby_event(&self) -> bool {
        *self == SBFR::NoStandbyEvent
    }
    #[doc = "Device has been in Standby mode"]
    #[inline(always)]
    pub fn is_standby_event(&self) -> bool {
        *self == SBFR::StandbyEvent
    }
}
#[doc = "PVD output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDOR {
    #[doc = "0: VDD is higher than the PVD threshold selected with the PLS\\[2:0\\]
bits"]
    AboveThreshold = 0,
    #[doc = "1: VDD is lower than the PVD threshold selected with the PLS\\[2:0\\]
bits"]
    BelowThreshold = 1,
}
impl From<PVDOR> for bool {
    #[inline(always)]
    fn from(variant: PVDOR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDO` reader - PVD output"]
pub type PVDO_R = crate::BitReader<PVDOR>;
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDOR {
        match self.bits {
            false => PVDOR::AboveThreshold,
            true => PVDOR::BelowThreshold,
        }
    }
    #[doc = "VDD is higher than the PVD threshold selected with the PLS\\[2:0\\]
bits"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == PVDOR::AboveThreshold
    }
    #[doc = "VDD is lower than the PVD threshold selected with the PLS\\[2:0\\]
bits"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == PVDOR::BelowThreshold
    }
}
#[doc = "Internal voltage reference ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFINTRDYFR {
    #[doc = "0: VREFINT is OFF"]
    NotReady = 0,
    #[doc = "1: VREFINT is ready"]
    Ready = 1,
}
impl From<VREFINTRDYFR> for bool {
    #[inline(always)]
    fn from(variant: VREFINTRDYFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFINTRDYF` reader - Internal voltage reference ready flag"]
pub type VREFINTRDYF_R = crate::BitReader<VREFINTRDYFR>;
impl VREFINTRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFINTRDYFR {
        match self.bits {
            false => VREFINTRDYFR::NotReady,
            true => VREFINTRDYFR::Ready,
        }
    }
    #[doc = "VREFINT is OFF"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VREFINTRDYFR::NotReady
    }
    #[doc = "VREFINT is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VREFINTRDYFR::Ready
    }
}
#[doc = "Voltage Scaling select flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSFR {
    #[doc = "0: Regulator is ready in the selected voltage range"]
    Ready = 0,
    #[doc = "1: Regulator voltage output is changing to the required VOS level"]
    NotReady = 1,
}
impl From<VOSFR> for bool {
    #[inline(always)]
    fn from(variant: VOSFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOSF` reader - Voltage Scaling select flag"]
pub type VOSF_R = crate::BitReader<VOSFR>;
impl VOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOSFR {
        match self.bits {
            false => VOSFR::Ready,
            true => VOSFR::NotReady,
        }
    }
    #[doc = "Regulator is ready in the selected voltage range"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSFR::Ready
    }
    #[doc = "Regulator voltage output is changing to the required VOS level"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSFR::NotReady
    }
}
#[doc = "Regulator LP flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPFR {
    #[doc = "0: Regulator is ready in Main mode"]
    Ready = 0,
    #[doc = "1: Regulator voltage is in low-power mode"]
    NotReady = 1,
}
impl From<REGLPFR> for bool {
    #[inline(always)]
    fn from(variant: REGLPFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPF` reader - Regulator LP flag"]
pub type REGLPF_R = crate::BitReader<REGLPFR>;
impl REGLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REGLPFR {
        match self.bits {
            false => REGLPFR::Ready,
            true => REGLPFR::NotReady,
        }
    }
    #[doc = "Regulator is ready in Main mode"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPFR::Ready
    }
    #[doc = "Regulator voltage is in low-power mode"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPFR::NotReady
    }
}
#[doc = "Enable WKUP pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1 {
    #[doc = "0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP1> for bool {
    #[inline(always)]
    fn from(variant: EWUP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWUP1` reader - Enable WKUP pin 1"]
pub type EWUP1_R = crate::BitReader<EWUP1>;
impl EWUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWUP1 {
        match self.bits {
            false => EWUP1::Disabled,
            true => EWUP1::Enabled,
        }
    }
    #[doc = "WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1::Disabled
    }
    #[doc = "WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1::Enabled
    }
}
#[doc = "Field `EWUP1` writer - Enable WKUP pin 1"]
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG, EWUP1>;
impl<'a, REG> EWUP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Disabled)
    }
    #[doc = "WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Enabled)
    }
}
#[doc = "Enable WKUP pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP2 {
    #[doc = "0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP2> for bool {
    #[inline(always)]
    fn from(variant: EWUP2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWUP2` reader - Enable WKUP pin 2"]
pub type EWUP2_R = crate::BitReader<EWUP2>;
impl EWUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWUP2 {
        match self.bits {
            false => EWUP2::Disabled,
            true => EWUP2::Enabled,
        }
    }
    #[doc = "WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP2::Disabled
    }
    #[doc = "WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP2::Enabled
    }
}
#[doc = "Field `EWUP2` writer - Enable WKUP pin 2"]
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG, EWUP2>;
impl<'a, REG> EWUP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::Disabled)
    }
    #[doc = "WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::Enabled)
    }
}
#[doc = "Enable WKUP pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP3 {
    #[doc = "0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP3> for bool {
    #[inline(always)]
    fn from(variant: EWUP3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWUP3` reader - Enable WKUP pin 3"]
pub type EWUP3_R = crate::BitReader<EWUP3>;
impl EWUP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWUP3 {
        match self.bits {
            false => EWUP3::Disabled,
            true => EWUP3::Enabled,
        }
    }
    #[doc = "WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP3::Disabled
    }
    #[doc = "WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP3::Enabled
    }
}
#[doc = "Field `EWUP3` writer - Enable WKUP pin 3"]
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG, EWUP3>;
impl<'a, REG> EWUP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::Disabled)
    }
    #[doc = "WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal voltage reference ready flag"]
    #[inline(always)]
    pub fn vrefintrdyf(&self) -> VREFINTRDYF_R {
        VREFINTRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Voltage Scaling select flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Regulator LP flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable WKUP pin 3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<CSRrs> {
        EWUP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<CSRrs> {
        EWUP2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable WKUP pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<CSRrs> {
        EWUP3_W::new(self, 10)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
