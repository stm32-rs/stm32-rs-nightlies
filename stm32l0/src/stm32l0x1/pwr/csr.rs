///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**Wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFR {
    ///0: No wakeup event occurred
    NoWakeupEvent = 0,
    ///1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)
    WakeupEvent = 1,
}
impl From<WUFR> for bool {
    #[inline(always)]
    fn from(variant: WUFR) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF` reader - Wakeup flag
pub type WUF_R = crate::BitReader<WUFR>;
impl WUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUFR {
        match self.bits {
            false => WUFR::NoWakeupEvent,
            true => WUFR::WakeupEvent,
        }
    }
    ///No wakeup event occurred
    #[inline(always)]
    pub fn is_no_wakeup_event(&self) -> bool {
        *self == WUFR::NoWakeupEvent
    }
    ///A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)
    #[inline(always)]
    pub fn is_wakeup_event(&self) -> bool {
        *self == WUFR::WakeupEvent
    }
}
/**Standby flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBFR {
    ///0: Device has not been in Standby mode
    NoStandbyEvent = 0,
    ///1: Device has been in Standby mode
    StandbyEvent = 1,
}
impl From<SBFR> for bool {
    #[inline(always)]
    fn from(variant: SBFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader<SBFR>;
impl SBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBFR {
        match self.bits {
            false => SBFR::NoStandbyEvent,
            true => SBFR::StandbyEvent,
        }
    }
    ///Device has not been in Standby mode
    #[inline(always)]
    pub fn is_no_standby_event(&self) -> bool {
        *self == SBFR::NoStandbyEvent
    }
    ///Device has been in Standby mode
    #[inline(always)]
    pub fn is_standby_event(&self) -> bool {
        *self == SBFR::StandbyEvent
    }
}
/**PVD output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDOR {
    ///0: VDD is higher than the PVD threshold selected with the PLS\[2:0\] bits
    AboveThreshold = 0,
    ///1: VDD is lower than the PVD threshold selected with the PLS\[2:0\] bits
    BelowThreshold = 1,
}
impl From<PVDOR> for bool {
    #[inline(always)]
    fn from(variant: PVDOR) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDO` reader - PVD output
pub type PVDO_R = crate::BitReader<PVDOR>;
impl PVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDOR {
        match self.bits {
            false => PVDOR::AboveThreshold,
            true => PVDOR::BelowThreshold,
        }
    }
    ///VDD is higher than the PVD threshold selected with the PLS\[2:0\] bits
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == PVDOR::AboveThreshold
    }
    ///VDD is lower than the PVD threshold selected with the PLS\[2:0\] bits
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == PVDOR::BelowThreshold
    }
}
/**Internal voltage reference (VREFINT) ready flag This bit indicates the state of the internal voltage reference, VREFINT.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFINTRDYFR {
    ///0: VREFINT is OFF
    NotReady = 0,
    ///1: VREFINT is ready
    Ready = 1,
}
impl From<VREFINTRDYFR> for bool {
    #[inline(always)]
    fn from(variant: VREFINTRDYFR) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFINTRDYF` reader - Internal voltage reference (VREFINT) ready flag This bit indicates the state of the internal voltage reference, VREFINT.
pub type VREFINTRDYF_R = crate::BitReader<VREFINTRDYFR>;
impl VREFINTRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFINTRDYFR {
        match self.bits {
            false => VREFINTRDYFR::NotReady,
            true => VREFINTRDYFR::Ready,
        }
    }
    ///VREFINT is OFF
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VREFINTRDYFR::NotReady
    }
    ///VREFINT is ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VREFINTRDYFR::Ready
    }
}
/**Voltage Scaling select flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSFR {
    ///0: Regulator is ready in the selected voltage range
    Ready = 0,
    ///1: Regulator voltage output is changing to the required VOS level
    NotReady = 1,
}
impl From<VOSFR> for bool {
    #[inline(always)]
    fn from(variant: VOSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `VOSF` reader - Voltage Scaling select flag
pub type VOSF_R = crate::BitReader<VOSFR>;
impl VOSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOSFR {
        match self.bits {
            false => VOSFR::Ready,
            true => VOSFR::NotReady,
        }
    }
    ///Regulator is ready in the selected voltage range
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSFR::Ready
    }
    ///Regulator voltage output is changing to the required VOS level
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSFR::NotReady
    }
}
/**Regulator LP flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPFR {
    ///0: Regulator is ready in Main mode
    Ready = 0,
    ///1: Regulator voltage is in low-power mode
    NotReady = 1,
}
impl From<REGLPFR> for bool {
    #[inline(always)]
    fn from(variant: REGLPFR) -> Self {
        variant as u8 != 0
    }
}
///Field `REGLPF` reader - Regulator LP flag
pub type REGLPF_R = crate::BitReader<REGLPFR>;
impl REGLPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGLPFR {
        match self.bits {
            false => REGLPFR::Ready,
            true => REGLPFR::NotReady,
        }
    }
    ///Regulator is ready in Main mode
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPFR::Ready
    }
    ///Regulator voltage is in low-power mode
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPFR::NotReady
    }
}
/**Enable WKUP pin 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1 {
    ///0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP1> for bool {
    #[inline(always)]
    fn from(variant: EWUP1) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP1` reader - Enable WKUP pin 1
pub type EWUP1_R = crate::BitReader<EWUP1>;
impl EWUP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP1 {
        match self.bits {
            false => EWUP1::Disabled,
            true => EWUP1::Enabled,
        }
    }
    ///WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1::Disabled
    }
    ///WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1::Enabled
    }
}
///Field `EWUP1` writer - Enable WKUP pin 1
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG, EWUP1>;
impl<'a, REG> EWUP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Disabled)
    }
    ///WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Enabled)
    }
}
/**Enable WKUP pin 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP2 {
    ///0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP2> for bool {
    #[inline(always)]
    fn from(variant: EWUP2) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP2` reader - Enable WKUP pin 2
pub type EWUP2_R = crate::BitReader<EWUP2>;
impl EWUP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP2 {
        match self.bits {
            false => EWUP2::Disabled,
            true => EWUP2::Enabled,
        }
    }
    ///WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP2::Disabled
    }
    ///WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP2::Enabled
    }
}
///Field `EWUP2` writer - Enable WKUP pin 2
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG, EWUP2>;
impl<'a, REG> EWUP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::Disabled)
    }
    ///WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::Enabled)
    }
}
/**Enable WKUP pin 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP3 {
    ///0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP3> for bool {
    #[inline(always)]
    fn from(variant: EWUP3) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP3` reader - Enable WKUP pin 3
pub type EWUP3_R = crate::BitReader<EWUP3>;
impl EWUP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP3 {
        match self.bits {
            false => EWUP3::Disabled,
            true => EWUP3::Enabled,
        }
    }
    ///WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP3::Disabled
    }
    ///WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP3::Enabled
    }
}
///Field `EWUP3` writer - Enable WKUP pin 3
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG, EWUP3>;
impl<'a, REG> EWUP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::Disabled)
    }
    ///WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::Enabled)
    }
}
impl R {
    ///Bit 0 - Wakeup flag
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Internal voltage reference (VREFINT) ready flag This bit indicates the state of the internal voltage reference, VREFINT.
    #[inline(always)]
    pub fn vrefintrdyf(&self) -> VREFINTRDYF_R {
        VREFINTRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Voltage Scaling select flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Regulator LP flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable WKUP pin 3
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("ewup3", &self.ewup3())
            .field("ewup2", &self.ewup2())
            .field("ewup1", &self.ewup1())
            .field("reglpf", &self.reglpf())
            .field("vosf", &self.vosf())
            .field("vrefintrdyf", &self.vrefintrdyf())
            .field("pvdo", &self.pvdo())
            .field("sbf", &self.sbf())
            .field("wuf", &self.wuf())
            .finish()
    }
}
impl W {
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, CSRrs> {
        EWUP1_W::new(self, 8)
    }
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, CSRrs> {
        EWUP2_W::new(self, 9)
    }
    ///Bit 10 - Enable WKUP pin 3
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, CSRrs> {
        EWUP3_W::new(self, 10)
    }
}
/**power control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#PWR:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
