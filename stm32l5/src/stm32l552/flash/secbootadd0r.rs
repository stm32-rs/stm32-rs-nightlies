#[doc = "Register `SECBOOTADD0R` reader"]
pub struct R(crate::R<SECBOOTADD0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECBOOTADD0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECBOOTADD0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECBOOTADD0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECBOOTADD0R` writer"]
pub struct W(crate::W<SECBOOTADD0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECBOOTADD0R_SPEC>;
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
impl From<crate::W<SECBOOTADD0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECBOOTADD0R_SPEC>) -> Self {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SECBOOTADD0` writer - SECBOOTADD0"]
pub struct SECBOOTADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBOOTADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | ((value as u32 & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W {
        BOOT_LOCK_W { w: self }
    }
    #[doc = "Bits 7:31 - SECBOOTADD0"]
    #[inline(always)]
    pub fn secbootadd0(&mut self) -> SECBOOTADD0_W {
        SECBOOTADD0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FFlash secure boot address 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbootadd0r](index.html) module"]
pub struct SECBOOTADD0R_SPEC;
impl crate::RegisterSpec for SECBOOTADD0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secbootadd0r::R](R) reader structure"]
impl crate::Readable for SECBOOTADD0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secbootadd0r::W](W) writer structure"]
impl crate::Writable for SECBOOTADD0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECBOOTADD0R to value 0"]
impl crate::Resettable for SECBOOTADD0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
