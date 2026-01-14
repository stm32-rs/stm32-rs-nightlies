///Register `TIM15_BDTR` reader
pub type R = crate::R<TIM15_BDTRrs>;
///Register `TIM15_BDTR` writer
pub type W = crate::W<TIM15_BDTRrs>;
///Field `DTG` reader - Dead-time generator setup
pub type DTG_R = crate::FieldReader;
///Field `DTG` writer - Dead-time generator setup
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Lock configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK {
    ///0: LOCK OFF - No bit is write protected
    B0x0 = 0,
    ///1: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written
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
///Field `LOCK` reader - Lock configuration
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
    ///LOCK OFF - No bit is write protected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LOCK::B0x0
    }
    ///LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written
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
///Field `LOCK` writer - Lock configuration
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LOCK, crate::Safe>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LOCK OFF - No bit is write protected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::B0x0)
    }
    ///LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written
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
/**Off-state selection for Idle mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI {
    ///0: When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0)
    B0x0 = 0,
    ///1: When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1.
    B0x1 = 1,
}
impl From<OSSI> for bool {
    #[inline(always)]
    fn from(variant: OSSI) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSI` reader - Off-state selection for Idle mode
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
    ///When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSSI::B0x0
    }
    ///When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSSI::B0x1
    }
}
///Field `OSSI` writer - Off-state selection for Idle mode
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::B0x0)
    }
    ///When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::B0x1)
    }
}
/**Off-state selection for Run mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    ///0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO, which forces a Hi-Z state)
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
///Field `OSSR` reader - Off-state selection for Run mode
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
    ///When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO, which forces a Hi-Z state)
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
///Field `OSSR` writer - Off-state selection for Run mode
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO, which forces a Hi-Z state)
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
/**Break enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    ///0: Break inputs (BRK and CCS clock failure event) disabled
    B0x0 = 0,
}
impl From<BKE> for bool {
    #[inline(always)]
    fn from(variant: BKE) -> Self {
        variant as u8 != 0
    }
}
///Field `BKE` reader - Break enable
pub type BKE_R = crate::BitReader<BKE>;
impl BKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BKE> {
        match self.bits {
            false => Some(BKE::B0x0),
            _ => None,
        }
    }
    ///Break inputs (BRK and CCS clock failure event) disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKE::B0x0
    }
}
///Field `BKE` writer - Break enable
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break inputs (BRK and CCS clock failure event) disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::B0x0)
    }
}
/**Break polarity

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
///Field `BKP` reader - Break polarity
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
///Field `BKP` writer - Break polarity
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
/**Automatic output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE {
    ///0: MOE can be set only by software
    B0x0 = 0,
    ///1: MOE can be set by software or automatically at the next update event (if the break input is not be active)
    B0x1 = 1,
}
impl From<AOE> for bool {
    #[inline(always)]
    fn from(variant: AOE) -> Self {
        variant as u8 != 0
    }
}
///Field `AOE` reader - Automatic output enable
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
    ///MOE can be set by software or automatically at the next update event (if the break input is not be active)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AOE::B0x1
    }
}
///Field `AOE` writer - Automatic output enable
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
    ///MOE can be set by software or automatically at the next update event (if the break input is not be active)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::B0x1)
    }
}
/**Main output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    ///0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit.
    B0x0 = 0,
    ///1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
    B0x1 = 1,
}
impl From<MOE> for bool {
    #[inline(always)]
    fn from(variant: MOE) -> Self {
        variant as u8 != 0
    }
}
///Field `MOE` reader - Main output enable
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
    ///OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MOE::B0x0
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MOE::B0x1
    }
}
///Field `MOE` writer - Main output enable
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::B0x0)
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::B0x1)
    }
}
/**Break filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKF {
    ///0: No filter, BRK acts asynchronously
    B0x0 = 0,
    ///1: fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=2
    B0x1 = 1,
    ///2: fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=4
    B0x2 = 2,
    ///3: fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=8
    B0x3 = 3,
    ///4: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=6
    B0x4 = 4,
    ///5: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=8
    B0x5 = 5,
    ///6: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=6
    B0x6 = 6,
    ///7: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=8
    B0x7 = 7,
    ///8: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=6
    B0x8 = 8,
    ///9: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=8
    B0x9 = 9,
    ///10: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=5
    B0xA = 10,
    ///11: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=6
    B0xB = 11,
    ///12: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=8
    B0xC = 12,
    ///13: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=5
    B0xD = 13,
    ///14: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=6
    B0xE = 14,
    ///15: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=8
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
///Field `BKF` reader - Break filter
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
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKF::B0x1
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BKF::B0x2
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BKF::B0x3
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=6
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == BKF::B0x4
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=8
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == BKF::B0x5
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == BKF::B0x6
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=8
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == BKF::B0x7
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=6
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == BKF::B0x8
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=8
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == BKF::B0x9
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=5
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == BKF::B0xA
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=6
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == BKF::B0xB
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=8
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == BKF::B0xC
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=5
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == BKF::B0xD
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=6
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == BKF::B0xE
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=8
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == BKF::B0xF
    }
}
///Field `BKF` writer - Break filter
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
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x1)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x2)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x3)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=6
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x4)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=8
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x5)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x6)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x7)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=6
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x8)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=8
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0x9)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=5
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xA)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=6
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xB)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=8
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xC)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=5
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xD)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=6
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xE)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=8
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(BKF::B0xF)
    }
}
/**Break Disarm

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
///Field `BKDSRM` reader - Break Disarm
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
///Field `BKDSRM` writer - Break Disarm
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
/**Break Bidirectional

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
///Field `BKBID` reader - Break Bidirectional
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
///Field `BKBID` writer - Break Bidirectional
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
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 26 - Break Disarm
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Break Bidirectional
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_BDTR")
            .field("dtg", &self.dtg())
            .field("lock", &self.lock())
            .field("ossi", &self.ossi())
            .field("ossr", &self.ossr())
            .field("bke", &self.bke())
            .field("bkp", &self.bkp())
            .field("aoe", &self.aoe())
            .field("moe", &self.moe())
            .field("bkf", &self.bkf())
            .field("bkdsrm", &self.bkdsrm())
            .field("bkbid", &self.bkbid())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<'_, TIM15_BDTRrs> {
        DTG_W::new(self, 0)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, TIM15_BDTRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<'_, TIM15_BDTRrs> {
        OSSI_W::new(self, 10)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<'_, TIM15_BDTRrs> {
        OSSR_W::new(self, 11)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<'_, TIM15_BDTRrs> {
        BKE_W::new(self, 12)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, TIM15_BDTRrs> {
        BKP_W::new(self, 13)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<'_, TIM15_BDTRrs> {
        AOE_W::new(self, 14)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<'_, TIM15_BDTRrs> {
        MOE_W::new(self, 15)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W<'_, TIM15_BDTRrs> {
        BKF_W::new(self, 16)
    }
    ///Bit 26 - Break Disarm
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<'_, TIM15_BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    ///Bit 28 - Break Bidirectional
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W<'_, TIM15_BDTRrs> {
        BKBID_W::new(self, 28)
    }
}
/**TIM15 break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`tim15_bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_BDTR)*/
pub struct TIM15_BDTRrs;
impl crate::RegisterSpec for TIM15_BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`tim15_bdtr::R`](R) reader structure
impl crate::Readable for TIM15_BDTRrs {}
///`write(|w| ..)` method takes [`tim15_bdtr::W`](W) writer structure
impl crate::Writable for TIM15_BDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_BDTR to value 0
impl crate::Resettable for TIM15_BDTRrs {}
