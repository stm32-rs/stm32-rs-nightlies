#[doc = "Register `SEC1R` reader"]
pub struct R(crate::R<SEC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC1R` writer"]
pub struct W(crate::W<SEC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC1R_SPEC>;
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
impl From<crate::W<SEC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_LOCK` reader - BOOT_LOCK"]
pub struct BOOT_LOCK_R(crate::FieldReader<bool, bool>);
impl BOOT_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_LOCK` writer - BOOT_LOCK"]
pub struct BOOT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_LOCK_W<'a> {
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
#[doc = "Field `SEC_SIZE1` reader - SEC_SIZE1"]
pub struct SEC_SIZE1_R(crate::FieldReader<u8, u8>);
impl SEC_SIZE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_SIZE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_SIZE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_SIZE1` writer - SEC_SIZE1"]
pub struct SEC_SIZE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_SIZE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    pub fn sec_size1(&self) -> SEC_SIZE1_R {
        SEC_SIZE1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W {
        BOOT_LOCK_W { w: self }
    }
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    pub fn sec_size1(&mut self) -> SEC_SIZE1_W {
        SEC_SIZE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "securable area bank1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec1r](index.html) module"]
pub struct SEC1R_SPEC;
impl crate::RegisterSpec for SEC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec1r::R](R) reader structure"]
impl crate::Readable for SEC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec1r::W](W) writer structure"]
impl crate::Writable for SEC1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC1R to value 0xff00_ff00"]
impl crate::Resettable for SEC1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00_ff00
    }
}
