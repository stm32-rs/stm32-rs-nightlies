///Register `SETB1R` reader
pub type R = crate::R<SETB1Rrs>;
///Register `SETB1R` writer
pub type W = crate::W<SETB1Rrs>;
/**Software Set trigger

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SST {
    ///0: No effect
    NoEffect = 0,
    ///1: Force output to its active state
    SetActive = 1,
}
impl From<SST> for bool {
    #[inline(always)]
    fn from(variant: SST) -> Self {
        variant as u8 != 0
    }
}
///Field `SST` reader - Software Set trigger
pub type SST_R = crate::BitReader<SST>;
impl SST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SST {
        match self.bits {
            false => SST::NoEffect,
            true => SST::SetActive,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SST::NoEffect
    }
    ///Force output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == SST::SetActive
    }
}
///Field `SST` writer - Software Set trigger
pub type SST_W<'a, REG> = crate::BitWriter<'a, REG, SST>;
impl<'a, REG> SST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SST::NoEffect)
    }
    ///Force output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(SST::SetActive)
    }
}
/**Timer A resynchronizaton

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESYNC {
    ///0: Timer reset event coming solely from software or SYNC input event has no effect
    NoEffect = 0,
    ///1: Timer reset event coming solely from software or SYNC input event forces the output to its active state
    SetActive = 1,
}
impl From<RESYNC> for bool {
    #[inline(always)]
    fn from(variant: RESYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `RESYNC` reader - Timer A resynchronizaton
pub type RESYNC_R = crate::BitReader<RESYNC>;
impl RESYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESYNC {
        match self.bits {
            false => RESYNC::NoEffect,
            true => RESYNC::SetActive,
        }
    }
    ///Timer reset event coming solely from software or SYNC input event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RESYNC::NoEffect
    }
    ///Timer reset event coming solely from software or SYNC input event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == RESYNC::SetActive
    }
}
///Field `RESYNC` writer - Timer A resynchronizaton
pub type RESYNC_W<'a, REG> = crate::BitWriter<'a, REG, RESYNC>;
impl<'a, REG> RESYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer reset event coming solely from software or SYNC input event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RESYNC::NoEffect)
    }
    ///Timer reset event coming solely from software or SYNC input event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(RESYNC::SetActive)
    }
}
/**Timer A Period

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER {
    ///0: Timer period event has no effect
    NoEffect = 0,
    ///1: Timer period event forces the output to its active state
    SetActive = 1,
}
impl From<PER> for bool {
    #[inline(always)]
    fn from(variant: PER) -> Self {
        variant as u8 != 0
    }
}
///Field `PER` reader - Timer A Period
pub type PER_R = crate::BitReader<PER>;
impl PER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PER {
        match self.bits {
            false => PER::NoEffect,
            true => PER::SetActive,
        }
    }
    ///Timer period event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PER::NoEffect
    }
    ///Timer period event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == PER::SetActive
    }
}
///Field `PER` writer - Timer A Period
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG, PER>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer period event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PER::NoEffect)
    }
    ///Timer period event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(PER::SetActive)
    }
}
/**Timer A compare 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1 {
    ///0: Timer compare event has no effect
    NoEffect = 0,
    ///1: Timer compare event forces the output to its active state
    SetActive = 1,
}
impl From<CMP1> for bool {
    #[inline(always)]
    fn from(variant: CMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `CMP1` reader - Timer A compare 1
pub type CMP1_R = crate::BitReader<CMP1>;
impl CMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP1 {
        match self.bits {
            false => CMP1::NoEffect,
            true => CMP1::SetActive,
        }
    }
    ///Timer compare event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CMP1::NoEffect
    }
    ///Timer compare event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == CMP1::SetActive
    }
}
///Field `CMP1` writer - Timer A compare 1
pub type CMP1_W<'a, REG> = crate::BitWriter<'a, REG, CMP1>;
impl<'a, REG> CMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer compare event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1::NoEffect)
    }
    ///Timer compare event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1::SetActive)
    }
}
///Field `CMP2` reader - Timer A compare 2
pub use CMP1_R as CMP2_R;
///Field `CMP3` reader - Timer A compare 3
pub use CMP1_R as CMP3_R;
///Field `CMP4` reader - Timer A compare 4
pub use CMP1_R as CMP4_R;
///Field `CMP2` writer - Timer A compare 2
pub use CMP1_W as CMP2_W;
///Field `CMP3` writer - Timer A compare 3
pub use CMP1_W as CMP3_W;
///Field `CMP4` writer - Timer A compare 4
pub use CMP1_W as CMP4_W;
/**Master Period

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPER {
    ///0: Master timer counter roll-over/reset has no effect
    NoEffect = 0,
    ///1: Master timer counter roll-over/reset forces the output to its active state
    SetActive = 1,
}
impl From<MSTPER> for bool {
    #[inline(always)]
    fn from(variant: MSTPER) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPER` reader - Master Period
pub type MSTPER_R = crate::BitReader<MSTPER>;
impl MSTPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPER {
        match self.bits {
            false => MSTPER::NoEffect,
            true => MSTPER::SetActive,
        }
    }
    ///Master timer counter roll-over/reset has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTPER::NoEffect
    }
    ///Master timer counter roll-over/reset forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == MSTPER::SetActive
    }
}
///Field `MSTPER` writer - Master Period
pub type MSTPER_W<'a, REG> = crate::BitWriter<'a, REG, MSTPER>;
impl<'a, REG> MSTPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer counter roll-over/reset has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPER::NoEffect)
    }
    ///Master timer counter roll-over/reset forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPER::SetActive)
    }
}
/**Master Compare 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1 {
    ///0: Master timer compare event has no effect
    NoEffect = 0,
    ///1: Master timer compare event forces the output to its active state
    SetActive = 1,
}
impl From<MSTCMP1> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTCMP1` reader - Master Compare 1
pub type MSTCMP1_R = crate::BitReader<MSTCMP1>;
impl MSTCMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTCMP1 {
        match self.bits {
            false => MSTCMP1::NoEffect,
            true => MSTCMP1::SetActive,
        }
    }
    ///Master timer compare event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1::NoEffect
    }
    ///Master timer compare event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == MSTCMP1::SetActive
    }
}
///Field `MSTCMP1` writer - Master Compare 1
pub type MSTCMP1_W<'a, REG> = crate::BitWriter<'a, REG, MSTCMP1>;
impl<'a, REG> MSTCMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer compare event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::NoEffect)
    }
    ///Master timer compare event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::SetActive)
    }
}
///Field `MSTCMP2` reader - Master Compare 2
pub use MSTCMP1_R as MSTCMP2_R;
///Field `MSTCMP3` reader - Master Compare 3
pub use MSTCMP1_R as MSTCMP3_R;
///Field `MSTCMP4` reader - Master Compare 4
pub use MSTCMP1_R as MSTCMP4_R;
///Field `MSTCMP2` writer - Master Compare 2
pub use MSTCMP1_W as MSTCMP2_W;
///Field `MSTCMP3` writer - Master Compare 3
pub use MSTCMP1_W as MSTCMP3_W;
///Field `MSTCMP4` writer - Master Compare 4
pub use MSTCMP1_W as MSTCMP4_W;
/**Timer Event 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEVNT1 {
    ///0: Timer event has no effect
    NoEffect = 0,
    ///1: Timer event forces the output to its active state
    SetActive = 1,
}
impl From<TIMEVNT1> for bool {
    #[inline(always)]
    fn from(variant: TIMEVNT1) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMEVNT1` reader - Timer Event 1
pub type TIMEVNT1_R = crate::BitReader<TIMEVNT1>;
impl TIMEVNT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMEVNT1 {
        match self.bits {
            false => TIMEVNT1::NoEffect,
            true => TIMEVNT1::SetActive,
        }
    }
    ///Timer event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMEVNT1::NoEffect
    }
    ///Timer event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == TIMEVNT1::SetActive
    }
}
///Field `TIMEVNT1` writer - Timer Event 1
pub type TIMEVNT1_W<'a, REG> = crate::BitWriter<'a, REG, TIMEVNT1>;
impl<'a, REG> TIMEVNT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEVNT1::NoEffect)
    }
    ///Timer event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEVNT1::SetActive)
    }
}
///Field `TIMEVNT2` reader - Timer Event 2
pub use TIMEVNT1_R as TIMEVNT2_R;
///Field `TIMEVNT3` reader - Timer Event 3
pub use TIMEVNT1_R as TIMEVNT3_R;
///Field `TIMEVNT4` reader - Timer Event 4
pub use TIMEVNT1_R as TIMEVNT4_R;
///Field `TIMEVNT5` reader - Timer Event 5
pub use TIMEVNT1_R as TIMEVNT5_R;
///Field `TIMEVNT6` reader - Timer Event 6
pub use TIMEVNT1_R as TIMEVNT6_R;
///Field `TIMEVNT7` reader - Timer Event 7
pub use TIMEVNT1_R as TIMEVNT7_R;
///Field `TIMEVNT8` reader - Timer Event 8
pub use TIMEVNT1_R as TIMEVNT8_R;
///Field `TIMEVNT9` reader - Timer Event 9
pub use TIMEVNT1_R as TIMEVNT9_R;
///Field `TIMEVNT2` writer - Timer Event 2
pub use TIMEVNT1_W as TIMEVNT2_W;
///Field `TIMEVNT3` writer - Timer Event 3
pub use TIMEVNT1_W as TIMEVNT3_W;
///Field `TIMEVNT4` writer - Timer Event 4
pub use TIMEVNT1_W as TIMEVNT4_W;
///Field `TIMEVNT5` writer - Timer Event 5
pub use TIMEVNT1_W as TIMEVNT5_W;
///Field `TIMEVNT6` writer - Timer Event 6
pub use TIMEVNT1_W as TIMEVNT6_W;
///Field `TIMEVNT7` writer - Timer Event 7
pub use TIMEVNT1_W as TIMEVNT7_W;
///Field `TIMEVNT8` writer - Timer Event 8
pub use TIMEVNT1_W as TIMEVNT8_W;
///Field `TIMEVNT9` writer - Timer Event 9
pub use TIMEVNT1_W as TIMEVNT9_W;
/**External Event 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEVNT1 {
    ///0: External event has no effect
    NoEffect = 0,
    ///1: External event forces the output to its active state
    SetActive = 1,
}
impl From<EXTEVNT1> for bool {
    #[inline(always)]
    fn from(variant: EXTEVNT1) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTEVNT1` reader - External Event 1
pub type EXTEVNT1_R = crate::BitReader<EXTEVNT1>;
impl EXTEVNT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTEVNT1 {
        match self.bits {
            false => EXTEVNT1::NoEffect,
            true => EXTEVNT1::SetActive,
        }
    }
    ///External event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXTEVNT1::NoEffect
    }
    ///External event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == EXTEVNT1::SetActive
    }
}
///Field `EXTEVNT1` writer - External Event 1
pub type EXTEVNT1_W<'a, REG> = crate::BitWriter<'a, REG, EXTEVNT1>;
impl<'a, REG> EXTEVNT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEVNT1::NoEffect)
    }
    ///External event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEVNT1::SetActive)
    }
}
///Field `EXTEVNT2` reader - External Event 2
pub use EXTEVNT1_R as EXTEVNT2_R;
///Field `EXTEVNT3` reader - External Event 3
pub use EXTEVNT1_R as EXTEVNT3_R;
///Field `EXTEVNT4` reader - External Event 4
pub use EXTEVNT1_R as EXTEVNT4_R;
///Field `EXTEVNT5` reader - External Event 5
pub use EXTEVNT1_R as EXTEVNT5_R;
///Field `EXTEVNT6` reader - External Event 6
pub use EXTEVNT1_R as EXTEVNT6_R;
///Field `EXTEVNT7` reader - External Event 7
pub use EXTEVNT1_R as EXTEVNT7_R;
///Field `EXTEVNT8` reader - External Event 8
pub use EXTEVNT1_R as EXTEVNT8_R;
///Field `EXTEVNT9` reader - External Event 9
pub use EXTEVNT1_R as EXTEVNT9_R;
///Field `EXTEVNT10` reader - External Event 10
pub use EXTEVNT1_R as EXTEVNT10_R;
///Field `EXTEVNT2` writer - External Event 2
pub use EXTEVNT1_W as EXTEVNT2_W;
///Field `EXTEVNT3` writer - External Event 3
pub use EXTEVNT1_W as EXTEVNT3_W;
///Field `EXTEVNT4` writer - External Event 4
pub use EXTEVNT1_W as EXTEVNT4_W;
///Field `EXTEVNT5` writer - External Event 5
pub use EXTEVNT1_W as EXTEVNT5_W;
///Field `EXTEVNT6` writer - External Event 6
pub use EXTEVNT1_W as EXTEVNT6_W;
///Field `EXTEVNT7` writer - External Event 7
pub use EXTEVNT1_W as EXTEVNT7_W;
///Field `EXTEVNT8` writer - External Event 8
pub use EXTEVNT1_W as EXTEVNT8_W;
///Field `EXTEVNT9` writer - External Event 9
pub use EXTEVNT1_W as EXTEVNT9_W;
///Field `EXTEVNT10` writer - External Event 10
pub use EXTEVNT1_W as EXTEVNT10_W;
/**Registers update (transfer preload to active)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDATE {
    ///0: Register update event has no effect
    NoEffect = 0,
    ///1: Register update event forces the output to its active state
    SetActive = 1,
}
impl From<UPDATE> for bool {
    #[inline(always)]
    fn from(variant: UPDATE) -> Self {
        variant as u8 != 0
    }
}
///Field `UPDATE` reader - Registers update (transfer preload to active)
pub type UPDATE_R = crate::BitReader<UPDATE>;
impl UPDATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPDATE {
        match self.bits {
            false => UPDATE::NoEffect,
            true => UPDATE::SetActive,
        }
    }
    ///Register update event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDATE::NoEffect
    }
    ///Register update event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == UPDATE::SetActive
    }
}
///Field `UPDATE` writer - Registers update (transfer preload to active)
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG, UPDATE>;
impl<'a, REG> UPDATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Register update event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UPDATE::NoEffect)
    }
    ///Register update event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(UPDATE::SetActive)
    }
}
impl R {
    ///Bit 0 - Software Set trigger
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A resynchronizaton
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer A Period
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timer Event 1
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timer Event 2
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Timer Event 3
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timer Event 4
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timer Event 5
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Timer Event 6
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer Event 7
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer Event 8
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer Event 9
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Registers update (transfer preload to active)
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETB1R")
            .field("update", &self.update())
            .field("extevnt1", &self.extevnt1())
            .field("extevnt10", &self.extevnt10())
            .field("extevnt9", &self.extevnt9())
            .field("extevnt8", &self.extevnt8())
            .field("extevnt7", &self.extevnt7())
            .field("extevnt6", &self.extevnt6())
            .field("extevnt5", &self.extevnt5())
            .field("extevnt4", &self.extevnt4())
            .field("extevnt3", &self.extevnt3())
            .field("extevnt2", &self.extevnt2())
            .field("timevnt1", &self.timevnt1())
            .field("timevnt9", &self.timevnt9())
            .field("timevnt8", &self.timevnt8())
            .field("timevnt7", &self.timevnt7())
            .field("timevnt6", &self.timevnt6())
            .field("timevnt5", &self.timevnt5())
            .field("timevnt4", &self.timevnt4())
            .field("timevnt3", &self.timevnt3())
            .field("timevnt2", &self.timevnt2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstcmp4", &self.mstcmp4())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstper", &self.mstper())
            .field("cmp1", &self.cmp1())
            .field("cmp4", &self.cmp4())
            .field("cmp3", &self.cmp3())
            .field("cmp2", &self.cmp2())
            .field("per", &self.per())
            .field("resync", &self.resync())
            .field("sst", &self.sst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Set trigger
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<SETB1Rrs> {
        SST_W::new(self, 0)
    }
    ///Bit 1 - Timer A resynchronizaton
    #[inline(always)]
    #[must_use]
    pub fn resync(&mut self) -> RESYNC_W<SETB1Rrs> {
        RESYNC_W::new(self, 1)
    }
    ///Bit 2 - Timer A Period
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<SETB1Rrs> {
        PER_W::new(self, 2)
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<SETB1Rrs> {
        CMP1_W::new(self, 3)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<SETB1Rrs> {
        CMP2_W::new(self, 4)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> CMP3_W<SETB1Rrs> {
        CMP3_W::new(self, 5)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<SETB1Rrs> {
        CMP4_W::new(self, 6)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<SETB1Rrs> {
        MSTPER_W::new(self, 7)
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<SETB1Rrs> {
        MSTCMP1_W::new(self, 8)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<SETB1Rrs> {
        MSTCMP2_W::new(self, 9)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<SETB1Rrs> {
        MSTCMP3_W::new(self, 10)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<SETB1Rrs> {
        MSTCMP4_W::new(self, 11)
    }
    ///Bit 12 - Timer Event 1
    #[inline(always)]
    #[must_use]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W<SETB1Rrs> {
        TIMEVNT1_W::new(self, 12)
    }
    ///Bit 13 - Timer Event 2
    #[inline(always)]
    #[must_use]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W<SETB1Rrs> {
        TIMEVNT2_W::new(self, 13)
    }
    ///Bit 14 - Timer Event 3
    #[inline(always)]
    #[must_use]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W<SETB1Rrs> {
        TIMEVNT3_W::new(self, 14)
    }
    ///Bit 15 - Timer Event 4
    #[inline(always)]
    #[must_use]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W<SETB1Rrs> {
        TIMEVNT4_W::new(self, 15)
    }
    ///Bit 16 - Timer Event 5
    #[inline(always)]
    #[must_use]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W<SETB1Rrs> {
        TIMEVNT5_W::new(self, 16)
    }
    ///Bit 17 - Timer Event 6
    #[inline(always)]
    #[must_use]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W<SETB1Rrs> {
        TIMEVNT6_W::new(self, 17)
    }
    ///Bit 18 - Timer Event 7
    #[inline(always)]
    #[must_use]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W<SETB1Rrs> {
        TIMEVNT7_W::new(self, 18)
    }
    ///Bit 19 - Timer Event 8
    #[inline(always)]
    #[must_use]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W<SETB1Rrs> {
        TIMEVNT8_W::new(self, 19)
    }
    ///Bit 20 - Timer Event 9
    #[inline(always)]
    #[must_use]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W<SETB1Rrs> {
        TIMEVNT9_W::new(self, 20)
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<SETB1Rrs> {
        EXTEVNT1_W::new(self, 21)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<SETB1Rrs> {
        EXTEVNT2_W::new(self, 22)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<SETB1Rrs> {
        EXTEVNT3_W::new(self, 23)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<SETB1Rrs> {
        EXTEVNT4_W::new(self, 24)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<SETB1Rrs> {
        EXTEVNT5_W::new(self, 25)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<SETB1Rrs> {
        EXTEVNT6_W::new(self, 26)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<SETB1Rrs> {
        EXTEVNT7_W::new(self, 27)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<SETB1Rrs> {
        EXTEVNT8_W::new(self, 28)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<SETB1Rrs> {
        EXTEVNT9_W::new(self, 29)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<SETB1Rrs> {
        EXTEVNT10_W::new(self, 30)
    }
    ///Bit 31 - Registers update (transfer preload to active)
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<SETB1Rrs> {
        UPDATE_W::new(self, 31)
    }
}
/**Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`setb1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setb1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMB:SETB1R)*/
pub struct SETB1Rrs;
impl crate::RegisterSpec for SETB1Rrs {
    type Ux = u32;
}
///`read()` method returns [`setb1r::R`](R) reader structure
impl crate::Readable for SETB1Rrs {}
///`write(|w| ..)` method takes [`setb1r::W`](W) writer structure
impl crate::Writable for SETB1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SETB1R to value 0
impl crate::Resettable for SETB1Rrs {
    const RESET_VALUE: u32 = 0;
}
