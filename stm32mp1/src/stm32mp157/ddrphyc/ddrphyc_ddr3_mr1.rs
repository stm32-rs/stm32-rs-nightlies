#[doc = "Register `DDRPHYC_DDR3_MR1` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR1` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR1_SPEC>;
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
impl From<crate::W<DDRPHYC_DDR3_MR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DE` reader - DE"]
pub struct DE_R(crate::FieldReader<bool, bool>);
impl DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DE` writer - DE"]
pub struct DE_W<'a> {
    w: &'a mut W,
}
impl<'a> DE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `DIC0` reader - DIC0"]
pub struct DIC0_R(crate::FieldReader<bool, bool>);
impl DIC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIC0` writer - DIC0"]
pub struct DIC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RTT0` reader - RTT0"]
pub struct RTT0_R(crate::FieldReader<bool, bool>);
impl RTT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTT0` writer - RTT0"]
pub struct RTT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `AL` reader - AL"]
pub struct AL_R(crate::FieldReader<u8, u8>);
impl AL_R {
    pub(crate) fn new(bits: u8) -> Self {
        AL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AL` writer - AL"]
pub struct AL_W<'a> {
    w: &'a mut W,
}
impl<'a> AL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u16 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `DIC1` reader - DIC1"]
pub struct DIC1_R(crate::FieldReader<bool, bool>);
impl DIC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIC1` writer - DIC1"]
pub struct DIC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RTT1` reader - RTT1"]
pub struct RTT1_R(crate::FieldReader<bool, bool>);
impl RTT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTT1` writer - RTT1"]
pub struct RTT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `LEVEL` reader - LEVEL"]
pub struct LEVEL_R(crate::FieldReader<bool, bool>);
impl LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEVEL` writer - LEVEL"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RTT2` reader - RTT2"]
pub struct RTT2_R(crate::FieldReader<bool, bool>);
impl RTT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTT2` writer - RTT2"]
pub struct RTT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TDQS` reader - TDQS"]
pub struct TDQS_R(crate::FieldReader<bool, bool>);
impl TDQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDQS` writer - TDQS"]
pub struct TDQS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDQS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `QOFF` reader - QOFF"]
pub struct QOFF_R(crate::FieldReader<bool, bool>);
impl QOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        QOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QOFF` writer - QOFF"]
pub struct QOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> QOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    pub fn dic0(&self) -> DIC0_R {
        DIC0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    pub fn rtt0(&self) -> RTT0_R {
        RTT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    pub fn dic1(&self) -> DIC1_R {
        DIC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    pub fn rtt1(&self) -> RTT1_R {
        RTT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    pub fn rtt2(&self) -> RTT2_R {
        RTT2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    pub fn tdqs(&self) -> TDQS_R {
        TDQS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    pub fn qoff(&self) -> QOFF_R {
        QOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    pub fn de(&mut self) -> DE_W {
        DE_W { w: self }
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    pub fn dic0(&mut self) -> DIC0_W {
        DIC0_W { w: self }
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    pub fn rtt0(&mut self) -> RTT0_W {
        RTT0_W { w: self }
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    pub fn al(&mut self) -> AL_W {
        AL_W { w: self }
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    pub fn dic1(&mut self) -> DIC1_W {
        DIC1_W { w: self }
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    pub fn rtt1(&mut self) -> RTT1_W {
        RTT1_W { w: self }
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    pub fn rtt2(&mut self) -> RTT2_W {
        RTT2_W { w: self }
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    pub fn tdqs(&mut self) -> TDQS_W {
        TDQS_W { w: self }
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    pub fn qoff(&mut self) -> QOFF_W {
        QOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR1 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr1](index.html) module"]
pub struct DDRPHYC_DDR3_MR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR1 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
