#[doc = "Register `WUCR` reader"]
pub type R = crate::R<WUCRrs>;
#[doc = "Register `WUCR` writer"]
pub type W = crate::W<WUCRrs>;
#[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN1 {
    #[doc = "0: An event on WUPx pin does not wakeup the system from Standby mode"]
    Disabled = 0,
    #[doc = "1: A rising or falling edge on WUPx pin wakes up the system from Standby mode"]
    Enabled = 1,
}
impl From<WUPEN1> for bool {
    #[inline(always)]
    fn from(variant: WUPEN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN1` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN1_R = crate::BitReader<WUPEN1>;
impl WUPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN1 {
        match self.bits {
            false => WUPEN1::Disabled,
            true => WUPEN1::Enabled,
        }
    }
    #[doc = "An event on WUPx pin does not wakeup the system from Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN1::Disabled
    }
    #[doc = "A rising or falling edge on WUPx pin wakes up the system from Standby mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN1::Enabled
    }
}
#[doc = "Field `WUPEN1` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub type WUPEN1_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN1>;
impl<'a, REG> WUPEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An event on WUPx pin does not wakeup the system from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1::Disabled)
    }
    #[doc = "A rising or falling edge on WUPx pin wakes up the system from Standby mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN1::Enabled)
    }
}
#[doc = "Field `WUPEN2` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_R as WUPEN2_R;
#[doc = "Field `WUPEN3` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_R as WUPEN3_R;
#[doc = "Field `WUPEN4` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_R as WUPEN4_R;
#[doc = "Field `WUPEN5` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_R as WUPEN5_R;
#[doc = "Field `WUPEN2` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_W as WUPEN2_W;
#[doc = "Field `WUPEN3` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_W as WUPEN3_W;
#[doc = "Field `WUPEN4` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_W as WUPEN4_W;
#[doc = "Field `WUPEN5` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
pub use WUPEN1_W as WUPEN5_W;
#[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPP1 {
    #[doc = "0: Detection on high level"]
    HighLevel = 0,
    #[doc = "1: Detection on low level"]
    LowLevel = 1,
}
impl From<WUPP1> for bool {
    #[inline(always)]
    fn from(variant: WUPP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPP1` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP1_R = crate::BitReader<WUPP1>;
impl WUPP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPP1 {
        match self.bits {
            false => WUPP1::HighLevel,
            true => WUPP1::LowLevel,
        }
    }
    #[doc = "Detection on high level"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == WUPP1::HighLevel
    }
    #[doc = "Detection on low level"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == WUPP1::LowLevel
    }
}
#[doc = "Field `WUPP1` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG, WUPP1>;
impl<'a, REG> WUPP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1::HighLevel)
    }
    #[doc = "Detection on low level"]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(WUPP1::LowLevel)
    }
}
#[doc = "Field `WUPP2` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_R as WUPP2_R;
#[doc = "Field `WUPP3` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_R as WUPP3_R;
#[doc = "Field `WUPP4` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_R as WUPP4_R;
#[doc = "Field `WUPP5` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_R as WUPP5_R;
#[doc = "Field `WUPP2` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_W as WUPP2_W;
#[doc = "Field `WUPP3` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_W as WUPP3_W;
#[doc = "Field `WUPP4` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_W as WUPP4_W;
#[doc = "Field `WUPP5` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
pub use WUPP1_W as WUPP5_W;
#[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPPUPD1 {
    #[doc = "0: No pull-up or pull-down"]
    NoPull = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
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
#[doc = "Field `WUPPUPD1` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD1_R = crate::FieldReader<WUPPUPD1>;
impl WUPPUPD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUPPUPD1> {
        match self.bits {
            0 => Some(WUPPUPD1::NoPull),
            1 => Some(WUPPUPD1::PullUp),
            2 => Some(WUPPUPD1::PullDown),
            _ => None,
        }
    }
    #[doc = "No pull-up or pull-down"]
    #[inline(always)]
    pub fn is_no_pull(&self) -> bool {
        *self == WUPPUPD1::NoPull
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == WUPPUPD1::PullUp
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == WUPPUPD1::PullDown
    }
}
#[doc = "Field `WUPPUPD1` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub type WUPPUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUPPUPD1>;
impl<'a, REG> WUPPUPD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up or pull-down"]
    #[inline(always)]
    pub fn no_pull(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1::NoPull)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(WUPPUPD1::PullDown)
    }
}
#[doc = "Field `WUPPUPD2` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_R as WUPPUPD2_R;
#[doc = "Field `WUPPUPD3` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_R as WUPPUPD3_R;
#[doc = "Field `WUPPUPD4` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_R as WUPPUPD4_R;
#[doc = "Field `WUPPUPD5` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_R as WUPPUPD5_R;
#[doc = "Field `WUPPUPD2` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_W as WUPPUPD2_W;
#[doc = "Field `WUPPUPD3` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_W as WUPPUPD3_W;
#[doc = "Field `WUPPUPD4` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_W as WUPPUPD4_W;
#[doc = "Field `WUPPUPD5` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
pub use WUPPUPD1_W as WUPPUPD5_W;
impl R {
    #[doc = "Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn wupen2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn wupen3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn wupen4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wuppupd1(&self) -> WUPPUPD1_R {
        WUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wuppupd2(&self) -> WUPPUPD2_R {
        WUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wuppupd3(&self) -> WUPPUPD3_R {
        WUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wuppupd4(&self) -> WUPPUPD4_R {
        WUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    pub fn wuppupd5(&self) -> WUPPUPD5_R {
        WUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    #[must_use]
    pub fn wupen1(&mut self) -> WUPEN1_W<WUCRrs> {
        WUPEN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    #[must_use]
    pub fn wupen2(&mut self) -> WUPEN2_W<WUCRrs> {
        WUPEN2_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    #[must_use]
    pub fn wupen3(&mut self) -> WUPEN3_W<WUCRrs> {
        WUPEN3_W::new(self, 2)
    }
    #[doc = "Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    #[must_use]
    pub fn wupen4(&mut self) -> WUPEN4_W<WUCRrs> {
        WUPEN4_W::new(self, 3)
    }
    #[doc = "Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
    #[inline(always)]
    #[must_use]
    pub fn wupen5(&mut self) -> WUPEN5_W<WUCRrs> {
        WUPEN5_W::new(self, 4)
    }
    #[doc = "Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    #[must_use]
    pub fn wupp1(&mut self) -> WUPP1_W<WUCRrs> {
        WUPP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    #[must_use]
    pub fn wupp2(&mut self) -> WUPP2_W<WUCRrs> {
        WUPP2_W::new(self, 9)
    }
    #[doc = "Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    #[must_use]
    pub fn wupp3(&mut self) -> WUPP3_W<WUCRrs> {
        WUPP3_W::new(self, 10)
    }
    #[doc = "Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    #[must_use]
    pub fn wupp4(&mut self) -> WUPP4_W<WUCRrs> {
        WUPP4_W::new(self, 11)
    }
    #[doc = "Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
    #[inline(always)]
    #[must_use]
    pub fn wupp5(&mut self) -> WUPP5_W<WUCRrs> {
        WUPP5_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn wuppupd1(&mut self) -> WUPPUPD1_W<WUCRrs> {
        WUPPUPD1_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn wuppupd2(&mut self) -> WUPPUPD2_W<WUCRrs> {
        WUPPUPD2_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn wuppupd3(&mut self) -> WUPPUPD3_W<WUCRrs> {
        WUPPUPD3_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn wuppupd4(&mut self) -> WUPPUPD4_W<WUCRrs> {
        WUPPUPD4_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn wuppupd5(&mut self) -> WUPPUPD5_W<WUCRrs> {
        WUPPUPD5_W::new(self, 24)
    }
}
#[doc = "PWR wakeup configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUCRrs;
impl crate::RegisterSpec for WUCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wucr::R`](R) reader structure"]
impl crate::Readable for WUCRrs {}
#[doc = "`write(|w| ..)` method takes [`wucr::W`](W) writer structure"]
impl crate::Writable for WUCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUCR to value 0"]
impl crate::Resettable for WUCRrs {
    const RESET_VALUE: u32 = 0;
}
