///Register `CPT1CR` reader
pub type R = crate::R<CPT1CRrs>;
///Register `CPT1CR` writer
pub type W = crate::W<CPT1CRrs>;
/**Software Capture

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWCPT {
    ///0: No effect
    NoEffect = 0,
    ///1: Force capture Z
    TriggerCapture = 1,
}
impl From<SWCPT> for bool {
    #[inline(always)]
    fn from(variant: SWCPT) -> Self {
        variant as u8 != 0
    }
}
///Field `SWCPT` reader - Software Capture
pub type SWCPT_R = crate::BitReader<SWCPT>;
impl SWCPT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWCPT {
        match self.bits {
            false => SWCPT::NoEffect,
            true => SWCPT::TriggerCapture,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SWCPT::NoEffect
    }
    ///Force capture Z
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == SWCPT::TriggerCapture
    }
}
///Field `SWCPT` writer - Software Capture
pub type SWCPT_W<'a, REG> = crate::BitWriter<'a, REG, SWCPT>;
impl<'a, REG> SWCPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SWCPT::NoEffect)
    }
    ///Force capture Z
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(SWCPT::TriggerCapture)
    }
}
/**Update Capture

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDCPT {
    ///0: Update event has no effect
    NoEffect = 0,
    ///1: Update event triggers capture Z
    TriggerCapture = 1,
}
impl From<UPDCPT> for bool {
    #[inline(always)]
    fn from(variant: UPDCPT) -> Self {
        variant as u8 != 0
    }
}
///Field `UPDCPT` reader - Update Capture
pub type UPDCPT_R = crate::BitReader<UPDCPT>;
impl UPDCPT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPDCPT {
        match self.bits {
            false => UPDCPT::NoEffect,
            true => UPDCPT::TriggerCapture,
        }
    }
    ///Update event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDCPT::NoEffect
    }
    ///Update event triggers capture Z
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == UPDCPT::TriggerCapture
    }
}
///Field `UPDCPT` writer - Update Capture
pub type UPDCPT_W<'a, REG> = crate::BitWriter<'a, REG, UPDCPT>;
impl<'a, REG> UPDCPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UPDCPT::NoEffect)
    }
    ///Update event triggers capture Z
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(UPDCPT::TriggerCapture)
    }
}
/**External Event %s Capture

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXEV1CPT {
    ///0: External event Y has no effect
    NoEffect = 0,
    ///1: External event Y triggers capture Z
    TriggerCapture = 1,
}
impl From<EXEV1CPT> for bool {
    #[inline(always)]
    fn from(variant: EXEV1CPT) -> Self {
        variant as u8 != 0
    }
}
///Field `EXEVCPT(1-10)` reader - External Event %s Capture
pub type EXEVCPT_R = crate::BitReader<EXEV1CPT>;
impl EXEVCPT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXEV1CPT {
        match self.bits {
            false => EXEV1CPT::NoEffect,
            true => EXEV1CPT::TriggerCapture,
        }
    }
    ///External event Y has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXEV1CPT::NoEffect
    }
    ///External event Y triggers capture Z
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == EXEV1CPT::TriggerCapture
    }
}
///Field `EXEVCPT(1-10)` writer - External Event %s Capture
pub type EXEVCPT_W<'a, REG> = crate::BitWriter<'a, REG, EXEV1CPT>;
impl<'a, REG> EXEVCPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External event Y has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EXEV1CPT::NoEffect)
    }
    ///External event Y triggers capture Z
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(EXEV1CPT::TriggerCapture)
    }
}
/**Timer B output 1 Set

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TB1SET {
    ///0: Timer X output Y inactive to active transition has no effect
    NoEffect = 0,
    ///1: Timer X output Y inactive to active transition triggers capture Z
    TriggerCapture = 1,
}
impl From<TB1SET> for bool {
    #[inline(always)]
    fn from(variant: TB1SET) -> Self {
        variant as u8 != 0
    }
}
///Field `TB1SET` reader - Timer B output 1 Set
pub type TB1SET_R = crate::BitReader<TB1SET>;
impl TB1SET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TB1SET {
        match self.bits {
            false => TB1SET::NoEffect,
            true => TB1SET::TriggerCapture,
        }
    }
    ///Timer X output Y inactive to active transition has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TB1SET::NoEffect
    }
    ///Timer X output Y inactive to active transition triggers capture Z
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TB1SET::TriggerCapture
    }
}
///Field `TB1SET` writer - Timer B output 1 Set
pub type TB1SET_W<'a, REG> = crate::BitWriter<'a, REG, TB1SET>;
impl<'a, REG> TB1SET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer X output Y inactive to active transition has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TB1SET::NoEffect)
    }
    ///Timer X output Y inactive to active transition triggers capture Z
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(TB1SET::TriggerCapture)
    }
}
/**Timer B output 1 Reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TB1RST {
    ///0: Timer X output Y active to inactive transition has no effect
    NoEffect = 0,
    ///1: Timer X output Y active to inactive transition triggers capture Z
    TriggerCapture = 1,
}
impl From<TB1RST> for bool {
    #[inline(always)]
    fn from(variant: TB1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TB1RST` reader - Timer B output 1 Reset
pub type TB1RST_R = crate::BitReader<TB1RST>;
impl TB1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TB1RST {
        match self.bits {
            false => TB1RST::NoEffect,
            true => TB1RST::TriggerCapture,
        }
    }
    ///Timer X output Y active to inactive transition has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TB1RST::NoEffect
    }
    ///Timer X output Y active to inactive transition triggers capture Z
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TB1RST::TriggerCapture
    }
}
///Field `TB1RST` writer - Timer B output 1 Reset
pub type TB1RST_W<'a, REG> = crate::BitWriter<'a, REG, TB1RST>;
impl<'a, REG> TB1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer X output Y active to inactive transition has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TB1RST::NoEffect)
    }
    ///Timer X output Y active to inactive transition triggers capture Z
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(TB1RST::TriggerCapture)
    }
}
/**Timer B Compare 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCMP1 {
    ///0: Timer X compare Y has no effect
    NoEffect = 0,
    ///1: Timer X compare Y triggers capture Z
    TriggerCapture = 1,
}
impl From<TBCMP1> for bool {
    #[inline(always)]
    fn from(variant: TBCMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `TBCMP1` reader - Timer B Compare 1
pub type TBCMP1_R = crate::BitReader<TBCMP1>;
impl TBCMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TBCMP1 {
        match self.bits {
            false => TBCMP1::NoEffect,
            true => TBCMP1::TriggerCapture,
        }
    }
    ///Timer X compare Y has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TBCMP1::NoEffect
    }
    ///Timer X compare Y triggers capture Z
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TBCMP1::TriggerCapture
    }
}
///Field `TBCMP1` writer - Timer B Compare 1
pub type TBCMP1_W<'a, REG> = crate::BitWriter<'a, REG, TBCMP1>;
impl<'a, REG> TBCMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer X compare Y has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TBCMP1::NoEffect)
    }
    ///Timer X compare Y triggers capture Z
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(TBCMP1::TriggerCapture)
    }
}
///Field `TC1RST` reader - Timer C output 1 Reset
pub use TB1RST_R as TC1RST_R;
///Field `TD1RST` reader - Timer D output 1 Reset
pub use TB1RST_R as TD1RST_R;
///Field `TE1RST` reader - Timer E output 1 Reset
pub use TB1RST_R as TE1RST_R;
///Field `TC1RST` writer - Timer C output 1 Reset
pub use TB1RST_W as TC1RST_W;
///Field `TD1RST` writer - Timer D output 1 Reset
pub use TB1RST_W as TD1RST_W;
///Field `TE1RST` writer - Timer E output 1 Reset
pub use TB1RST_W as TE1RST_W;
///Field `TC1SET` reader - Timer C output 1 Set
pub use TB1SET_R as TC1SET_R;
///Field `TD1SET` reader - Timer D output 1 Set
pub use TB1SET_R as TD1SET_R;
///Field `TE1SET` reader - Timer E output 1 Set
pub use TB1SET_R as TE1SET_R;
///Field `TC1SET` writer - Timer C output 1 Set
pub use TB1SET_W as TC1SET_W;
///Field `TD1SET` writer - Timer D output 1 Set
pub use TB1SET_W as TD1SET_W;
///Field `TE1SET` writer - Timer E output 1 Set
pub use TB1SET_W as TE1SET_W;
///Field `TBCMP2` reader - Timer B Compare 2
pub use TBCMP1_R as TBCMP2_R;
///Field `TCCMP1` reader - Timer C Compare 1
pub use TBCMP1_R as TCCMP1_R;
///Field `TCCMP2` reader - Timer C Compare 2
pub use TBCMP1_R as TCCMP2_R;
///Field `TDCMP1` reader - Timer D Compare 1
pub use TBCMP1_R as TDCMP1_R;
///Field `TDCMP2` reader - Timer D Compare 2
pub use TBCMP1_R as TDCMP2_R;
///Field `TECMP1` reader - Timer E Compare 1
pub use TBCMP1_R as TECMP1_R;
///Field `TECMP2` reader - Timer E Compare 2
pub use TBCMP1_R as TECMP2_R;
///Field `TBCMP2` writer - Timer B Compare 2
pub use TBCMP1_W as TBCMP2_W;
///Field `TCCMP1` writer - Timer C Compare 1
pub use TBCMP1_W as TCCMP1_W;
///Field `TCCMP2` writer - Timer C Compare 2
pub use TBCMP1_W as TCCMP2_W;
///Field `TDCMP1` writer - Timer D Compare 1
pub use TBCMP1_W as TDCMP1_W;
///Field `TDCMP2` writer - Timer D Compare 2
pub use TBCMP1_W as TDCMP2_W;
///Field `TECMP1` writer - Timer E Compare 1
pub use TBCMP1_W as TECMP1_W;
///Field `TECMP2` writer - Timer E Compare 2
pub use TBCMP1_W as TECMP2_W;
impl R {
    ///Bit 0 - Software Capture
    #[inline(always)]
    pub fn swcpt(&self) -> SWCPT_R {
        SWCPT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Update Capture
    #[inline(always)]
    pub fn updcpt(&self) -> UPDCPT_R {
        UPDCPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///External Event (1-10) Capture
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EXEV1CPT` field.</div>
    #[inline(always)]
    pub fn exevcpt(&self, n: u8) -> EXEVCPT_R {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        EXEVCPT_R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///External Event (1-10) Capture
    #[inline(always)]
    pub fn exevcpt_iter(&self) -> impl Iterator<Item = EXEVCPT_R> + '_ {
        (0..10).map(move |n| EXEVCPT_R::new(((self.bits >> (n + 2)) & 1) != 0))
    }
    ///Bit 2 - External Event 1 Capture
    #[inline(always)]
    pub fn exev1cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - External Event 2 Capture
    #[inline(always)]
    pub fn exev2cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - External Event 3 Capture
    #[inline(always)]
    pub fn exev3cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - External Event 4 Capture
    #[inline(always)]
    pub fn exev4cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - External Event 5 Capture
    #[inline(always)]
    pub fn exev5cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - External Event 6 Capture
    #[inline(always)]
    pub fn exev6cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - External Event 7 Capture
    #[inline(always)]
    pub fn exev7cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - External Event 8 Capture
    #[inline(always)]
    pub fn exev8cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - External Event 9 Capture
    #[inline(always)]
    pub fn exev9cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - External Event 10 Capture
    #[inline(always)]
    pub fn exev10cpt(&self) -> EXEVCPT_R {
        EXEVCPT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Timer B output 1 Set
    #[inline(always)]
    pub fn tb1set(&self) -> TB1SET_R {
        TB1SET_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Timer B output 1 Reset
    #[inline(always)]
    pub fn tb1rst(&self) -> TB1RST_R {
        TB1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer B Compare 1
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer B Compare 2
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer C output 1 Set
    #[inline(always)]
    pub fn tc1set(&self) -> TC1SET_R {
        TC1SET_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer C output 1 Reset
    #[inline(always)]
    pub fn tc1rst(&self) -> TC1RST_R {
        TC1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Timer C Compare 1
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Timer C Compare 2
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Timer D output 1 Set
    #[inline(always)]
    pub fn td1set(&self) -> TD1SET_R {
        TD1SET_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Timer D output 1 Reset
    #[inline(always)]
    pub fn td1rst(&self) -> TD1RST_R {
        TD1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Timer D Compare 1
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Timer D Compare 2
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Timer E output 1 Set
    #[inline(always)]
    pub fn te1set(&self) -> TE1SET_R {
        TE1SET_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Timer E output 1 Reset
    #[inline(always)]
    pub fn te1rst(&self) -> TE1RST_R {
        TE1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Timer E Compare 1
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Timer E Compare 2
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT1CR")
            .field("tbcmp1", &self.tbcmp1())
            .field("tecmp2", &self.tecmp2())
            .field("tecmp1", &self.tecmp1())
            .field("tb1rst", &self.tb1rst())
            .field("te1rst", &self.te1rst())
            .field("tb1set", &self.tb1set())
            .field("te1set", &self.te1set())
            .field("tdcmp2", &self.tdcmp2())
            .field("tdcmp1", &self.tdcmp1())
            .field("td1rst", &self.td1rst())
            .field("td1set", &self.td1set())
            .field("tccmp2", &self.tccmp2())
            .field("tccmp1", &self.tccmp1())
            .field("tc1rst", &self.tc1rst())
            .field("tc1set", &self.tc1set())
            .field("tbcmp2", &self.tbcmp2())
            .field("exev1cpt", &self.exev1cpt())
            .field("exev2cpt", &self.exev2cpt())
            .field("exev3cpt", &self.exev3cpt())
            .field("exev4cpt", &self.exev4cpt())
            .field("exev5cpt", &self.exev5cpt())
            .field("exev6cpt", &self.exev6cpt())
            .field("exev7cpt", &self.exev7cpt())
            .field("exev8cpt", &self.exev8cpt())
            .field("exev9cpt", &self.exev9cpt())
            .field("exev10cpt", &self.exev10cpt())
            .field("updcpt", &self.updcpt())
            .field("swcpt", &self.swcpt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Capture
    #[inline(always)]
    pub fn swcpt(&mut self) -> SWCPT_W<'_, CPT1CRrs> {
        SWCPT_W::new(self, 0)
    }
    ///Bit 1 - Update Capture
    #[inline(always)]
    pub fn updcpt(&mut self) -> UPDCPT_W<'_, CPT1CRrs> {
        UPDCPT_W::new(self, 1)
    }
    ///External Event (1-10) Capture
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EXEV1CPT` field.</div>
    #[inline(always)]
    pub fn exevcpt(&mut self, n: u8) -> EXEVCPT_W<'_, CPT1CRrs> {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        EXEVCPT_W::new(self, n + 2)
    }
    ///Bit 2 - External Event 1 Capture
    #[inline(always)]
    pub fn exev1cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 2)
    }
    ///Bit 3 - External Event 2 Capture
    #[inline(always)]
    pub fn exev2cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 3)
    }
    ///Bit 4 - External Event 3 Capture
    #[inline(always)]
    pub fn exev3cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 4)
    }
    ///Bit 5 - External Event 4 Capture
    #[inline(always)]
    pub fn exev4cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 5)
    }
    ///Bit 6 - External Event 5 Capture
    #[inline(always)]
    pub fn exev5cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 6)
    }
    ///Bit 7 - External Event 6 Capture
    #[inline(always)]
    pub fn exev6cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 7)
    }
    ///Bit 8 - External Event 7 Capture
    #[inline(always)]
    pub fn exev7cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 8)
    }
    ///Bit 9 - External Event 8 Capture
    #[inline(always)]
    pub fn exev8cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 9)
    }
    ///Bit 10 - External Event 9 Capture
    #[inline(always)]
    pub fn exev9cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 10)
    }
    ///Bit 11 - External Event 10 Capture
    #[inline(always)]
    pub fn exev10cpt(&mut self) -> EXEVCPT_W<'_, CPT1CRrs> {
        EXEVCPT_W::new(self, 11)
    }
    ///Bit 16 - Timer B output 1 Set
    #[inline(always)]
    pub fn tb1set(&mut self) -> TB1SET_W<'_, CPT1CRrs> {
        TB1SET_W::new(self, 16)
    }
    ///Bit 17 - Timer B output 1 Reset
    #[inline(always)]
    pub fn tb1rst(&mut self) -> TB1RST_W<'_, CPT1CRrs> {
        TB1RST_W::new(self, 17)
    }
    ///Bit 18 - Timer B Compare 1
    #[inline(always)]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<'_, CPT1CRrs> {
        TBCMP1_W::new(self, 18)
    }
    ///Bit 19 - Timer B Compare 2
    #[inline(always)]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<'_, CPT1CRrs> {
        TBCMP2_W::new(self, 19)
    }
    ///Bit 20 - Timer C output 1 Set
    #[inline(always)]
    pub fn tc1set(&mut self) -> TC1SET_W<'_, CPT1CRrs> {
        TC1SET_W::new(self, 20)
    }
    ///Bit 21 - Timer C output 1 Reset
    #[inline(always)]
    pub fn tc1rst(&mut self) -> TC1RST_W<'_, CPT1CRrs> {
        TC1RST_W::new(self, 21)
    }
    ///Bit 22 - Timer C Compare 1
    #[inline(always)]
    pub fn tccmp1(&mut self) -> TCCMP1_W<'_, CPT1CRrs> {
        TCCMP1_W::new(self, 22)
    }
    ///Bit 23 - Timer C Compare 2
    #[inline(always)]
    pub fn tccmp2(&mut self) -> TCCMP2_W<'_, CPT1CRrs> {
        TCCMP2_W::new(self, 23)
    }
    ///Bit 24 - Timer D output 1 Set
    #[inline(always)]
    pub fn td1set(&mut self) -> TD1SET_W<'_, CPT1CRrs> {
        TD1SET_W::new(self, 24)
    }
    ///Bit 25 - Timer D output 1 Reset
    #[inline(always)]
    pub fn td1rst(&mut self) -> TD1RST_W<'_, CPT1CRrs> {
        TD1RST_W::new(self, 25)
    }
    ///Bit 26 - Timer D Compare 1
    #[inline(always)]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<'_, CPT1CRrs> {
        TDCMP1_W::new(self, 26)
    }
    ///Bit 27 - Timer D Compare 2
    #[inline(always)]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<'_, CPT1CRrs> {
        TDCMP2_W::new(self, 27)
    }
    ///Bit 28 - Timer E output 1 Set
    #[inline(always)]
    pub fn te1set(&mut self) -> TE1SET_W<'_, CPT1CRrs> {
        TE1SET_W::new(self, 28)
    }
    ///Bit 29 - Timer E output 1 Reset
    #[inline(always)]
    pub fn te1rst(&mut self) -> TE1RST_W<'_, CPT1CRrs> {
        TE1RST_W::new(self, 29)
    }
    ///Bit 30 - Timer E Compare 1
    #[inline(always)]
    pub fn tecmp1(&mut self) -> TECMP1_W<'_, CPT1CRrs> {
        TECMP1_W::new(self, 30)
    }
    ///Bit 31 - Timer E Compare 2
    #[inline(always)]
    pub fn tecmp2(&mut self) -> TECMP2_W<'_, CPT1CRrs> {
        TECMP2_W::new(self, 31)
    }
}
/**Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#HRTIM_TIMA:CPT1CR)*/
pub struct CPT1CRrs;
impl crate::RegisterSpec for CPT1CRrs {
    type Ux = u32;
}
///`read()` method returns [`cpt1cr::R`](R) reader structure
impl crate::Readable for CPT1CRrs {}
///`write(|w| ..)` method takes [`cpt1cr::W`](W) writer structure
impl crate::Writable for CPT1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPT1CR to value 0
impl crate::Resettable for CPT1CRrs {}
