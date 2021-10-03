#[doc = "Register `SRRVR` reader"]
pub struct R(crate::R<SRRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRRVR` writer"]
pub struct W(crate::W<SRRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRRVR_SPEC>;
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
impl From<crate::W<SRRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBRV` reader - cortex M0 access control register"]
pub struct SBRV_R(crate::FieldReader<u32, u32>);
impl SBRV_R {
    pub(crate) fn new(bits: u32) -> Self {
        SBRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBRV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBRV` writer - cortex M0 access control register"]
pub struct SBRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
#[doc = "Field `SBRSA` reader - Secure backup SRAM2a start address"]
pub struct SBRSA_R(crate::FieldReader<u8, u8>);
impl SBRSA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SBRSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBRSA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBRSA` writer - Secure backup SRAM2a start address"]
pub struct SBRSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `BRSD` reader - backup SRAM2a security disable"]
pub struct BRSD_R(crate::FieldReader<bool, bool>);
impl BRSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRSD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSD` writer - backup SRAM2a security disable"]
pub struct BRSD_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSD_W<'a> {
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
#[doc = "Field `SNBRSA` reader - Secure non backup SRAM2a start address"]
pub struct SNBRSA_R(crate::FieldReader<u8, u8>);
impl SNBRSA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNBRSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNBRSA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNBRSA` writer - Secure non backup SRAM2a start address"]
pub struct SNBRSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SNBRSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `C2OPT` reader - CPU2 cortex M0 boot reset vector memory selection"]
pub struct C2OPT_R(crate::FieldReader<bool, bool>);
impl C2OPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2OPT` writer - CPU2 cortex M0 boot reset vector memory selection"]
pub struct C2OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> C2OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `NBRSD` reader - non-backup SRAM2b security disable"]
pub struct NBRSD_R(crate::FieldReader<bool, bool>);
impl NBRSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBRSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBRSD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBRSD` writer - non-backup SRAM2b security disable"]
pub struct NBRSD_W<'a> {
    w: &'a mut W,
}
impl<'a> NBRSD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    pub fn sbrv(&mut self) -> SBRV_W {
        SBRV_W { w: self }
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    pub fn sbrsa(&mut self) -> SBRSA_W {
        SBRSA_W { w: self }
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    pub fn brsd(&mut self) -> BRSD_W {
        BRSD_W { w: self }
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    pub fn snbrsa(&mut self) -> SNBRSA_W {
        SNBRSA_W { w: self }
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
    #[inline(always)]
    pub fn c2opt(&mut self) -> C2OPT_W {
        C2OPT_W { w: self }
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    pub fn nbrsd(&mut self) -> NBRSD_W {
        NBRSD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srrvr](index.html) module"]
pub struct SRRVR_SPEC;
impl crate::RegisterSpec for SRRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srrvr::R](R) reader structure"]
impl crate::Readable for SRRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srrvr::W](W) writer structure"]
impl crate::Writable for SRRVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRRVR to value 0x0100_0000"]
impl crate::Resettable for SRRVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
