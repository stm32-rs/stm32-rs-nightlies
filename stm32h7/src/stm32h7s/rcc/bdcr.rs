///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
/**LSE oscillator enabled Set and reset by software.

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
///Field `LSEON` reader - LSE oscillator enabled Set and reset by software.
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
///Field `LSEON` writer - LSE oscillator enabled Set and reset by software.
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
/**LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.

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
///Field `LSERDY` reader - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.
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
/**LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)

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
///Field `LSEBYP` reader - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)
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
///Field `LSEBYP` writer - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)
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
/**LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator.

Value on reset: 2*/
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
///Field `LSEDRV` reader - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator.
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
///Field `LSEDRV` writer - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator.
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
/**LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset

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
///Field `LSECSSON` reader - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset
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
///Field `LSECSSON` writer - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset
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
/**LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.

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
///Field `LSECSSD` reader - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.
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
///Field `LSEEXT` reader - low-speed external clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled.
pub type LSEEXT_R = crate::BitReader;
///Field `LSEEXT` writer - low-speed external clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled.
pub type LSEEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
/**RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).

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
///Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).
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
///Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).
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
///Field `LSECSSRA` reader - Re-Arm the LSECSS function Set by software. After a LSE failure detection, the software application can re-enable the LSECSS by writing this bit to 1. Reading this bit returns the written value. Prior to set this bit to 1, LSECSSON must be set to 0. Please refer to Section : CSS on LSE for details.
pub type LSECSSRA_R = crate::BitReader;
///Field `LSECSSRA` writer - Re-Arm the LSECSS function Set by software. After a LSE failure detection, the software application can re-enable the LSECSS by writing this bit to 1. Reading this bit returns the written value. Prior to set this bit to 1, LSECSSON must be set to 0. Please refer to Section : CSS on LSE for details.
pub type LSECSSRA_W<'a, REG> = crate::BitWriter<'a, REG>;
/**RTC clock enable Set and reset by software.

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
///Field `RTCEN` reader - RTC clock enable Set and reset by software.
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
///Field `RTCEN` writer - RTC clock enable Set and reset by software.
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
/**VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0.

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
///Field `VSWRST` reader - VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0.
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
///Field `VSWRST` writer - VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0.
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
impl R {
    ///Bit 0 - LSE oscillator enabled Set and reset by software.
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator.
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - low-speed external clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled.
    #[inline(always)]
    pub fn lseext(&self) -> LSEEXT_R {
        LSEEXT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 12 - Re-Arm the LSECSS function Set by software. After a LSE failure detection, the software application can re-enable the LSECSS by writing this bit to 1. Reading this bit returns the written value. Prior to set this bit to 1, LSECSSON must be set to 0. Please refer to Section : CSS on LSE for details.
    #[inline(always)]
    pub fn lsecssra(&self) -> LSECSSRA_R {
        LSECSSRA_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - RTC clock enable Set and reset by software.
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0.
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 16) & 1) != 0)
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
            .field("lsecssra", &self.lsecssra())
            .field("rtcen", &self.rtcen())
            .field("vswrst", &self.vswrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enabled Set and reset by software.
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator.
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit can only be disabled, After a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON. After a back-up domain reset
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bit 7 - low-speed external clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled.
    #[inline(always)]
    pub fn lseext(&mut self) -> LSEEXT_W<BDCRrs> {
        LSEEXT_W::new(self, 7)
    }
    ///Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bit 12 - Re-Arm the LSECSS function Set by software. After a LSE failure detection, the software application can re-enable the LSECSS by writing this bit to 1. Reading this bit returns the written value. Prior to set this bit to 1, LSECSSON must be set to 0. Please refer to Section : CSS on LSE for details.
    #[inline(always)]
    pub fn lsecssra(&mut self) -> LSECSSRA_W<BDCRrs> {
        LSECSSRA_W::new(self, 12)
    }
    ///Bit 15 - RTC clock enable Set and reset by software.
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    ///Bit 16 - VSwitch domain software reset Set and reset by software. To generate a VSW reset, it is recommended to write this bit to 1, then back to 0.
    #[inline(always)]
    pub fn vswrst(&mut self) -> VSWRST_W<BDCRrs> {
        VSWRST_W::new(self, 16)
    }
}
/**RCC Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:BDCR)*/
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
///`reset()` method sets BDCR to value 0x10
impl crate::Resettable for BDCRrs {
    const RESET_VALUE: u32 = 0x10;
}
