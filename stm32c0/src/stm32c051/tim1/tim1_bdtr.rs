///Register `TIM1_BDTR` reader
pub type R = crate::R<TIM1_BDTRrs>;
///Register `TIM1_BDTR` writer
pub type W = crate::W<TIM1_BDTRrs>;
///Field `DTG` reader - Dead-time generator setup
pub type DTG_R = crate::FieldReader;
///Field `DTG` writer - Dead-time generator setup
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK {
    ///0: LOCK OFF - No bit is write protected.
    B0x0 = 0,
    ///1: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] bits in TIMx_BDTR register can no longer be written.
    B0x1 = 1,
    ///2: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written.
    B0x2 = 2,
    ///3: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written.
    B0x3 = 3,
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
impl crate::IsEnum for LOCK {}
///Field `LOCK` reader - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.
pub type LOCK_R = crate::FieldReader<LOCK>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            0 => LOCK::B0x0,
            1 => LOCK::B0x1,
            2 => LOCK::B0x2,
            3 => LOCK::B0x3,
            _ => unreachable!(),
        }
    }
    ///LOCK OFF - No bit is write protected.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LOCK::B0x0
    }
    ///LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] bits in TIMx_BDTR register can no longer be written.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LOCK::B0x1
    }
    ///LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written.
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LOCK::B0x2
    }
    ///LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LOCK::B0x3
    }
}
///Field `LOCK` writer - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LOCK, crate::Safe>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LOCK OFF - No bit is write protected.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::B0x0)
    }
    ///LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] bits in TIMx_BDTR register can no longer be written.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::B0x1)
    }
    ///LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::B0x2)
    }
    ///LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::B0x3)
    }
}
/**Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI {
    ///0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state).
    B0x0 = 0,
    ///1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output.
    B0x1 = 1,
}
impl From<OSSI> for bool {
    #[inline(always)]
    fn from(variant: OSSI) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSI` reader - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OSSI_R = crate::BitReader<OSSI>;
impl OSSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSSI {
        match self.bits {
            false => OSSI::B0x0,
            true => OSSI::B0x1,
        }
    }
    ///When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSSI::B0x0
    }
    ///When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSSI::B0x1
    }
}
///Field `OSSI` writer - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::B0x0)
    }
    ///When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::B0x1)
    }
}
/**Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    ///0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state).
    B0x0 = 0,
    ///1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer).
    B0x1 = 1,
}
impl From<OSSR> for bool {
    #[inline(always)]
    fn from(variant: OSSR) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSR` reader - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OSSR_R = crate::BitReader<OSSR>;
impl OSSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSSR {
        match self.bits {
            false => OSSR::B0x0,
            true => OSSR::B0x1,
        }
    }
    ///When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSSR::B0x0
    }
    ///When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSSR::B0x1
    }
}
///Field `OSSR` writer - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::B0x0)
    }
    ///When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::B0x1)
    }
}
/**Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure 100: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    ///0: Break function disabled
    B0x0 = 0,
    ///1: Break function enabled
    B0x1 = 1,
}
impl From<BKE> for bool {
    #[inline(always)]
    fn from(variant: BKE) -> Self {
        variant as u8 != 0
    }
}
///Field `BKE` reader - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure 100: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKE_R = crate::BitReader<BKE>;
impl BKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKE {
        match self.bits {
            false => BKE::B0x0,
            true => BKE::B0x1,
        }
    }
    ///Break function disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKE::B0x0
    }
    ///Break function enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKE::B0x1
    }
}
///Field `BKE` writer - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure 100: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break function disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::B0x0)
    }
    ///Break function enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::B0x1)
    }
}
/**Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    ///0: Break input BRK is active low
    B0x0 = 0,
    ///1: Break input BRK is active high
    B0x1 = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
///Field `BKP` reader - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::B0x0,
            true => BKP::B0x1,
        }
    }
    ///Break input BRK is active low
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKP::B0x0
    }
    ///Break input BRK is active high
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKP::B0x1
    }
}
///Field `BKP` writer - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK is active low
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::B0x0)
    }
    ///Break input BRK is active high
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::B0x1)
    }
}
/**Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE {
    ///0: MOE can be set only by software
    B0x0 = 0,
    ///1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    B0x1 = 1,
}
impl From<AOE> for bool {
    #[inline(always)]
    fn from(variant: AOE) -> Self {
        variant as u8 != 0
    }
}
///Field `AOE` reader - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type AOE_R = crate::BitReader<AOE>;
impl AOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AOE {
        match self.bits {
            false => AOE::B0x0,
            true => AOE::B0x1,
        }
    }
    ///MOE can be set only by software
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AOE::B0x0
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AOE::B0x1
    }
}
///Field `AOE` writer - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG, AOE>;
impl<'a, REG> AOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MOE can be set only by software
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::B0x0)
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::B0x1)
    }
}
/**Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    ///0: In response to a break 2 event. OC and OCN outputs are disabled
    B0x0 = 0,
    ///1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register).
    B0x1 = 1,
}
impl From<MOE> for bool {
    #[inline(always)]
    fn from(variant: MOE) -> Self {
        variant as u8 != 0
    }
}
///Field `MOE` reader - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)).
pub type MOE_R = crate::BitReader<MOE>;
impl MOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOE {
        match self.bits {
            false => MOE::B0x0,
            true => MOE::B0x1,
        }
    }
    ///In response to a break 2 event. OC and OCN outputs are disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MOE::B0x0
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MOE::B0x1
    }
}
///Field `MOE` writer - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)).
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In response to a break 2 event. OC and OCN outputs are disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::B0x0)
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::B0x1)
    }
}
/**Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKF {
    ///0: No filter, BRK acts asynchronously
    B0x0 = 0,
    ///1: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    B0x1 = 1,
    ///2: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    B0x2 = 2,
    ///3: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    B0x3 = 3,
    ///4: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    B0x4 = 4,
    ///5: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    B0x5 = 5,
    ///6: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    B0x6 = 6,
    ///7: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    B0x7 = 7,
    ///8: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    B0x8 = 8,
    ///9: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    B0x9 = 9,
    ///10: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    B0xA = 10,
    ///11: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    B0xB = 11,
    ///12: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    B0xC = 12,
    ///13: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    B0xD = 13,
    ///14: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    B0xE = 14,
    ///15: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    B0xF = 15,
}
impl From<BKF> for u8 {
    #[inline(always)]
    fn from(variant: BKF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BKF {
    type Ux = u8;
}
impl crate::IsEnum for BKF {}
///Field `BKF` reader - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BKF_R = crate::FieldReader<BKF>;
impl BKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKF {
        match self.bits {
            0 => BKF::B0x0,
            1 => BKF::B0x1,
            2 => BKF::B0x2,
            3 => BKF::B0x3,
            4 => BKF::B0x4,
            5 => BKF::B0x5,
            6 => BKF::B0x6,
            7 => BKF::B0x7,
            8 => BKF::B0x8,
            9 => BKF::B0x9,
            10 => BKF::B0xA,
            11 => BKF::B0xB,
            12 => BKF::B0xC,
            13 => BKF::B0xD,
            14 => BKF::B0xE,
            15 => BKF::B0xF,
            _ => unreachable!(),
        }
    }
    ///No filter, BRK acts asynchronously
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKF::B0x0
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKF::B0x1
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BKF::B0x2
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BKF::B0x3
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == BKF::B0x4
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == BKF::B0x5
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == BKF::B0x6
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == BKF::B0x7
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == BKF::B0x8
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == BKF::B0x9
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == BKF::B0xA
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == BKF::B0xB
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == BKF::B0xC
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == BKF::B0xD
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == BKF::B0xE
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == BKF::B0xF
    }
}
///Field `BKF` writer - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BKF, crate::Safe>;
impl<'a, REG> BKF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, BRK acts asynchronously
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x0)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x1)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x2)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x3)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x4)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x5)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x6)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x7)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x8)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x9)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xA)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xB)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xC)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xD)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xE)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xF)
    }
}
/**Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BK2F {
    ///0: No filter, BRK2 acts asynchronously
    B0x0 = 0,
    ///1: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    B0x1 = 1,
    ///2: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    B0x2 = 2,
    ///3: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    B0x3 = 3,
    ///4: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    B0x4 = 4,
    ///5: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    B0x5 = 5,
    ///6: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    B0x6 = 6,
    ///7: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    B0x7 = 7,
    ///8: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    B0x8 = 8,
    ///9: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    B0x9 = 9,
    ///10: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    B0xA = 10,
    ///11: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    B0xB = 11,
    ///12: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    B0xC = 12,
    ///13: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    B0xD = 13,
    ///14: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    B0xE = 14,
    ///15: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    B0xF = 15,
}
impl From<BK2F> for u8 {
    #[inline(always)]
    fn from(variant: BK2F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BK2F {
    type Ux = u8;
}
impl crate::IsEnum for BK2F {}
///Field `BK2F` reader - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2F_R = crate::FieldReader<BK2F>;
impl BK2F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2F {
        match self.bits {
            0 => BK2F::B0x0,
            1 => BK2F::B0x1,
            2 => BK2F::B0x2,
            3 => BK2F::B0x3,
            4 => BK2F::B0x4,
            5 => BK2F::B0x5,
            6 => BK2F::B0x6,
            7 => BK2F::B0x7,
            8 => BK2F::B0x8,
            9 => BK2F::B0x9,
            10 => BK2F::B0xA,
            11 => BK2F::B0xB,
            12 => BK2F::B0xC,
            13 => BK2F::B0xD,
            14 => BK2F::B0xE,
            15 => BK2F::B0xF,
            _ => unreachable!(),
        }
    }
    ///No filter, BRK2 acts asynchronously
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2F::B0x0
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2F::B0x1
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BK2F::B0x2
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BK2F::B0x3
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == BK2F::B0x4
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == BK2F::B0x5
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == BK2F::B0x6
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == BK2F::B0x7
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == BK2F::B0x8
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == BK2F::B0x9
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == BK2F::B0xA
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == BK2F::B0xB
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == BK2F::B0xC
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == BK2F::B0xD
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == BK2F::B0xE
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == BK2F::B0xF
    }
}
///Field `BK2F` writer - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BK2F, crate::Safe>;
impl<'a, REG> BK2F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, BRK2 acts asynchronously
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x0)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x1)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x2)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x3)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x4)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x5)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x6)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x7)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x8)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0x9)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0xA)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0xB)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0xC)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0xD)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0xE)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F::B0xF)
    }
}
/**Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2E {
    ///0: Break input BRK2 disabled
    B0x0 = 0,
    ///1: Break input BRK2 enabled
    B0x1 = 1,
}
impl From<BK2E> for bool {
    #[inline(always)]
    fn from(variant: BK2E) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2E` reader - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BK2E_R = crate::BitReader<BK2E>;
impl BK2E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2E {
        match self.bits {
            false => BK2E::B0x0,
            true => BK2E::B0x1,
        }
    }
    ///Break input BRK2 disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2E::B0x0
    }
    ///Break input BRK2 enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2E::B0x1
    }
}
///Field `BK2E` writer - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BK2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2E>;
impl<'a, REG> BK2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK2 disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E::B0x0)
    }
    ///Break input BRK2 enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E::B0x1)
    }
}
/**Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2P {
    ///0: Break input BRK2 is active low
    B0x0 = 0,
    ///1: Break input BRK2 is active high
    B0x1 = 1,
}
impl From<BK2P> for bool {
    #[inline(always)]
    fn from(variant: BK2P) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2P` reader - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BK2P_R = crate::BitReader<BK2P>;
impl BK2P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2P {
        match self.bits {
            false => BK2P::B0x0,
            true => BK2P::B0x1,
        }
    }
    ///Break input BRK2 is active low
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2P::B0x0
    }
    ///Break input BRK2 is active high
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2P::B0x1
    }
}
///Field `BK2P` writer - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BK2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2P>;
impl<'a, REG> BK2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK2 is active low
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P::B0x0)
    }
    ///Break input BRK2 is active high
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P::B0x1)
    }
}
/**Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKDSRM {
    ///0: Break input BRK is armed
    B0x0 = 0,
    ///1: Break input BRK is disarmed
    B0x1 = 1,
}
impl From<BKDSRM> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM) -> Self {
        variant as u8 != 0
    }
}
///Field `BKDSRM` reader - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKDSRM_R = crate::BitReader<BKDSRM>;
impl BKDSRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKDSRM {
        match self.bits {
            false => BKDSRM::B0x0,
            true => BKDSRM::B0x1,
        }
    }
    ///Break input BRK is armed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKDSRM::B0x0
    }
    ///Break input BRK is disarmed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKDSRM::B0x1
    }
}
///Field `BKDSRM` writer - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG, BKDSRM>;
impl<'a, REG> BKDSRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK is armed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::B0x0)
    }
    ///Break input BRK is disarmed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::B0x1)
    }
}
///Field `BK2DSRM` reader - Break2 Disarm Refer to BKDSRM description
pub type BK2DSRM_R = crate::BitReader;
///Field `BK2DSRM` writer - Break2 Disarm Refer to BKDSRM description
pub type BK2DSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKBID {
    ///0: Break input BRK in input mode
    B0x0 = 0,
    ///1: Break input BRK in bidirectional mode
    B0x1 = 1,
}
impl From<BKBID> for bool {
    #[inline(always)]
    fn from(variant: BKBID) -> Self {
        variant as u8 != 0
    }
}
///Field `BKBID` reader - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKBID_R = crate::BitReader<BKBID>;
impl BKBID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKBID {
        match self.bits {
            false => BKBID::B0x0,
            true => BKBID::B0x1,
        }
    }
    ///Break input BRK in input mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKBID::B0x0
    }
    ///Break input BRK in bidirectional mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKBID::B0x1
    }
}
///Field `BKBID` writer - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG, BKBID>;
impl<'a, REG> BKBID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break input BRK in input mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::B0x0)
    }
    ///Break input BRK in bidirectional mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::B0x1)
    }
}
///Field `BK2BID` reader - Break2 bidirectional Refer to BKBID description
pub type BK2BID_R = crate::BitReader;
///Field `BK2BID` writer - Break2 bidirectional Refer to BKBID description
pub type BK2BID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure 100: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)).
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Break2 Disarm Refer to BKDSRM description
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Break2 bidirectional Refer to BKBID description
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_BDTR")
            .field("dtg", &self.dtg())
            .field("lock", &self.lock())
            .field("ossi", &self.ossi())
            .field("ossr", &self.ossr())
            .field("bke", &self.bke())
            .field("bkp", &self.bkp())
            .field("aoe", &self.aoe())
            .field("moe", &self.moe())
            .field("bkf", &self.bkf())
            .field("bk2f", &self.bk2f())
            .field("bk2e", &self.bk2e())
            .field("bk2p", &self.bk2p())
            .field("bkdsrm", &self.bkdsrm())
            .field("bk2dsrm", &self.bk2dsrm())
            .field("bkbid", &self.bkbid())
            .field("bk2bid", &self.bk2bid())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<'_, TIM1_BDTRrs> {
        DTG_W::new(self, 0)
    }
    ///Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, TIM1_BDTRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<'_, TIM1_BDTRrs> {
        OSSI_W::new(self, 10)
    }
    ///Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<'_, TIM1_BDTRrs> {
        OSSR_W::new(self, 11)
    }
    ///Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BRK sources, as per Figure 100: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<'_, TIM1_BDTRrs> {
        BKE_W::new(self, 12)
    }
    ///Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, TIM1_BDTRrs> {
        BKP_W::new(self, 13)
    }
    ///Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<'_, TIM1_BDTRrs> {
        AOE_W::new(self, 14)
    }
    ///Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 17.4.11: TIM1 capture/compare enable register (TIM1_CCER)).
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<'_, TIM1_BDTRrs> {
        MOE_W::new(self, 15)
    }
    ///Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W<'_, TIM1_BDTRrs> {
        BKF_W::new(self, 16)
    }
    ///Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2f(&mut self) -> BK2F_W<'_, TIM1_BDTRrs> {
        BK2F_W::new(self, 20)
    }
    ///Bit 24 - Break 2 enable Note: The BRK2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bk2e(&mut self) -> BK2E_W<'_, TIM1_BDTRrs> {
        BK2E_W::new(self, 24)
    }
    ///Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bk2p(&mut self) -> BK2P_W<'_, TIM1_BDTRrs> {
        BK2P_W::new(self, 25)
    }
    ///Bit 26 - Break Disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<'_, TIM1_BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    ///Bit 27 - Break2 Disarm Refer to BKDSRM description
    #[inline(always)]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<'_, TIM1_BDTRrs> {
        BK2DSRM_W::new(self, 27)
    }
    ///Bit 28 - Break Bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W<'_, TIM1_BDTRrs> {
        BKBID_W::new(self, 28)
    }
    ///Bit 29 - Break2 bidirectional Refer to BKBID description
    #[inline(always)]
    pub fn bk2bid(&mut self) -> BK2BID_W<'_, TIM1_BDTRrs> {
        BK2BID_W::new(self, 29)
    }
}
/**TIM1 break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`tim1_bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM1:TIM1_BDTR)*/
pub struct TIM1_BDTRrs;
impl crate::RegisterSpec for TIM1_BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`tim1_bdtr::R`](R) reader structure
impl crate::Readable for TIM1_BDTRrs {}
///`write(|w| ..)` method takes [`tim1_bdtr::W`](W) writer structure
impl crate::Writable for TIM1_BDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_BDTR to value 0
impl crate::Resettable for TIM1_BDTRrs {}
