#[doc = "Register `FMC_CSQCFGR2` reader"]
pub struct R(crate::R<FMC_CSQCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQCFGR2` writer"]
pub struct W(crate::W<FMC_CSQCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQCFGR2_SPEC>;
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
impl From<crate::W<FMC_CSQCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQSDTEN` reader - SQSDTEN"]
pub struct SQSDTEN_R(crate::FieldReader<bool, bool>);
impl SQSDTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SQSDTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQSDTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQSDTEN` writer - SQSDTEN"]
pub struct SQSDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SQSDTEN_W<'a> {
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
#[doc = "Field `RCMD2EN` reader - RCMD2EN"]
pub struct RCMD2EN_R(crate::FieldReader<bool, bool>);
impl RCMD2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCMD2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCMD2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCMD2EN` writer - RCMD2EN"]
pub struct RCMD2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD2EN_W<'a> {
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
#[doc = "Field `DMASEN` reader - DMASEN"]
pub struct DMASEN_R(crate::FieldReader<bool, bool>);
impl DMASEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEN` writer - DMASEN"]
pub struct DMASEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEN_W<'a> {
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
#[doc = "Field `RCMD1` reader - RCMD1"]
pub struct RCMD1_R(crate::FieldReader<u8, u8>);
impl RCMD1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCMD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCMD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCMD1` writer - RCMD1"]
pub struct RCMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `RCMD2` reader - RCMD2"]
pub struct RCMD2_R(crate::FieldReader<u8, u8>);
impl RCMD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCMD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCMD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCMD2` writer - RCMD2"]
pub struct RCMD2_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RCMD1T` reader - RCMD1T"]
pub struct RCMD1T_R(crate::FieldReader<bool, bool>);
impl RCMD1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCMD1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCMD1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCMD1T` writer - RCMD1T"]
pub struct RCMD1T_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD1T_W<'a> {
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
#[doc = "Field `RCMD2T` reader - RCMD2T"]
pub struct RCMD2T_R(crate::FieldReader<bool, bool>);
impl RCMD2T_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCMD2T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCMD2T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCMD2T` writer - RCMD2T"]
pub struct RCMD2T_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD2T_W<'a> {
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
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    pub fn sqsdten(&self) -> SQSDTEN_R {
        SQSDTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    pub fn rcmd2en(&self) -> RCMD2EN_R {
        RCMD2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    pub fn dmasen(&self) -> DMASEN_R {
        DMASEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    pub fn rcmd1(&self) -> RCMD1_R {
        RCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    pub fn rcmd2(&self) -> RCMD2_R {
        RCMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    pub fn rcmd1t(&self) -> RCMD1T_R {
        RCMD1T_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    pub fn rcmd2t(&self) -> RCMD2T_R {
        RCMD2T_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    pub fn sqsdten(&mut self) -> SQSDTEN_W {
        SQSDTEN_W { w: self }
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    pub fn rcmd2en(&mut self) -> RCMD2EN_W {
        RCMD2EN_W { w: self }
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    pub fn dmasen(&mut self) -> DMASEN_W {
        DMASEN_W { w: self }
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    pub fn rcmd1(&mut self) -> RCMD1_W {
        RCMD1_W { w: self }
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    pub fn rcmd2(&mut self) -> RCMD2_W {
        RCMD2_W { w: self }
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    pub fn rcmd1t(&mut self) -> RCMD1T_W {
        RCMD1T_W { w: self }
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    pub fn rcmd2t(&mut self) -> RCMD2T_W {
        RCMD2T_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr2](index.html) module"]
pub struct FMC_CSQCFGR2_SPEC;
impl crate::RegisterSpec for FMC_CSQCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqcfgr2::R](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr2::W](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_CSQCFGR2 to value 0"]
impl crate::Resettable for FMC_CSQCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
