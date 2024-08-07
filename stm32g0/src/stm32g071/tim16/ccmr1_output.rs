///Register `CCMR1_Output` reader
pub type R = crate::R<CCMR1_OUTPUTrs>;
///Register `CCMR1_Output` writer
pub type W = crate::W<CCMR1_OUTPUTrs>;
///Field `CCS(1-1)` reader - Capture/Compare %s selection
pub type CCS_R = crate::FieldReader;
///Field `CCS(1-1)` writer - Capture/Compare %s selection
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OCFE(1-1)` reader - Output compare %s fast enable
pub type OCFE_R = crate::BitReader;
///Field `OCFE(1-1)` writer - Output compare %s fast enable
pub type OCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCPE(1-1)` reader - Output compare %s preload enable
pub type OCPE_R = crate::BitReader;
///Field `OCPE(1-1)` writer - Output compare %s preload enable
pub type OCPE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output compare %s mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC1M {
    ///0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    Frozen = 0,
    ///1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    ActiveOnMatch = 1,
    ///2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    InactiveOnMatch = 2,
    ///3: OCyREF toggles when TIMx_CNT=TIMx_CCRy
    Toggle = 3,
    ///4: OCyREF is forced low
    ForceInactive = 4,
    ///5: OCyREF is forced high
    ForceActive = 5,
    ///6: In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    PwmMode1 = 6,
    ///7: Inversely to PwmMode1
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
///Field `OCM(1-1)` reader - Output compare %s mode
pub type OCM_R = crate::FieldReader<OC1M>;
impl OCM_R {
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
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC1M::Frozen
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC1M::ActiveOnMatch
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC1M::InactiveOnMatch
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC1M::Toggle
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC1M::ForceInactive
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC1M::ForceActive
    }
    ///In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC1M::PwmMode1
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC1M::PwmMode2
    }
}
///Field `OCM(1-1)` writer - Output compare %s mode
pub type OCM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OC1M, crate::Safe>;
impl<'a, REG> OCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::Toggle)
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ForceInactive)
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT&lt;TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::PwmMode1)
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::PwmMode2)
    }
}
///Field `OC1M_2` reader - Output Compare 1 mode
pub type OC1M_2_R = crate::BitReader;
///Field `OC1M_2` writer - Output Compare 1 mode
pub type OC1M_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Capture/Compare (1-1) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_R::new(((self.bits >> (n * 0)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) selection
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..1).map(move |n| CCS_R::new(((self.bits >> (n * 0)) & 3) as u8))
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    ///Output compare (1-1) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCFE_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (1-1) fast enable
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..1).map(move |n| OCFE_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0))
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Output compare (1-1) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCPE_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (1-1) preload enable
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..1).map(move |n| OCPE_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    ///Bit 3 - Output compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Output compare (1-1) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1M` field.</div>
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Output compare (1-1) mode
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..1).map(move |n| OCM_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8))
    }
    ///Bits 4:6 - Output compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 16 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m_2(&self) -> OC1M_2_R {
        OC1M_2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1_Output")
            .field("oc1m_2", &self.oc1m_2())
            .field("oc1m", &self.oc1m())
            .field("oc1pe", &self.oc1pe())
            .field("oc1fe", &self.oc1fe())
            .field("cc1s", &self.cc1s())
            .finish()
    }
}
impl W {
    ///Capture/Compare (1-1) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_W::new(self, n * 0)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_OUTPUTrs> {
        CCS_W::new(self, 0)
    }
    ///Output compare (1-1) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1FE` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCFE_W::new(self, n * 0 + 2)
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OCFE_W<CCMR1_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    ///Output compare (1-1) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1PE` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCPE_W::new(self, n * 0 + 3)
    }
    ///Bit 3 - Output compare 1 preload enable
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OCPE_W<CCMR1_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    ///Output compare (1-1) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1M` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_W::new(self, n * 0 + 4)
    }
    ///Bits 4:6 - Output compare 1 mode
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OCM_W<CCMR1_OUTPUTrs> {
        OCM_W::new(self, 4)
    }
    ///Bit 16 - Output Compare 1 mode
    #[inline(always)]
    #[must_use]
    pub fn oc1m_2(&mut self) -> OC1M_2_W<CCMR1_OUTPUTrs> {
        OC1M_2_W::new(self, 16)
    }
}
/**capture/compare mode register (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#TIM16:CCMR1_Output)*/
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
