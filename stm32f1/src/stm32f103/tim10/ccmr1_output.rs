#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<CCMR1_OUTPUTrs>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<CCMR1_OUTPUTrs>;
#[doc = "Field `CCS(1-1)` reader - Capture/Compare %s selection"]
pub type CCS_R = crate::FieldReader;
#[doc = "Field `CCS(1-1)` writer - Capture/Compare %s selection"]
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OCPE(1-1)` reader - Output compare %s preload enable"]
pub type OCPE_R = crate::BitReader;
#[doc = "Field `OCPE(1-1)` writer - Output compare %s preload enable"]
pub type OCPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFE(1-1)` reader - Output compare %s fast enable"]
pub type OCFE_R = crate::BitReader;
#[doc = "Field `OCFE(1-1)` writer - Output compare %s fast enable"]
pub type OCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output compare %s mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC1M {
    #[doc = "0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    Frozen = 0,
    #[doc = "1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    ActiveOnMatch = 1,
    #[doc = "2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    InactiveOnMatch = 2,
    #[doc = "3: OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    Toggle = 3,
    #[doc = "4: OCyREF is forced low"]
    ForceInactive = 4,
    #[doc = "5: OCyREF is forced high"]
    ForceActive = 5,
    #[doc = "6: In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    PwmMode1 = 6,
    #[doc = "7: Inversely to PwmMode1"]
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
#[doc = "Field `OCM(1-1)` reader - Output compare %s mode"]
pub type OCM_R = crate::FieldReader<OC1M>;
impl OCM_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC1M::Frozen
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC1M::ActiveOnMatch
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC1M::InactiveOnMatch
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC1M::Toggle
    }
    #[doc = "OCyREF is forced low"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC1M::ForceInactive
    }
    #[doc = "OCyREF is forced high"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC1M::ForceActive
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC1M::PwmMode1
    }
    #[doc = "Inversely to PwmMode1"]
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC1M::PwmMode2
    }
}
#[doc = "Field `OCM(1-1)` writer - Output compare %s mode"]
pub type OCM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OC1M>;
impl<'a, REG> OCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::Frozen)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ActiveOnMatch)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::InactiveOnMatch)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::Toggle)
    }
    #[doc = "OCyREF is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ForceInactive)
    }
    #[doc = "OCyREF is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ForceActive)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::PwmMode1)
    }
    #[doc = "Inversely to PwmMode1"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::PwmMode2)
    }
}
impl R {
    #[doc = "Capture/Compare (1-1) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_R::new(((self.bits >> (n * 0)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) selection"]
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..1).map(move |n| CCS_R::new(((self.bits >> (n * 0)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Output compare (1-1) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1PE` field"]
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCPE_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (1-1) preload enable"]
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..1).map(move |n| OCPE_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Output compare (1-1) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1FE` field"]
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCFE_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (1-1) fast enable"]
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..1).map(move |n| OCFE_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Output compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Output compare (1-1) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1M` field"]
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (1-1) mode"]
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..1).map(move |n| OCM_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Capture/Compare (1-1) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_W::new(self, n * 0)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_OUTPUTrs> {
        CCS_W::new(self, 0)
    }
    #[doc = "Output compare (1-1) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1PE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCPE_W::new(self, n * 0 + 3)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OCPE_W<CCMR1_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    #[doc = "Output compare (1-1) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1FE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCFE_W::new(self, n * 0 + 3)
    }
    #[doc = "Bit 3 - Output compare 1 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OCFE_W<CCMR1_OUTPUTrs> {
        OCFE_W::new(self, 3)
    }
    #[doc = "Output compare (1-1) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC1M` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_W::new(self, n * 0 + 4)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OCM_W<CCMR1_OUTPUTrs> {
        OCM_W::new(self, 4)
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
