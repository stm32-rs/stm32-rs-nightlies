#[doc = "Register `ODSR` reader"]
pub struct R(crate::R<ODSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TE2ODS` reader - Timer E Output 2 disable status"]
pub struct TE2ODS_R(crate::FieldReader<bool, bool>);
impl TE2ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE2ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE2ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE1ODS` reader - Timer E Output 1 disable status"]
pub struct TE1ODS_R(crate::FieldReader<bool, bool>);
impl TE1ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE1ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE1ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TD2ODS` reader - Timer D Output 2 disable status"]
pub struct TD2ODS_R(crate::FieldReader<bool, bool>);
impl TD2ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TD2ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TD2ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TD1ODS` reader - Timer D Output 1 disable status"]
pub struct TD1ODS_R(crate::FieldReader<bool, bool>);
impl TD1ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TD1ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TD1ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC2ODS` reader - Timer C Output 2 disable status"]
pub struct TC2ODS_R(crate::FieldReader<bool, bool>);
impl TC2ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC2ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC2ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC1ODS` reader - Timer C Output 1 disable status"]
pub struct TC1ODS_R(crate::FieldReader<bool, bool>);
impl TC1ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC1ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC1ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TB2ODS` reader - Timer B Output 2 disable status"]
pub struct TB2ODS_R(crate::FieldReader<bool, bool>);
impl TB2ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TB2ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB2ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TB1ODS` reader - Timer B Output 1 disable status"]
pub struct TB1ODS_R(crate::FieldReader<bool, bool>);
impl TB1ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TB1ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB1ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA2ODS` reader - Timer A Output 2 disable status"]
pub struct TA2ODS_R(crate::FieldReader<bool, bool>);
impl TA2ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TA2ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA2ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA1ODS` reader - Timer A Output 1 disable status"]
pub struct TA1ODS_R(crate::FieldReader<bool, bool>);
impl TA1ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TA1ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA1ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
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
#[doc = "Output Disable Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odsr](index.html) module"]
pub struct ODSR_SPEC;
impl crate::RegisterSpec for ODSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odsr::R](R) reader structure"]
impl crate::Readable for ODSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ODSR to value 0"]
impl crate::Resettable for ODSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
