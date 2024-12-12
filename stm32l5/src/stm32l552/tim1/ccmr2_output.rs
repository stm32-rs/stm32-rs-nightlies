///Register `CCMR2_Output` reader
pub type R = crate::R<CCMR2_OUTPUTrs>;
///Register `CCMR2_Output` writer
pub type W = crate::W<CCMR2_OUTPUTrs>;
///Field `CC3S` reader - Capture/Compare 3 selection
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - Capture/Compare 3 selection
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC3FE` reader - Output compare 3 fast enable
pub type OC3FE_R = crate::BitReader;
///Field `OC3FE` writer - Output compare 3 fast enable
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3PE` reader - Output compare 3 preload enable
pub type OC3PE_R = crate::BitReader;
///Field `OC3PE` writer - Output compare 3 preload enable
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output compare 3 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC3M {
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
impl From<OC3M> for u8 {
    #[inline(always)]
    fn from(variant: OC3M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC3M {
    type Ux = u8;
}
impl crate::IsEnum for OC3M {}
///Field `OC3M` reader - Output compare 3 mode
pub type OC3M_R = crate::FieldReader<OC3M>;
impl OC3M_R {
    ///Get enumerated values variant
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
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == OC3M::Frozen
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        *self == OC3M::ActiveOnMatch
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        *self == OC3M::InactiveOnMatch
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == OC3M::Toggle
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC3M::ForceInactive
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        *self == OC3M::ForceActive
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        *self == OC3M::PwmMode1
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        *self == OC3M::PwmMode2
    }
}
///Field `OC3M` writer - Output compare 3 mode
pub type OC3M_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OC3M, crate::Safe>;
impl<'a, REG> OC3M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::Frozen)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ActiveOnMatch)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::InactiveOnMatch)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::Toggle)
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ForceInactive)
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_active(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::ForceActive)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::PwmMode1)
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M::PwmMode2)
    }
}
///Field `OC3CE` reader - Output compare 3 clear enable
pub type OC3CE_R = crate::BitReader;
///Field `OC3CE` writer - Output compare 3 clear enable
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4S` reader - Capture/Compare 4 selection
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - Capture/Compare 4 selection
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC4FE` reader - Output compare 4 fast enable
pub type OC4FE_R = crate::BitReader;
///Field `OC4FE` writer - Output compare 4 fast enable
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4PE` reader - Output compare 4 preload enable
pub type OC4PE_R = crate::BitReader;
///Field `OC4PE` writer - Output compare 4 preload enable
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M` reader - Output compare 4 mode
pub use OC3M_R as OC4M_R;
///Field `OC4M` writer - Output compare 4 mode
pub use OC3M_W as OC4M_W;
///Field `OC4CE` reader - Output compare 4 clear enable
pub type OC4CE_R = crate::BitReader;
///Field `OC4CE` writer - Output compare 4 clear enable
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Output Compare 3 mode - bit 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC3M_3 {
    ///0: Normal output compare mode (modes 0-7)
    Normal = 0,
    ///1: Extended output compare mode (modes 7-15)
    Extended = 1,
}
impl From<OC3M_3> for bool {
    #[inline(always)]
    fn from(variant: OC3M_3) -> Self {
        variant as u8 != 0
    }
}
///Field `OC3M_3` reader - Output Compare 3 mode - bit 3
pub type OC3M_3_R = crate::BitReader<OC3M_3>;
impl OC3M_3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC3M_3 {
        match self.bits {
            false => OC3M_3::Normal,
            true => OC3M_3::Extended,
        }
    }
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OC3M_3::Normal
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == OC3M_3::Extended
    }
}
///Field `OC3M_3` writer - Output Compare 3 mode - bit 3
pub type OC3M_3_W<'a, REG> = crate::BitWriter<'a, REG, OC3M_3>;
impl<'a, REG> OC3M_3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_3::Normal)
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(OC3M_3::Extended)
    }
}
///Field `OC4M_3` reader - Output Compare 4 mode - bit 3
pub use OC3M_3_R as OC4M_3_R;
///Field `OC4M_3` writer - Output Compare 4 mode - bit 3
pub use OC3M_3_W as OC4M_3_W;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output Compare 3 mode - bit 3
    #[inline(always)]
    pub fn oc3m_3(&self) -> OC3M_3_R {
        OC3M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output Compare 4 mode - bit 3
    #[inline(always)]
    pub fn oc4m_3(&self) -> OC4M_3_R {
        OC4M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Output")
            .field("oc4ce", &self.oc4ce())
            .field("oc3m", &self.oc3m())
            .field("oc4m", &self.oc4m())
            .field("oc4pe", &self.oc4pe())
            .field("oc4fe", &self.oc4fe())
            .field("cc4s", &self.cc4s())
            .field("oc3ce", &self.oc3ce())
            .field("oc3pe", &self.oc3pe())
            .field("oc3fe", &self.oc3fe())
            .field("cc3s", &self.cc3s())
            .field("oc3m_3", &self.oc3m_3())
            .field("oc4m_3", &self.oc4m_3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR2_OUTPUTrs> {
        CC3S_W::new(self, 0)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W<CCMR2_OUTPUTrs> {
        OC3FE_W::new(self, 2)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W<CCMR2_OUTPUTrs> {
        OC3PE_W::new(self, 3)
    }
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&mut self) -> OC3M_W<CCMR2_OUTPUTrs> {
        OC3M_W::new(self, 4)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W<CCMR2_OUTPUTrs> {
        OC3CE_W::new(self, 7)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR2_OUTPUTrs> {
        CC4S_W::new(self, 8)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W<CCMR2_OUTPUTrs> {
        OC4FE_W::new(self, 10)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W<CCMR2_OUTPUTrs> {
        OC4PE_W::new(self, 11)
    }
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&mut self) -> OC4M_W<CCMR2_OUTPUTrs> {
        OC4M_W::new(self, 12)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&mut self) -> OC4CE_W<CCMR2_OUTPUTrs> {
        OC4CE_W::new(self, 15)
    }
    ///Bit 16 - Output Compare 3 mode - bit 3
    #[inline(always)]
    pub fn oc3m_3(&mut self) -> OC3M_3_W<CCMR2_OUTPUTrs> {
        OC3M_3_W::new(self, 16)
    }
    ///Bit 24 - Output Compare 4 mode - bit 3
    #[inline(always)]
    pub fn oc4m_3(&mut self) -> OC4M_3_W<CCMR2_OUTPUTrs> {
        OC4M_3_W::new(self, 24)
    }
}
/**capture/compare mode register 2 (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TIM1:CCMR2_Output)*/
pub struct CCMR2_OUTPUTrs;
impl crate::RegisterSpec for CCMR2_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_output::R`](R) reader structure
impl crate::Readable for CCMR2_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure
impl crate::Writable for CCMR2_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR2_Output to value 0
impl crate::Resettable for CCMR2_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
