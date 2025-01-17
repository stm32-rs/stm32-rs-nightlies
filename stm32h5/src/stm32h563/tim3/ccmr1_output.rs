///Register `CCMR1_Output` reader
pub type R = crate::R<CCMR1_OUTPUTrs>;
///Register `CCMR1_Output` writer
pub type W = crate::W<CCMR1_OUTPUTrs>;
/**Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S {
    ///0: CC1 channel is configured as output
    Output = 0,
}
impl From<CC1S> for u8 {
    #[inline(always)]
    fn from(variant: CC1S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC1S {
    type Ux = u8;
}
impl crate::IsEnum for CC1S {}
///Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
pub type CC1S_R = crate::FieldReader<CC1S>;
impl CC1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC1S> {
        match self.bits {
            0 => Some(CC1S::Output),
            _ => None,
        }
    }
    ///CC1 channel is configured as output
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC1S::Output
    }
}
///Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1S>;
impl<'a, REG> CC1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC1 channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Output)
    }
}
///Field `OC1FE` reader - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
pub type OC1FE_R = crate::BitReader;
///Field `OC1FE` writer - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1PE {
    ///0: Preload register on CCR1 disabled. New values written to CCR1 are taken into account immediately
    Disabled = 0,
    ///1: Preload register on CCR1 enabled. Preload value is loaded into active register on each update event
    Enabled = 1,
}
impl From<OC1PE> for bool {
    #[inline(always)]
    fn from(variant: OC1PE) -> Self {
        variant as u8 != 0
    }
}
///Field `OC1PE` reader - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
pub type OC1PE_R = crate::BitReader<OC1PE>;
impl OC1PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1PE {
        match self.bits {
            false => OC1PE::Disabled,
            true => OC1PE::Enabled,
        }
    }
    ///Preload register on CCR1 disabled. New values written to CCR1 are taken into account immediately
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC1PE::Disabled
    }
    ///Preload register on CCR1 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC1PE::Enabled
    }
}
///Field `OC1PE` writer - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG, OC1PE>;
impl<'a, REG> OC1PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preload register on CCR1 disabled. New values written to CCR1 are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE::Disabled)
    }
    ///Preload register on CCR1 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE::Enabled)
    }
}
/**Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC1M {
    ///0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    Frozen = 0,
    ///1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    ActiveOnMatch = 1,
    ///2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    InactiveOnMatch = 2,
    ///3: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    Toggle = 3,
    ///4: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    ForceInactive = 4,
    ///5: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    ForceActive = 5,
    ///6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    PwmMode1 = 6,
    ///7: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    PwmMode2 = 7,
}
impl From<OC1M> for u8 {
    #[inline(always)]
    fn from(variant: OC1M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC1M {
    type Ux = u8;
}
impl crate::IsEnum for OC1M {}
///Field `OC1M` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
pub type OC1M_R = crate::FieldReader<OC1M>;
impl OC1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1M {
        match self.bits {
            0 => OC1M::Frozen,
            1 => OC1M::ActiveOnMatch,
            2 => OC1M::InactiveOnMatch,
            3 => OC1M::Toggle,
            4 => OC1M::ForceInactive,
            5 => OC1M::ForceActive,
            6 => OC1M::PwmMode1,
            7 => OC1M::PwmMode2,
            _ => unreachable!(),
        }
    }
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC1M::Frozen
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC1M::ActiveOnMatch
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC1M::InactiveOnMatch
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC1M::Toggle
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC1M::ForceInactive
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC1M::ForceActive
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC1M::PwmMode1
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC1M::PwmMode2
    }
}
///Field `OC1M` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OC1M, crate::Safe>;
impl<'a, REG> OC1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::Toggle)
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ForceInactive)
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::PwmMode1)
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::PwmMode2)
    }
}
///Field `OC1CE` reader - Output compare 1 clear enable
pub type OC1CE_R = crate::BitReader;
///Field `OC1CE` writer - Output compare 1 clear enable
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC2S {
    ///0: CC2 channel is configured as output
    Output = 0,
}
impl From<CC2S> for u8 {
    #[inline(always)]
    fn from(variant: CC2S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC2S {
    type Ux = u8;
}
impl crate::IsEnum for CC2S {}
///Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_R = crate::FieldReader<CC2S>;
impl CC2S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC2S> {
        match self.bits {
            0 => Some(CC2S::Output),
            _ => None,
        }
    }
    ///CC2 channel is configured as output
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC2S::Output
    }
}
///Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC2S>;
impl<'a, REG> CC2S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC2 channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::Output)
    }
}
///Field `OC2FE` reader - Output compare 2 fast enable
pub type OC2FE_R = crate::BitReader;
///Field `OC2FE` writer - Output compare 2 fast enable
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output compare 2 preload enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC2PE {
    ///0: Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately
    Disabled = 0,
    ///1: Preload register on CCR2 enabled. Preload value is loaded into active register on each update event
    Enabled = 1,
}
impl From<OC2PE> for bool {
    #[inline(always)]
    fn from(variant: OC2PE) -> Self {
        variant as u8 != 0
    }
}
///Field `OC2PE` reader - Output compare 2 preload enable
pub type OC2PE_R = crate::BitReader<OC2PE>;
impl OC2PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC2PE {
        match self.bits {
            false => OC2PE::Disabled,
            true => OC2PE::Enabled,
        }
    }
    ///Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC2PE::Disabled
    }
    ///Preload register on CCR2 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC2PE::Enabled
    }
}
///Field `OC2PE` writer - Output compare 2 preload enable
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG, OC2PE>;
impl<'a, REG> OC2PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC2PE::Disabled)
    }
    ///Preload register on CCR2 enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC2PE::Enabled)
    }
}
///Field `OC2M` reader - Output compare 2 mode refer to OC1M description on bits 6:4
pub use OC1M_R as OC2M_R;
///Field `OC2M` writer - Output compare 2 mode refer to OC1M description on bits 6:4
pub use OC1M_W as OC2M_W;
///Field `OC2CE` reader - Output compare 2 clear enable
pub type OC2CE_R = crate::BitReader;
///Field `OC2CE` writer - Output compare 2 clear enable
pub type OC2CE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1M_3 {
    ///0: Normal output compare mode (modes 0-7)
    Normal = 0,
    ///1: Extended output compare mode (modes 7-15)
    Extended = 1,
}
impl From<OC1M_3> for bool {
    #[inline(always)]
    fn from(variant: OC1M_3) -> Self {
        variant as u8 != 0
    }
}
///Field `OC1M_3` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
pub type OC1M_3_R = crate::BitReader<OC1M_3>;
impl OC1M_3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1M_3 {
        match self.bits {
            false => OC1M_3::Normal,
            true => OC1M_3::Extended,
        }
    }
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC1M_3::Normal
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC1M_3::Extended
    }
}
///Field `OC1M_3` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
pub type OC1M_3_W<'a, REG> = crate::BitWriter<'a, REG, OC1M_3>;
impl<'a, REG> OC1M_3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M_3::Normal)
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M_3::Extended)
    }
}
///Field `OC2M_3` reader - Output compare 2 mode refer to OC1M description on bits 6:4
pub use OC1M_3_R as OC2M_3_R;
///Field `OC2M_3` writer - Output compare 2 mode refer to OC1M description on bits 6:4
pub use OC1M_3_W as OC2M_3_W;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4
    #[inline(always)]
    pub fn oc2m_3(&self) -> OC2M_3_R {
        OC2M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1_Output")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m", &self.oc1m())
            .field("oc1ce", &self.oc1ce())
            .field("cc2s", &self.cc2s())
            .field("oc2fe", &self.oc2fe())
            .field("oc2pe", &self.oc2pe())
            .field("oc2m", &self.oc2m())
            .field("oc2ce", &self.oc2ce())
            .field("oc1m_3", &self.oc1m_3())
            .field("oc2m_3", &self.oc2m_3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_OUTPUTrs> {
        CC1S_W::new(self, 0)
    }
    ///Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W<CCMR1_OUTPUTrs> {
        OC1FE_W::new(self, 2)
    }
    ///Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W<CCMR1_OUTPUTrs> {
        OC1PE_W::new(self, 3)
    }
    ///Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W<CCMR1_OUTPUTrs> {
        OC1M_W::new(self, 4)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OC1CE_W<CCMR1_OUTPUTrs> {
        OC1CE_W::new(self, 7)
    }
    ///Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W<CCMR1_OUTPUTrs> {
        CC2S_W::new(self, 8)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OC2FE_W<CCMR1_OUTPUTrs> {
        OC2FE_W::new(self, 10)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OC2PE_W<CCMR1_OUTPUTrs> {
        OC2PE_W::new(self, 11)
    }
    ///Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4
    #[inline(always)]
    pub fn oc2m(&mut self) -> OC2M_W<CCMR1_OUTPUTrs> {
        OC2M_W::new(self, 12)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    pub fn oc2ce(&mut self) -> OC2CE_W<CCMR1_OUTPUTrs> {
        OC2CE_W::new(self, 15)
    }
    ///Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
    #[inline(always)]
    pub fn oc1m_3(&mut self) -> OC1M_3_W<CCMR1_OUTPUTrs> {
        OC1M_3_W::new(self, 16)
    }
    ///Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4
    #[inline(always)]
    pub fn oc2m_3(&mut self) -> OC2M_3_W<CCMR1_OUTPUTrs> {
        OC2M_3_W::new(self, 24)
    }
}
/**TIM3 capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#TIM3:CCMR1_Output)*/
pub struct CCMR1_OUTPUTrs;
impl crate::RegisterSpec for CCMR1_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1_output::R`](R) reader structure
impl crate::Readable for CCMR1_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure
impl crate::Writable for CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR1_Output to value 0
impl crate::Resettable for CCMR1_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
