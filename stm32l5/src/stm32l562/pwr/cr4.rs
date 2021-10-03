#[doc = "Register `CR4` reader"]
pub struct R(crate::R<CR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR4` writer"]
pub struct W(crate::W<CR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR4_SPEC>;
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
impl From<crate::W<CR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMPSLPEN` reader - SMPSLPEN"]
pub struct SMPSLPEN_R(crate::FieldReader<bool, bool>);
impl SMPSLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSLPEN` writer - SMPSLPEN"]
pub struct SMPSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SMPSFSTEN` reader - SMPSFSTEN"]
pub struct SMPSFSTEN_R(crate::FieldReader<bool, bool>);
impl SMPSFSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSFSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSFSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSFSTEN` writer - SMPSFSTEN"]
pub struct SMPSFSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSFSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `EXTSMPSEN` reader - EXTSMPSEN"]
pub struct EXTSMPSEN_R(crate::FieldReader<bool, bool>);
impl EXTSMPSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTSMPSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTSMPSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSMPSEN` writer - EXTSMPSEN"]
pub struct EXTSMPSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSMPSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SMPSBYP` reader - SMPSBYP"]
pub struct SMPSBYP_R(crate::FieldReader<bool, bool>);
impl SMPSBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSBYP` writer - SMPSBYP"]
pub struct SMPSBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `VBRS` reader - VBAT battery charging resistor selection"]
pub struct VBRS_R(crate::FieldReader<bool, bool>);
impl VBRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub struct VBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBRS_W<'a> {
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
#[doc = "Field `VBE` reader - VBAT battery charging enable"]
pub struct VBE_R(crate::FieldReader<bool, bool>);
impl VBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBE` writer - VBAT battery charging enable"]
pub struct VBE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBE_W<'a> {
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
#[doc = "Field `WUPP5` reader - Wakeup pin WKUP5 polarity"]
pub struct WUPP5_R(crate::FieldReader<bool, bool>);
impl WUPP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP5` writer - Wakeup pin WKUP5 polarity"]
pub struct WUPP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP5_W<'a> {
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
#[doc = "Field `WUPP4` reader - Wakeup pin WKUP4 polarity"]
pub struct WUPP4_R(crate::FieldReader<bool, bool>);
impl WUPP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP4` writer - Wakeup pin WKUP4 polarity"]
pub struct WUPP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP4_W<'a> {
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
#[doc = "Field `WUPP3` reader - Wakeup pin WKUP3 polarity"]
pub struct WUPP3_R(crate::FieldReader<bool, bool>);
impl WUPP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP3` writer - Wakeup pin WKUP3 polarity"]
pub struct WUPP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP3_W<'a> {
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
#[doc = "Field `WUPP2` reader - Wakeup pin WKUP2 polarity"]
pub struct WUPP2_R(crate::FieldReader<bool, bool>);
impl WUPP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP2` writer - Wakeup pin WKUP2 polarity"]
pub struct WUPP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP2_W<'a> {
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
#[doc = "Field `WUPP1` reader - Wakeup pin WKUP1 polarity"]
pub struct WUPP1_R(crate::FieldReader<bool, bool>);
impl WUPP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP1` writer - Wakeup pin WKUP1 polarity"]
pub struct WUPP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP1_W<'a> {
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
    #[doc = "Bit 15 - SMPSLPEN"]
    #[inline(always)]
    pub fn smpslpen(&self) -> SMPSLPEN_R {
        SMPSLPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SMPSFSTEN"]
    #[inline(always)]
    pub fn smpsfsten(&self) -> SMPSFSTEN_R {
        SMPSFSTEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTSMPSEN"]
    #[inline(always)]
    pub fn extsmpsen(&self) -> EXTSMPSEN_R {
        EXTSMPSEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SMPSBYP"]
    #[inline(always)]
    pub fn smpsbyp(&self) -> SMPSBYP_R {
        SMPSBYP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - SMPSLPEN"]
    #[inline(always)]
    pub fn smpslpen(&mut self) -> SMPSLPEN_W {
        SMPSLPEN_W { w: self }
    }
    #[doc = "Bit 14 - SMPSFSTEN"]
    #[inline(always)]
    pub fn smpsfsten(&mut self) -> SMPSFSTEN_W {
        SMPSFSTEN_W { w: self }
    }
    #[doc = "Bit 13 - EXTSMPSEN"]
    #[inline(always)]
    pub fn extsmpsen(&mut self) -> EXTSMPSEN_W {
        EXTSMPSEN_W { w: self }
    }
    #[doc = "Bit 12 - SMPSBYP"]
    #[inline(always)]
    pub fn smpsbyp(&mut self) -> SMPSBYP_W {
        SMPSBYP_W { w: self }
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W {
        VBRS_W { w: self }
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W {
        VBE_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W {
        WUPP5_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W {
        WUPP4_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W {
        WUPP3_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W {
        WUPP2_W { w: self }
    }
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W {
        WUPP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr4](index.html) module"]
pub struct CR4_SPEC;
impl crate::RegisterSpec for CR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr4::R](R) reader structure"]
impl crate::Readable for CR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr4::W](W) writer structure"]
impl crate::Writable for CR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
