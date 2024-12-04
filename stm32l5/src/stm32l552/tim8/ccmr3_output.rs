///Register `CCMR3_Output` reader
pub type R = crate::R<CCMR3_OUTPUTrs>;
///Register `CCMR3_Output` writer
pub type W = crate::W<CCMR3_OUTPUTrs>;
///Field `OC5FE` reader - Output compare 5 fast enable
pub type OC5FE_R = crate::BitReader;
///Field `OC5FE` writer - Output compare 5 fast enable
pub type OC5FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC5PE` reader - Output compare 5 preload enable
pub type OC5PE_R = crate::BitReader;
///Field `OC5PE` writer - Output compare 5 preload enable
pub type OC5PE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output compare 5 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC5M {
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
impl From<OC5M> for u8 {
    #[inline(always)]
    fn from(variant: OC5M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC5M {
    type Ux = u8;
}
impl crate::IsEnum for OC5M {}
///Field `OC5M` reader - Output compare 5 mode
pub type OC5M_R = crate::FieldReader<OC5M>;
impl OC5M_R {
    ///Get enumerated values variant
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
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC5M::Frozen
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC5M::ActiveOnMatch
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC5M::InactiveOnMatch
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC5M::Toggle
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC5M::ForceInactive
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC5M::ForceActive
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC5M::PwmMode1
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC5M::PwmMode2
    }
}
///Field `OC5M` writer - Output compare 5 mode
pub type OC5M_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OC5M, crate::Safe>;
impl<'a, REG> OC5M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::Toggle)
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::ForceInactive)
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::PwmMode1)
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M::PwmMode2)
    }
}
///Field `OC5CE` reader - Output compare 5 clear enable
pub type OC5CE_R = crate::BitReader;
///Field `OC5CE` writer - Output compare 5 clear enable
pub type OC5CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6FE` reader - Output compare 6 fast enable
pub type OC6FE_R = crate::BitReader;
///Field `OC6FE` writer - Output compare 6 fast enable
pub type OC6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6PE` reader - Output compare 6 preload enable
pub type OC6PE_R = crate::BitReader;
///Field `OC6PE` writer - Output compare 6 preload enable
pub type OC6PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6M` reader - Output compare 6 mode
pub use OC5M_R as OC6M_R;
///Field `OC6M` writer - Output compare 6 mode
pub use OC5M_W as OC6M_W;
///Field `OC6CE` reader - Output compare 6 clear enable
pub type OC6CE_R = crate::BitReader;
///Field `OC6CE` writer - Output compare 6 clear enable
pub type OC6CE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output Compare 5 mode bit 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC5M_3 {
    ///0: Normal output compare mode (modes 0-7)
    Normal = 0,
    ///1: Extended output compare mode (modes 7-15)
    Extended = 1,
}
impl From<OC5M_3> for bool {
    #[inline(always)]
    fn from(variant: OC5M_3) -> Self {
        variant as u8 != 0
    }
}
///Field `OC5M_3` reader - Output Compare 5 mode bit 3
pub type OC5M_3_R = crate::BitReader<OC5M_3>;
impl OC5M_3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC5M_3 {
        match self.bits {
            false => OC5M_3::Normal,
            true => OC5M_3::Extended,
        }
    }
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC5M_3::Normal
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC5M_3::Extended
    }
}
///Field `OC5M_3` writer - Output Compare 5 mode bit 3
pub type OC5M_3_W<'a, REG> = crate::BitWriter<'a, REG, OC5M_3>;
impl<'a, REG> OC5M_3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M_3::Normal)
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(OC5M_3::Extended)
    }
}
///Field `OC6M_3` reader - Output Compare 6 mode bit 3
pub use OC5M_3_R as OC6M_3_R;
///Field `OC6M_3` writer - Output Compare 6 mode bit 3
pub use OC5M_3_W as OC6M_3_W;
impl R {
    ///Bit 2 - Output compare 5 fast enable
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 5 preload enable
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 5 clear enable
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Output compare 6 fast enable
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 6 preload enable
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 6 clear enable
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output Compare 5 mode bit 3
    #[inline(always)]
    pub fn oc5m_3(&self) -> OC5M_3_R {
        OC5M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output Compare 6 mode bit 3
    #[inline(always)]
    pub fn oc6m_3(&self) -> OC6M_3_R {
        OC6M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR3_Output")
            .field("oc5m_3", &self.oc5m_3())
            .field("oc6m_3", &self.oc6m_3())
            .field("oc6ce", &self.oc6ce())
            .field("oc5m", &self.oc5m())
            .field("oc6m", &self.oc6m())
            .field("oc6pe", &self.oc6pe())
            .field("oc6fe", &self.oc6fe())
            .field("oc5ce", &self.oc5ce())
            .field("oc5pe", &self.oc5pe())
            .field("oc5fe", &self.oc5fe())
            .finish()
    }
}
impl W {
    ///Bit 2 - Output compare 5 fast enable
    #[inline(always)]
    pub fn oc5fe(&mut self) -> OC5FE_W<CCMR3_OUTPUTrs> {
        OC5FE_W::new(self, 2)
    }
    ///Bit 3 - Output compare 5 preload enable
    #[inline(always)]
    pub fn oc5pe(&mut self) -> OC5PE_W<CCMR3_OUTPUTrs> {
        OC5PE_W::new(self, 3)
    }
    ///Bits 4:6 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m(&mut self) -> OC5M_W<CCMR3_OUTPUTrs> {
        OC5M_W::new(self, 4)
    }
    ///Bit 7 - Output compare 5 clear enable
    #[inline(always)]
    pub fn oc5ce(&mut self) -> OC5CE_W<CCMR3_OUTPUTrs> {
        OC5CE_W::new(self, 7)
    }
    ///Bit 10 - Output compare 6 fast enable
    #[inline(always)]
    pub fn oc6fe(&mut self) -> OC6FE_W<CCMR3_OUTPUTrs> {
        OC6FE_W::new(self, 10)
    }
    ///Bit 11 - Output compare 6 preload enable
    #[inline(always)]
    pub fn oc6pe(&mut self) -> OC6PE_W<CCMR3_OUTPUTrs> {
        OC6PE_W::new(self, 11)
    }
    ///Bits 12:14 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m(&mut self) -> OC6M_W<CCMR3_OUTPUTrs> {
        OC6M_W::new(self, 12)
    }
    ///Bit 15 - Output compare 6 clear enable
    #[inline(always)]
    pub fn oc6ce(&mut self) -> OC6CE_W<CCMR3_OUTPUTrs> {
        OC6CE_W::new(self, 15)
    }
    ///Bit 16 - Output Compare 5 mode bit 3
    #[inline(always)]
    pub fn oc5m_3(&mut self) -> OC5M_3_W<CCMR3_OUTPUTrs> {
        OC5M_3_W::new(self, 16)
    }
    ///Bit 24 - Output Compare 6 mode bit 3
    #[inline(always)]
    pub fn oc6m_3(&mut self) -> OC6M_3_W<CCMR3_OUTPUTrs> {
        OC6M_3_W::new(self, 24)
    }
}
/**capture/compare mode register 2 (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr3_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TIM8:CCMR3_Output)*/
pub struct CCMR3_OUTPUTrs;
impl crate::RegisterSpec for CCMR3_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr3_output::R`](R) reader structure
impl crate::Readable for CCMR3_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr3_output::W`](W) writer structure
impl crate::Writable for CCMR3_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR3_Output to value 0
impl crate::Resettable for CCMR3_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
