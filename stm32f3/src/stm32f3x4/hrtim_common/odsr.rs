#[doc = "Register `ODSR` reader"]
pub type R = crate::R<ODSRrs>;
#[doc = "Timer A Output 1 disable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1ODS {
    #[doc = "0: Output disabled in idle state"]
    Idle = 0,
    #[doc = "1: Output disabled in fault state"]
    Fault = 1,
}
impl From<TA1ODS> for bool {
    #[inline(always)]
    fn from(variant: TA1ODS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1ODS` reader - Timer A Output 1 disable status"]
pub type TA1ODS_R = crate::BitReader<TA1ODS>;
impl TA1ODS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TA1ODS {
        match self.bits {
            false => TA1ODS::Idle,
            true => TA1ODS::Fault,
        }
    }
    #[doc = "Output disabled in idle state"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TA1ODS::Idle
    }
    #[doc = "Output disabled in fault state"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == TA1ODS::Fault
    }
}
#[doc = "Field `TA2ODS` reader - Timer A Output 2 disable status"]
pub use TA1ODS_R as TA2ODS_R;
#[doc = "Field `TB1ODS` reader - Timer B Output 1 disable status"]
pub use TA1ODS_R as TB1ODS_R;
#[doc = "Field `TB2ODS` reader - Timer B Output 2 disable status"]
pub use TA1ODS_R as TB2ODS_R;
#[doc = "Field `TC1ODS` reader - Timer C Output 1 disable status"]
pub use TA1ODS_R as TC1ODS_R;
#[doc = "Field `TC2ODS` reader - Timer C Output 2 disable status"]
pub use TA1ODS_R as TC2ODS_R;
#[doc = "Field `TD1ODS` reader - Timer D Output 1 disable status"]
pub use TA1ODS_R as TD1ODS_R;
#[doc = "Field `TD2ODS` reader - Timer D Output 2 disable status"]
pub use TA1ODS_R as TD2ODS_R;
#[doc = "Field `TE1ODS` reader - Timer E Output 1 disable status"]
pub use TA1ODS_R as TE1ODS_R;
#[doc = "Field `TE2ODS` reader - Timer E Output 2 disable status"]
pub use TA1ODS_R as TE2ODS_R;
impl R {
    #[doc = "Bit 0 - Timer A Output 1 disable status"]
    #[inline(always)]
    pub fn ta1ods(&self) -> TA1ODS_R {
        TA1ODS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 disable status"]
    #[inline(always)]
    pub fn ta2ods(&self) -> TA2ODS_R {
        TA2ODS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Output 1 disable status"]
    #[inline(always)]
    pub fn tb1ods(&self) -> TB1ODS_R {
        TB1ODS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer B Output 2 disable status"]
    #[inline(always)]
    pub fn tb2ods(&self) -> TB2ODS_R {
        TB2ODS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer C Output 1 disable status"]
    #[inline(always)]
    pub fn tc1ods(&self) -> TC1ODS_R {
        TC1ODS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer C Output 2 disable status"]
    #[inline(always)]
    pub fn tc2ods(&self) -> TC2ODS_R {
        TC2ODS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer D Output 1 disable status"]
    #[inline(always)]
    pub fn td1ods(&self) -> TD1ODS_R {
        TD1ODS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer D Output 2 disable status"]
    #[inline(always)]
    pub fn td2ods(&self) -> TD2ODS_R {
        TD2ODS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer E Output 1 disable status"]
    #[inline(always)]
    pub fn te1ods(&self) -> TE1ODS_R {
        TE1ODS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer E Output 2 disable status"]
    #[inline(always)]
    pub fn te2ods(&self) -> TE2ODS_R {
        TE2ODS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Output Disable Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODSRrs;
impl crate::RegisterSpec for ODSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odsr::R`](R) reader structure"]
impl crate::Readable for ODSRrs {}
#[doc = "`reset()` method sets ODSR to value 0"]
impl crate::Resettable for ODSRrs {
    const RESET_VALUE: u32 = 0;
}
