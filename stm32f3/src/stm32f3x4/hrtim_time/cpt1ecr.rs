#[doc = "Register `CPT1ECR` reader"]
pub type R = crate::R<CPT1ECRrs>;
#[doc = "Register `CPT1ECR` writer"]
pub type W = crate::W<CPT1ECRrs>;
#[doc = "Software Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWCPT {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Force capture Z"]
    TriggerCapture = 1,
}
impl From<SWCPT> for bool {
    #[inline(always)]
    fn from(variant: SWCPT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWCPT` reader - Software Capture"]
pub type SWCPT_R = crate::BitReader<SWCPT>;
impl SWCPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWCPT {
        match self.bits {
            false => SWCPT::NoEffect,
            true => SWCPT::TriggerCapture,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SWCPT::NoEffect
    }
    #[doc = "Force capture Z"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == SWCPT::TriggerCapture
    }
}
#[doc = "Field `SWCPT` writer - Software Capture"]
pub type SWCPT_W<'a, REG> = crate::BitWriter<'a, REG, SWCPT>;
impl<'a, REG> SWCPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SWCPT::NoEffect)
    }
    #[doc = "Force capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(SWCPT::TriggerCapture)
    }
}
#[doc = "Update Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDCPT {
    #[doc = "0: Update event has no effect"]
    NoEffect = 0,
    #[doc = "1: Update event triggers capture Z"]
    TriggerCapture = 1,
}
impl From<UPDCPT> for bool {
    #[inline(always)]
    fn from(variant: UPDCPT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDCPT` reader - Update Capture"]
pub type UPDCPT_R = crate::BitReader<UPDCPT>;
impl UPDCPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UPDCPT {
        match self.bits {
            false => UPDCPT::NoEffect,
            true => UPDCPT::TriggerCapture,
        }
    }
    #[doc = "Update event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDCPT::NoEffect
    }
    #[doc = "Update event triggers capture Z"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == UPDCPT::TriggerCapture
    }
}
#[doc = "Field `UPDCPT` writer - Update Capture"]
pub type UPDCPT_W<'a, REG> = crate::BitWriter<'a, REG, UPDCPT>;
impl<'a, REG> UPDCPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UPDCPT::NoEffect)
    }
    #[doc = "Update event triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(UPDCPT::TriggerCapture)
    }
}
#[doc = "External Event 1 Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXEV1CPT {
    #[doc = "0: External event Y has no effect"]
    NoEffect = 0,
    #[doc = "1: External event Y triggers capture Z"]
    TriggerCapture = 1,
}
impl From<EXEV1CPT> for bool {
    #[inline(always)]
    fn from(variant: EXEV1CPT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXEV1CPT` reader - External Event 1 Capture"]
pub type EXEV1CPT_R = crate::BitReader<EXEV1CPT>;
impl EXEV1CPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXEV1CPT {
        match self.bits {
            false => EXEV1CPT::NoEffect,
            true => EXEV1CPT::TriggerCapture,
        }
    }
    #[doc = "External event Y has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXEV1CPT::NoEffect
    }
    #[doc = "External event Y triggers capture Z"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == EXEV1CPT::TriggerCapture
    }
}
#[doc = "Field `EXEV1CPT` writer - External Event 1 Capture"]
pub type EXEV1CPT_W<'a, REG> = crate::BitWriter<'a, REG, EXEV1CPT>;
impl<'a, REG> EXEV1CPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External event Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EXEV1CPT::NoEffect)
    }
    #[doc = "External event Y triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(EXEV1CPT::TriggerCapture)
    }
}
#[doc = "Field `EXEV2CPT` reader - External Event 2 Capture"]
pub use EXEV1CPT_R as EXEV2CPT_R;
#[doc = "Field `EXEV3CPT` reader - External Event 3 Capture"]
pub use EXEV1CPT_R as EXEV3CPT_R;
#[doc = "Field `EXEV4CPT` reader - External Event 4 Capture"]
pub use EXEV1CPT_R as EXEV4CPT_R;
#[doc = "Field `EXEV5CPT` reader - External Event 5 Capture"]
pub use EXEV1CPT_R as EXEV5CPT_R;
#[doc = "Field `EXEV6CPT` reader - External Event 6 Capture"]
pub use EXEV1CPT_R as EXEV6CPT_R;
#[doc = "Field `EXEV7CPT` reader - External Event 7 Capture"]
pub use EXEV1CPT_R as EXEV7CPT_R;
#[doc = "Field `EXEV8CPT` reader - External Event 8 Capture"]
pub use EXEV1CPT_R as EXEV8CPT_R;
#[doc = "Field `EXEV9CPT` reader - External Event 9 Capture"]
pub use EXEV1CPT_R as EXEV9CPT_R;
#[doc = "Field `EXEV10CPT` reader - External Event 10 Capture"]
pub use EXEV1CPT_R as EXEV10CPT_R;
#[doc = "Field `EXEV2CPT` writer - External Event 2 Capture"]
pub use EXEV1CPT_W as EXEV2CPT_W;
#[doc = "Field `EXEV3CPT` writer - External Event 3 Capture"]
pub use EXEV1CPT_W as EXEV3CPT_W;
#[doc = "Field `EXEV4CPT` writer - External Event 4 Capture"]
pub use EXEV1CPT_W as EXEV4CPT_W;
#[doc = "Field `EXEV5CPT` writer - External Event 5 Capture"]
pub use EXEV1CPT_W as EXEV5CPT_W;
#[doc = "Field `EXEV6CPT` writer - External Event 6 Capture"]
pub use EXEV1CPT_W as EXEV6CPT_W;
#[doc = "Field `EXEV7CPT` writer - External Event 7 Capture"]
pub use EXEV1CPT_W as EXEV7CPT_W;
#[doc = "Field `EXEV8CPT` writer - External Event 8 Capture"]
pub use EXEV1CPT_W as EXEV8CPT_W;
#[doc = "Field `EXEV9CPT` writer - External Event 9 Capture"]
pub use EXEV1CPT_W as EXEV9CPT_W;
#[doc = "Field `EXEV10CPT` writer - External Event 10 Capture"]
pub use EXEV1CPT_W as EXEV10CPT_W;
#[doc = "Timer A output 1 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1SET {
    #[doc = "0: Timer X output Y inactive to active transition has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X output Y inactive to active transition triggers capture Z"]
    TriggerCapture = 1,
}
impl From<TA1SET> for bool {
    #[inline(always)]
    fn from(variant: TA1SET) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1SET` reader - Timer A output 1 Set"]
pub type TA1SET_R = crate::BitReader<TA1SET>;
impl TA1SET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TA1SET {
        match self.bits {
            false => TA1SET::NoEffect,
            true => TA1SET::TriggerCapture,
        }
    }
    #[doc = "Timer X output Y inactive to active transition has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TA1SET::NoEffect
    }
    #[doc = "Timer X output Y inactive to active transition triggers capture Z"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TA1SET::TriggerCapture
    }
}
#[doc = "Field `TA1SET` writer - Timer A output 1 Set"]
pub type TA1SET_W<'a, REG> = crate::BitWriter<'a, REG, TA1SET>;
impl<'a, REG> TA1SET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer X output Y inactive to active transition has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TA1SET::NoEffect)
    }
    #[doc = "Timer X output Y inactive to active transition triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(TA1SET::TriggerCapture)
    }
}
#[doc = "Timer A output 1 Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1RST {
    #[doc = "0: Timer X output Y active to inactive transition has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X output Y active to inactive transition triggers capture Z"]
    TriggerCapture = 1,
}
impl From<TA1RST> for bool {
    #[inline(always)]
    fn from(variant: TA1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1RST` reader - Timer A output 1 Reset"]
pub type TA1RST_R = crate::BitReader<TA1RST>;
impl TA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TA1RST {
        match self.bits {
            false => TA1RST::NoEffect,
            true => TA1RST::TriggerCapture,
        }
    }
    #[doc = "Timer X output Y active to inactive transition has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TA1RST::NoEffect
    }
    #[doc = "Timer X output Y active to inactive transition triggers capture Z"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TA1RST::TriggerCapture
    }
}
#[doc = "Field `TA1RST` writer - Timer A output 1 Reset"]
pub type TA1RST_W<'a, REG> = crate::BitWriter<'a, REG, TA1RST>;
impl<'a, REG> TA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer X output Y active to inactive transition has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TA1RST::NoEffect)
    }
    #[doc = "Timer X output Y active to inactive transition triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(TA1RST::TriggerCapture)
    }
}
#[doc = "Timer A Compare 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACMP1 {
    #[doc = "0: Timer X compare Y has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X compare Y triggers capture Z"]
    TriggerCapture = 1,
}
impl From<TACMP1> for bool {
    #[inline(always)]
    fn from(variant: TACMP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACMP1` reader - Timer A Compare 1"]
pub type TACMP1_R = crate::BitReader<TACMP1>;
impl TACMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TACMP1 {
        match self.bits {
            false => TACMP1::NoEffect,
            true => TACMP1::TriggerCapture,
        }
    }
    #[doc = "Timer X compare Y has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TACMP1::NoEffect
    }
    #[doc = "Timer X compare Y triggers capture Z"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TACMP1::TriggerCapture
    }
}
#[doc = "Field `TACMP1` writer - Timer A Compare 1"]
pub type TACMP1_W<'a, REG> = crate::BitWriter<'a, REG, TACMP1>;
impl<'a, REG> TACMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer X compare Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TACMP1::NoEffect)
    }
    #[doc = "Timer X compare Y triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut crate::W<REG> {
        self.variant(TACMP1::TriggerCapture)
    }
}
#[doc = "Field `TB1RST` reader - Timer B output 1 Reset"]
pub use TA1RST_R as TB1RST_R;
#[doc = "Field `TC1RST` reader - Timer C output 1 Reset"]
pub use TA1RST_R as TC1RST_R;
#[doc = "Field `TD1RST` reader - Timer D output 1 Reset"]
pub use TA1RST_R as TD1RST_R;
#[doc = "Field `TB1RST` writer - Timer B output 1 Reset"]
pub use TA1RST_W as TB1RST_W;
#[doc = "Field `TC1RST` writer - Timer C output 1 Reset"]
pub use TA1RST_W as TC1RST_W;
#[doc = "Field `TD1RST` writer - Timer D output 1 Reset"]
pub use TA1RST_W as TD1RST_W;
#[doc = "Field `TB1SET` reader - Timer B output 1 Set"]
pub use TA1SET_R as TB1SET_R;
#[doc = "Field `TC1SET` reader - Timer C output 1 Set"]
pub use TA1SET_R as TC1SET_R;
#[doc = "Field `TD1SET` reader - Timer D output 1 Set"]
pub use TA1SET_R as TD1SET_R;
#[doc = "Field `TB1SET` writer - Timer B output 1 Set"]
pub use TA1SET_W as TB1SET_W;
#[doc = "Field `TC1SET` writer - Timer C output 1 Set"]
pub use TA1SET_W as TC1SET_W;
#[doc = "Field `TD1SET` writer - Timer D output 1 Set"]
pub use TA1SET_W as TD1SET_W;
#[doc = "Field `TACMP2` reader - Timer A Compare 2"]
pub use TACMP1_R as TACMP2_R;
#[doc = "Field `TBCMP1` reader - Timer B Compare 1"]
pub use TACMP1_R as TBCMP1_R;
#[doc = "Field `TBCMP2` reader - Timer B Compare 2"]
pub use TACMP1_R as TBCMP2_R;
#[doc = "Field `TCCMP1` reader - Timer C Compare 1"]
pub use TACMP1_R as TCCMP1_R;
#[doc = "Field `TCCMP2` reader - Timer C Compare 2"]
pub use TACMP1_R as TCCMP2_R;
#[doc = "Field `TDCMP1` reader - Timer D Compare 1"]
pub use TACMP1_R as TDCMP1_R;
#[doc = "Field `TDCMP2` reader - Timer D Compare 2"]
pub use TACMP1_R as TDCMP2_R;
#[doc = "Field `TACMP2` writer - Timer A Compare 2"]
pub use TACMP1_W as TACMP2_W;
#[doc = "Field `TBCMP1` writer - Timer B Compare 1"]
pub use TACMP1_W as TBCMP1_W;
#[doc = "Field `TBCMP2` writer - Timer B Compare 2"]
pub use TACMP1_W as TBCMP2_W;
#[doc = "Field `TCCMP1` writer - Timer C Compare 1"]
pub use TACMP1_W as TCCMP1_W;
#[doc = "Field `TCCMP2` writer - Timer C Compare 2"]
pub use TACMP1_W as TCCMP2_W;
#[doc = "Field `TDCMP1` writer - Timer D Compare 1"]
pub use TACMP1_W as TDCMP1_W;
#[doc = "Field `TDCMP2` writer - Timer D Compare 2"]
pub use TACMP1_W as TDCMP2_W;
impl R {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    pub fn swcpt(&self) -> SWCPT_R {
        SWCPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    pub fn updcpt(&self) -> UPDCPT_R {
        UPDCPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    pub fn exev1cpt(&self) -> EXEV1CPT_R {
        EXEV1CPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    pub fn exev2cpt(&self) -> EXEV2CPT_R {
        EXEV2CPT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    pub fn exev3cpt(&self) -> EXEV3CPT_R {
        EXEV3CPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    pub fn exev4cpt(&self) -> EXEV4CPT_R {
        EXEV4CPT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    pub fn exev5cpt(&self) -> EXEV5CPT_R {
        EXEV5CPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    pub fn exev6cpt(&self) -> EXEV6CPT_R {
        EXEV6CPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    pub fn exev7cpt(&self) -> EXEV7CPT_R {
        EXEV7CPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    pub fn exev8cpt(&self) -> EXEV8CPT_R {
        EXEV8CPT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    pub fn exev9cpt(&self) -> EXEV9CPT_R {
        EXEV9CPT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    pub fn exev10cpt(&self) -> EXEV10CPT_R {
        EXEV10CPT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    pub fn ta1set(&self) -> TA1SET_R {
        TA1SET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    pub fn ta1rst(&self) -> TA1RST_R {
        TA1RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    pub fn tb1set(&self) -> TB1SET_R {
        TB1SET_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    pub fn tb1rst(&self) -> TB1RST_R {
        TB1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    pub fn tc1set(&self) -> TC1SET_R {
        TC1SET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    pub fn tc1rst(&self) -> TC1RST_R {
        TC1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    pub fn td1set(&self) -> TD1SET_R {
        TD1SET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    pub fn td1rst(&self) -> TD1RST_R {
        TD1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    #[must_use]
    pub fn swcpt(&mut self) -> SWCPT_W<CPT1ECRrs> {
        SWCPT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    #[must_use]
    pub fn updcpt(&mut self) -> UPDCPT_W<CPT1ECRrs> {
        UPDCPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev1cpt(&mut self) -> EXEV1CPT_W<CPT1ECRrs> {
        EXEV1CPT_W::new(self, 2)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev2cpt(&mut self) -> EXEV2CPT_W<CPT1ECRrs> {
        EXEV2CPT_W::new(self, 3)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev3cpt(&mut self) -> EXEV3CPT_W<CPT1ECRrs> {
        EXEV3CPT_W::new(self, 4)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev4cpt(&mut self) -> EXEV4CPT_W<CPT1ECRrs> {
        EXEV4CPT_W::new(self, 5)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev5cpt(&mut self) -> EXEV5CPT_W<CPT1ECRrs> {
        EXEV5CPT_W::new(self, 6)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev6cpt(&mut self) -> EXEV6CPT_W<CPT1ECRrs> {
        EXEV6CPT_W::new(self, 7)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev7cpt(&mut self) -> EXEV7CPT_W<CPT1ECRrs> {
        EXEV7CPT_W::new(self, 8)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev8cpt(&mut self) -> EXEV8CPT_W<CPT1ECRrs> {
        EXEV8CPT_W::new(self, 9)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev9cpt(&mut self) -> EXEV9CPT_W<CPT1ECRrs> {
        EXEV9CPT_W::new(self, 10)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev10cpt(&mut self) -> EXEV10CPT_W<CPT1ECRrs> {
        EXEV10CPT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn ta1set(&mut self) -> TA1SET_W<CPT1ECRrs> {
        TA1SET_W::new(self, 12)
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ta1rst(&mut self) -> TA1RST_W<CPT1ECRrs> {
        TA1RST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp1(&mut self) -> TACMP1_W<CPT1ECRrs> {
        TACMP1_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp2(&mut self) -> TACMP2_W<CPT1ECRrs> {
        TACMP2_W::new(self, 15)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn tb1set(&mut self) -> TB1SET_W<CPT1ECRrs> {
        TB1SET_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tb1rst(&mut self) -> TB1RST_W<CPT1ECRrs> {
        TB1RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<CPT1ECRrs> {
        TBCMP1_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<CPT1ECRrs> {
        TBCMP2_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn tc1set(&mut self) -> TC1SET_W<CPT1ECRrs> {
        TC1SET_W::new(self, 20)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tc1rst(&mut self) -> TC1RST_W<CPT1ECRrs> {
        TC1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp1(&mut self) -> TCCMP1_W<CPT1ECRrs> {
        TCCMP1_W::new(self, 22)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp2(&mut self) -> TCCMP2_W<CPT1ECRrs> {
        TCCMP2_W::new(self, 23)
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn td1set(&mut self) -> TD1SET_W<CPT1ECRrs> {
        TD1SET_W::new(self, 24)
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn td1rst(&mut self) -> TD1RST_W<CPT1ECRrs> {
        TD1RST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<CPT1ECRrs> {
        TDCMP1_W::new(self, 26)
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<CPT1ECRrs> {
        TDCMP2_W::new(self, 27)
    }
}
#[doc = "Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPT1ECRrs;
impl crate::RegisterSpec for CPT1ECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt1ecr::R`](R) reader structure"]
impl crate::Readable for CPT1ECRrs {}
#[doc = "`write(|w| ..)` method takes [`cpt1ecr::W`](W) writer structure"]
impl crate::Writable for CPT1ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPT1ECR to value 0"]
impl crate::Resettable for CPT1ECRrs {
    const RESET_VALUE: u32 = 0;
}
