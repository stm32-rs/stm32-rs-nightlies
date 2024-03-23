#[doc = "Register `AF1` reader"]
pub type R = crate::R<AF1rs>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<AF1rs>;
#[doc = "TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer’s tim_brk input. TIMx_BKIN input is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINE {
    #[doc = "0: BKIN input disabled"]
    Disabled = 0,
    #[doc = "1: BKIN input enabled"]
    Enabled = 1,
}
impl From<BKINE> for bool {
    #[inline(always)]
    fn from(variant: BKINE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINE` reader - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer’s tim_brk input. TIMx_BKIN input is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_R = crate::BitReader<BKINE>;
impl BKINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKINE {
        match self.bits {
            false => BKINE::Disabled,
            true => BKINE::Enabled,
        }
    }
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKINE::Disabled
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKINE::Enabled
    }
}
#[doc = "Field `BKINE` writer - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer’s tim_brk input. TIMx_BKIN input is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG, BKINE>;
impl<'a, REG> BKINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE::Disabled)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE::Enabled)
    }
}
#[doc = "tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer’s tim_brk input. tim_brk_cmp1 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1E {
    #[doc = "0: Input disabled"]
    Disabled = 0,
    #[doc = "1: Input enabled"]
    Enabled = 1,
}
impl From<BKCMP1E> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1E` reader - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer’s tim_brk input. tim_brk_cmp1 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_R = crate::BitReader<BKCMP1E>;
impl BKCMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP1E {
        match self.bits {
            false => BKCMP1E::Disabled,
            true => BKCMP1E::Enabled,
        }
    }
    #[doc = "Input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKCMP1E::Disabled
    }
    #[doc = "Input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKCMP1E::Enabled
    }
}
#[doc = "Field `BKCMP1E` writer - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer’s tim_brk input. tim_brk_cmp1 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP1E>;
impl<'a, REG> BKCMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1E::Disabled)
    }
    #[doc = "Input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1E::Enabled)
    }
}
#[doc = "Field `BKCMP2E` reader - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer’s tim_brk input. tim_brk_cmp2 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_R as BKCMP2E_R;
#[doc = "Field `BKCMP3E` reader - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer’s tim_brk input. tim_brk_cmp3 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_R as BKCMP3E_R;
#[doc = "Field `BKCMP4E` reader - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer’s tim_brk input. tim_brk_cmp4 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_R as BKCMP4E_R;
#[doc = "Field `BKCMP5E` reader - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer’s tim_brk input. tim_brk_cmp5 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_R as BKCMP5E_R;
#[doc = "Field `BKCMP6E` reader - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer’s tim_brk input. tim_brk_cmp6 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_R as BKCMP6E_R;
#[doc = "Field `BKCMP7E` reader - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer’s tim_brk input. tim_brk_cmp7 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_R as BKCMP7E_R;
#[doc = "Field `BKCMP8E` reader - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer’s tim_brk input. tim_brk_cmp8 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_R as BKCMP8E_R;
#[doc = "Field `BKCMP2E` writer - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer’s tim_brk input. tim_brk_cmp2 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_W as BKCMP2E_W;
#[doc = "Field `BKCMP3E` writer - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer’s tim_brk input. tim_brk_cmp3 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_W as BKCMP3E_W;
#[doc = "Field `BKCMP4E` writer - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer’s tim_brk input. tim_brk_cmp4 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_W as BKCMP4E_W;
#[doc = "Field `BKCMP5E` writer - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer’s tim_brk input. tim_brk_cmp5 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_W as BKCMP5E_W;
#[doc = "Field `BKCMP6E` writer - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer’s tim_brk input. tim_brk_cmp6 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_W as BKCMP6E_W;
#[doc = "Field `BKCMP7E` writer - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer’s tim_brk input. tim_brk_cmp7 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_W as BKCMP7E_W;
#[doc = "Field `BKCMP8E` writer - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer’s tim_brk input. tim_brk_cmp8 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1E_W as BKCMP8E_W;
#[doc = "TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINP {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKINP> for bool {
    #[inline(always)]
    fn from(variant: BKINP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINP` reader - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_R = crate::BitReader<BKINP>;
impl BKINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKINP {
        match self.bits {
            false => BKINP::NotInverted,
            true => BKINP::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKINP::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKINP::Inverted
    }
}
#[doc = "Field `BKINP` writer - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG, BKINP>;
impl<'a, REG> BKINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::Inverted)
    }
}
#[doc = "tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1P {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKCMP1P> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1P` reader - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_R = crate::BitReader<BKCMP1P>;
impl BKCMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP1P {
        match self.bits {
            false => BKCMP1P::NotInverted,
            true => BKCMP1P::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKCMP1P::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKCMP1P::Inverted
    }
}
#[doc = "Field `BKCMP1P` writer - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP1P>;
impl<'a, REG> BKCMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1P::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1P::Inverted)
    }
}
#[doc = "Field `BKCMP2P` reader - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1P_R as BKCMP2P_R;
#[doc = "Field `BKCMP3P` reader - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1P_R as BKCMP3P_R;
#[doc = "Field `BKCMP4P` reader - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1P_R as BKCMP4P_R;
#[doc = "Field `BKCMP2P` writer - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1P_W as BKCMP2P_W;
#[doc = "Field `BKCMP3P` writer - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1P_W as BKCMP3P_W;
#[doc = "Field `BKCMP4P` writer - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub use BKCMP1P_W as BKCMP4P_W;
#[doc = "etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL {
    #[doc = "0: ETR legacy mode"]
    Legacy = 0,
    #[doc = "1: COMP1 output"]
    Comp1 = 1,
    #[doc = "2: COMP2 output"]
    Comp2 = 2,
}
impl From<ETRSEL> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRSEL {
    type Ux = u8;
}
#[doc = "Field `ETRSEL` reader - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type ETRSEL_R = crate::FieldReader<ETRSEL>;
impl ETRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL> {
        match self.bits {
            0 => Some(ETRSEL::Legacy),
            1 => Some(ETRSEL::Comp1),
            2 => Some(ETRSEL::Comp2),
            _ => None,
        }
    }
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == ETRSEL::Legacy
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == ETRSEL::Comp1
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == ETRSEL::Comp2
    }
}
#[doc = "Field `ETRSEL` writer - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Legacy)
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Comp1)
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn comp2(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Comp2)
    }
}
impl R {
    #[doc = "Bit 0 - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer’s tim_brk input. TIMx_BKIN input is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer’s tim_brk input. tim_brk_cmp1 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer’s tim_brk input. tim_brk_cmp2 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer’s tim_brk input. tim_brk_cmp3 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp3e(&self) -> BKCMP3E_R {
        BKCMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer’s tim_brk input. tim_brk_cmp4 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp4e(&self) -> BKCMP4E_R {
        BKCMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer’s tim_brk input. tim_brk_cmp5 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp5e(&self) -> BKCMP5E_R {
        BKCMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer’s tim_brk input. tim_brk_cmp6 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp6e(&self) -> BKCMP6E_R {
        BKCMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer’s tim_brk input. tim_brk_cmp7 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp7e(&self) -> BKCMP7E_R {
        BKCMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer’s tim_brk input. tim_brk_cmp8 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp8e(&self) -> BKCMP8E_R {
        BKCMP8E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp3p(&self) -> BKCMP3P_R {
        BKCMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp4p(&self) -> BKCMP4P_R {
        BKCMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer’s tim_brk input. TIMx_BKIN input is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<AF1rs> {
        BKINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer’s tim_brk input. tim_brk_cmp1 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<AF1rs> {
        BKCMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer’s tim_brk input. tim_brk_cmp2 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<AF1rs> {
        BKCMP2E_W::new(self, 2)
    }
    #[doc = "Bit 3 - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer’s tim_brk input. tim_brk_cmp3 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3e(&mut self) -> BKCMP3E_W<AF1rs> {
        BKCMP3E_W::new(self, 3)
    }
    #[doc = "Bit 4 - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer’s tim_brk input. tim_brk_cmp4 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp4e(&mut self) -> BKCMP4E_W<AF1rs> {
        BKCMP4E_W::new(self, 4)
    }
    #[doc = "Bit 5 - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer’s tim_brk input. tim_brk_cmp5 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp5e(&mut self) -> BKCMP5E_W<AF1rs> {
        BKCMP5E_W::new(self, 5)
    }
    #[doc = "Bit 6 - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer’s tim_brk input. tim_brk_cmp6 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp6e(&mut self) -> BKCMP6E_W<AF1rs> {
        BKCMP6E_W::new(self, 6)
    }
    #[doc = "Bit 7 - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer’s tim_brk input. tim_brk_cmp7 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp7e(&mut self) -> BKCMP7E_W<AF1rs> {
        BKCMP7E_W::new(self, 7)
    }
    #[doc = "Bit 8 - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer’s tim_brk input. tim_brk_cmp8 output is ‘ORed’ with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp8e(&mut self) -> BKCMP8E_W<AF1rs> {
        BKCMP8E_W::new(self, 8)
    }
    #[doc = "Bit 9 - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<AF1rs> {
        BKINP_W::new(self, 9)
    }
    #[doc = "Bit 10 - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<AF1rs> {
        BKCMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<AF1rs> {
        BKCMP2P_W::new(self, 11)
    }
    #[doc = "Bit 12 - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3p(&mut self) -> BKCMP3P_W<AF1rs> {
        BKCMP3P_W::new(self, 12)
    }
    #[doc = "Bit 13 - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp4p(&mut self) -> BKCMP4P_W<AF1rs> {
        BKCMP4P_W::new(self, 13)
    }
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "TIM1 alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for AF1rs {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF1 to value 0x01"]
impl crate::Resettable for AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
