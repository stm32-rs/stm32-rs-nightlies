#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BDTRrs>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BDTRrs>;
#[doc = "Field `DTG` reader - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tDTG with tDTG=tDTS. DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtDTG with tDTG=2xtDTS. DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=8xtDTS. DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=16xtDTS. Example if tDTS=125Â ns (8Â MHz), dead-time possible values are: 0 to 15875Â ns by 125Â ns steps, 16Â Î¼s to 31750Â nsÂ by 250Â ns steps, 32Â Î¼s to 63Â Î¼s by 1Â Î¼s steps, 64Â Î¼s to 126Â Î¼s by 2Â Î¼s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTG_R = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tDTG with tDTG=tDTS. DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtDTG with tDTG=2xtDTS. DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=8xtDTS. DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=16xtDTS. Example if tDTS=125Â ns (8Â MHz), dead-time possible values are: 0 to 15875Â ns by 125Â ns steps, 16Â Î¼s to 31750Â nsÂ by 250Â ns steps, 32Â Î¼s to 63Â Î¼s by 1Â Î¼s steps, 64Â Î¼s to 126Â Î¼s by 2Â Î¼s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK {
    #[doc = "0: No bit is write protected"]
    Off = 0,
    #[doc = "1: Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written"]
    Level1 = 1,
    #[doc = "2: LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written"]
    Level2 = 2,
    #[doc = "3: LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written"]
    Level3 = 3,
}
impl From<LOCK> for u8 {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK {
    type Ux = u8;
}
#[doc = "Field `LOCK` reader - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LOCK_R = crate::FieldReader<LOCK>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            0 => LOCK::Off,
            1 => LOCK::Level1,
            2 => LOCK::Level2,
            3 => LOCK::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "No bit is write protected"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK::Off
    }
    #[doc = "Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK::Level1
    }
    #[doc = "LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK::Level2
    }
    #[doc = "LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == LOCK::Level3
    }
}
#[doc = "Field `LOCK` writer - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LOCK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No bit is write protected"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Off)
    }
    #[doc = "Any bits except MOE, OSSR, OSSI and LOCK in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register can no longer be written"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level1)
    }
    #[doc = "LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level2)
    }
    #[doc = "LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level3)
    }
}
#[doc = "Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI {
    #[doc = "0: When inactive, OC/OCN outputs are disabled"]
    Disabled = 0,
    #[doc = "1: When inactive, OC/OCN outputs are forced to idle level"]
    IdleLevel = 1,
}
impl From<OSSI> for bool {
    #[inline(always)]
    fn from(variant: OSSI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSI_R = crate::BitReader<OSSI>;
impl OSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSI {
        match self.bits {
            false => OSSI::Disabled,
            true => OSSI::IdleLevel,
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI::Disabled
    }
    #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSI::IdleLevel
    }
}
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::Disabled)
    }
    #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::IdleLevel)
    }
}
#[doc = "Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    #[doc = "0: When inactive, OC/OCN outputs are disabled"]
    Disabled = 0,
    #[doc = "1: When inactive, OC/OCN outputs are enabled with their inactive level"]
    IdleLevel = 1,
}
impl From<OSSR> for bool {
    #[inline(always)]
    fn from(variant: OSSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSR` reader - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSR_R = crate::BitReader<OSSR>;
impl OSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSR {
        match self.bits {
            false => OSSR::Disabled,
            true => OSSR::IdleLevel,
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR::Disabled
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSR::IdleLevel
    }
}
#[doc = "Field `OSSR` writer - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::Disabled)
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::IdleLevel)
    }
}
#[doc = "Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    #[doc = "0: Break function x disabled"]
    Disabled = 0,
    #[doc = "1: Break function x disabled"]
    Enabled = 1,
}
impl From<BKE> for bool {
    #[inline(always)]
    fn from(variant: BKE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKE` reader - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKE_R = crate::BitReader<BKE>;
impl BKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKE {
        match self.bits {
            false => BKE::Disabled,
            true => BKE::Enabled,
        }
    }
    #[doc = "Break function x disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE::Disabled
    }
    #[doc = "Break function x disabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKE::Enabled
    }
}
#[doc = "Field `BKE` writer - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break function x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Disabled)
    }
    #[doc = "Break function x disabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Enabled)
    }
}
#[doc = "Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    #[doc = "0: Break input BRKx is active low"]
    ActiveLow = 0,
    #[doc = "1: Break input BRKx is active high"]
    ActiveHigh = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKP` reader - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::ActiveLow,
            true => BKP::ActiveHigh,
        }
    }
    #[doc = "Break input BRKx is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP::ActiveLow
    }
    #[doc = "Break input BRKx is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKP::ActiveHigh
    }
}
#[doc = "Field `BKP` writer - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRKx is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveLow)
    }
    #[doc = "Break input BRKx is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveHigh)
    }
}
#[doc = "Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE {
    #[doc = "0: MOE can be set only by software"]
    Manual = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    Automatic = 1,
}
impl From<AOE> for bool {
    #[inline(always)]
    fn from(variant: AOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOE` reader - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AOE_R = crate::BitReader<AOE>;
impl AOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AOE {
        match self.bits {
            false => AOE::Manual,
            true => AOE::Automatic,
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == AOE::Manual
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AOE::Automatic
    }
}
#[doc = "Field `AOE` writer - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG, AOE>;
impl<'a, REG> AOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Manual)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Automatic)
    }
}
#[doc = "Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    #[doc = "0: OC/OCN are disabled or forced idle depending on OSSI"]
    DisabledIdle = 0,
    #[doc = "1: OC/OCN are enabled if CCxE/CCxNE are set"]
    Enabled = 1,
}
impl From<MOE> for bool {
    #[inline(always)]
    fn from(variant: MOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOE` reader - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A)."]
pub type MOE_R = crate::BitReader<MOE>;
impl MOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOE {
        match self.bits {
            false => MOE::DisabledIdle,
            true => MOE::Enabled,
        }
    }
    #[doc = "OC/OCN are disabled or forced idle depending on OSSI"]
    #[inline(always)]
    pub fn is_disabled_idle(&self) -> bool {
        *self == MOE::DisabledIdle
    }
    #[doc = "OC/OCN are enabled if CCxE/CCxNE are set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE::Enabled
    }
}
#[doc = "Field `MOE` writer - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A)."]
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC/OCN are disabled or forced idle depending on OSSI"]
    #[inline(always)]
    pub fn disabled_idle(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::DisabledIdle)
    }
    #[doc = "OC/OCN are enabled if CCxE/CCxNE are set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Enabled)
    }
}
#[doc = "Field `BKF` reader - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKF_R = crate::FieldReader;
#[doc = "Field `BKF` writer - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BK2F` reader - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2F_R = crate::FieldReader;
#[doc = "Field `BK2F` writer - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BK2E` reader - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub use BKE_R as BK2E_R;
#[doc = "Field `BK2E` writer - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub use BKE_W as BK2E_W;
#[doc = "Field `BK2P` reader - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub use BKP_R as BK2P_R;
#[doc = "Field `BK2P` writer - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub use BKP_W as BK2P_W;
#[doc = "Field `BKDSRM` reader - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKDSRM_R = crate::BitReader;
#[doc = "Field `BKDSRM` writer - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2DSRM` reader - Break2 Disarm Refer to BKDSRM description"]
pub type BK2DSRM_R = crate::BitReader;
#[doc = "Field `BK2DSRM` writer - Break2 Disarm Refer to BKDSRM description"]
pub type BK2DSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKBID` reader - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKBID_R = crate::BitReader;
#[doc = "Field `BKBID` writer - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2BID` reader - Break2 bidirectional Refer to BKBID description"]
pub type BK2BID_R = crate::BitReader;
#[doc = "Field `BK2BID` writer - Break2 bidirectional Refer to BKBID description"]
pub type BK2BID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tDTG with tDTG=tDTS. DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtDTG with tDTG=2xtDTS. DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=8xtDTS. DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=16xtDTS. Example if tDTS=125Â ns (8Â MHz), dead-time possible values are: 0 to 15875Â ns by 125Â ns steps, 16Â Î¼s to 31750Â nsÂ by 250Â ns steps, 32Â Î¼s to 63Â Î¼s by 1Â Î¼s steps, 64Â Î¼s to 126Â Î¼s by 2Â Î¼s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A)."]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Break2 Disarm Refer to BKDSRM description"]
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tDTG with tDTG=tDTS. DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtDTG with tDTG=2xtDTS. DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=8xtDTS. DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtDTG with tDTG=16xtDTS. Example if tDTS=125Â ns (8Â MHz), dead-time possible values are: 0 to 15875Â ns by 125Â ns steps, 16Â Î¼s to 31750Â nsÂ by 250Â ns steps, 32Â Î¼s to 63Â Î¼s by 1Â Î¼s steps, 64Â Î¼s to 126Â Î¼s by 2Â Î¼s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<BDTRrs> {
        DTG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<BDTRrs> {
        LOCK_W::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<BDTRrs> {
        OSSI_W::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<BDTRrs> {
        OSSR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per ). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<BDTRrs> {
        BKE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BDTRrs> {
        BKP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<BDTRrs> {
        AOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (enable register (TIM1_CCERTIMx_CCER)N/A)."]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<BDTRrs> {
        MOE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<BDTRrs> {
        BKF_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> BK2F_W<BDTRrs> {
        BK2F_W::new(self, 20)
    }
    #[doc = "Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> BK2E_W<BDTRrs> {
        BK2E_W::new(self, 24)
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> BK2P_W<BDTRrs> {
        BK2P_W::new(self, 25)
    }
    #[doc = "Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    #[doc = "Bit 27 - Break2 Disarm Refer to BKDSRM description"]
    #[inline(always)]
    #[must_use]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    #[doc = "Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<BDTRrs> {
        BKBID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    #[must_use]
    pub fn bk2bid(&mut self) -> BK2BID_W<BDTRrs> {
        BK2BID_W::new(self, 29)
    }
}
#[doc = "break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BDTRrs {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTRrs {
    const RESET_VALUE: u32 = 0;
}
