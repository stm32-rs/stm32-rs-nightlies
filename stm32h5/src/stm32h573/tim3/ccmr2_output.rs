#[doc = "Register `CCMR2_Output` reader"]
pub type R = crate::R<CCMR2_OUTPUTrs>;
#[doc = "Register `CCMR2_Output` writer"]
pub type W = crate::W<CCMR2_OUTPUTrs>;
#[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S {
    #[doc = "0: CC3 channel is configured as output"]
    Output = 0,
}
impl From<CC3S> for u8 {
    #[inline(always)]
    fn from(variant: CC3S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC3S {
    type Ux = u8;
}
#[doc = "Field `CC3S` reader - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
pub type CC3S_R = crate::FieldReader<CC3S>;
impl CC3S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC3S> {
        match self.bits {
            0 => Some(CC3S::Output),
            _ => None,
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC3S::Output
    }
}
#[doc = "Field `CC3S` writer - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC3S>;
impl<'a, REG> CC3S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Output)
    }
}
#[doc = "Field `OC3FE` reader - Output compare 3 fast enable"]
pub type OC3FE_R = crate::BitReader;
#[doc = "Field `OC3FE` writer - Output compare 3 fast enable"]
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output compare 3 preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC3PE {
    #[doc = "0: Preload register on CCR3 disabled. New values written to CCR3 are taken into account immediately"]
    Disabled = 0,
    #[doc = "1: Preload register on CCR3 enabled. Preload value is loaded into active register on each update event"]
    Enabled = 1,
}
impl From<OC3PE> for bool {
    #[inline(always)]
    fn from(variant: OC3PE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC3PE` reader - Output compare 3 preload enable"]
pub type OC3PE_R = crate::BitReader<OC3PE>;
impl OC3PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC3PE {
        match self.bits {
            false => OC3PE::Disabled,
            true => OC3PE::Enabled,
        }
    }
    #[doc = "Preload register on CCR3 disabled. New values written to CCR3 are taken into account immediately"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC3PE::Disabled
    }
    #[doc = "Preload register on CCR3 enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC3PE::Enabled
    }
}
#[doc = "Field `OC3PE` writer - Output compare 3 preload enable"]
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG, OC3PE>;
impl<'a, REG> OC3PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload register on CCR3 disabled. New values written to CCR3 are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC3PE::Disabled)
    }
    #[doc = "Preload register on CCR3 enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC3PE::Enabled)
    }
}
#[doc = "Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC3M {
    #[doc = "0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    Frozen = 0,
    #[doc = "1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    ActiveOnMatch = 1,
    #[doc = "2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    InactiveOnMatch = 2,
    #[doc = "3: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    Toggle = 3,
    #[doc = "4: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    ForceInactive = 4,
    #[doc = "5: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    ForceActive = 5,
    #[doc = "6: In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    PwmMode1 = 6,
    #[doc = "7: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1"]
    PwmMode2 = 7,
}
impl From<OC3M> for u8 {
    #[inline(always)]
    fn from(variant: OC3M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC3M {
    type Ux = u8;
}
#[doc = "Field `OC3M` reader - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub type OC3M_R = crate::FieldReader<OC3M>;
impl OC3M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC3M {
        match self.bits {
            0 => OC3M::Frozen,
            1 => OC3M::ActiveOnMatch,
            2 => OC3M::InactiveOnMatch,
            3 => OC3M::Toggle,
            4 => OC3M::ForceInactive,
            5 => OC3M::ForceActive,
            6 => OC3M::PwmMode1,
            7 => OC3M::PwmMode2,
            _ => unreachable!(),
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC3M::Frozen
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC3M::ActiveOnMatch
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC3M::InactiveOnMatch
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC3M::Toggle
    }
    #[doc = "OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC3M::ForceInactive
    }
    #[doc = "OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC3M::ForceActive
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC3M::PwmMode1
    }
    #[doc = "Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1"]
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC3M::PwmMode2
    }
}
#[doc = "Field `OC3M` writer - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub type OC3M_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OC3M>;
impl<'a, REG> OC3M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::Frozen)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ActiveOnMatch)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::InactiveOnMatch)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::Toggle)
    }
    #[doc = "OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ForceInactive)
    }
    #[doc = "OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ForceActive)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::PwmMode1)
    }
    #[doc = "Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::PwmMode2)
    }
}
#[doc = "Field `OC3CE` reader - Output compare 3 clear enable"]
pub type OC3CE_R = crate::BitReader;
#[doc = "Field `OC3CE` writer - Output compare 3 clear enable"]
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC4S {
    #[doc = "0: CC4 channel is configured as output"]
    Output = 0,
}
impl From<CC4S> for u8 {
    #[inline(always)]
    fn from(variant: CC4S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC4S {
    type Ux = u8;
}
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
pub type CC4S_R = crate::FieldReader<CC4S>;
impl CC4S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC4S> {
        match self.bits {
            0 => Some(CC4S::Output),
            _ => None,
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC4S::Output
    }
}
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC4S>;
impl<'a, REG> CC4S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::Output)
    }
}
#[doc = "Field `OC4FE` reader - Output compare 4 fast enable"]
pub type OC4FE_R = crate::BitReader;
#[doc = "Field `OC4FE` writer - Output compare 4 fast enable"]
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output compare 4 preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC4PE {
    #[doc = "0: Preload register on CCR4 disabled. New values written to CCR4 are taken into account immediately"]
    Disabled = 0,
    #[doc = "1: Preload register on CCR4 enabled. Preload value is loaded into active register on each update event"]
    Enabled = 1,
}
impl From<OC4PE> for bool {
    #[inline(always)]
    fn from(variant: OC4PE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC4PE` reader - Output compare 4 preload enable"]
pub type OC4PE_R = crate::BitReader<OC4PE>;
impl OC4PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC4PE {
        match self.bits {
            false => OC4PE::Disabled,
            true => OC4PE::Enabled,
        }
    }
    #[doc = "Preload register on CCR4 disabled. New values written to CCR4 are taken into account immediately"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC4PE::Disabled
    }
    #[doc = "Preload register on CCR4 enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC4PE::Enabled
    }
}
#[doc = "Field `OC4PE` writer - Output compare 4 preload enable"]
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG, OC4PE>;
impl<'a, REG> OC4PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload register on CCR4 disabled. New values written to CCR4 are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC4PE::Disabled)
    }
    #[doc = "Preload register on CCR4 enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC4PE::Enabled)
    }
}
#[doc = "Field `OC4M` reader - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub use OC3M_R as OC4M_R;
#[doc = "Field `OC4M` writer - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub use OC3M_W as OC4M_W;
#[doc = "Field `OC4CE` reader - Output compare 4 clear enable"]
pub type OC4CE_R = crate::BitReader;
#[doc = "Field `OC4CE` writer - Output compare 4 clear enable"]
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC3M_3 {
    #[doc = "0: Normal output compare mode (modes 0-7)"]
    Normal = 0,
    #[doc = "1: Extended output compare mode (modes 7-15)"]
    Extended = 1,
}
impl From<OC3M_3> for bool {
    #[inline(always)]
    fn from(variant: OC3M_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC3M_3` reader - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub type OC3M_3_R = crate::BitReader<OC3M_3>;
impl OC3M_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC3M_3 {
        match self.bits {
            false => OC3M_3::Normal,
            true => OC3M_3::Extended,
        }
    }
    #[doc = "Normal output compare mode (modes 0-7)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC3M_3::Normal
    }
    #[doc = "Extended output compare mode (modes 7-15)"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC3M_3::Extended
    }
}
#[doc = "Field `OC3M_3` writer - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub type OC3M_3_W<'a, REG> = crate::BitWriter<'a, REG, OC3M_3>;
impl<'a, REG> OC3M_3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal output compare mode (modes 0-7)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_3::Normal)
    }
    #[doc = "Extended output compare mode (modes 7-15)"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_3::Extended)
    }
}
#[doc = "Field `OC4M_3` reader - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub use OC3M_3_R as OC4M_3_R;
#[doc = "Field `OC4M_3` writer - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
pub use OC3M_3_W as OC4M_3_W;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    pub fn oc3m_3(&self) -> OC3M_3_R {
        OC3M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    pub fn oc4m_3(&self) -> OC4M_3_R {
        OC4M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR2_OUTPUTrs> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OC3FE_W<CCMR2_OUTPUTrs> {
        OC3FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OC3PE_W<CCMR2_OUTPUTrs> {
        OC3PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    #[must_use]
    pub fn oc3m(&mut self) -> OC3M_W<CCMR2_OUTPUTrs> {
        OC3M_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OC3CE_W<CCMR2_OUTPUTrs> {
        OC3CE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR2_OUTPUTrs> {
        CC4S_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OC4FE_W<CCMR2_OUTPUTrs> {
        OC4FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OC4PE_W<CCMR2_OUTPUTrs> {
        OC4PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    #[must_use]
    pub fn oc4m(&mut self) -> OC4M_W<CCMR2_OUTPUTrs> {
        OC4M_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OC4CE_W<CCMR2_OUTPUTrs> {
        OC4CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    #[must_use]
    pub fn oc3m_3(&mut self) -> OC3M_3_W<CCMR2_OUTPUTrs> {
        OC3M_3_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC1M description (bits 6:4 in TIMx_CCMR1 register)"]
    #[inline(always)]
    #[must_use]
    pub fn oc4m_3(&mut self) -> OC4M_3_W<CCMR2_OUTPUTrs> {
        OC4M_3_W::new(self, 24)
    }
}
#[doc = "TIM3 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR2_OUTPUTrs;
impl crate::RegisterSpec for CCMR2_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for CCMR2_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for CCMR2_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for CCMR2_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
