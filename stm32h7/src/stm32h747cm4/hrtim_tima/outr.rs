///Register `OUTR` reader
pub type R = crate::R<OUTRrs>;
///Register `OUTR` writer
pub type W = crate::W<OUTRrs>;
/**Output 1 polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL1 {
    ///0: Positive polarity (output active high)
    ActiveHigh = 0,
    ///1: Negative polarity (output active low)
    ActiveLow = 1,
}
impl From<POL1> for bool {
    #[inline(always)]
    fn from(variant: POL1) -> Self {
        variant as u8 != 0
    }
}
///Field `POL1` reader - Output 1 polarity
pub type POL1_R = crate::BitReader<POL1>;
impl POL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POL1 {
        match self.bits {
            false => POL1::ActiveHigh,
            true => POL1::ActiveLow,
        }
    }
    ///Positive polarity (output active high)
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == POL1::ActiveHigh
    }
    ///Negative polarity (output active low)
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == POL1::ActiveLow
    }
}
///Field `POL1` writer - Output 1 polarity
pub type POL1_W<'a, REG> = crate::BitWriter<'a, REG, POL1>;
impl<'a, REG> POL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive polarity (output active high)
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(POL1::ActiveHigh)
    }
    ///Negative polarity (output active low)
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(POL1::ActiveLow)
    }
}
/**Output 1 Idle mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEM1 {
    ///0: No action: the output is not affected by the burst mode operation
    NoEffect = 0,
    ///1: The output is in idle state when requested by the burst mode controller
    SetIdle = 1,
}
impl From<IDLEM1> for bool {
    #[inline(always)]
    fn from(variant: IDLEM1) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLEM1` reader - Output 1 Idle mode
pub type IDLEM1_R = crate::BitReader<IDLEM1>;
impl IDLEM1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLEM1 {
        match self.bits {
            false => IDLEM1::NoEffect,
            true => IDLEM1::SetIdle,
        }
    }
    ///No action: the output is not affected by the burst mode operation
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IDLEM1::NoEffect
    }
    ///The output is in idle state when requested by the burst mode controller
    #[inline(always)]
    pub fn is_set_idle(&self) -> bool {
        *self == IDLEM1::SetIdle
    }
}
///Field `IDLEM1` writer - Output 1 Idle mode
pub type IDLEM1_W<'a, REG> = crate::BitWriter<'a, REG, IDLEM1>;
impl<'a, REG> IDLEM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action: the output is not affected by the burst mode operation
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEM1::NoEffect)
    }
    ///The output is in idle state when requested by the burst mode controller
    #[inline(always)]
    pub fn set_idle(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEM1::SetIdle)
    }
}
/**Output 1 Idle State

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLES1 {
    ///0: Output idle state is inactive
    Inactive = 0,
    ///1: Output idle state is active
    Active = 1,
}
impl From<IDLES1> for bool {
    #[inline(always)]
    fn from(variant: IDLES1) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLES1` reader - Output 1 Idle State
pub type IDLES1_R = crate::BitReader<IDLES1>;
impl IDLES1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLES1 {
        match self.bits {
            false => IDLES1::Inactive,
            true => IDLES1::Active,
        }
    }
    ///Output idle state is inactive
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IDLES1::Inactive
    }
    ///Output idle state is active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IDLES1::Active
    }
}
///Field `IDLES1` writer - Output 1 Idle State
pub type IDLES1_W<'a, REG> = crate::BitWriter<'a, REG, IDLES1>;
impl<'a, REG> IDLES1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output idle state is inactive
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IDLES1::Inactive)
    }
    ///Output idle state is active
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(IDLES1::Active)
    }
}
/**Output 1 Fault state

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAULT1 {
    ///0: No action: the output is not affected by the fault input and stays in run mode
    Disabled = 0,
    ///1: Output goes to active state after a fault event
    SetActive = 1,
    ///2: Output goes to inactive state after a fault event
    SetInactive = 2,
    ///3: Output goes to high-z state after a fault event
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
impl crate::IsEnum for FAULT1 {}
///Field `FAULT1` reader - Output 1 Fault state
pub type FAULT1_R = crate::FieldReader<FAULT1>;
impl FAULT1_R {
    ///Get enumerated values variant
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
    ///No action: the output is not affected by the fault input and stays in run mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAULT1::Disabled
    }
    ///Output goes to active state after a fault event
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == FAULT1::SetActive
    }
    ///Output goes to inactive state after a fault event
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == FAULT1::SetInactive
    }
    ///Output goes to high-z state after a fault event
    #[inline(always)]
    pub fn is_set_high_z(&self) -> bool {
        *self == FAULT1::SetHighZ
    }
}
///Field `FAULT1` writer - Output 1 Fault state
pub type FAULT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FAULT1, crate::Safe>;
impl<'a, REG> FAULT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No action: the output is not affected by the fault input and stays in run mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::Disabled)
    }
    ///Output goes to active state after a fault event
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::SetActive)
    }
    ///Output goes to inactive state after a fault event
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::SetInactive)
    }
    ///Output goes to high-z state after a fault event
    #[inline(always)]
    pub fn set_high_z(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT1::SetHighZ)
    }
}
/**Output 1 Chopper enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHP1 {
    ///0: Output signal not altered
    Disabled = 0,
    ///1: Output signal is chopped by a carrier signal
    Enabled = 1,
}
impl From<CHP1> for bool {
    #[inline(always)]
    fn from(variant: CHP1) -> Self {
        variant as u8 != 0
    }
}
///Field `CHP1` reader - Output 1 Chopper enable
pub type CHP1_R = crate::BitReader<CHP1>;
impl CHP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHP1 {
        match self.bits {
            false => CHP1::Disabled,
            true => CHP1::Enabled,
        }
    }
    ///Output signal not altered
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHP1::Disabled
    }
    ///Output signal is chopped by a carrier signal
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHP1::Enabled
    }
}
///Field `CHP1` writer - Output 1 Chopper enable
pub type CHP1_W<'a, REG> = crate::BitWriter<'a, REG, CHP1>;
impl<'a, REG> CHP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output signal not altered
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHP1::Disabled)
    }
    ///Output signal is chopped by a carrier signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHP1::Enabled)
    }
}
/**Output 1 Deadtime upon burst mode Idle entry

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIDL1 {
    ///0: The programmed idle state is applied immediately to the output
    Disabled = 0,
    ///1: Deadtime (inactive level) is inserted on output before entering the idle mode
    Enabled = 1,
}
impl From<DIDL1> for bool {
    #[inline(always)]
    fn from(variant: DIDL1) -> Self {
        variant as u8 != 0
    }
}
///Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry
pub type DIDL1_R = crate::BitReader<DIDL1>;
impl DIDL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIDL1 {
        match self.bits {
            false => DIDL1::Disabled,
            true => DIDL1::Enabled,
        }
    }
    ///The programmed idle state is applied immediately to the output
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIDL1::Disabled
    }
    ///Deadtime (inactive level) is inserted on output before entering the idle mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIDL1::Enabled
    }
}
///Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry
pub type DIDL1_W<'a, REG> = crate::BitWriter<'a, REG, DIDL1>;
impl<'a, REG> DIDL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The programmed idle state is applied immediately to the output
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIDL1::Disabled)
    }
    ///Deadtime (inactive level) is inserted on output before entering the idle mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIDL1::Enabled)
    }
}
/**Deadtime enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN {
    ///0: Output 1 and 2 signals are independent
    Disabled = 0,
    ///1: Deadtime is inserted between output 1 and output 2
    Enabled = 1,
}
impl From<DTEN> for bool {
    #[inline(always)]
    fn from(variant: DTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DTEN` reader - Deadtime enable
pub type DTEN_R = crate::BitReader<DTEN>;
impl DTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTEN {
        match self.bits {
            false => DTEN::Disabled,
            true => DTEN::Enabled,
        }
    }
    ///Output 1 and 2 signals are independent
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN::Disabled
    }
    ///Deadtime is inserted between output 1 and output 2
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN::Enabled
    }
}
///Field `DTEN` writer - Deadtime enable
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG, DTEN>;
impl<'a, REG> DTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output 1 and 2 signals are independent
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Disabled)
    }
    ///Deadtime is inserted between output 1 and output 2
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Enabled)
    }
}
/**Delayed Protection Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTEN {
    ///0: No action
    Disabled = 0,
    ///1: Delayed protection is enabled, as per DLYPRT bits
    Enabled = 1,
}
impl From<DLYPRTEN> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYPRTEN` reader - Delayed Protection Enable
pub type DLYPRTEN_R = crate::BitReader<DLYPRTEN>;
impl DLYPRTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRTEN {
        match self.bits {
            false => DLYPRTEN::Disabled,
            true => DLYPRTEN::Enabled,
        }
    }
    ///No action
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTEN::Disabled
    }
    ///Delayed protection is enabled, as per DLYPRT bits
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTEN::Enabled
    }
}
///Field `DLYPRTEN` writer - Delayed Protection Enable
pub type DLYPRTEN_W<'a, REG> = crate::BitWriter<'a, REG, DLYPRTEN>;
impl<'a, REG> DLYPRTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTEN::Disabled)
    }
    ///Delayed protection is enabled, as per DLYPRT bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTEN::Enabled)
    }
}
/**Delayed Protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYPRT {
    ///0: Output 1 delayed idle on external event 6
    Output1Ee6 = 0,
    ///1: Output 2 delayed idle on external event 6
    Output2Ee6 = 1,
    ///2: Output 1 and 2 delayed idle on external event 6
    Output1_2Ee6 = 2,
    ///3: Balanced idle on external event 6
    BalancedEe6 = 3,
    ///4: Output 1 delayed idle on external event 7
    Output1Ee7 = 4,
    ///5: Output 2 delayed idle on external event 7
    Output2Ee7 = 5,
    ///6: Output 1 and 2 delayed idle on external event 7
    Output1_2Ee7 = 6,
    ///7: Balanced idle on external event 7
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
impl crate::IsEnum for DLYPRT {}
///Field `DLYPRT` reader - Delayed Protection
pub type DLYPRT_R = crate::FieldReader<DLYPRT>;
impl DLYPRT_R {
    ///Get enumerated values variant
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
    ///Output 1 delayed idle on external event 6
    #[inline(always)]
    pub fn is_output1_ee6(&self) -> bool {
        *self == DLYPRT::Output1Ee6
    }
    ///Output 2 delayed idle on external event 6
    #[inline(always)]
    pub fn is_output2_ee6(&self) -> bool {
        *self == DLYPRT::Output2Ee6
    }
    ///Output 1 and 2 delayed idle on external event 6
    #[inline(always)]
    pub fn is_output1_2_ee6(&self) -> bool {
        *self == DLYPRT::Output1_2Ee6
    }
    ///Balanced idle on external event 6
    #[inline(always)]
    pub fn is_balanced_ee6(&self) -> bool {
        *self == DLYPRT::BalancedEe6
    }
    ///Output 1 delayed idle on external event 7
    #[inline(always)]
    pub fn is_output1_ee7(&self) -> bool {
        *self == DLYPRT::Output1Ee7
    }
    ///Output 2 delayed idle on external event 7
    #[inline(always)]
    pub fn is_output2_ee7(&self) -> bool {
        *self == DLYPRT::Output2Ee7
    }
    ///Output 1 and 2 delayed idle on external event 7
    #[inline(always)]
    pub fn is_output1_2_ee7(&self) -> bool {
        *self == DLYPRT::Output1_2Ee7
    }
    ///Balanced idle on external event 7
    #[inline(always)]
    pub fn is_balanced_ee7(&self) -> bool {
        *self == DLYPRT::BalancedEe7
    }
}
///Field `DLYPRT` writer - Delayed Protection
pub type DLYPRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DLYPRT, crate::Safe>;
impl<'a, REG> DLYPRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output 1 delayed idle on external event 6
    #[inline(always)]
    pub fn output1_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1Ee6)
    }
    ///Output 2 delayed idle on external event 6
    #[inline(always)]
    pub fn output2_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output2Ee6)
    }
    ///Output 1 and 2 delayed idle on external event 6
    #[inline(always)]
    pub fn output1_2_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1_2Ee6)
    }
    ///Balanced idle on external event 6
    #[inline(always)]
    pub fn balanced_ee6(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::BalancedEe6)
    }
    ///Output 1 delayed idle on external event 7
    #[inline(always)]
    pub fn output1_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1Ee7)
    }
    ///Output 2 delayed idle on external event 7
    #[inline(always)]
    pub fn output2_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output2Ee7)
    }
    ///Output 1 and 2 delayed idle on external event 7
    #[inline(always)]
    pub fn output1_2_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::Output1_2Ee7)
    }
    ///Balanced idle on external event 7
    #[inline(always)]
    pub fn balanced_ee7(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRT::BalancedEe7)
    }
}
///Field `CHP2` reader - Output 2 Chopper enable
pub use CHP1_R as CHP2_R;
///Field `CHP2` writer - Output 2 Chopper enable
pub use CHP1_W as CHP2_W;
///Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry
pub use DIDL1_R as DIDL2_R;
///Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry
pub use DIDL1_W as DIDL2_W;
///Field `FAULT2` reader - Output 2 Fault state
pub use FAULT1_R as FAULT2_R;
///Field `FAULT2` writer - Output 2 Fault state
pub use FAULT1_W as FAULT2_W;
///Field `IDLEM2` reader - Output 2 Idle mode
pub use IDLEM1_R as IDLEM2_R;
///Field `IDLEM2` writer - Output 2 Idle mode
pub use IDLEM1_W as IDLEM2_W;
///Field `IDLES2` reader - Output 2 Idle State
pub use IDLES1_R as IDLES2_R;
///Field `IDLES2` writer - Output 2 Idle State
pub use IDLES1_W as IDLES2_W;
///Field `POL2` reader - Output 2 polarity
pub use POL1_R as POL2_R;
///Field `POL2` writer - Output 2 polarity
pub use POL1_W as POL2_W;
impl R {
    ///Bit 1 - Output 1 polarity
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output 1 Idle mode
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output 1 Idle State
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Output 1 Fault state
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Output 1 Chopper enable
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Output 1 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Deadtime enable
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Delayed Protection Enable
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Delayed Protection
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 17 - Output 2 polarity
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Output 2 Idle mode
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output 2 Idle State
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Output 2 Fault state
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Output 2 Chopper enable
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Output 2 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTR")
            .field("didl1", &self.didl1())
            .field("didl2", &self.didl2())
            .field("chp1", &self.chp1())
            .field("chp2", &self.chp2())
            .field("fault1", &self.fault1())
            .field("fault2", &self.fault2())
            .field("idles1", &self.idles1())
            .field("idles2", &self.idles2())
            .field("idlem1", &self.idlem1())
            .field("idlem2", &self.idlem2())
            .field("pol1", &self.pol1())
            .field("pol2", &self.pol2())
            .field("dlyprt", &self.dlyprt())
            .field("dlyprten", &self.dlyprten())
            .field("dten", &self.dten())
            .finish()
    }
}
impl W {
    ///Bit 1 - Output 1 polarity
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W<'_, OUTRrs> {
        POL1_W::new(self, 1)
    }
    ///Bit 2 - Output 1 Idle mode
    #[inline(always)]
    pub fn idlem1(&mut self) -> IDLEM1_W<'_, OUTRrs> {
        IDLEM1_W::new(self, 2)
    }
    ///Bit 3 - Output 1 Idle State
    #[inline(always)]
    pub fn idles1(&mut self) -> IDLES1_W<'_, OUTRrs> {
        IDLES1_W::new(self, 3)
    }
    ///Bits 4:5 - Output 1 Fault state
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W<'_, OUTRrs> {
        FAULT1_W::new(self, 4)
    }
    ///Bit 6 - Output 1 Chopper enable
    #[inline(always)]
    pub fn chp1(&mut self) -> CHP1_W<'_, OUTRrs> {
        CHP1_W::new(self, 6)
    }
    ///Bit 7 - Output 1 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl1(&mut self) -> DIDL1_W<'_, OUTRrs> {
        DIDL1_W::new(self, 7)
    }
    ///Bit 8 - Deadtime enable
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<'_, OUTRrs> {
        DTEN_W::new(self, 8)
    }
    ///Bit 9 - Delayed Protection Enable
    #[inline(always)]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W<'_, OUTRrs> {
        DLYPRTEN_W::new(self, 9)
    }
    ///Bits 10:12 - Delayed Protection
    #[inline(always)]
    pub fn dlyprt(&mut self) -> DLYPRT_W<'_, OUTRrs> {
        DLYPRT_W::new(self, 10)
    }
    ///Bit 17 - Output 2 polarity
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W<'_, OUTRrs> {
        POL2_W::new(self, 17)
    }
    ///Bit 18 - Output 2 Idle mode
    #[inline(always)]
    pub fn idlem2(&mut self) -> IDLEM2_W<'_, OUTRrs> {
        IDLEM2_W::new(self, 18)
    }
    ///Bit 19 - Output 2 Idle State
    #[inline(always)]
    pub fn idles2(&mut self) -> IDLES2_W<'_, OUTRrs> {
        IDLES2_W::new(self, 19)
    }
    ///Bits 20:21 - Output 2 Fault state
    #[inline(always)]
    pub fn fault2(&mut self) -> FAULT2_W<'_, OUTRrs> {
        FAULT2_W::new(self, 20)
    }
    ///Bit 22 - Output 2 Chopper enable
    #[inline(always)]
    pub fn chp2(&mut self) -> CHP2_W<'_, OUTRrs> {
        CHP2_W::new(self, 22)
    }
    ///Bit 23 - Output 2 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl2(&mut self) -> DIDL2_W<'_, OUTRrs> {
        DIDL2_W::new(self, 23)
    }
}
/**Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#HRTIM_TIMA:OUTR)*/
pub struct OUTRrs;
impl crate::RegisterSpec for OUTRrs {
    type Ux = u32;
}
///`read()` method returns [`outr::R`](R) reader structure
impl crate::Readable for OUTRrs {}
///`write(|w| ..)` method takes [`outr::W`](W) writer structure
impl crate::Writable for OUTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OUTR to value 0
impl crate::Resettable for OUTRrs {}
