///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**Wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF {
    ///0: No wakeup event occurred
    NotOccurred = 0,
    ///1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)
    Occurred = 1,
}
impl From<WUF> for bool {
    #[inline(always)]
    fn from(variant: WUF) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF` reader - Wakeup flag
pub type WUF_R = crate::BitReader<WUF>;
impl WUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF {
        match self.bits {
            false => WUF::NotOccurred,
            true => WUF::Occurred,
        }
    }
    ///No wakeup event occurred
    #[inline(always)]
    pub fn is_not_occurred(&self) -> bool {
        *self == WUF::NotOccurred
    }
    ///A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == WUF::Occurred
    }
}
/**Standby flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBF {
    ///0: Device has not been in Standby mode
    InStandby = 0,
    ///1: Device has been in Standby mode
    NotInStandby = 1,
}
impl From<SBF> for bool {
    #[inline(always)]
    fn from(variant: SBF) -> Self {
        variant as u8 != 0
    }
}
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader<SBF>;
impl SBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBF {
        match self.bits {
            false => SBF::InStandby,
            true => SBF::NotInStandby,
        }
    }
    ///Device has not been in Standby mode
    #[inline(always)]
    pub fn is_in_standby(&self) -> bool {
        *self == SBF::InStandby
    }
    ///Device has been in Standby mode
    #[inline(always)]
    pub fn is_not_in_standby(&self) -> bool {
        *self == SBF::NotInStandby
    }
}
/**PVD output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO {
    ///0: Vdd is higher than the PVD threshold selected with the PLS\[2:0\] bits
    Higher = 0,
    ///1: Vdd is lower than the PVD threshold selected with the PLS\[2:0\] bits
    Lower = 1,
}
impl From<PVDO> for bool {
    #[inline(always)]
    fn from(variant: PVDO) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDO` reader - PVD output
pub type PVDO_R = crate::BitReader<PVDO>;
impl PVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDO {
        match self.bits {
            false => PVDO::Higher,
            true => PVDO::Lower,
        }
    }
    ///Vdd is higher than the PVD threshold selected with the PLS\[2:0\] bits
    #[inline(always)]
    pub fn is_higher(&self) -> bool {
        *self == PVDO::Higher
    }
    ///Vdd is lower than the PVD threshold selected with the PLS\[2:0\] bits
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        *self == PVDO::Lower
    }
}
/**Backup regulator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRR {
    ///0: Backup Regulator not ready
    NotReady = 0,
    ///1: Backup Regulator ready
    Ready = 1,
}
impl From<BRR> for bool {
    #[inline(always)]
    fn from(variant: BRR) -> Self {
        variant as u8 != 0
    }
}
///Field `BRR` reader - Backup regulator ready
pub type BRR_R = crate::BitReader<BRR>;
impl BRR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRR {
        match self.bits {
            false => BRR::NotReady,
            true => BRR::Ready,
        }
    }
    ///Backup Regulator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == BRR::NotReady
    }
    ///Backup Regulator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BRR::Ready
    }
}
/**Enable WKUP1 pin (PC1)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP3 {
    ///0: WKUP3 pin is used for general purpose I/O. An event on the WKUP3 pin does not wakeup the device from Standby mode
    Gpio = 0,
    ///1: WKUP3 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP3 pin wakes-up the system from Standby mode)
    WakeUp = 1,
}
impl From<EWUP3> for bool {
    #[inline(always)]
    fn from(variant: EWUP3) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP3` reader - Enable WKUP1 pin (PC1)
pub type EWUP3_R = crate::BitReader<EWUP3>;
impl EWUP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP3 {
        match self.bits {
            false => EWUP3::Gpio,
            true => EWUP3::WakeUp,
        }
    }
    ///WKUP3 pin is used for general purpose I/O. An event on the WKUP3 pin does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == EWUP3::Gpio
    }
    ///WKUP3 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP3 pin wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_wake_up(&self) -> bool {
        *self == EWUP3::WakeUp
    }
}
///Field `EWUP3` writer - Enable WKUP1 pin (PC1)
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG, EWUP3>;
impl<'a, REG> EWUP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP3 pin is used for general purpose I/O. An event on the WKUP3 pin does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::Gpio)
    }
    ///WKUP3 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP3 pin wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::WakeUp)
    }
}
/**Enable WKUP1 pin (PC0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP2 {
    ///0: WKUP2 pin is used for general purpose I/O. An event on the WKUP2 pin does not wakeup the device from Standby mode
    Gpio = 0,
    ///1: WKUP2 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP2 pin wakes-up the system from Standby mode)
    WakeUp = 1,
}
impl From<EWUP2> for bool {
    #[inline(always)]
    fn from(variant: EWUP2) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP2` reader - Enable WKUP1 pin (PC0)
pub type EWUP2_R = crate::BitReader<EWUP2>;
impl EWUP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP2 {
        match self.bits {
            false => EWUP2::Gpio,
            true => EWUP2::WakeUp,
        }
    }
    ///WKUP2 pin is used for general purpose I/O. An event on the WKUP2 pin does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == EWUP2::Gpio
    }
    ///WKUP2 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP2 pin wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_wake_up(&self) -> bool {
        *self == EWUP2::WakeUp
    }
}
///Field `EWUP2` writer - Enable WKUP1 pin (PC0)
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG, EWUP2>;
impl<'a, REG> EWUP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP2 pin is used for general purpose I/O. An event on the WKUP2 pin does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::Gpio)
    }
    ///WKUP2 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP2 pin wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::WakeUp)
    }
}
/**Enable WKUP1 pin (PA0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1 {
    ///0: WKUP1 pin is used for general purpose I/O. An event on the WKUP1 pin does not wakeup the device from Standby mode
    Gpio = 0,
    ///1: WKUP1 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP1 pin wakes-up the system from Standby mode)
    WakeUp = 1,
}
impl From<EWUP1> for bool {
    #[inline(always)]
    fn from(variant: EWUP1) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP1` reader - Enable WKUP1 pin (PA0)
pub type EWUP1_R = crate::BitReader<EWUP1>;
impl EWUP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP1 {
        match self.bits {
            false => EWUP1::Gpio,
            true => EWUP1::WakeUp,
        }
    }
    ///WKUP1 pin is used for general purpose I/O. An event on the WKUP1 pin does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == EWUP1::Gpio
    }
    ///WKUP1 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP1 pin wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_wake_up(&self) -> bool {
        *self == EWUP1::WakeUp
    }
}
///Field `EWUP1` writer - Enable WKUP1 pin (PA0)
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG, EWUP1>;
impl<'a, REG> EWUP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP1 pin is used for general purpose I/O. An event on the WKUP1 pin does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Gpio)
    }
    ///WKUP1 pin is used for wakeup from Standby mode and forced in input pull down configuration (rising edge or falling on WKUP1 pin wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::WakeUp)
    }
}
/**Backup regulator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRE {
    ///0: Backup regulator disabled
    Disabled = 0,
    ///1: Backup regulator enabled
    Enabled = 1,
}
impl From<BRE> for bool {
    #[inline(always)]
    fn from(variant: BRE) -> Self {
        variant as u8 != 0
    }
}
///Field `BRE` reader - Backup regulator enable
pub type BRE_R = crate::BitReader<BRE>;
impl BRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRE {
        match self.bits {
            false => BRE::Disabled,
            true => BRE::Enabled,
        }
    }
    ///Backup regulator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRE::Disabled
    }
    ///Backup regulator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRE::Enabled
    }
}
///Field `BRE` writer - Backup regulator enable
pub type BRE_W<'a, REG> = crate::BitWriter<'a, REG, BRE>;
impl<'a, REG> BRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Backup regulator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRE::Disabled)
    }
    ///Backup regulator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRE::Enabled)
    }
}
/**Regulator voltage scaling output selection ready bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSRDY {
    ///0: Not ready
    NotReady = 0,
    ///1: Ready
    Ready = 1,
}
impl From<VOSRDY> for bool {
    #[inline(always)]
    fn from(variant: VOSRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit
pub type VOSRDY_R = crate::BitReader<VOSRDY>;
impl VOSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOSRDY {
        match self.bits {
            false => VOSRDY::NotReady,
            true => VOSRDY::Ready,
        }
    }
    ///Not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSRDY::NotReady
    }
    ///Ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSRDY::Ready
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
    ///Bit 3 - Backup regulator ready
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Enable WKUP1 pin (PC1)
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable WKUP1 pin (PC0)
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Enable WKUP1 pin (PA0)
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - Regulator voltage scaling output selection ready bit
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("wuf", &self.wuf())
            .field("sbf", &self.sbf())
            .field("pvdo", &self.pvdo())
            .field("brr", &self.brr())
            .field("ewup1", &self.ewup1())
            .field("bre", &self.bre())
            .field("vosrdy", &self.vosrdy())
            .field("ewup2", &self.ewup2())
            .field("ewup3", &self.ewup3())
            .finish()
    }
}
impl W {
    ///Bit 6 - Enable WKUP1 pin (PC1)
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, CSRrs> {
        EWUP3_W::new(self, 6)
    }
    ///Bit 7 - Enable WKUP1 pin (PC0)
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, CSRrs> {
        EWUP2_W::new(self, 7)
    }
    ///Bit 8 - Enable WKUP1 pin (PA0)
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, CSRrs> {
        EWUP1_W::new(self, 8)
    }
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W<'_, CSRrs> {
        BRE_W::new(self, 9)
    }
}
/**power control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#PWR:CSR)*/
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
