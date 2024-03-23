#[doc = "Register `CCMR3_Output` reader"]
pub type R = crate::R<CCMR3_OUTPUTrs>;
#[doc = "Register `CCMR3_Output` writer"]
pub type W = crate::W<CCMR3_OUTPUTrs>;
#[doc = "Output compare %s fast enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5FE {
    #[doc = "0: Fast output disabled"]
    Disabled = 0,
    #[doc = "1: Fast output enabled"]
    Enabled = 1,
}
impl From<OC5FE> for bool {
    #[inline(always)]
    fn from(variant: OC5FE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCFE(5-6)` reader - Output compare %s fast enable"]
pub type OCFE_R = crate::BitReader<OC5FE>;
impl OCFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC5FE {
        match self.bits {
            false => OC5FE::Disabled,
            true => OC5FE::Enabled,
        }
    }
    #[doc = "Fast output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC5FE::Disabled
    }
    #[doc = "Fast output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC5FE::Enabled
    }
}
#[doc = "Field `OCFE(5-6)` writer - Output compare %s fast enable"]
pub type OCFE_W<'a, REG> = crate::BitWriter<'a, REG, OC5FE>;
impl<'a, REG> OCFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC5FE::Disabled)
    }
    #[doc = "Fast output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC5FE::Enabled)
    }
}
#[doc = "Output compare %s preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5PE {
    #[doc = "0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately"]
    Disabled = 0,
    #[doc = "1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event"]
    Enabled = 1,
}
impl From<OC5PE> for bool {
    #[inline(always)]
    fn from(variant: OC5PE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCPE(5-6)` reader - Output compare %s preload enable"]
pub type OCPE_R = crate::BitReader<OC5PE>;
impl OCPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC5PE {
        match self.bits {
            false => OC5PE::Disabled,
            true => OC5PE::Enabled,
        }
    }
    #[doc = "Preload register on CCRx disabled. New values written to CCRx are taken into account immediately"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC5PE::Disabled
    }
    #[doc = "Preload register on CCRx enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC5PE::Enabled
    }
}
#[doc = "Field `OCPE(5-6)` writer - Output compare %s preload enable"]
pub type OCPE_W<'a, REG> = crate::BitWriter<'a, REG, OC5PE>;
impl<'a, REG> OCPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload register on CCRx disabled. New values written to CCRx are taken into account immediately"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC5PE::Disabled)
    }
    #[doc = "Preload register on CCRx enabled. Preload value is loaded into active register on each update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC5PE::Enabled)
    }
}
#[doc = "Output compare %s mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC5M {
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
impl From<OC5M> for u8 {
    #[inline(always)]
    fn from(variant: OC5M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC5M {
    type Ux = u8;
}
#[doc = "Field `OCM(5-6)` reader - Output compare %s mode"]
pub type OCM_R = crate::FieldReader<OC5M>;
impl OCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC5M {
        match self.bits {
            0 => OC5M::Frozen,
            1 => OC5M::ActiveOnMatch,
            2 => OC5M::InactiveOnMatch,
            3 => OC5M::Toggle,
            4 => OC5M::ForceInactive,
            5 => OC5M::ForceActive,
            6 => OC5M::PwmMode1,
            7 => OC5M::PwmMode2,
            _ => unreachable!(),
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC5M::Frozen
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC5M::ActiveOnMatch
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC5M::InactiveOnMatch
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC5M::Toggle
    }
    #[doc = "OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC5M::ForceInactive
    }
    #[doc = "OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC5M::ForceActive
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC5M::PwmMode1
    }
    #[doc = "Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1"]
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC5M::PwmMode2
    }
}
#[doc = "Field `OCM(5-6)` writer - Output compare %s mode"]
pub type OCM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OC5M>;
impl<'a, REG> OCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::Frozen)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::ActiveOnMatch)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::InactiveOnMatch)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::Toggle)
    }
    #[doc = "OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::ForceInactive)
    }
    #[doc = "OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::ForceActive)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::PwmMode1)
    }
    #[doc = "Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::PwmMode2)
    }
}
#[doc = "Output compare %s clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5CE {
    #[doc = "0: OCxRef is not affected by the ocref_clr_int signal"]
    Disabled = 0,
    #[doc = "1: OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal"]
    Enabled = 1,
}
impl From<OC5CE> for bool {
    #[inline(always)]
    fn from(variant: OC5CE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCCE(5-6)` reader - Output compare %s clear enable"]
pub type OCCE_R = crate::BitReader<OC5CE>;
impl OCCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC5CE {
        match self.bits {
            false => OC5CE::Disabled,
            true => OC5CE::Enabled,
        }
    }
    #[doc = "OCxRef is not affected by the ocref_clr_int signal"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC5CE::Disabled
    }
    #[doc = "OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC5CE::Enabled
    }
}
#[doc = "Field `OCCE(5-6)` writer - Output compare %s clear enable"]
pub type OCCE_W<'a, REG> = crate::BitWriter<'a, REG, OC5CE>;
impl<'a, REG> OCCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCxRef is not affected by the ocref_clr_int signal"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC5CE::Disabled)
    }
    #[doc = "OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC5CE::Enabled)
    }
}
#[doc = "Output compare %s mode, bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5M_3 {
    #[doc = "0: Normal output compare mode (modes 0-7)"]
    Normal = 0,
    #[doc = "1: Extended output compare mode (modes 7-15)"]
    Extended = 1,
}
impl From<OC5M_3> for bool {
    #[inline(always)]
    fn from(variant: OC5M_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCM_3(5-6)` reader - Output compare %s mode, bit 3"]
pub type OCM_3_R = crate::BitReader<OC5M_3>;
impl OCM_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC5M_3 {
        match self.bits {
            false => OC5M_3::Normal,
            true => OC5M_3::Extended,
        }
    }
    #[doc = "Normal output compare mode (modes 0-7)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC5M_3::Normal
    }
    #[doc = "Extended output compare mode (modes 7-15)"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC5M_3::Extended
    }
}
#[doc = "Field `OCM_3(5-6)` writer - Output compare %s mode, bit 3"]
pub type OCM_3_W<'a, REG> = crate::BitWriter<'a, REG, OC5M_3>;
impl<'a, REG> OCM_3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal output compare mode (modes 0-7)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M_3::Normal)
    }
    #[doc = "Extended output compare mode (modes 7-15)"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M_3::Extended)
    }
}
impl R {
    #[doc = "Output compare (5-6) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5FE` field"]
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (5-6) fast enable"]
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..2).map(move |n| OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Output compare (5-6) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5PE` field"]
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (5-6) preload enable"]
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..2).map(move |n| OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Output compare (5-6) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5M` field"]
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (5-6) mode"]
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..2).map(move |n| OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Output compare (5-6) clear enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5CE` field"]
    #[inline(always)]
    pub fn occe(&self, n: u8) -> OCCE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (5-6) clear enable"]
    #[inline(always)]
    pub fn occe_iter(&self) -> impl Iterator<Item = OCCE_R> + '_ {
        (0..2).map(move |n| OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Output compare (5-6) mode, bit 3"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5M_3` field"]
    #[inline(always)]
    pub fn ocm_3(&self, n: u8) -> OCM_3_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (5-6) mode, bit 3"]
    #[inline(always)]
    pub fn ocm_3_iter(&self) -> impl Iterator<Item = OCM_3_R> + '_ {
        (0..2).map(move |n| OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - Output compare 5 mode, bit 3"]
    #[inline(always)]
    pub fn oc5m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 6 mode, bit 3"]
    #[inline(always)]
    pub fn oc6m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Output compare (5-6) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5FE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_W::new(self, n * 8 + 2)
    }
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> OCFE_W<CCMR3_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> OCFE_W<CCMR3_OUTPUTrs> {
        OCFE_W::new(self, 10)
    }
    #[doc = "Output compare (5-6) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5PE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_W::new(self, n * 8 + 3)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> OCPE_W<CCMR3_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> OCPE_W<CCMR3_OUTPUTrs> {
        OCPE_W::new(self, 11)
    }
    #[doc = "Output compare (5-6) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5M` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m(&mut self) -> OCM_W<CCMR3_OUTPUTrs> {
        OCM_W::new(self, 4)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m(&mut self) -> OCM_W<CCMR3_OUTPUTrs> {
        OCM_W::new(self, 12)
    }
    #[doc = "Output compare (5-6) clear enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5CE` field"]
    #[inline(always)]
    #[must_use]
    pub fn occe(&mut self, n: u8) -> OCCE_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_W::new(self, n * 8 + 7)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> OCCE_W<CCMR3_OUTPUTrs> {
        OCCE_W::new(self, 7)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> OCCE_W<CCMR3_OUTPUTrs> {
        OCCE_W::new(self, 15)
    }
    #[doc = "Output compare (5-6) mode, bit 3"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC5M_3` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocm_3(&mut self, n: u8) -> OCM_3_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_W::new(self, n * 8 + 16)
    }
    #[doc = "Bit 16 - Output compare 5 mode, bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m_3(&mut self) -> OCM_3_W<CCMR3_OUTPUTrs> {
        OCM_3_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 6 mode, bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m_3(&mut self) -> OCM_3_W<CCMR3_OUTPUTrs> {
        OCM_3_W::new(self, 24)
    }
}
#[doc = "capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr3_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr3_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR3_OUTPUTrs;
impl crate::RegisterSpec for CCMR3_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3_output::R`](R) reader structure"]
impl crate::Readable for CCMR3_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr3_output::W`](W) writer structure"]
impl crate::Writable for CCMR3_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR3_Output to value 0"]
impl crate::Resettable for CCMR3_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
