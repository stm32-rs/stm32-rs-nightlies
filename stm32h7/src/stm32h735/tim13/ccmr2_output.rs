#[doc = "Register `CCMR2_Output` reader"]
pub type R = crate::R<CCMR2_OUTPUTrs>;
#[doc = "Register `CCMR2_Output` writer"]
pub type W = crate::W<CCMR2_OUTPUTrs>;
#[doc = "Field `CCS(3-4)` reader - Capture/Compare %s selection"]
pub type CCS_R = crate::FieldReader;
#[doc = "Field `CCS(3-4)` writer - Capture/Compare %s selection"]
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OCFE(3-4)` reader - Output compare %s fast enable"]
pub type OCFE_R = crate::BitReader;
#[doc = "Field `OCFE(3-4)` writer - Output compare %s fast enable"]
pub type OCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPE(3-4)` reader - Output compare %s preload enable"]
pub type OCPE_R = crate::BitReader;
#[doc = "Field `OCPE(3-4)` writer - Output compare %s preload enable"]
pub type OCPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output compare %s mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC3M {
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
impl From<OC3M> for u8 {
    #[inline(always)]
    fn from(variant: OC3M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC3M {
    type Ux = u8;
}
#[doc = "Field `OCM(3-4)` reader - Output compare %s mode"]
pub type OCM_R = crate::FieldReader<OC3M>;
impl OCM_R {
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
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC3M::Frozen
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC3M::ActiveOnMatch
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC3M::InactiveOnMatch
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC3M::Toggle
    }
    #[doc = "OCyREF is forced low"]
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC3M::ForceInactive
    }
    #[doc = "OCyREF is forced high"]
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC3M::ForceActive
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC3M::PwmMode1
    }
    #[doc = "Inversely to PwmMode1"]
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC3M::PwmMode2
    }
}
#[doc = "Field `OCM(3-4)` writer - Output compare %s mode"]
pub type OCM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OC3M>;
impl<'a, REG> OCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::Frozen)
    }
    #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ActiveOnMatch)
    }
    #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::InactiveOnMatch)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::Toggle)
    }
    #[doc = "OCyREF is forced low"]
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ForceInactive)
    }
    #[doc = "OCyREF is forced high"]
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ForceActive)
    }
    #[doc = "In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::PwmMode1)
    }
    #[doc = "Inversely to PwmMode1"]
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::PwmMode2)
    }
}
#[doc = "Field `OCCE(3-4)` reader - Output compare %s clear enable"]
pub type OCCE_R = crate::BitReader;
#[doc = "Field `OCCE(3-4)` writer - Output compare %s clear enable"]
pub type OCCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCM_3(3-4)` reader - Output compare %s mode, bit 3"]
pub type OCM_3_R = crate::BitReader;
#[doc = "Field `OCM_3(3-4)` writer - Output compare %s mode, bit 3"]
pub type OCM_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Capture/Compare (3-4) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC3S` field"]
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_R::new(((self.bits >> (n * 8)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (3-4) selection"]
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..2).map(move |n| CCS_R::new(((self.bits >> (n * 8)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Output compare (3-4) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3FE` field"]
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (3-4) fast enable"]
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..2).map(move |n| OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Output compare (3-4) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3PE` field"]
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (3-4) preload enable"]
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..2).map(move |n| OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Output compare (3-4) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3M` field"]
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (3-4) mode"]
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..2).map(move |n| OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Output compare (3-4) clear enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3CE` field"]
    #[inline(always)]
    pub fn occe(&self, n: u8) -> OCCE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (3-4) clear enable"]
    #[inline(always)]
    pub fn occe_iter(&self) -> impl Iterator<Item = OCCE_R> + '_ {
        (0..2).map(move |n| OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Output compare (3-4) mode, bit 3"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3M_3` field"]
    #[inline(always)]
    pub fn ocm_3(&self, n: u8) -> OCM_3_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Output compare (3-4) mode, bit 3"]
    #[inline(always)]
    pub fn ocm_3_iter(&self) -> impl Iterator<Item = OCM_3_R> + '_ {
        (0..2).map(move |n| OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - Output compare 3 mode, bit 3"]
    #[inline(always)]
    pub fn oc3m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 4 mode, bit 3"]
    #[inline(always)]
    pub fn oc4m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Capture/Compare (3-4) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC3S` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_W::new(self, n * 8)
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CCS_W<CCMR2_OUTPUTrs> {
        CCS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CCS_W<CCMR2_OUTPUTrs> {
        CCS_W::new(self, 8)
    }
    #[doc = "Output compare (3-4) fast enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3FE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_W::new(self, n * 8 + 2)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OCFE_W<CCMR2_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OCFE_W<CCMR2_OUTPUTrs> {
        OCFE_W::new(self, 10)
    }
    #[doc = "Output compare (3-4) preload enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3PE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_W::new(self, n * 8 + 3)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OCPE_W<CCMR2_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OCPE_W<CCMR2_OUTPUTrs> {
        OCPE_W::new(self, 11)
    }
    #[doc = "Output compare (3-4) mode"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3M` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc3m(&mut self) -> OCM_W<CCMR2_OUTPUTrs> {
        OCM_W::new(self, 4)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc4m(&mut self) -> OCM_W<CCMR2_OUTPUTrs> {
        OCM_W::new(self, 12)
    }
    #[doc = "Output compare (3-4) clear enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3CE` field"]
    #[inline(always)]
    #[must_use]
    pub fn occe(&mut self, n: u8) -> OCCE_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_W::new(self, n * 8 + 7)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OCCE_W<CCMR2_OUTPUTrs> {
        OCCE_W::new(self, 7)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OCCE_W<CCMR2_OUTPUTrs> {
        OCCE_W::new(self, 15)
    }
    #[doc = "Output compare (3-4) mode, bit 3"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OC3M_3` field"]
    #[inline(always)]
    #[must_use]
    pub fn ocm_3(&mut self, n: u8) -> OCM_3_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_W::new(self, n * 8 + 16)
    }
    #[doc = "Bit 16 - Output compare 3 mode, bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc3m_3(&mut self) -> OCM_3_W<CCMR2_OUTPUTrs> {
        OCM_3_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 4 mode, bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc4m_3(&mut self) -> OCM_3_W<CCMR2_OUTPUTrs> {
        OCM_3_W::new(self, 24)
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
