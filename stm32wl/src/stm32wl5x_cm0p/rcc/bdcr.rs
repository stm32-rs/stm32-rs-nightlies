///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
/**LSE oscillator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON {
    ///0: LSE oscillator off
    Off = 0,
    ///1: LSE oscillator on
    On = 1,
}
impl From<LSEON> for bool {
    #[inline(always)]
    fn from(variant: LSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEON` reader - LSE oscillator enable
pub type LSEON_R = crate::BitReader<LSEON>;
impl LSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEON {
        match self.bits {
            false => LSEON::Off,
            true => LSEON::On,
        }
    }
    ///LSE oscillator off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON::Off
    }
    ///LSE oscillator on
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON::On
    }
}
///Field `LSEON` writer - LSE oscillator enable
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG, LSEON>;
impl<'a, REG> LSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE oscillator off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::Off)
    }
    ///LSE oscillator on
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::On)
    }
}
/**LSE oscillator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDY {
    ///0: LSE oscillator not ready
    NotReady = 0,
    ///1: LSE oscillator ready
    Ready = 1,
}
impl From<LSERDY> for bool {
    #[inline(always)]
    fn from(variant: LSERDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDY` reader - LSE oscillator ready
pub type LSERDY_R = crate::BitReader<LSERDY>;
impl LSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDY {
        match self.bits {
            false => LSERDY::NotReady,
            true => LSERDY::Ready,
        }
    }
    ///LSE oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDY::NotReady
    }
    ///LSE oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDY::Ready
    }
}
/**LSE oscillator bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP {
    ///0: LSE oscillator not bypassed
    Disabled = 0,
    ///1: LSE oscillator bypassed
    Enabled = 1,
}
impl From<LSEBYP> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEBYP` reader - LSE oscillator bypass
pub type LSEBYP_R = crate::BitReader<LSEBYP>;
impl LSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEBYP {
        match self.bits {
            false => LSEBYP::Disabled,
            true => LSEBYP::Enabled,
        }
    }
    ///LSE oscillator not bypassed
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEBYP::Disabled
    }
    ///LSE oscillator bypassed
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEBYP::Enabled
    }
}
///Field `LSEBYP` writer - LSE oscillator bypass
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE oscillator not bypassed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Disabled)
    }
    ///LSE oscillator bypassed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Enabled)
    }
}
/**LSE oscillator drive capability

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV {
    ///0: Xtal mode lower driving capability
    Low = 0,
    ///1: Xtal mode medium-low driving capability
    MedLow = 1,
    ///2: Xtal mode medium-high driving capability
    MedHigh = 2,
    ///3: Xtal mode higher driving capability
    High = 3,
}
impl From<LSEDRV> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSEDRV {
    type Ux = u8;
}
impl crate::IsEnum for LSEDRV {}
///Field `LSEDRV` reader - LSE oscillator drive capability
pub type LSEDRV_R = crate::FieldReader<LSEDRV>;
impl LSEDRV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV {
        match self.bits {
            0 => LSEDRV::Low,
            1 => LSEDRV::MedLow,
            2 => LSEDRV::MedHigh,
            3 => LSEDRV::High,
            _ => unreachable!(),
        }
    }
    ///Xtal mode lower driving capability
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV::Low
    }
    ///Xtal mode medium-low driving capability
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == LSEDRV::MedLow
    }
    ///Xtal mode medium-high driving capability
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == LSEDRV::MedHigh
    }
    ///Xtal mode higher driving capability
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV::High
    }
}
///Field `LSEDRV` writer - LSE oscillator drive capability
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LSEDRV, crate::Safe>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Xtal mode lower driving capability
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Low)
    }
    ///Xtal mode medium-low driving capability
    #[inline(always)]
    pub fn med_low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MedLow)
    }
    ///Xtal mode medium-high driving capability
    #[inline(always)]
    pub fn med_high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MedHigh)
    }
    ///Xtal mode higher driving capability
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::High)
    }
}
/**CSS on LSE enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON {
    ///0: CSS on LSE disabled
    Disabled = 0,
    ///1: CSS on LSE enabled
    Enabled = 1,
}
impl From<LSECSSON> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSON` reader - CSS on LSE enable
pub type LSECSSON_R = crate::BitReader<LSECSSON>;
impl LSECSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSON {
        match self.bits {
            false => LSECSSON::Disabled,
            true => LSECSSON::Enabled,
        }
    }
    ///CSS on LSE disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSON::Disabled
    }
    ///CSS on LSE enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSON::Enabled
    }
}
///Field `LSECSSON` writer - CSS on LSE enable
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSON>;
impl<'a, REG> LSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CSS on LSE disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::Disabled)
    }
    ///CSS on LSE enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::Enabled)
    }
}
/**CSS on LSE failure Detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD {
    ///0: No failure detected on LSE
    NoFailure = 0,
    ///1: Failure detected on LSE
    Failure = 1,
}
impl From<LSECSSD> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSD` reader - CSS on LSE failure Detection
pub type LSECSSD_R = crate::BitReader<LSECSSD>;
impl LSECSSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSD {
        match self.bits {
            false => LSECSSD::NoFailure,
            true => LSECSSD::Failure,
        }
    }
    ///No failure detected on LSE
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSD::NoFailure
    }
    ///Failure detected on LSE
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSD::Failure
    }
}
/**LSE system clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSEN {
    ///0: LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    Disabled = 0,
    ///1: LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    Enabled = 1,
}
impl From<LSESYSEN> for bool {
    #[inline(always)]
    fn from(variant: LSESYSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LSESYSEN` reader - LSE system clock enable
pub type LSESYSEN_R = crate::BitReader<LSESYSEN>;
impl LSESYSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSESYSEN {
        match self.bits {
            false => LSESYSEN::Disabled,
            true => LSESYSEN::Enabled,
        }
    }
    ///LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSESYSEN::Disabled
    }
    ///LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSESYSEN::Enabled
    }
}
///Field `LSESYSEN` writer - LSE system clock enable
pub type LSESYSEN_W<'a, REG> = crate::BitWriter<'a, REG, LSESYSEN>;
impl<'a, REG> LSESYSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSESYSEN::Disabled)
    }
    ///LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSESYSEN::Enabled)
    }
}
/**RTC clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL {
    ///0: No clock
    NoClock = 0,
    ///1: LSE oscillator clock selected
    Lse = 1,
    ///2: LSI oscillator clock selected
    Lsi = 2,
    ///3: HSE32 oscillator clock divided by 32 selected
    Hse32 = 3,
}
impl From<RTCSEL> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL {
    type Ux = u8;
}
impl crate::IsEnum for RTCSEL {}
///Field `RTCSEL` reader - RTC clock source selection
pub type RTCSEL_R = crate::FieldReader<RTCSEL>;
impl RTCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCSEL {
        match self.bits {
            0 => RTCSEL::NoClock,
            1 => RTCSEL::Lse,
            2 => RTCSEL::Lsi,
            3 => RTCSEL::Hse32,
            _ => unreachable!(),
        }
    }
    ///No clock
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL::NoClock
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL::Lse
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL::Lsi
    }
    ///HSE32 oscillator clock divided by 32 selected
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == RTCSEL::Hse32
    }
}
///Field `RTCSEL` writer - RTC clock source selection
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTCSEL, crate::Safe>;
impl<'a, REG> RTCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::NoClock)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lse)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lsi)
    }
    ///HSE32 oscillator clock divided by 32 selected
    #[inline(always)]
    pub fn hse32(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Hse32)
    }
}
/**LSE system clock ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSRDY {
    ///0: LSE system clock not ready
    NotReady = 0,
    ///1: LSE system clock ready
    Ready = 1,
}
impl From<LSESYSRDY> for bool {
    #[inline(always)]
    fn from(variant: LSESYSRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LSESYSRDY` reader - LSE system clock ready
pub type LSESYSRDY_R = crate::BitReader<LSESYSRDY>;
impl LSESYSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSESYSRDY {
        match self.bits {
            false => LSESYSRDY::NotReady,
            true => LSESYSRDY::Ready,
        }
    }
    ///LSE system clock not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSESYSRDY::NotReady
    }
    ///LSE system clock ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSESYSRDY::Ready
    }
}
/**RTC clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN {
    ///0: RTC kernel clock disabled
    Disabled = 0,
    ///1: RTC kernel clock enabled
    Enabled = 1,
}
impl From<RTCEN> for bool {
    #[inline(always)]
    fn from(variant: RTCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCEN` reader - RTC clock enable
pub type RTCEN_R = crate::BitReader<RTCEN>;
impl RTCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCEN {
        match self.bits {
            false => RTCEN::Disabled,
            true => RTCEN::Enabled,
        }
    }
    ///RTC kernel clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN::Disabled
    }
    ///RTC kernel clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN::Enabled
    }
}
///Field `RTCEN` writer - RTC clock enable
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC kernel clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Disabled)
    }
    ///RTC kernel clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Enabled)
    }
}
/**Backup domain software reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST {
    ///0: Reset not activated
    NotActive = 0,
    ///1: Entire Backup domain reset
    Reset = 1,
}
impl From<BDRST> for bool {
    #[inline(always)]
    fn from(variant: BDRST) -> Self {
        variant as u8 != 0
    }
}
///Field `BDRST` reader - Backup domain software reset
pub type BDRST_R = crate::BitReader<BDRST>;
impl BDRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BDRST {
        match self.bits {
            false => BDRST::NotActive,
            true => BDRST::Reset,
        }
    }
    ///Reset not activated
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == BDRST::NotActive
    }
    ///Entire Backup domain reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRST::Reset
    }
}
///Field `BDRST` writer - Backup domain software reset
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG, BDRST>;
impl<'a, REG> BDRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset not activated
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::NotActive)
    }
    ///Entire Backup domain reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::Reset)
    }
}
/**Low speed clock output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN {
    ///0: LSCO disabled
    Disabled = 0,
    ///1: LSCO enabled
    Enabled = 1,
}
impl From<LSCOEN> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LSCOEN` reader - Low speed clock output enable
pub type LSCOEN_R = crate::BitReader<LSCOEN>;
impl LSCOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSCOEN {
        match self.bits {
            false => LSCOEN::Disabled,
            true => LSCOEN::Enabled,
        }
    }
    ///LSCO disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSCOEN::Disabled
    }
    ///LSCO enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSCOEN::Enabled
    }
}
///Field `LSCOEN` writer - Low speed clock output enable
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG, LSCOEN>;
impl<'a, REG> LSCOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSCO disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::Disabled)
    }
    ///LSCO enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::Enabled)
    }
}
/**Low speed clock output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL {
    ///0: LSI clock selected
    Lsi = 0,
    ///1: LSE clock selected
    Lse = 1,
}
impl From<LSCOSEL> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `LSCOSEL` reader - Low speed clock output selection
pub type LSCOSEL_R = crate::BitReader<LSCOSEL>;
impl LSCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSCOSEL {
        match self.bits {
            false => LSCOSEL::Lsi,
            true => LSCOSEL::Lse,
        }
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LSCOSEL::Lsi
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LSCOSEL::Lse
    }
}
///Field `LSCOSEL` writer - Low speed clock output selection
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, LSCOSEL>;
impl<'a, REG> LSCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::Lse)
    }
}
impl R {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - LSE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - CSS on LSE enable
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSS on LSE failure Detection
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE system clock enable
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - LSE system clock ready
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low speed clock output selection
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("lscosel", &self.lscosel())
            .field("lscoen", &self.lscoen())
            .field("bdrst", &self.bdrst())
            .field("rtcen", &self.rtcen())
            .field("lsesysrdy", &self.lsesysrdy())
            .field("rtcsel", &self.rtcsel())
            .field("lsesysen", &self.lsesysen())
            .field("lsecssd", &self.lsecssd())
            .field("lsecsson", &self.lsecsson())
            .field("lsedrv", &self.lsedrv())
            .field("lsebyp", &self.lsebyp())
            .field("lserdy", &self.lserdy())
            .field("lseon", &self.lseon())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bits 3:4 - LSE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - CSS on LSE enable
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bit 7 - LSE system clock enable
    #[inline(always)]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<'_, BDCRrs> {
        LSESYSEN_W::new(self, 7)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<'_, BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<'_, BDCRrs> {
        BDRST_W::new(self, 16)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<'_, BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    ///Bit 25 - Low speed clock output selection
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W<'_, BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
}
/**Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCRrs {}
