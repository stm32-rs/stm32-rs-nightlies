#[doc = "Register `FMC_CSQCFGR1` reader"]
pub struct R(crate::R<FMC_CSQCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQCFGR1` writer"]
pub struct W(crate::W<FMC_CSQCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQCFGR1_SPEC>;
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
impl From<crate::W<FMC_CSQCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD2EN` reader - CMD2EN"]
pub struct CMD2EN_R(crate::FieldReader<bool, bool>);
impl CMD2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD2EN` writer - CMD2EN"]
pub struct CMD2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD2EN_W<'a> {
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
#[doc = "Field `DMADEN` reader - DMADEN"]
pub struct DMADEN_R(crate::FieldReader<bool, bool>);
impl DMADEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMADEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMADEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMADEN` writer - DMADEN"]
pub struct DMADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADEN_W<'a> {
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
#[doc = "Field `ACYNBR` reader - ACYNBR"]
pub struct ACYNBR_R(crate::FieldReader<u8, u8>);
impl ACYNBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACYNBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACYNBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACYNBR` writer - ACYNBR"]
pub struct ACYNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACYNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `CMD1` reader - CMD1"]
pub struct CMD1_R(crate::FieldReader<u8, u8>);
impl CMD1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD1` writer - CMD1"]
pub struct CMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CMD2` reader - CMD2"]
pub struct CMD2_R(crate::FieldReader<u8, u8>);
impl CMD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD2` writer - CMD2"]
pub struct CMD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CMD1T` reader - CMD1T"]
pub struct CMD1T_R(crate::FieldReader<bool, bool>);
impl CMD1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD1T` writer - CMD1T"]
pub struct CMD1T_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD1T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CMD2T` reader - CMD2T"]
pub struct CMD2T_R(crate::FieldReader<bool, bool>);
impl CMD2T_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD2T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD2T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD2T` writer - CMD2T"]
pub struct CMD2T_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD2T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    pub fn cmd2en(&self) -> CMD2EN_R {
        CMD2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    pub fn dmaden(&self) -> DMADEN_R {
        DMADEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    pub fn acynbr(&self) -> ACYNBR_R {
        ACYNBR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    pub fn cmd1(&self) -> CMD1_R {
        CMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    pub fn cmd2(&self) -> CMD2_R {
        CMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    pub fn cmd1t(&self) -> CMD1T_R {
        CMD1T_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    pub fn cmd2t(&self) -> CMD2T_R {
        CMD2T_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    pub fn cmd2en(&mut self) -> CMD2EN_W {
        CMD2EN_W { w: self }
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    pub fn dmaden(&mut self) -> DMADEN_W {
        DMADEN_W { w: self }
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    pub fn acynbr(&mut self) -> ACYNBR_W {
        ACYNBR_W { w: self }
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    pub fn cmd1(&mut self) -> CMD1_W {
        CMD1_W { w: self }
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    pub fn cmd2(&mut self) -> CMD2_W {
        CMD2_W { w: self }
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    pub fn cmd1t(&mut self) -> CMD1T_W {
        CMD1T_W { w: self }
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    pub fn cmd2t(&mut self) -> CMD2T_W {
        CMD2T_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr1](index.html) module"]
pub struct FMC_CSQCFGR1_SPEC;
impl crate::RegisterSpec for FMC_CSQCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqcfgr1::R](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr1::W](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_CSQCFGR1 to value 0"]
impl crate::Resettable for FMC_CSQCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
