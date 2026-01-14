///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Low-power deep sleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDS {
    ///0: Main voltage regulator ON during Stop mode
    Main = 0,
    ///1: Low-power voltage regulator ON during Stop mode
    LowPower = 1,
}
impl From<LPDS> for bool {
    #[inline(always)]
    fn from(variant: LPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `LPDS` reader - Low-power deep sleep
pub type LPDS_R = crate::BitReader<LPDS>;
impl LPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPDS {
        match self.bits {
            false => LPDS::Main,
            true => LPDS::LowPower,
        }
    }
    ///Main voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == LPDS::Main
    }
    ///Low-power voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == LPDS::LowPower
    }
}
///Field `LPDS` writer - Low-power deep sleep
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG, LPDS>;
impl<'a, REG> LPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn main(self) -> &'a mut crate::W<REG> {
        self.variant(LPDS::Main)
    }
    ///Low-power voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(LPDS::LowPower)
    }
}
/**Power down deepsleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS {
    ///0: Enter Stop mode when the CPU enters deepsleep. The regulator status depends on the LPDS bit
    EnterStop = 0,
    ///1: Enter Standby mode when the CPU enters deepsleep
    EnterStandby = 1,
}
impl From<PDDS> for bool {
    #[inline(always)]
    fn from(variant: PDDS) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDS` reader - Power down deepsleep
pub type PDDS_R = crate::BitReader<PDDS>;
impl PDDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDDS {
        match self.bits {
            false => PDDS::EnterStop,
            true => PDDS::EnterStandby,
        }
    }
    ///Enter Stop mode when the CPU enters deepsleep. The regulator status depends on the LPDS bit
    #[inline(always)]
    pub fn is_enter_stop(&self) -> bool {
        *self == PDDS::EnterStop
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn is_enter_standby(&self) -> bool {
        *self == PDDS::EnterStandby
    }
}
///Field `PDDS` writer - Power down deepsleep
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG, PDDS>;
impl<'a, REG> PDDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enter Stop mode when the CPU enters deepsleep. The regulator status depends on the LPDS bit
    #[inline(always)]
    pub fn enter_stop(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::EnterStop)
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn enter_standby(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::EnterStandby)
    }
}
/**Clear wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUFR {
    ///0: This bit is always read as 0
    Zero = 0,
}
impl From<CWUFR> for bool {
    #[inline(always)]
    fn from(variant: CWUFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF` reader - Clear wakeup flag
pub type CWUF_R = crate::BitReader<CWUFR>;
impl CWUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CWUFR> {
        match self.bits {
            false => Some(CWUFR::Zero),
            _ => None,
        }
    }
    ///This bit is always read as 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CWUFR::Zero
    }
}
/**Clear wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUFW {
    ///1: Clear the WUPF Wakeup Flag **after 2 System clock cycles**
    Clear = 1,
}
impl From<CWUFW> for bool {
    #[inline(always)]
    fn from(variant: CWUFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF` writer - Clear wakeup flag
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG, CWUFW>;
impl<'a, REG> CWUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the WUPF Wakeup Flag **after 2 System clock cycles**
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUFW::Clear)
    }
}
/**Clear standby flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBFR {
    ///0: This bit is always read as 0
    Zero = 0,
}
impl From<CSBFR> for bool {
    #[inline(always)]
    fn from(variant: CSBFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` reader - Clear standby flag
pub type CSBF_R = crate::BitReader<CSBFR>;
impl CSBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSBFR> {
        match self.bits {
            false => Some(CSBFR::Zero),
            _ => None,
        }
    }
    ///This bit is always read as 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSBFR::Zero
    }
}
/**Clear standby flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBFW {
    ///1: Clear the SBF Standby Flag
    Clear = 1,
}
impl From<CSBFW> for bool {
    #[inline(always)]
    fn from(variant: CSBFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG, CSBFW>;
impl<'a, REG> CSBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the SBF Standby Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSBFW::Clear)
    }
}
/**Power voltage detector enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    ///0: PVD disabled
    Disabled = 0,
    ///1: PVD enabled
    Enabled = 1,
}
impl From<PVDE> for bool {
    #[inline(always)]
    fn from(variant: PVDE) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader<PVDE>;
impl PVDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDE {
        match self.bits {
            false => PVDE::Disabled,
            true => PVDE::Enabled,
        }
    }
    ///PVD disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    ///PVD enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE::Enabled
    }
}
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    ///PVD enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
///Field `PLS` reader - PVD level selection
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - PVD level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Disable backup domain write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    ///0: Access to RTC and RTC Backup registers and backup SRAM disabled
    Protected = 0,
    ///1: Access to RTC and RTC Backup registers and backup SRAM enabled
    Writable = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Protected,
            true => DBP::Writable,
        }
    }
    ///Access to RTC and RTC Backup registers and backup SRAM disabled
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == DBP::Protected
    }
    ///Access to RTC and RTC Backup registers and backup SRAM enabled
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == DBP::Writable
    }
}
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access to RTC and RTC Backup registers and backup SRAM disabled
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Protected)
    }
    ///Access to RTC and RTC Backup registers and backup SRAM enabled
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Writable)
    }
}
/**Flash power down in Stop mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDS {
    ///0: Flash memory not in power-down when the device is in Stop mode
    Idle = 0,
    ///1: Flash memory in power-down when the device is in Stop mode
    PowerDown = 1,
}
impl From<FPDS> for bool {
    #[inline(always)]
    fn from(variant: FPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `FPDS` reader - Flash power down in Stop mode
pub type FPDS_R = crate::BitReader<FPDS>;
impl FPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPDS {
        match self.bits {
            false => FPDS::Idle,
            true => FPDS::PowerDown,
        }
    }
    ///Flash memory not in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDS::Idle
    }
    ///Flash memory in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDS::PowerDown
    }
}
///Field `FPDS` writer - Flash power down in Stop mode
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG, FPDS>;
impl<'a, REG> FPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory not in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::Idle)
    }
    ///Flash memory in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::PowerDown)
    }
}
/**Regulator voltage scaling output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOS {
    ///0: Scale 2 mode
    Scale2 = 0,
    ///1: Scale 1 mode (default value at reset)
    Scale1 = 1,
}
impl From<VOS> for bool {
    #[inline(always)]
    fn from(variant: VOS) -> Self {
        variant as u8 != 0
    }
}
///Field `VOS` reader - Regulator voltage scaling output selection
pub type VOS_R = crate::BitReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOS {
        match self.bits {
            false => VOS::Scale2,
            true => VOS::Scale1,
        }
    }
    ///Scale 2 mode
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == VOS::Scale2
    }
    ///Scale 1 mode (default value at reset)
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == VOS::Scale1
    }
}
///Field `VOS` writer - Regulator voltage scaling output selection
pub type VOS_W<'a, REG> = crate::BitWriter<'a, REG, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Scale 2 mode
    #[inline(always)]
    pub fn scale2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale2)
    }
    ///Scale 1 mode (default value at reset)
    #[inline(always)]
    pub fn scale1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale1)
    }
}
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("fpds", &self.fpds())
            .field("dbp", &self.dbp())
            .field("pls", &self.pls())
            .field("pvde", &self.pvde())
            .field("csbf", &self.csbf())
            .field("cwuf", &self.cwuf())
            .field("pdds", &self.pdds())
            .field("lpds", &self.lpds())
            .field("vos", &self.vos())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<'_, CRrs> {
        LPDS_W::new(self, 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CRrs> {
        PDDS_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W<'_, CRrs> {
        CWUF_W::new(self, 2)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, CRrs> {
        CSBF_W::new(self, 3)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CRrs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CRrs> {
        DBP_W::new(self, 8)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W<'_, CRrs> {
        FPDS_W::new(self, 9)
    }
    ///Bit 14 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CRrs> {
        VOS_W::new(self, 14)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F405.html#PWR:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
