#[doc = "Register `OUTCR` reader"]
pub type R = crate::R<OUTCRrs>;
#[doc = "Register `OUTCR` writer"]
pub type W = crate::W<OUTCRrs>;
#[doc = "Output 1 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL1 {
    #[doc = "0: Positive polarity (output active high)"]
    ActiveHigh = 0,
    #[doc = "1: Negative polarity (output active low)"]
    ActiveLow = 1,
}
impl From<POL1> for bool {
    #[inline(always)]
    fn from(variant: POL1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL1` reader - Output 1 polarity"]
pub type POL1_R = crate::BitReader<POL1>;
impl POL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POL1 {
        match self.bits {
            false => POL1::ActiveHigh,
            true => POL1::ActiveLow,
        }
    }
    #[doc = "Positive polarity (output active high)"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == POL1::ActiveHigh
    }
    #[doc = "Negative polarity (output active low)"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == POL1::ActiveLow
    }
}
#[doc = "Field `POL1` writer - Output 1 polarity"]
pub type POL1_W<'a, REG> = crate::BitWriter<'a, REG, POL1>;
impl<'a, REG> POL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive polarity (output active high)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(POL1::ActiveHigh)
    }
    #[doc = "Negative polarity (output active low)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(POL1::ActiveLow)
    }
}
#[doc = "Output 1 Idle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEM1 {
    #[doc = "0: No action: the output is not affected by the burst mode operation"]
    NoEffect = 0,
    #[doc = "1: The output is in idle state when requested by the burst mode controller"]
    SetIdle = 1,
}
impl From<IDLEM1> for bool {
    #[inline(always)]
    fn from(variant: IDLEM1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEM1` reader - Output 1 Idle mode"]
pub type IDLEM1_R = crate::BitReader<IDLEM1>;
impl IDLEM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLEM1 {
        match self.bits {
            false => IDLEM1::NoEffect,
            true => IDLEM1::SetIdle,
        }
    }
    #[doc = "No action: the output is not affected by the burst mode operation"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IDLEM1::NoEffect
    }
    #[doc = "The output is in idle state when requested by the burst mode controller"]
    #[inline(always)]
    pub fn is_set_idle(&self) -> bool {
        *self == IDLEM1::SetIdle
    }
}
#[doc = "Field `IDLEM1` writer - Output 1 Idle mode"]
pub type IDLEM1_W<'a, REG> = crate::BitWriter<'a, REG, IDLEM1>;
impl<'a, REG> IDLEM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action: the output is not affected by the burst mode operation"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEM1::NoEffect)
    }
    #[doc = "The output is in idle state when requested by the burst mode controller"]
    #[inline(always)]
    pub fn set_idle(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEM1::SetIdle)
    }
}
#[doc = "Output 1 Idle State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLES1 {
    #[doc = "0: Output idle state is inactive"]
    Inactive = 0,
    #[doc = "1: Output idle state is active"]
    Active = 1,
}
impl From<IDLES1> for bool {
    #[inline(always)]
    fn from(variant: IDLES1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLES1` reader - Output 1 Idle State"]
pub type IDLES1_R = crate::BitReader<IDLES1>;
impl IDLES1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLES1 {
        match self.bits {
            false => IDLES1::Inactive,
            true => IDLES1::Active,
        }
    }
    #[doc = "Output idle state is inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IDLES1::Inactive
    }
    #[doc = "Output idle state is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IDLES1::Active
    }
}
#[doc = "Field `IDLES1` writer - Output 1 Idle State"]
pub type IDLES1_W<'a, REG> = crate::BitWriter<'a, REG, IDLES1>;
impl<'a, REG> IDLES1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output idle state is inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IDLES1::Inactive)
    }
    #[doc = "Output idle state is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(IDLES1::Active)
    }
}
#[doc = "Output 1 Fault state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAULT1 {
    #[doc = "0: No action: the output is not affected by the fault input and stays in run mode"]
    Disabled = 0,
    #[doc = "1: Output goes to active state after a fault event"]
    SetActive = 1,
    #[doc = "2: Output goes to inactive state after a fault event"]
    SetInactive = 2,
    #[doc = "3: Output goes to high-z state after a fault event"]
    SetHighZ = 3,
}
impl From<FAULT1> for u8 {
    #[inline(always)]
    fn from(variant: FAULT1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FAULT1 {
    type Ux = u8;
}
#[doc = "Field `FAULT1` reader - Output 1 Fault state"]
pub type FAULT1_R = crate::FieldReader<FAULT1>;
impl FAULT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FAULT1 {
        match self.bits {
            0 => FAULT1::Disabled,
            1 => FAULT1::SetActive,
            2 => FAULT1::SetInactive,
            3 => FAULT1::SetHighZ,
            _ => unreachable!(),
        }
    }
    #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAULT1::Disabled
    }
    #[doc = "Output goes to active state after a fault event"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == FAULT1::SetActive
    }
    #[doc = "Output goes to inactive state after a fault event"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == FAULT1::SetInactive
    }
    #[doc = "Output goes to high-z state after a fault event"]
    #[inline(always)]
    pub fn is_set_high_z(&self) -> bool {
        *self == FAULT1::SetHighZ
    }
}
#[doc = "Field `FAULT1` writer - Output 1 Fault state"]
pub type FAULT1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FAULT1>;
impl<'a, REG> FAULT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::Disabled)
    }
    #[doc = "Output goes to active state after a fault event"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::SetActive)
    }
    #[doc = "Output goes to inactive state after a fault event"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::SetInactive)
    }
    #[doc = "Output goes to high-z state after a fault event"]
    #[inline(always)]
    pub fn set_high_z(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::SetHighZ)
    }
}
#[doc = "Output 1 Chopper enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHP1 {
    #[doc = "0: Output signal not altered"]
    Disabled = 0,
    #[doc = "1: Output signal is chopped by a carrier signal"]
    Enabled = 1,
}
impl From<CHP1> for bool {
    #[inline(always)]
    fn from(variant: CHP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHP1` reader - Output 1 Chopper enable"]
pub type CHP1_R = crate::BitReader<CHP1>;
impl CHP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHP1 {
        match self.bits {
            false => CHP1::Disabled,
            true => CHP1::Enabled,
        }
    }
    #[doc = "Output signal not altered"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHP1::Disabled
    }
    #[doc = "Output signal is chopped by a carrier signal"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHP1::Enabled
    }
}
#[doc = "Field `CHP1` writer - Output 1 Chopper enable"]
pub type CHP1_W<'a, REG> = crate::BitWriter<'a, REG, CHP1>;
impl<'a, REG> CHP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output signal not altered"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHP1::Disabled)
    }
    #[doc = "Output signal is chopped by a carrier signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHP1::Enabled)
    }
}
#[doc = "Output 1 Deadtime upon burst mode Idle entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIDL1 {
    #[doc = "0: The programmed idle state is applied immediately to the output"]
    Disabled = 0,
    #[doc = "1: Deadtime (inactive level) is inserted on output before entering the idle mode"]
    Enabled = 1,
}
impl From<DIDL1> for bool {
    #[inline(always)]
    fn from(variant: DIDL1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry"]
pub type DIDL1_R = crate::BitReader<DIDL1>;
impl DIDL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIDL1 {
        match self.bits {
            false => DIDL1::Disabled,
            true => DIDL1::Enabled,
        }
    }
    #[doc = "The programmed idle state is applied immediately to the output"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIDL1::Disabled
    }
    #[doc = "Deadtime (inactive level) is inserted on output before entering the idle mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIDL1::Enabled
    }
}
#[doc = "Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry"]
pub type DIDL1_W<'a, REG> = crate::BitWriter<'a, REG, DIDL1>;
impl<'a, REG> DIDL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The programmed idle state is applied immediately to the output"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIDL1::Disabled)
    }
    #[doc = "Deadtime (inactive level) is inserted on output before entering the idle mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIDL1::Enabled)
    }
}
#[doc = "Deadtime enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN {
    #[doc = "0: Output 1 and 2 signals are independent"]
    Disabled = 0,
    #[doc = "1: Deadtime is inserted between output 1 and output 2"]
    Enabled = 1,
}
impl From<DTEN> for bool {
    #[inline(always)]
    fn from(variant: DTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN` reader - Deadtime enable"]
pub type DTEN_R = crate::BitReader<DTEN>;
impl DTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTEN {
        match self.bits {
            false => DTEN::Disabled,
            true => DTEN::Enabled,
        }
    }
    #[doc = "Output 1 and 2 signals are independent"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN::Disabled
    }
    #[doc = "Deadtime is inserted between output 1 and output 2"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN::Enabled
    }
}
#[doc = "Field `DTEN` writer - Deadtime enable"]
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG, DTEN>;
impl<'a, REG> DTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 1 and 2 signals are independent"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Disabled)
    }
    #[doc = "Deadtime is inserted between output 1 and output 2"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Enabled)
    }
}
#[doc = "Delayed Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTEN {
    #[doc = "0: No action"]
    Disabled = 0,
    #[doc = "1: Delayed protection is enabled, as per DLYPRT bits"]
    Enabled = 1,
}
impl From<DLYPRTEN> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLYPRTEN` reader - Delayed Protection Enable"]
pub type DLYPRTEN_R = crate::BitReader<DLYPRTEN>;
impl DLYPRTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRTEN {
        match self.bits {
            false => DLYPRTEN::Disabled,
            true => DLYPRTEN::Enabled,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTEN::Disabled
    }
    #[doc = "Delayed protection is enabled, as per DLYPRT bits"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTEN::Enabled
    }
}
#[doc = "Field `DLYPRTEN` writer - Delayed Protection Enable"]
pub type DLYPRTEN_W<'a, REG> = crate::BitWriter<'a, REG, DLYPRTEN>;
impl<'a, REG> DLYPRTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTEN::Disabled)
    }
    #[doc = "Delayed protection is enabled, as per DLYPRT bits"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTEN::Enabled)
    }
}
#[doc = "Delayed Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYPRT {
    #[doc = "0: Output 1 delayed idle on external event 6"]
    Output1Ee6 = 0,
    #[doc = "1: Output 2 delayed idle on external event 6"]
    Output2Ee6 = 1,
    #[doc = "2: Output 1 and 2 delayed idle on external event 6"]
    Output1_2Ee6 = 2,
    #[doc = "3: Balanced idle on external event 6"]
    BalancedEe6 = 3,
    #[doc = "4: Output 1 delayed idle on external event 7"]
    Output1Ee7 = 4,
    #[doc = "5: Output 2 delayed idle on external event 7"]
    Output2Ee7 = 5,
    #[doc = "6: Output 1 and 2 delayed idle on external event 7"]
    Output1_2Ee7 = 6,
    #[doc = "7: Balanced idle on external event 7"]
    BalancedEe7 = 7,
}
impl From<DLYPRT> for u8 {
    #[inline(always)]
    fn from(variant: DLYPRT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLYPRT {
    type Ux = u8;
}
#[doc = "Field `DLYPRT` reader - Delayed Protection"]
pub type DLYPRT_R = crate::FieldReader<DLYPRT>;
impl DLYPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRT {
        match self.bits {
            0 => DLYPRT::Output1Ee6,
            1 => DLYPRT::Output2Ee6,
            2 => DLYPRT::Output1_2Ee6,
            3 => DLYPRT::BalancedEe6,
            4 => DLYPRT::Output1Ee7,
            5 => DLYPRT::Output2Ee7,
            6 => DLYPRT::Output1_2Ee7,
            7 => DLYPRT::BalancedEe7,
            _ => unreachable!(),
        }
    }
    #[doc = "Output 1 delayed idle on external event 6"]
    #[inline(always)]
    pub fn is_output1_ee6(&self) -> bool {
        *self == DLYPRT::Output1Ee6
    }
    #[doc = "Output 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn is_output2_ee6(&self) -> bool {
        *self == DLYPRT::Output2Ee6
    }
    #[doc = "Output 1 and 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn is_output1_2_ee6(&self) -> bool {
        *self == DLYPRT::Output1_2Ee6
    }
    #[doc = "Balanced idle on external event 6"]
    #[inline(always)]
    pub fn is_balanced_ee6(&self) -> bool {
        *self == DLYPRT::BalancedEe6
    }
    #[doc = "Output 1 delayed idle on external event 7"]
    #[inline(always)]
    pub fn is_output1_ee7(&self) -> bool {
        *self == DLYPRT::Output1Ee7
    }
    #[doc = "Output 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn is_output2_ee7(&self) -> bool {
        *self == DLYPRT::Output2Ee7
    }
    #[doc = "Output 1 and 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn is_output1_2_ee7(&self) -> bool {
        *self == DLYPRT::Output1_2Ee7
    }
    #[doc = "Balanced idle on external event 7"]
    #[inline(always)]
    pub fn is_balanced_ee7(&self) -> bool {
        *self == DLYPRT::BalancedEe7
    }
}
#[doc = "Field `DLYPRT` writer - Delayed Protection"]
pub type DLYPRT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DLYPRT>;
impl<'a, REG> DLYPRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output 1 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output1_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1Ee6)
    }
    #[doc = "Output 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output2_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output2Ee6)
    }
    #[doc = "Output 1 and 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output1_2_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1_2Ee6)
    }
    #[doc = "Balanced idle on external event 6"]
    #[inline(always)]
    pub fn balanced_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::BalancedEe6)
    }
    #[doc = "Output 1 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output1_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1Ee7)
    }
    #[doc = "Output 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output2_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output2Ee7)
    }
    #[doc = "Output 1 and 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output1_2_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1_2Ee7)
    }
    #[doc = "Balanced idle on external event 7"]
    #[inline(always)]
    pub fn balanced_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::BalancedEe7)
    }
}
#[doc = "Field `CHP2` reader - Output 2 Chopper enable"]
pub use CHP1_R as CHP2_R;
#[doc = "Field `CHP2` writer - Output 2 Chopper enable"]
pub use CHP1_W as CHP2_W;
#[doc = "Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry"]
pub use DIDL1_R as DIDL2_R;
#[doc = "Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry"]
pub use DIDL1_W as DIDL2_W;
#[doc = "Field `FAULT2` reader - Output 2 Fault state"]
pub use FAULT1_R as FAULT2_R;
#[doc = "Field `FAULT2` writer - Output 2 Fault state"]
pub use FAULT1_W as FAULT2_W;
#[doc = "Field `IDLEM2` reader - Output 2 Idle mode"]
pub use IDLEM1_R as IDLEM2_R;
#[doc = "Field `IDLEM2` writer - Output 2 Idle mode"]
pub use IDLEM1_W as IDLEM2_W;
#[doc = "Field `IDLES2` reader - Output 2 Idle State"]
pub use IDLES1_R as IDLES2_R;
#[doc = "Field `IDLES2` writer - Output 2 Idle State"]
pub use IDLES1_W as IDLES2_W;
#[doc = "Field `POL2` reader - Output 2 polarity"]
pub use POL1_R as POL2_R;
#[doc = "Field `POL2` writer - Output 2 polarity"]
pub use POL1_W as POL2_W;
impl R {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> POL1_W<OUTCRrs> {
        POL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn idlem1(&mut self) -> IDLEM1_W<OUTCRrs> {
        IDLEM1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    #[must_use]
    pub fn idles1(&mut self) -> IDLES1_W<OUTCRrs> {
        IDLES1_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<OUTCRrs> {
        FAULT1_W::new(self, 4)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    #[must_use]
    pub fn chp1(&mut self) -> CHP1_W<OUTCRrs> {
        CHP1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    #[must_use]
    pub fn didl1(&mut self) -> DIDL1_W<OUTCRrs> {
        DIDL1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<OUTCRrs> {
        DTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W<OUTCRrs> {
        DLYPRTEN_W::new(self, 9)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprt(&mut self) -> DLYPRT_W<OUTCRrs> {
        DLYPRT_W::new(self, 10)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> POL2_W<OUTCRrs> {
        POL2_W::new(self, 17)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn idlem2(&mut self) -> IDLEM2_W<OUTCRrs> {
        IDLEM2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    #[must_use]
    pub fn idles2(&mut self) -> IDLES2_W<OUTCRrs> {
        IDLES2_W::new(self, 19)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    #[must_use]
    pub fn fault2(&mut self) -> FAULT2_W<OUTCRrs> {
        FAULT2_W::new(self, 20)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    #[must_use]
    pub fn chp2(&mut self) -> CHP2_W<OUTCRrs> {
        CHP2_W::new(self, 22)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    #[must_use]
    pub fn didl2(&mut self) -> DIDL2_W<OUTCRrs> {
        DIDL2_W::new(self, 23)
    }
}
#[doc = "Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTCRrs;
impl crate::RegisterSpec for OUTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outcr::R`](R) reader structure"]
impl crate::Readable for OUTCRrs {}
#[doc = "`write(|w| ..)` method takes [`outcr::W`](W) writer structure"]
impl crate::Writable for OUTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCR to value 0"]
impl crate::Resettable for OUTCRrs {
    const RESET_VALUE: u32 = 0;
}
