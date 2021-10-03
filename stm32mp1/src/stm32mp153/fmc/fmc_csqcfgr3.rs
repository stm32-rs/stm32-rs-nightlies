#[doc = "Register `FMC_CSQCFGR3` reader"]
pub struct R(crate::R<FMC_CSQCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQCFGR3` writer"]
pub struct W(crate::W<FMC_CSQCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQCFGR3_SPEC>;
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
impl From<crate::W<FMC_CSQCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNBR` reader - SNBR"]
pub struct SNBR_R(crate::FieldReader<u8, u8>);
impl SNBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNBR` writer - SNBR"]
pub struct SNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `AC1T` reader - AC1T"]
pub struct AC1T_R(crate::FieldReader<bool, bool>);
impl AC1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        AC1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC1T` writer - AC1T"]
pub struct AC1T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC1T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `AC2T` reader - AC2T"]
pub struct AC2T_R(crate::FieldReader<bool, bool>);
impl AC2T_R {
    pub(crate) fn new(bits: bool) -> Self {
        AC2T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC2T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC2T` writer - AC2T"]
pub struct AC2T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC2T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `AC3T` reader - AC3T"]
pub struct AC3T_R(crate::FieldReader<bool, bool>);
impl AC3T_R {
    pub(crate) fn new(bits: bool) -> Self {
        AC3T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC3T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC3T` writer - AC3T"]
pub struct AC3T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC3T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `AC4T` reader - AC4T"]
pub struct AC4T_R(crate::FieldReader<bool, bool>);
impl AC4T_R {
    pub(crate) fn new(bits: bool) -> Self {
        AC4T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC4T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC4T` writer - AC4T"]
pub struct AC4T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC4T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `AC5T` reader - AC5T"]
pub struct AC5T_R(crate::FieldReader<bool, bool>);
impl AC5T_R {
    pub(crate) fn new(bits: bool) -> Self {
        AC5T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC5T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC5T` writer - AC5T"]
pub struct AC5T_W<'a> {
    w: &'a mut W,
}
impl<'a> AC5T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SDT` reader - SDT"]
pub struct SDT_R(crate::FieldReader<bool, bool>);
impl SDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDT` writer - SDT"]
pub struct SDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `RAC1T` reader - RAC1T"]
pub struct RAC1T_R(crate::FieldReader<bool, bool>);
impl RAC1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAC1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAC1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAC1T` writer - RAC1T"]
pub struct RAC1T_W<'a> {
    w: &'a mut W,
}
impl<'a> RAC1T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RAC2T` reader - RAC2T"]
pub struct RAC2T_R(crate::FieldReader<bool, bool>);
impl RAC2T_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAC2T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAC2T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAC2T` writer - RAC2T"]
pub struct RAC2T_W<'a> {
    w: &'a mut W,
}
impl<'a> RAC2T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    pub fn snbr(&self) -> SNBR_R {
        SNBR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    pub fn ac1t(&self) -> AC1T_R {
        AC1T_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    pub fn ac2t(&self) -> AC2T_R {
        AC2T_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    pub fn ac3t(&self) -> AC3T_R {
        AC3T_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    pub fn ac4t(&self) -> AC4T_R {
        AC4T_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    pub fn ac5t(&self) -> AC5T_R {
        AC5T_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    pub fn rac1t(&self) -> RAC1T_R {
        RAC1T_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    pub fn rac2t(&self) -> RAC2T_R {
        RAC2T_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    pub fn snbr(&mut self) -> SNBR_W {
        SNBR_W { w: self }
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    pub fn ac1t(&mut self) -> AC1T_W {
        AC1T_W { w: self }
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    pub fn ac2t(&mut self) -> AC2T_W {
        AC2T_W { w: self }
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    pub fn ac3t(&mut self) -> AC3T_W {
        AC3T_W { w: self }
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    pub fn ac4t(&mut self) -> AC4T_W {
        AC4T_W { w: self }
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    pub fn ac5t(&mut self) -> AC5T_W {
        AC5T_W { w: self }
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    pub fn sdt(&mut self) -> SDT_W {
        SDT_W { w: self }
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    pub fn rac1t(&mut self) -> RAC1T_W {
        RAC1T_W { w: self }
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    pub fn rac2t(&mut self) -> RAC2T_W {
        RAC2T_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND sequencer configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr3](index.html) module"]
pub struct FMC_CSQCFGR3_SPEC;
impl crate::RegisterSpec for FMC_CSQCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqcfgr3::R](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr3::W](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_CSQCFGR3 to value 0"]
impl crate::Resettable for FMC_CSQCFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
