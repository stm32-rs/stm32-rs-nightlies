///Register `CCMR1_Output` reader
pub type R = crate::R<CCMR1_OUTPUTrs>;
///Register `CCMR1_Output` writer
pub type W = crate::W<CCMR1_OUTPUTrs>;
/**Capture/Compare %s selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S {
    ///0: CCx channel is configured as output
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
///Field `CCS(1-1)` reader - Capture/Compare %s selection
pub type CCS_R = crate::FieldReader<CC1S>;
impl CCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC1S> {
        match self.bits {
            0 => Some(CC1S::Output),
            _ => None,
        }
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC1S::Output
    }
}
///Field `CCS(1-1)` writer - Capture/Compare %s selection
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1S>;
impl<'a, REG> CCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Output)
    }
}
/**Output compare %s fast enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1FE {
    ///0: Fast output disabled
    Disabled = 0,
    ///1: Fast output enabled
    Enabled = 1,
}
impl From<OC1FE> for bool {
    #[inline(always)]
    fn from(variant: OC1FE) -> Self {
        variant as u8 != 0
    }
}
///Field `OCFE(1-1)` reader - Output compare %s fast enable
pub type OCFE_R = crate::BitReader<OC1FE>;
impl OCFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1FE {
        match self.bits {
            false => OC1FE::Disabled,
            true => OC1FE::Enabled,
        }
    }
    ///Fast output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC1FE::Disabled
    }
    ///Fast output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC1FE::Enabled
    }
}
///Field `OCFE(1-1)` writer - Output compare %s fast enable
pub type OCFE_W<'a, REG> = crate::BitWriter<'a, REG, OC1FE>;
impl<'a, REG> OCFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fast output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC1FE::Disabled)
    }
    ///Fast output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC1FE::Enabled)
    }
}
/**Output compare %s preload enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1PE {
    ///0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    Disabled = 0,
    ///1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    Enabled = 1,
}
impl From<OC1PE> for bool {
    #[inline(always)]
    fn from(variant: OC1PE) -> Self {
        variant as u8 != 0
    }
}
///Field `OCPE(1-1)` reader - Output compare %s preload enable
pub type OCPE_R = crate::BitReader<OC1PE>;
impl OCPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1PE {
        match self.bits {
            false => OC1PE::Disabled,
            true => OC1PE::Enabled,
        }
    }
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OC1PE::Disabled
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OC1PE::Enabled
    }
}
///Field `OCPE(1-1)` writer - Output compare %s preload enable
pub type OCPE_W<'a, REG> = crate::BitWriter<'a, REG, OC1PE>;
impl<'a, REG> OCPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE::Disabled)
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE::Enabled)
    }
}
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
    ///6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
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
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
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
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
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
///Field `OCCE(1-1)` reader - Output compare %s clear enable
pub type OCCE_R = crate::BitReader;
///Field `OCCE(1-1)` writer - Output compare %s clear enable
pub type OCCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M_1` reader - OC1M\[3\]
pub type OC1M_1_R = crate::BitReader;
///Field `OC1M_1` writer - OC1M\[3\]
pub type OC1M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Output compare (1-1) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1CE` field.</div>
    #[inline(always)]
    pub fn occe(&self, n: u8) -> OCCE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCCE_R::new(((self.bits >> (n * 0 + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (1-1) clear enable
    #[inline(always)]
    pub fn occe_iter(&self) -> impl Iterator<Item = OCCE_R> + '_ {
        (0..1).map(move |n| OCCE_R::new(((self.bits >> (n * 0 + 7)) & 1) != 0))
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&self) -> OC1M_1_R {
        OC1M_1_R::new(((self.bits >> 16) & 1) != 0)
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
            .field("oc1m_1", &self.oc1m_1())
            .finish()
    }
}
impl W {
    ///Capture/Compare (1-1) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>
    #[inline(always)]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_W::new(self, n * 0)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_OUTPUTrs> {
        CCS_W::new(self, 0)
    }
    ///Output compare (1-1) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCFE_W::new(self, n * 0 + 2)
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OCFE_W<CCMR1_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    ///Output compare (1-1) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCPE_W::new(self, n * 0 + 3)
    }
    ///Bit 3 - Output compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OCPE_W<CCMR1_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    ///Output compare (1-1) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1M` field.</div>
    #[inline(always)]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCM_W::new(self, n * 0 + 4)
    }
    ///Bits 4:6 - Output compare 1 mode
    #[inline(always)]
    pub fn oc1m(&mut self) -> OCM_W<CCMR1_OUTPUTrs> {
        OCM_W::new(self, 4)
    }
    ///Output compare (1-1) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1CE` field.</div>
    #[inline(always)]
    pub fn occe(&mut self, n: u8) -> OCCE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        OCCE_W::new(self, n * 0 + 7)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OCCE_W<CCMR1_OUTPUTrs> {
        OCCE_W::new(self, 7)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&mut self) -> OC1M_1_W<CCMR1_OUTPUTrs> {
        OC1M_1_W::new(self, 16)
    }
}
/**TIM capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TIM16:CCMR1_Output)*/
pub struct CCMR1_OUTPUTrs;
impl crate::RegisterSpec for CCMR1_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1_output::R`](R) reader structure
impl crate::Readable for CCMR1_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure
impl crate::Writable for CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR1_Output to value 0
impl crate::Resettable for CCMR1_OUTPUTrs {}
