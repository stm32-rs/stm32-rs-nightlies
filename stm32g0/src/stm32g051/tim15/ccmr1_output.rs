#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<CCMR1_OUTPUTrs>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<CCMR1_OUTPUTrs>;
#[doc = "Field `CCS(1-2)` reader - Capture/Compare %s selection"]
pub type CCS_R = crate::FieldReader;
#[doc = "Field `CCS(1-2)` writer - Capture/Compare %s selection"]
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OCFE(1-2)` reader - Output compare %s fast enable"]
pub type OCFE_R = crate::BitReader;
#[doc = "Field `OCFE(1-2)` writer - Output compare %s fast enable"]
pub type OCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPE(1-2)` reader - Output compare %s preload enable"]
pub type OCPE_R = crate::BitReader;
#[doc = "Field `OCPE(1-2)` writer - Output compare %s preload enable"]
pub type OCPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M1` reader - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type OC1M1_R = crate::FieldReader;
#[doc = "Field `OC1M1` writer - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type OC1M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Output compare %s mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC2M {
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
    #[doc = "6: In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / Reserved"]
    PwmMode1 = 6,
    #[doc = "7: Inversely to PwmMode1 / Reserved"]
    PwmMode2 = 7,
}
impl From<OC2M> for u8 {
    #[inline(always)]
    fn from(variant: OC2M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC2M {
    type Ux = u8;
}
#[doc = "Field `OCM(2-2)` reader - Output compare %s mode"]
pub type OCM_R = crate::FieldReader<OC2M>;
impl OCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC2M {
        match self.bits {
            0 => OC2M::Frozen,
            1 => OC2M::ActiveOnMatch,
            2 => OC2M::InactiveOnMatch,
            3 => OC2M::Toggle,
            4 => OC2M::ForceInactive,
            5 => OC2M::ForceActive,
            6 => OC2M::PwmMode1,
            7 => OC2M::PwmMode2,
            _ => unreachable!(),
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC2M::Frozen
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC2M::ActiveOnMatch
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC2M::InactiveOnMatch
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC2M::Toggle
    }
    #[doc = "OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC2M::ForceInactive
    }
    #[doc = "OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC2M::ForceActive
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / Reserved"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC2M::PwmMode1
    }
    #[doc = "Inversely to PwmMode1 / Reserved"]
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC2M::PwmMode2
    }
}
#[doc = "Field `OCM(2-2)` writer - Output compare %s mode"]
pub type OCM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OC2M>;
impl<'a, REG> OCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::Frozen)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::ActiveOnMatch)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::InactiveOnMatch)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::Toggle)
    }
    #[doc = "OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::ForceInactive)
    }
    #[doc = "OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::ForceActive)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / Reserved"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::PwmMode1)
    }
    #[doc = "Inversely to PwmMode1 / Reserved"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M::PwmMode2)
    }
}
#[doc = "Field `OC1M2` reader - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type OC1M2_R = crate::BitReader;
#[doc = "Field `OC1M2` writer - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
pub type OC1M2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output compare %s mode, bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC2M_3 {
    #[doc = "0: Normal output compare mode (modes 0-7)"]
    Normal = 0,
    #[doc = "1: Extended output compare mode (modes 7-15)"]
    Extended = 1,
}
impl From<OC2M_3> for bool {
    #[inline(always)]
    fn from(variant: OC2M_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCM_3(2-2)` reader - Output compare %s mode, bit 3"]
pub type OCM_3_R = crate::BitReader<OC2M_3>;
impl OCM_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC2M_3 {
        match self.bits {
            false => OC2M_3::Normal,
            true => OC2M_3::Extended,
        }
    }
    #[doc = "Normal output compare mode (modes 0-7)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC2M_3::Normal
    }
    #[doc = "Extended output compare mode (modes 7-15)"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC2M_3::Extended
    }
}
#[doc = "Field `OCM_3(2-2)` writer - Output compare %s mode, bit 3"]
pub type OCM_3_W<'a, REG> = crate::BitWriter<'a, REG, OC2M_3>;
impl<'a, REG> OCM_3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal output compare mode (modes 0-7)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M_3::Normal)
    }
    #[doc = "Extended output compare mode (modes 7-15)"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(OC2M_3::Extended)
    }
}
impl R {
    #[doc = "Capture/Compare (1-2) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_R::new(((self.bits >> (n * 8)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-2) selection"]
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..2).map(move |n| CCS_R::new(((self.bits >> (n * 8)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Output compare (1-2) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1FE` field"]
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (1-2) fast enable"]
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..2).map(move |n| OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Output compare (1-2) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1PE` field"]
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (1-2) preload enable"]
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..2).map(move |n| OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Output compare (2-2) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC2M` field"]
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_R::new(((self.bits >> (n * 0 + 12)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (2-2) mode"]
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..1).map(move |n| OCM_R::new(((self.bits >> (n * 0 + 12)) & 7) as u8))
    }
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    pub fn oc1m2(&self) -> OC1M2_R {
        OC1M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Output compare (2-2) mode, bit 3"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC2M_3` field"]
    #[inline(always)]
    pub fn ocm_3(&self, n: u8) -> OCM_3_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_3_R::new(((self.bits >> (n * 0 + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (2-2) mode, bit 3"]
    #[inline(always)]
    pub fn ocm_3_iter(&self) -> impl Iterator<Item = OCM_3_R> + '_ {
        (0..1).map(move |n| OCM_3_R::new(((self.bits >> (n * 0 + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - Output compare 2 mode, bit 3"]
    #[inline(always)]
    pub fn oc2m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Capture/Compare (1-2) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_W::new(self, n * 8)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_OUTPUTrs> {
        CCS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CCS_W<CCMR1_OUTPUTrs> {
        CCS_W::new(self, 8)
    }
    #[doc = "Output compare (1-2) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1FE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_W::new(self, n * 8 + 2)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OCFE_W<CCMR1_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OCFE_W<CCMR1_OUTPUTrs> {
        OCFE_W::new(self, 10)
    }
    #[doc = "Output compare (1-2) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1PE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_W::new(self, n * 8 + 3)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OCPE_W<CCMR1_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OCPE_W<CCMR1_OUTPUTrs> {
        OCPE_W::new(self, 11)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m1(&mut self) -> OC1M1_W<CCMR1_OUTPUTrs> {
        OC1M1_W::new(self, 4)
    }
    #[doc = "Output compare (2-2) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC2M` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_W::new(self, n * 0 + 12)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OCM_W<CCMR1_OUTPUTrs> {
        OCM_W::new(self, 12)
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=â\u{80}\u{99}00â\u{80}\u{99} (the channel is configured in output). In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from â\u{80}\u{9c}frozenâ\u{80}\u{9d} mode to â\u{80}\u{9c}PWMâ\u{80}\u{9d} mode. On channels that have a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. The OC1M\\[3\\]
bit is not contiguous, located in bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m2(&mut self) -> OC1M2_W<CCMR1_OUTPUTrs> {
        OC1M2_W::new(self, 16)
    }
    #[doc = "Output compare (2-2) mode, bit 3"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC2M_3` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocm_3(&mut self, n: u8) -> OCM_3_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_3_W::new(self, n * 0 + 24)
    }
    #[doc = "Bit 24 - Output compare 2 mode, bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m_3(&mut self) -> OCM_3_W<CCMR1_OUTPUTrs> {
        OCM_3_W::new(self, 24)
    }
}
#[doc = "capture/compare mode register (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_OUTPUTrs;
impl crate::RegisterSpec for CCMR1_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for CCMR1_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for CCMR1_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
