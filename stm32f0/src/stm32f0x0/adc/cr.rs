///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**ADC enable command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR {
    ///0: ADC disabled
    Disabled = 0,
    ///1: ADC enabled
    Enabled = 1,
}
impl From<ADENR> for bool {
    #[inline(always)]
    fn from(variant: ADENR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` reader - ADC enable command
pub type ADEN_R = crate::BitReader<ADENR>;
impl ADEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADENR {
        match self.bits {
            false => ADENR::Disabled,
            true => ADENR::Enabled,
        }
    }
    ///ADC disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR::Disabled
    }
    ///ADC enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR::Enabled
    }
}
/**ADC enable command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW {
    ///1: Enable the ADC
    Enabled = 1,
}
impl From<ADENW> for bool {
    #[inline(always)]
    fn from(variant: ADENW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` writer - ADC enable command
pub type ADEN_W<'a, REG> = crate::BitWriter1S<'a, REG, ADENW>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable the ADC
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADENW::Enabled)
    }
}
/**ADC disable command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR {
    ///0: No disable command active
    NotDisabling = 0,
    ///1: ADC disabling
    Disabling = 1,
}
impl From<ADDISR> for bool {
    #[inline(always)]
    fn from(variant: ADDISR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` reader - ADC disable command
pub type ADDIS_R = crate::BitReader<ADDISR>;
impl ADDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDISR {
        match self.bits {
            false => ADDISR::NotDisabling,
            true => ADDISR::Disabling,
        }
    }
    ///No disable command active
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        *self == ADDISR::NotDisabling
    }
    ///ADC disabling
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        *self == ADDISR::Disabling
    }
}
/**ADC disable command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW {
    ///1: Disable the ADC
    Disable = 1,
}
impl From<ADDISW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` writer - ADC disable command
pub type ADDIS_W<'a, REG> = crate::BitWriter1S<'a, REG, ADDISW>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the ADC
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADDISW::Disable)
    }
}
/**ADC start conversion command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR {
    ///0: No conversion ongoing
    NotActive = 0,
    ///1: ADC operating and may be converting
    Active = 1,
}
impl From<ADSTARTR> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` reader - ADC start conversion command
pub type ADSTART_R = crate::BitReader<ADSTARTR>;
impl ADSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTARTR {
        match self.bits {
            false => ADSTARTR::NotActive,
            true => ADSTARTR::Active,
        }
    }
    ///No conversion ongoing
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR::NotActive
    }
    ///ADC operating and may be converting
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR::Active
    }
}
/**ADC start conversion command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW {
    ///1: Start the ADC conversion (may be delayed for hardware triggers)
    StartConversion = 1,
}
impl From<ADSTARTW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` writer - ADC start conversion command
pub type ADSTART_W<'a, REG> = crate::BitWriter1S<'a, REG, ADSTARTW>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start the ADC conversion (may be delayed for hardware triggers)
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTARTW::StartConversion)
    }
}
/**ADC stop conversion command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR {
    ///0: No stop command active
    NotStopping = 0,
    ///1: ADC stopping conversion
    Stopping = 1,
}
impl From<ADSTPR> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` reader - ADC stop conversion command
pub type ADSTP_R = crate::BitReader<ADSTPR>;
impl ADSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTPR {
        match self.bits {
            false => ADSTPR::NotStopping,
            true => ADSTPR::Stopping,
        }
    }
    ///No stop command active
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR::NotStopping
    }
    ///ADC stopping conversion
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR::Stopping
    }
}
/**ADC stop conversion command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW {
    ///1: Stop the active conversion
    StopConversion = 1,
}
impl From<ADSTPW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` writer - ADC stop conversion command
pub type ADSTP_W<'a, REG> = crate::BitWriter1S<'a, REG, ADSTPW>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop the active conversion
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTPW::StopConversion)
    }
}
/**ADC calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALR {
    ///0: ADC calibration either not yet performed or completed
    NotCalibrating = 0,
    ///1: ADC calibration in progress
    Calibrating = 1,
}
impl From<ADCALR> for bool {
    #[inline(always)]
    fn from(variant: ADCALR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` reader - ADC calibration
pub type ADCAL_R = crate::BitReader<ADCALR>;
impl ADCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALR {
        match self.bits {
            false => ADCALR::NotCalibrating,
            true => ADCALR::Calibrating,
        }
    }
    ///ADC calibration either not yet performed or completed
    #[inline(always)]
    pub fn is_not_calibrating(&self) -> bool {
        *self == ADCALR::NotCalibrating
    }
    ///ADC calibration in progress
    #[inline(always)]
    pub fn is_calibrating(&self) -> bool {
        *self == ADCALR::Calibrating
    }
}
/**ADC calibration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALW {
    ///1: Start the ADC calibration sequence
    StartCalibration = 1,
}
impl From<ADCALW> for bool {
    #[inline(always)]
    fn from(variant: ADCALW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` writer - ADC calibration
pub type ADCAL_W<'a, REG> = crate::BitWriter1S<'a, REG, ADCALW>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start the ADC calibration sequence
    #[inline(always)]
    pub fn start_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALW::StartCalibration)
    }
}
impl R {
    ///Bit 0 - ADC enable command
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC start conversion command
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - ADC stop conversion command
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("adcal", &self.adcal())
            .field("adstp", &self.adstp())
            .field("adstart", &self.adstart())
            .field("addis", &self.addis())
            .field("aden", &self.aden())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC enable command
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADC start conversion command
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 4 - ADC stop conversion command
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x0.html#ADC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_0017;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
