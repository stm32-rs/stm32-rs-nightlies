#[doc = "Register `ODSR` reader"]
pub type R = crate::R<ODSRrs>;
#[doc = "Field `TA1ODS` reader - Timer A Output 1 disable status"]
pub type TA1ODS_R = crate::BitReader;
#[doc = "Field `TA2ODS` reader - Timer A Output 2 disable status"]
pub type TA2ODS_R = crate::BitReader;
#[doc = "Field `TB1ODS` reader - Timer B Output 1 disable status"]
pub type TB1ODS_R = crate::BitReader;
#[doc = "Field `TB2ODS` reader - Timer B Output 2 disable status"]
pub type TB2ODS_R = crate::BitReader;
#[doc = "Field `TC1ODS` reader - Timer C Output 1 disable status"]
pub type TC1ODS_R = crate::BitReader;
#[doc = "Field `TC2ODS` reader - Timer C Output 2 disable status"]
pub type TC2ODS_R = crate::BitReader;
#[doc = "Field `TD1ODS` reader - Timer D Output 1 disable status"]
pub type TD1ODS_R = crate::BitReader;
#[doc = "Field `TD2ODS` reader - Timer D Output 2 disable status"]
pub type TD2ODS_R = crate::BitReader;
#[doc = "Field `TE1ODS` reader - Timer E Output 1 disable status"]
pub type TE1ODS_R = crate::BitReader;
#[doc = "Field `TE2ODS` reader - Timer E Output 2 disable status"]
pub type TE2ODS_R = crate::BitReader;
#[doc = "Field `TF1ODS` reader - TF1ODS"]
pub type TF1ODS_R = crate::BitReader;
#[doc = "Field `TF2ODS` reader - TF2ODS"]
pub type TF2ODS_R = crate::BitReader;
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
    #[doc = "Bit 10 - TF1ODS"]
    #[inline(always)]
    pub fn tf1ods(&self) -> TF1ODS_R {
        TF1ODS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TF2ODS"]
    #[inline(always)]
    pub fn tf2ods(&self) -> TF2ODS_R {
        TF2ODS_R::new(((self.bits >> 11) & 1) != 0)
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
