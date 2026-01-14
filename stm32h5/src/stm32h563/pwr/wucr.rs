///Register `WUCR` reader
pub type R = crate::R<WUCRrs>;
///Register `WUCR` writer
pub type W = crate::W<WUCRrs>;
/**enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN1 {
    ///0: An event on WUPx pin does not wakeup the system from Standby mode
    Disabled = 0,
    ///1: A rising or falling edge on WUPx pin wakes up the system from Standby mode
    Enabled = 1,
}
impl From<WUPEN1> for bool {
    #[inline(always)]
    fn from(variant: WUPEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPEN1` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN1_R = crate::BitReader<WUPEN1>;
impl WUPEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN1 {
        match self.bits {
            false => WUPEN1::Disabled,
            true => WUPEN1::Enabled,
        }
    }
    ///An event on WUPx pin does not wakeup the system from Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN1::Disabled
    }
    ///A rising or falling edge on WUPx pin wakes up the system from Standby mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN1::Enabled
    }
}
///Field `WUPEN1` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN1_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN1>;
impl<'a, REG> WUPEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///An event on WUPx pin does not wakeup the system from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1::Disabled)
    }
    ///A rising or falling edge on WUPx pin wakes up the system from Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1::Enabled)
    }
}
///Field `WUPEN2` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_R as WUPEN2_R;
///Field `WUPEN3` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_R as WUPEN3_R;
///Field `WUPEN4` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_R as WUPEN4_R;
///Field `WUPEN5` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_R as WUPEN5_R;
///Field `WUPEN6` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_R as WUPEN6_R;
///Field `WUPEN7` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_R as WUPEN7_R;
///Field `WUPEN8` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_R as WUPEN8_R;
///Field `WUPEN2` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_W as WUPEN2_W;
///Field `WUPEN3` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_W as WUPEN3_W;
///Field `WUPEN4` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_W as WUPEN4_W;
///Field `WUPEN5` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_W as WUPEN5_W;
///Field `WUPEN6` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_W as WUPEN6_W;
///Field `WUPEN7` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_W as WUPEN7_W;
///Field `WUPEN8` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub use WUPEN1_W as WUPEN8_W;
/**wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP1 {
    ///0: Detection on high level
    HighLevel = 0,
    ///1: Detection on low level
    LowLevel = 1,
}
impl From<WUPP1> for bool {
    #[inline(always)]
    fn from(variant: WUPP1) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPP1` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP1_R = crate::BitReader<WUPP1>;
impl WUPP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUPP1 {
        match self.bits {
            false => WUPP1::HighLevel,
            true => WUPP1::LowLevel,
        }
    }
    ///Detection on high level
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == WUPP1::HighLevel
    }
    ///Detection on low level
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == WUPP1::LowLevel
    }
}
///Field `WUPP1` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG, WUPP1>;
impl<'a, REG> WUPP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection on high level
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1::HighLevel)
    }
    ///Detection on low level
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1::LowLevel)
    }
}
///Field `WUPP2` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_R as WUPP2_R;
///Field `WUPP3` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_R as WUPP3_R;
///Field `WUPP4` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_R as WUPP4_R;
///Field `WUPP5` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_R as WUPP5_R;
///Field `WUPP6` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_R as WUPP6_R;
///Field `WUPP7` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_R as WUPP7_R;
///Field `WUPP8` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_R as WUPP8_R;
///Field `WUPP2` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_W as WUPP2_W;
///Field `WUPP3` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_W as WUPP3_W;
///Field `WUPP4` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_W as WUPP4_W;
///Field `WUPP5` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_W as WUPP5_W;
///Field `WUPP6` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_W as WUPP6_W;
///Field `WUPP7` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_W as WUPP7_W;
///Field `WUPP8` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub use WUPP1_W as WUPP8_W;
/**wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPPUPD1 {
    ///0: No pull-up or pull-down
    NoPull = 0,
    ///1: Pull-up
    PullUp = 1,
    ///2: Pull-down
    PullDown = 2,
}
impl From<WUPPUPD1> for u8 {
    #[inline(always)]
    fn from(variant: WUPPUPD1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUPPUPD1 {
    type Ux = u8;
}
impl crate::IsEnum for WUPPUPD1 {}
///Field `WUPPUPD1` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD1_R = crate::FieldReader<WUPPUPD1>;
impl WUPPUPD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUPPUPD1> {
        match self.bits {
            0 => Some(WUPPUPD1::NoPull),
            1 => Some(WUPPUPD1::PullUp),
            2 => Some(WUPPUPD1::PullDown),
            _ => None,
        }
    }
    ///No pull-up or pull-down
    #[inline(always)]
    pub fn is_no_pull(&self) -> bool {
        *self == WUPPUPD1::NoPull
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == WUPPUPD1::PullUp
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == WUPPUPD1::PullDown
    }
}
///Field `WUPPUPD1` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUPPUPD1>;
impl<'a, REG> WUPPUPD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up or pull-down
    #[inline(always)]
    pub fn no_pull(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1::NoPull)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1::PullUp)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1::PullDown)
    }
}
///Field `WUPPUPD2` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_R as WUPPUPD2_R;
///Field `WUPPUPD3` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_R as WUPPUPD3_R;
///Field `WUPPUPD4` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_R as WUPPUPD4_R;
///Field `WUPPUPD5` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_R as WUPPUPD5_R;
///Field `WUPPUPD6` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_R as WUPPUPD6_R;
///Field `WUPPUPD7` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_R as WUPPUPD7_R;
///Field `WUPPUPD8` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_R as WUPPUPD8_R;
///Field `WUPPUPD2` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_W as WUPPUPD2_W;
///Field `WUPPUPD3` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_W as WUPPUPD3_W;
///Field `WUPPUPD4` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_W as WUPPUPD4_W;
///Field `WUPPUPD5` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_W as WUPPUPD5_W;
///Field `WUPPUPD6` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_W as WUPPUPD6_W;
///Field `WUPPUPD7` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_W as WUPPUPD7_W;
///Field `WUPPUPD8` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub use WUPPUPD1_W as WUPPUPD8_W;
impl R {
    ///Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen6(&self) -> WUPEN6_R {
        WUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen7(&self) -> WUPEN7_R {
        WUPEN7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen8(&self) -> WUPEN8_R {
        WUPEN8_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp7(&self) -> WUPP7_R {
        WUPP7_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp8(&self) -> WUPP8_R {
        WUPP8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd1(&self) -> WUPPUPD1_R {
        WUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd2(&self) -> WUPPUPD2_R {
        WUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd3(&self) -> WUPPUPD3_R {
        WUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd4(&self) -> WUPPUPD4_R {
        WUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd5(&self) -> WUPPUPD5_R {
        WUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd6(&self) -> WUPPUPD6_R {
        WUPPUPD6_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd7(&self) -> WUPPUPD7_R {
        WUPPUPD7_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd8(&self) -> WUPPUPD8_R {
        WUPPUPD8_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUCR")
            .field("wupen1", &self.wupen1())
            .field("wupen2", &self.wupen2())
            .field("wupen3", &self.wupen3())
            .field("wupen4", &self.wupen4())
            .field("wupen5", &self.wupen5())
            .field("wupen6", &self.wupen6())
            .field("wupen7", &self.wupen7())
            .field("wupen8", &self.wupen8())
            .field("wupp1", &self.wupp1())
            .field("wupp2", &self.wupp2())
            .field("wupp3", &self.wupp3())
            .field("wupp4", &self.wupp4())
            .field("wupp5", &self.wupp5())
            .field("wupp6", &self.wupp6())
            .field("wupp7", &self.wupp7())
            .field("wupp8", &self.wupp8())
            .field("wuppupd1", &self.wuppupd1())
            .field("wuppupd2", &self.wuppupd2())
            .field("wuppupd3", &self.wuppupd3())
            .field("wuppupd4", &self.wuppupd4())
            .field("wuppupd5", &self.wuppupd5())
            .field("wuppupd6", &self.wuppupd6())
            .field("wuppupd7", &self.wuppupd7())
            .field("wuppupd8", &self.wuppupd8())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen1(&mut self) -> WUPEN1_W<'_, WUCRrs> {
        WUPEN1_W::new(self, 0)
    }
    ///Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen2(&mut self) -> WUPEN2_W<'_, WUCRrs> {
        WUPEN2_W::new(self, 1)
    }
    ///Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen3(&mut self) -> WUPEN3_W<'_, WUCRrs> {
        WUPEN3_W::new(self, 2)
    }
    ///Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen4(&mut self) -> WUPEN4_W<'_, WUCRrs> {
        WUPEN4_W::new(self, 3)
    }
    ///Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen5(&mut self) -> WUPEN5_W<'_, WUCRrs> {
        WUPEN5_W::new(self, 4)
    }
    ///Bit 5 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen6(&mut self) -> WUPEN6_W<'_, WUCRrs> {
        WUPEN6_W::new(self, 5)
    }
    ///Bit 6 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen7(&mut self) -> WUPEN7_W<'_, WUCRrs> {
        WUPEN7_W::new(self, 6)
    }
    ///Bit 7 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen8(&mut self) -> WUPEN8_W<'_, WUCRrs> {
        WUPEN8_W::new(self, 7)
    }
    ///Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W<'_, WUCRrs> {
        WUPP1_W::new(self, 8)
    }
    ///Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W<'_, WUCRrs> {
        WUPP2_W::new(self, 9)
    }
    ///Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W<'_, WUCRrs> {
        WUPP3_W::new(self, 10)
    }
    ///Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W<'_, WUCRrs> {
        WUPP4_W::new(self, 11)
    }
    ///Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W<'_, WUCRrs> {
        WUPP5_W::new(self, 12)
    }
    ///Bit 13 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp6(&mut self) -> WUPP6_W<'_, WUCRrs> {
        WUPP6_W::new(self, 13)
    }
    ///Bit 14 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp7(&mut self) -> WUPP7_W<'_, WUCRrs> {
        WUPP7_W::new(self, 14)
    }
    ///Bit 15 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp8(&mut self) -> WUPP8_W<'_, WUCRrs> {
        WUPP8_W::new(self, 15)
    }
    ///Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd1(&mut self) -> WUPPUPD1_W<'_, WUCRrs> {
        WUPPUPD1_W::new(self, 16)
    }
    ///Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd2(&mut self) -> WUPPUPD2_W<'_, WUCRrs> {
        WUPPUPD2_W::new(self, 18)
    }
    ///Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd3(&mut self) -> WUPPUPD3_W<'_, WUCRrs> {
        WUPPUPD3_W::new(self, 20)
    }
    ///Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd4(&mut self) -> WUPPUPD4_W<'_, WUCRrs> {
        WUPPUPD4_W::new(self, 22)
    }
    ///Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd5(&mut self) -> WUPPUPD5_W<'_, WUCRrs> {
        WUPPUPD5_W::new(self, 24)
    }
    ///Bits 26:27 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd6(&mut self) -> WUPPUPD6_W<'_, WUCRrs> {
        WUPPUPD6_W::new(self, 26)
    }
    ///Bits 28:29 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd7(&mut self) -> WUPPUPD7_W<'_, WUCRrs> {
        WUPPUPD7_W::new(self, 28)
    }
    ///Bits 30:31 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd8(&mut self) -> WUPPUPD8_W<'_, WUCRrs> {
        WUPPUPD8_W::new(self, 30)
    }
}
/**PWR wakeup configuration register

You can [`read`](crate::Reg::read) this register and get [`wucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#PWR:WUCR)*/
pub struct WUCRrs;
impl crate::RegisterSpec for WUCRrs {
    type Ux = u32;
}
///`read()` method returns [`wucr::R`](R) reader structure
impl crate::Readable for WUCRrs {}
///`write(|w| ..)` method takes [`wucr::W`](W) writer structure
impl crate::Writable for WUCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUCR to value 0
impl crate::Resettable for WUCRrs {}
