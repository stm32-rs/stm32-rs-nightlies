///Register `SET1R` reader
pub type R = crate::R<SET1Rrs>;
///Register `SET1R` writer
pub type W = crate::W<SET1Rrs>;
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
/**Timer A compare %s

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
///Field `CMP(1-4)` reader - Timer A compare %s
pub type CMP_R = crate::BitReader<CMP1>;
impl CMP_R {
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
///Field `CMP(1-4)` writer - Timer A compare %s
pub type CMP_W<'a, REG> = crate::BitWriter<'a, REG, CMP1>;
impl<'a, REG> CMP_W<'a, REG>
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
/**Master Compare %s

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
///Field `MSTCMP(1-4)` reader - Master Compare %s
pub type MSTCMP_R = crate::BitReader<MSTCMP1>;
impl MSTCMP_R {
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
///Field `MSTCMP(1-4)` writer - Master Compare %s
pub type MSTCMP_W<'a, REG> = crate::BitWriter<'a, REG, MSTCMP1>;
impl<'a, REG> MSTCMP_W<'a, REG>
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
/**Timer B Compare 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMBCMP1 {
    ///0: Timer event has no effect
    NoEffect = 0,
    ///1: Timer event forces the output to its active state
    SetActive = 1,
}
impl From<TIMBCMP1> for bool {
    #[inline(always)]
    fn from(variant: TIMBCMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMBCMP1` reader - Timer B Compare 1
pub type TIMBCMP1_R = crate::BitReader<TIMBCMP1>;
impl TIMBCMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMBCMP1 {
        match self.bits {
            false => TIMBCMP1::NoEffect,
            true => TIMBCMP1::SetActive,
        }
    }
    ///Timer event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMBCMP1::NoEffect
    }
    ///Timer event forces the output to its active state
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == TIMBCMP1::SetActive
    }
}
///Field `TIMBCMP1` writer - Timer B Compare 1
pub type TIMBCMP1_W<'a, REG> = crate::BitWriter<'a, REG, TIMBCMP1>;
impl<'a, REG> TIMBCMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TIMBCMP1::NoEffect)
    }
    ///Timer event forces the output to its active state
    #[inline(always)]
    pub fn set_active(self) -> &'a mut crate::W<REG> {
        self.variant(TIMBCMP1::SetActive)
    }
}
///Field `TIMBCMP2` reader - Timer B Compare 2
pub use TIMBCMP1_R as TIMBCMP2_R;
///Field `TIMBCMP4` reader - Timer B Compare 4
pub use TIMBCMP1_R as TIMBCMP4_R;
///Field `TIMCCMP2` reader - Timer C Compare 2
pub use TIMBCMP1_R as TIMCCMP2_R;
///Field `TIMCCMP3` reader - Timer C Compare 3
pub use TIMBCMP1_R as TIMCCMP3_R;
///Field `TIMDCMP1` reader - Timer D Compare 1
pub use TIMBCMP1_R as TIMDCMP1_R;
///Field `TIMDCMP2` reader - Timer D Compare 2
pub use TIMBCMP1_R as TIMDCMP2_R;
///Field `TIMECMP3` reader - Timer E Compare 3
pub use TIMBCMP1_R as TIMECMP3_R;
///Field `TIMECMP4` reader - Timer E Compare 4
pub use TIMBCMP1_R as TIMECMP4_R;
///Field `TIMBCMP2` writer - Timer B Compare 2
pub use TIMBCMP1_W as TIMBCMP2_W;
///Field `TIMBCMP4` writer - Timer B Compare 4
pub use TIMBCMP1_W as TIMBCMP4_W;
///Field `TIMCCMP2` writer - Timer C Compare 2
pub use TIMBCMP1_W as TIMCCMP2_W;
///Field `TIMCCMP3` writer - Timer C Compare 3
pub use TIMBCMP1_W as TIMCCMP3_W;
///Field `TIMDCMP1` writer - Timer D Compare 1
pub use TIMBCMP1_W as TIMDCMP1_W;
///Field `TIMDCMP2` writer - Timer D Compare 2
pub use TIMBCMP1_W as TIMDCMP2_W;
///Field `TIMECMP3` writer - Timer E Compare 3
pub use TIMBCMP1_W as TIMECMP3_W;
///Field `TIMECMP4` writer - Timer E Compare 4
pub use TIMBCMP1_W as TIMECMP4_W;
/**External Event %s

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
///Field `EXTEVNT(1-10)` reader - External Event %s
pub type EXTEVNT_R = crate::BitReader<EXTEVNT1>;
impl EXTEVNT_R {
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
///Field `EXTEVNT(1-10)` writer - External Event %s
pub type EXTEVNT_W<'a, REG> = crate::BitWriter<'a, REG, EXTEVNT1>;
impl<'a, REG> EXTEVNT_W<'a, REG>
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
    ///Timer A compare (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1` field.</div>
    #[inline(always)]
    pub fn cmp(&self, n: u8) -> CMP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMP_R::new(((self.bits >> (n + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer A compare (1-4)
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = CMP_R> + '_ {
        (0..4).map(move |n| CMP_R::new(((self.bits >> (n + 3)) & 1) != 0))
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    pub fn cmp1(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    pub fn cmp2(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    pub fn cmp3(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    pub fn cmp4(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Master Compare (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MSTCMP1` field.</div>
    #[inline(always)]
    pub fn mstcmp(&self, n: u8) -> MSTCMP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        MSTCMP_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Master Compare (1-4)
    #[inline(always)]
    pub fn mstcmp_iter(&self) -> impl Iterator<Item = MSTCMP_R> + '_ {
        (0..4).map(move |n| MSTCMP_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP_R {
        MSTCMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP_R {
        MSTCMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP_R {
        MSTCMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP_R {
        MSTCMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timer B Compare 1
    #[inline(always)]
    pub fn timbcmp1(&self) -> TIMBCMP1_R {
        TIMBCMP1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timer B Compare 2
    #[inline(always)]
    pub fn timbcmp2(&self) -> TIMBCMP2_R {
        TIMBCMP2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Timer B Compare 4
    #[inline(always)]
    pub fn timbcmp4(&self) -> TIMBCMP4_R {
        TIMBCMP4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timer C Compare 2
    #[inline(always)]
    pub fn timccmp2(&self) -> TIMCCMP2_R {
        TIMCCMP2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timer C Compare 3
    #[inline(always)]
    pub fn timccmp3(&self) -> TIMCCMP3_R {
        TIMCCMP3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Timer D Compare 1
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer D Compare 2
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer E Compare 3
    #[inline(always)]
    pub fn timecmp3(&self) -> TIMECMP3_R {
        TIMECMP3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer E Compare 4
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///External Event (1-10)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EXTEVNT1` field.</div>
    #[inline(always)]
    pub fn extevnt(&self, n: u8) -> EXTEVNT_R {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        EXTEVNT_R::new(((self.bits >> (n + 21)) & 1) != 0)
    }
    ///Iterator for array of:
    ///External Event (1-10)
    #[inline(always)]
    pub fn extevnt_iter(&self) -> impl Iterator<Item = EXTEVNT_R> + '_ {
        (0..10).map(move |n| EXTEVNT_R::new(((self.bits >> (n + 21)) & 1) != 0))
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT_R {
        EXTEVNT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Registers update (transfer preload to active)
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET1R")
            .field("update", &self.update())
            .field("extevnt1", &self.extevnt1())
            .field("extevnt2", &self.extevnt2())
            .field("extevnt3", &self.extevnt3())
            .field("extevnt4", &self.extevnt4())
            .field("extevnt5", &self.extevnt5())
            .field("extevnt6", &self.extevnt6())
            .field("extevnt7", &self.extevnt7())
            .field("extevnt8", &self.extevnt8())
            .field("extevnt9", &self.extevnt9())
            .field("extevnt10", &self.extevnt10())
            .field("timbcmp1", &self.timbcmp1())
            .field("timecmp4", &self.timecmp4())
            .field("timecmp3", &self.timecmp3())
            .field("timdcmp2", &self.timdcmp2())
            .field("timdcmp1", &self.timdcmp1())
            .field("timccmp3", &self.timccmp3())
            .field("timccmp2", &self.timccmp2())
            .field("timbcmp4", &self.timbcmp4())
            .field("timbcmp2", &self.timbcmp2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp4", &self.mstcmp4())
            .field("mstper", &self.mstper())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("cmp3", &self.cmp3())
            .field("cmp4", &self.cmp4())
            .field("per", &self.per())
            .field("resync", &self.resync())
            .field("sst", &self.sst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Set trigger
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W<SET1Rrs> {
        SST_W::new(self, 0)
    }
    ///Bit 1 - Timer A resynchronizaton
    #[inline(always)]
    pub fn resync(&mut self) -> RESYNC_W<SET1Rrs> {
        RESYNC_W::new(self, 1)
    }
    ///Bit 2 - Timer A Period
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<SET1Rrs> {
        PER_W::new(self, 2)
    }
    ///Timer A compare (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1` field.</div>
    #[inline(always)]
    pub fn cmp(&mut self, n: u8) -> CMP_W<SET1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMP_W::new(self, n + 3)
    }
    ///Bit 3 - Timer A compare 1
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP_W<SET1Rrs> {
        CMP_W::new(self, 3)
    }
    ///Bit 4 - Timer A compare 2
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP_W<SET1Rrs> {
        CMP_W::new(self, 4)
    }
    ///Bit 5 - Timer A compare 3
    #[inline(always)]
    pub fn cmp3(&mut self) -> CMP_W<SET1Rrs> {
        CMP_W::new(self, 5)
    }
    ///Bit 6 - Timer A compare 4
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP_W<SET1Rrs> {
        CMP_W::new(self, 6)
    }
    ///Bit 7 - Master Period
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W<SET1Rrs> {
        MSTPER_W::new(self, 7)
    }
    ///Master Compare (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MSTCMP1` field.</div>
    #[inline(always)]
    pub fn mstcmp(&mut self, n: u8) -> MSTCMP_W<SET1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        MSTCMP_W::new(self, n + 8)
    }
    ///Bit 8 - Master Compare 1
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP_W<SET1Rrs> {
        MSTCMP_W::new(self, 8)
    }
    ///Bit 9 - Master Compare 2
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP_W<SET1Rrs> {
        MSTCMP_W::new(self, 9)
    }
    ///Bit 10 - Master Compare 3
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP_W<SET1Rrs> {
        MSTCMP_W::new(self, 10)
    }
    ///Bit 11 - Master Compare 4
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP_W<SET1Rrs> {
        MSTCMP_W::new(self, 11)
    }
    ///Bit 12 - Timer B Compare 1
    #[inline(always)]
    pub fn timbcmp1(&mut self) -> TIMBCMP1_W<SET1Rrs> {
        TIMBCMP1_W::new(self, 12)
    }
    ///Bit 13 - Timer B Compare 2
    #[inline(always)]
    pub fn timbcmp2(&mut self) -> TIMBCMP2_W<SET1Rrs> {
        TIMBCMP2_W::new(self, 13)
    }
    ///Bit 14 - Timer B Compare 4
    #[inline(always)]
    pub fn timbcmp4(&mut self) -> TIMBCMP4_W<SET1Rrs> {
        TIMBCMP4_W::new(self, 14)
    }
    ///Bit 15 - Timer C Compare 2
    #[inline(always)]
    pub fn timccmp2(&mut self) -> TIMCCMP2_W<SET1Rrs> {
        TIMCCMP2_W::new(self, 15)
    }
    ///Bit 16 - Timer C Compare 3
    #[inline(always)]
    pub fn timccmp3(&mut self) -> TIMCCMP3_W<SET1Rrs> {
        TIMCCMP3_W::new(self, 16)
    }
    ///Bit 17 - Timer D Compare 1
    #[inline(always)]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W<SET1Rrs> {
        TIMDCMP1_W::new(self, 17)
    }
    ///Bit 18 - Timer D Compare 2
    #[inline(always)]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W<SET1Rrs> {
        TIMDCMP2_W::new(self, 18)
    }
    ///Bit 19 - Timer E Compare 3
    #[inline(always)]
    pub fn timecmp3(&mut self) -> TIMECMP3_W<SET1Rrs> {
        TIMECMP3_W::new(self, 19)
    }
    ///Bit 20 - Timer E Compare 4
    #[inline(always)]
    pub fn timecmp4(&mut self) -> TIMECMP4_W<SET1Rrs> {
        TIMECMP4_W::new(self, 20)
    }
    ///External Event (1-10)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EXTEVNT1` field.</div>
    #[inline(always)]
    pub fn extevnt(&mut self, n: u8) -> EXTEVNT_W<SET1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        EXTEVNT_W::new(self, n + 21)
    }
    ///Bit 21 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 21)
    }
    ///Bit 22 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 22)
    }
    ///Bit 23 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 23)
    }
    ///Bit 24 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 24)
    }
    ///Bit 25 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 25)
    }
    ///Bit 26 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 26)
    }
    ///Bit 27 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 27)
    }
    ///Bit 28 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 28)
    }
    ///Bit 29 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 29)
    }
    ///Bit 30 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT_W<SET1Rrs> {
        EXTEVNT_W::new(self, 30)
    }
    ///Bit 31 - Registers update (transfer preload to active)
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<SET1Rrs> {
        UPDATE_W::new(self, 31)
    }
}
/**Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`set1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HRTIM_TIMA:SET1R)*/
pub struct SET1Rrs;
impl crate::RegisterSpec for SET1Rrs {
    type Ux = u32;
}
///`read()` method returns [`set1r::R`](R) reader structure
impl crate::Readable for SET1Rrs {}
///`write(|w| ..)` method takes [`set1r::W`](W) writer structure
impl crate::Writable for SET1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SET1R to value 0
impl crate::Resettable for SET1Rrs {}
