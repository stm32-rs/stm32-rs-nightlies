#[doc = "Reader of register ODSR"]
pub type R = crate::R<u32, super::ODSR>;
#[doc = "Reader of field `TF2ODS`"]
pub type TF2ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF1ODS`"]
pub type TF1ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TE2ODS`"]
pub type TE2ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TE1ODS`"]
pub type TE1ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TD2ODS`"]
pub type TD2ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TD1ODS`"]
pub type TD1ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TC2ODS`"]
pub type TC2ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TC1ODS`"]
pub type TC1ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TB2ODS`"]
pub type TB2ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TB1ODS`"]
pub type TB1ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TA2ODS`"]
pub type TA2ODS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TA1ODS`"]
pub type TA1ODS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 11 - TF2ODS"]
    #[inline(always)]
    pub fn tf2ods(&self) -> TF2ODS_R {
        TF2ODS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TF1ODS"]
    #[inline(always)]
    pub fn tf1ods(&self) -> TF1ODS_R {
        TF1ODS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer E Output 2 disable status"]
    #[inline(always)]
    pub fn te2ods(&self) -> TE2ODS_R {
        TE2ODS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer E Output 1 disable status"]
    #[inline(always)]
    pub fn te1ods(&self) -> TE1ODS_R {
        TE1ODS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer D Output 2 disable status"]
    #[inline(always)]
    pub fn td2ods(&self) -> TD2ODS_R {
        TD2ODS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer D Output 1 disable status"]
    #[inline(always)]
    pub fn td1ods(&self) -> TD1ODS_R {
        TD1ODS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer C Output 2 disable status"]
    #[inline(always)]
    pub fn tc2ods(&self) -> TC2ODS_R {
        TC2ODS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer C Output 1 disable status"]
    #[inline(always)]
    pub fn tc1ods(&self) -> TC1ODS_R {
        TC1ODS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer B Output 2 disable status"]
    #[inline(always)]
    pub fn tb2ods(&self) -> TB2ODS_R {
        TB2ODS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer B Output 1 disable status"]
    #[inline(always)]
    pub fn tb1ods(&self) -> TB1ODS_R {
        TB1ODS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 disable status"]
    #[inline(always)]
    pub fn ta2ods(&self) -> TA2ODS_R {
        TA2ODS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timer A Output 1 disable status"]
    #[inline(always)]
    pub fn ta1ods(&self) -> TA1ODS_R {
        TA1ODS_R::new((self.bits & 0x01) != 0)
    }
}
