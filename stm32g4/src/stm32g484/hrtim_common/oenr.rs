#[doc = "Register `OENR` reader"]
pub struct R(crate::R<OENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OENR` writer"]
pub struct W(crate::W<OENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TF2ODS` reader - Timer F Output 2 disable status"]
pub struct TF2ODS_R(crate::FieldReader<bool, bool>);
impl TF2ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF2ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF2ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF2ODS` writer - Timer F Output 2 disable status"]
pub struct TF2ODS_W<'a> {
    w: &'a mut W,
}
impl<'a> TF2ODS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TF1ODS` reader - Timer F Output 1 disable status"]
pub struct TF1ODS_R(crate::FieldReader<bool, bool>);
impl TF1ODS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF1ODS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF1ODS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF1ODS` writer - Timer F Output 1 disable status"]
pub struct TF1ODS_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1ODS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TE2OEN` reader - Timer E Output 2 Enable"]
pub struct TE2OEN_R(crate::FieldReader<bool, bool>);
impl TE2OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE2OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE2OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE2OEN` writer - Timer E Output 2 Enable"]
pub struct TE2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TE2OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TE1OEN` reader - Timer E Output 1 Enable"]
pub struct TE1OEN_R(crate::FieldReader<bool, bool>);
impl TE1OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE1OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE1OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE1OEN` writer - Timer E Output 1 Enable"]
pub struct TE1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TE1OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TD2OEN` reader - Timer D Output 2 Enable"]
pub struct TD2OEN_R(crate::FieldReader<bool, bool>);
impl TD2OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TD2OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TD2OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TD2OEN` writer - Timer D Output 2 Enable"]
pub struct TD2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TD2OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TD1OEN` reader - Timer D Output 1 Enable"]
pub struct TD1OEN_R(crate::FieldReader<bool, bool>);
impl TD1OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TD1OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TD1OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TD1OEN` writer - Timer D Output 1 Enable"]
pub struct TD1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TD1OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TC2OEN` reader - Timer C Output 2 Enable"]
pub struct TC2OEN_R(crate::FieldReader<bool, bool>);
impl TC2OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC2OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC2OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC2OEN` writer - Timer C Output 2 Enable"]
pub struct TC2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TC1OEN` reader - Timer C Output 1 Enable"]
pub struct TC1OEN_R(crate::FieldReader<bool, bool>);
impl TC1OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC1OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC1OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC1OEN` writer - Timer C Output 1 Enable"]
pub struct TC1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TB2OEN` reader - Timer B Output 2 Enable"]
pub struct TB2OEN_R(crate::FieldReader<bool, bool>);
impl TB2OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TB2OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB2OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TB2OEN` writer - Timer B Output 2 Enable"]
pub struct TB2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TB2OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TB1OEN` reader - Timer B Output 1 Enable"]
pub struct TB1OEN_R(crate::FieldReader<bool, bool>);
impl TB1OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TB1OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB1OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TB1OEN` writer - Timer B Output 1 Enable"]
pub struct TB1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TB1OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TA2OEN` reader - Timer A Output 2 Enable"]
pub struct TA2OEN_R(crate::FieldReader<bool, bool>);
impl TA2OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TA2OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA2OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA2OEN` writer - Timer A Output 2 Enable"]
pub struct TA2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TA2OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TA1OEN` reader - Timer A Output 1 Enable"]
pub struct TA1OEN_R(crate::FieldReader<bool, bool>);
impl TA1OEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TA1OEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA1OEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA1OEN` writer - Timer A Output 1 Enable"]
pub struct TA1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TA1OEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Timer F Output 2 disable status"]
    #[inline(always)]
    pub fn tf2ods(&self) -> TF2ODS_R {
        TF2ODS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer F Output 1 disable status"]
    #[inline(always)]
    pub fn tf1ods(&self) -> TF1ODS_R {
        TF1ODS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    pub fn te2oen(&self) -> TE2OEN_R {
        TE2OEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    pub fn te1oen(&self) -> TE1OEN_R {
        TE1OEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    pub fn td2oen(&self) -> TD2OEN_R {
        TD2OEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    pub fn td1oen(&self) -> TD1OEN_R {
        TD1OEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    pub fn tc2oen(&self) -> TC2OEN_R {
        TC2OEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    pub fn tc1oen(&self) -> TC1OEN_R {
        TC1OEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    pub fn tb2oen(&self) -> TB2OEN_R {
        TB2OEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    pub fn tb1oen(&self) -> TB1OEN_R {
        TB1OEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    pub fn ta2oen(&self) -> TA2OEN_R {
        TA2OEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    pub fn ta1oen(&self) -> TA1OEN_R {
        TA1OEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Timer F Output 2 disable status"]
    #[inline(always)]
    pub fn tf2ods(&mut self) -> TF2ODS_W {
        TF2ODS_W { w: self }
    }
    #[doc = "Bit 10 - Timer F Output 1 disable status"]
    #[inline(always)]
    pub fn tf1ods(&mut self) -> TF1ODS_W {
        TF1ODS_W { w: self }
    }
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    pub fn te2oen(&mut self) -> TE2OEN_W {
        TE2OEN_W { w: self }
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    pub fn te1oen(&mut self) -> TE1OEN_W {
        TE1OEN_W { w: self }
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    pub fn td2oen(&mut self) -> TD2OEN_W {
        TD2OEN_W { w: self }
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    pub fn td1oen(&mut self) -> TD1OEN_W {
        TD1OEN_W { w: self }
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    pub fn tc2oen(&mut self) -> TC2OEN_W {
        TC2OEN_W { w: self }
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    pub fn tc1oen(&mut self) -> TC1OEN_W {
        TC1OEN_W { w: self }
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    pub fn tb2oen(&mut self) -> TB2OEN_W {
        TB2OEN_W { w: self }
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    pub fn tb1oen(&mut self) -> TB1OEN_W {
        TB1OEN_W { w: self }
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    pub fn ta2oen(&mut self) -> TA2OEN_W {
        TA2OEN_W { w: self }
    }
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    pub fn ta1oen(&mut self) -> TA1OEN_W {
        TA1OEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oenr](index.html) module"]
pub struct OENR_SPEC;
impl crate::RegisterSpec for OENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oenr::R](R) reader structure"]
impl crate::Readable for OENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oenr::W](W) writer structure"]
impl crate::Writable for OENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OENR to value 0"]
impl crate::Resettable for OENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
