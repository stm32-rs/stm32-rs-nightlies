///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
/**LSE oscillator enabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON {
    ///0: LSE oscillator Off
    Off = 0,
    ///1: LSE oscillator On
    On = 1,
}
impl From<LSEON> for bool {
    #[inline(always)]
    fn from(variant: LSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEON` reader - LSE oscillator enabled
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
    ///LSE oscillator Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON::Off
    }
    ///LSE oscillator On
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON::On
    }
}
///Field `LSEON` writer - LSE oscillator enabled
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG, LSEON>;
impl<'a, REG> LSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE oscillator Off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::Off)
    }
    ///LSE oscillator On
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::On)
    }
}
/**LSE oscillator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYR {
    ///0: LSE oscillator not ready
    NotReady = 0,
    ///1: LSE oscillator ready
    Ready = 1,
}
impl From<LSERDYR> for bool {
    #[inline(always)]
    fn from(variant: LSERDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDY` reader - LSE oscillator ready
pub type LSERDY_R = crate::BitReader<LSERDYR>;
impl LSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYR {
        match self.bits {
            false => LSERDYR::NotReady,
            true => LSERDYR::Ready,
        }
    }
    ///LSE oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDYR::NotReady
    }
    ///LSE oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDYR::Ready
    }
}
///Field `LSERDY` writer - LSE oscillator ready
pub type LSERDY_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYR>;
impl<'a, REG> LSERDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYR::NotReady)
    }
    ///LSE oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYR::Ready)
    }
}
/**LSE oscillator bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP {
    ///0: LSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: LSE crystal oscillator bypassed with external clock
    Bypassed = 1,
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
            false => LSEBYP::NotBypassed,
            true => LSEBYP::Bypassed,
        }
    }
    ///LSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP::NotBypassed
    }
    ///LSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP::Bypassed
    }
}
///Field `LSEBYP` writer - LSE oscillator bypass
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::NotBypassed)
    }
    ///LSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Bypassed)
    }
}
/**LSE oscillator driving capability

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV {
    ///0: Lowest LSE oscillator driving capability
    Lowest = 0,
    ///1: Medium low LSE oscillator driving capability
    MediumLow = 1,
    ///2: Medium high LSE oscillator driving capability
    MediumHigh = 2,
    ///3: Highest LSE oscillator driving capability
    Highest = 3,
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
///Field `LSEDRV` reader - LSE oscillator driving capability
pub type LSEDRV_R = crate::FieldReader<LSEDRV>;
impl LSEDRV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV {
        match self.bits {
            0 => LSEDRV::Lowest,
            1 => LSEDRV::MediumLow,
            2 => LSEDRV::MediumHigh,
            3 => LSEDRV::Highest,
            _ => unreachable!(),
        }
    }
    ///Lowest LSE oscillator driving capability
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == LSEDRV::Lowest
    }
    ///Medium low LSE oscillator driving capability
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV::MediumLow
    }
    ///Medium high LSE oscillator driving capability
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV::MediumHigh
    }
    ///Highest LSE oscillator driving capability
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == LSEDRV::Highest
    }
}
///Field `LSEDRV` writer - LSE oscillator driving capability
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LSEDRV, crate::Safe>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Lowest LSE oscillator driving capability
    #[inline(always)]
    pub fn lowest(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Lowest)
    }
    ///Medium low LSE oscillator driving capability
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumLow)
    }
    ///Medium high LSE oscillator driving capability
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumHigh)
    }
    ///Highest LSE oscillator driving capability
    #[inline(always)]
    pub fn highest(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Highest)
    }
}
/**LSE clock security system enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON {
    ///0: Clock security system on 32 kHz oscillator off
    SecurityOff = 0,
    ///1: Clock security system on 32 kHz oscillator on
    SecurityOn = 1,
}
impl From<LSECSSON> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSON` reader - LSE clock security system enable
pub type LSECSSON_R = crate::BitReader<LSECSSON>;
impl LSECSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSON {
        match self.bits {
            false => LSECSSON::SecurityOff,
            true => LSECSSON::SecurityOn,
        }
    }
    ///Clock security system on 32 kHz oscillator off
    #[inline(always)]
    pub fn is_security_off(&self) -> bool {
        *self == LSECSSON::SecurityOff
    }
    ///Clock security system on 32 kHz oscillator on
    #[inline(always)]
    pub fn is_security_on(&self) -> bool {
        *self == LSECSSON::SecurityOn
    }
}
///Field `LSECSSON` writer - LSE clock security system enable
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSON>;
impl<'a, REG> LSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock security system on 32 kHz oscillator off
    #[inline(always)]
    pub fn security_off(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::SecurityOff)
    }
    ///Clock security system on 32 kHz oscillator on
    #[inline(always)]
    pub fn security_on(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::SecurityOn)
    }
}
/**LSE clock security system failure detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSDR {
    ///0: No failure detected on 32 kHz oscillator
    NoFailure = 0,
    ///1: Failure detected on 32 kHz oscillator
    Failure = 1,
}
impl From<LSECSSDR> for bool {
    #[inline(always)]
    fn from(variant: LSECSSDR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSD` reader - LSE clock security system failure detection
pub type LSECSSD_R = crate::BitReader<LSECSSDR>;
impl LSECSSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSDR {
        match self.bits {
            false => LSECSSDR::NoFailure,
            true => LSECSSDR::Failure,
        }
    }
    ///No failure detected on 32 kHz oscillator
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSDR::NoFailure
    }
    ///Failure detected on 32 kHz oscillator
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSDR::Failure
    }
}
///Field `LSECSSD` writer - LSE clock security system failure detection
pub type LSECSSD_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSDR>;
impl<'a, REG> LSECSSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No failure detected on 32 kHz oscillator
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSDR::NoFailure)
    }
    ///Failure detected on 32 kHz oscillator
    #[inline(always)]
    pub fn failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSDR::Failure)
    }
}
/**low-speed external clock type in bypass mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEEXT {
    ///0: HSE in analog mode
    Analog = 0,
    ///1: HSE in digital mode
    Digital = 1,
}
impl From<LSEEXT> for bool {
    #[inline(always)]
    fn from(variant: LSEEXT) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEEXT` reader - low-speed external clock type in bypass mode
pub type LSEEXT_R = crate::BitReader<LSEEXT>;
impl LSEEXT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSEEXT {
        match self.bits {
            false => LSEEXT::Analog,
            true => LSEEXT::Digital,
        }
    }
    ///HSE in analog mode
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == LSEEXT::Analog
    }
    ///HSE in digital mode
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == LSEEXT::Digital
    }
}
///Field `LSEEXT` writer - low-speed external clock type in bypass mode
pub type LSEEXT_W<'a, REG> = crate::BitWriter<'a, REG, LSEEXT>;
impl<'a, REG> LSEEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE in analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(LSEEXT::Analog)
    }
    ///HSE in digital mode
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(LSEEXT::Digital)
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
    ///1: LSE oscillator clock used as RTC clock
    Lse = 1,
    ///2: LSI oscillator clock used as RTC clock
    Lsi = 2,
    ///3: HSE oscillator clock divided by a prescaler used as RTC clock
    Hse = 3,
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
            3 => RTCSEL::Hse,
            _ => unreachable!(),
        }
    }
    ///No clock
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL::NoClock
    }
    ///LSE oscillator clock used as RTC clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL::Lse
    }
    ///LSI oscillator clock used as RTC clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL::Lsi
    }
    ///HSE oscillator clock divided by a prescaler used as RTC clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL::Hse
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
    ///LSE oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lse)
    }
    ///LSI oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lsi)
    }
    ///HSE oscillator clock divided by a prescaler used as RTC clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Hse)
    }
}
/**RTC clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN {
    ///0: RTC clock disabled
    Disabled = 0,
    ///1: RTC clock enabled
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
    ///RTC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN::Disabled
    }
    ///RTC clock enabled
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
    ///RTC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Disabled)
    }
    ///RTC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Enabled)
    }
}
/**VSwitch domain software reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSWRST {
    ///0: Reset not activated
    NotActivated = 0,
    ///1: Resets the entire VSW domain
    Reset = 1,
}
impl From<VSWRST> for bool {
    #[inline(always)]
    fn from(variant: VSWRST) -> Self {
        variant as u8 != 0
    }
}
///Field `VSWRST` reader - VSwitch domain software reset
pub type VSWRST_R = crate::BitReader<VSWRST>;
impl VSWRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSWRST {
        match self.bits {
            false => VSWRST::NotActivated,
            true => VSWRST::Reset,
        }
    }
    ///Reset not activated
    #[inline(always)]
    pub fn is_not_activated(&self) -> bool {
        *self == VSWRST::NotActivated
    }
    ///Resets the entire VSW domain
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VSWRST::Reset
    }
}
///Field `VSWRST` writer - VSwitch domain software reset
pub type VSWRST_W<'a, REG> = crate::BitWriter<'a, REG, VSWRST>;
impl<'a, REG> VSWRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset not activated
    #[inline(always)]
    pub fn not_activated(self) -> &'a mut crate::W<REG> {
        self.variant(VSWRST::NotActivated)
    }
    ///Resets the entire VSW domain
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VSWRST::Reset)
    }
}
///Field `LSCOEN` reader - Low-speed clock output (LSCO) enable
pub type LSCOEN_R = crate::BitReader;
///Field `LSCOEN` writer - Low-speed clock output (LSCO) enable
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Low-speed clock output selection

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
///Field `LSCOSEL` reader - Low-speed clock output selection
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
///Field `LSCOSEL` writer - Low-speed clock output selection
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
/**LSI oscillator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    ///0: Oscillator disabled
    Disabled = 0,
    ///1: Oscillator enabled
    Enabled = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
///Field `LSION` reader - LSI oscillator enable
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Disabled,
            true => LSION::Enabled,
        }
    }
    ///Oscillator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSION::Disabled
    }
    ///Oscillator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSION::Enabled
    }
}
///Field `LSION` writer - LSI oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oscillator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Disabled)
    }
    ///Oscillator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Enabled)
    }
}
/**LSI oscillator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYR {
    ///0: Clock not ready
    NotReady = 0,
    ///1: Clock ready
    Ready = 1,
}
impl From<LSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDY` reader - LSI oscillator ready
pub type LSIRDY_R = crate::BitReader<LSIRDYR>;
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYR {
        match self.bits {
            false => LSIRDYR::NotReady,
            true => LSIRDYR::Ready,
        }
    }
    ///Clock not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDYR::NotReady
    }
    ///Clock ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDYR::Ready
    }
}
///Field `LSIRDY` writer - LSI oscillator ready
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYR>;
impl<'a, REG> LSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYR::NotReady)
    }
    ///Clock ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYR::Ready)
    }
}
impl R {
    ///Bit 0 - LSE oscillator enabled
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
    ///Bits 3:4 - LSE oscillator driving capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - LSE clock security system enable
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LSE clock security system failure detection
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - low-speed external clock type in bypass mode
    #[inline(always)]
    pub fn lseext(&self) -> LSEEXT_R {
        LSEEXT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - VSwitch domain software reset
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low-speed clock output selection
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("lsedrv", &self.lsedrv())
            .field("lsecsson", &self.lsecsson())
            .field("lsecssd", &self.lsecssd())
            .field("lseext", &self.lseext())
            .field("rtcsel", &self.rtcsel())
            .field("rtcen", &self.rtcen())
            .field("vswrst", &self.vswrst())
            .field("lscoen", &self.lscoen())
            .field("lscosel", &self.lscosel())
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enabled
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 1 - LSE oscillator ready
    #[inline(always)]
    pub fn lserdy(&mut self) -> LSERDY_W<'_, BDCRrs> {
        LSERDY_W::new(self, 1)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bits 3:4 - LSE oscillator driving capability
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - LSE clock security system enable
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bit 6 - LSE clock security system failure detection
    #[inline(always)]
    pub fn lsecssd(&mut self) -> LSECSSD_W<'_, BDCRrs> {
        LSECSSD_W::new(self, 6)
    }
    ///Bit 7 - low-speed external clock type in bypass mode
    #[inline(always)]
    pub fn lseext(&mut self) -> LSEEXT_W<'_, BDCRrs> {
        LSEEXT_W::new(self, 7)
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
    ///Bit 16 - VSwitch domain software reset
    #[inline(always)]
    pub fn vswrst(&mut self) -> VSWRST_W<'_, BDCRrs> {
        VSWRST_W::new(self, 16)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<'_, BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    ///Bit 25 - Low-speed clock output selection
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W<'_, BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
    ///Bit 26 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, BDCRrs> {
        LSION_W::new(self, 26)
    }
    ///Bit 27 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LSIRDY_W<'_, BDCRrs> {
        LSIRDY_W::new(self, 27)
    }
}
/**RCC Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:BDCR)*/
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
