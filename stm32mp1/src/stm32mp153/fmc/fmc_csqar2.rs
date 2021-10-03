#[doc = "Register `FMC_CSQAR2` reader"]
pub struct R(crate::R<FMC_CSQAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQAR2` writer"]
pub struct W(crate::W<FMC_CSQAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQAR2_SPEC>;
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
impl From<crate::W<FMC_CSQAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDC5` reader - ADDC5"]
pub struct ADDC5_R(crate::FieldReader<u8, u8>);
impl ADDC5_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDC5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDC5` writer - ADDC5"]
pub struct ADDC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `NANDCEN0` reader - NANDCEN0"]
pub struct NANDCEN0_R(crate::FieldReader<bool, bool>);
impl NANDCEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        NANDCEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NANDCEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NANDCEN0` writer - NANDCEN0"]
pub struct NANDCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> NANDCEN0_W<'a> {
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
#[doc = "Field `NANDCEN1` reader - NANDCEN1"]
pub struct NANDCEN1_R(crate::FieldReader<bool, bool>);
impl NANDCEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NANDCEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NANDCEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NANDCEN1` writer - NANDCEN1"]
pub struct NANDCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> NANDCEN1_W<'a> {
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
#[doc = "Field `SAO` reader - SAO"]
pub struct SAO_R(crate::FieldReader<u16, u16>);
impl SAO_R {
    pub(crate) fn new(bits: u16) -> Self {
        SAO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAO` writer - SAO"]
pub struct SAO_W<'a> {
    w: &'a mut W,
}
impl<'a> SAO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    pub fn addc5(&self) -> ADDC5_R {
        ADDC5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    pub fn nandcen0(&self) -> NANDCEN0_R {
        NANDCEN0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    pub fn nandcen1(&self) -> NANDCEN1_R {
        NANDCEN1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    pub fn addc5(&mut self) -> ADDC5_W {
        ADDC5_W { w: self }
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    pub fn nandcen0(&mut self) -> NANDCEN0_W {
        NANDCEN0_W { w: self }
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    pub fn nandcen1(&mut self) -> NANDCEN1_W {
        NANDCEN1_W { w: self }
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    pub fn sao(&mut self) -> SAO_W {
        SAO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqar2](index.html) module"]
pub struct FMC_CSQAR2_SPEC;
impl crate::RegisterSpec for FMC_CSQAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqar2::R](R) reader structure"]
impl crate::Readable for FMC_CSQAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqar2::W](W) writer structure"]
impl crate::Writable for FMC_CSQAR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_CSQAR2 to value 0x0002_0000"]
impl crate::Resettable for FMC_CSQAR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0000
    }
}
