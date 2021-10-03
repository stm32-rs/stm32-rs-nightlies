#[doc = "Register `BOOT_PRGR` reader"]
pub struct R(crate::R<BOOT_PRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_PRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_PRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_PRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_PRGR` writer"]
pub struct W(crate::W<BOOT_PRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_PRGR_SPEC>;
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
impl From<crate::W<BOOT_PRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_PRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_ADD0` reader - Boot address 0"]
pub struct BOOT_ADD0_R(crate::FieldReader<u16, u16>);
impl BOOT_ADD0_R {
    pub(crate) fn new(bits: u16) -> Self {
        BOOT_ADD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_ADD0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_ADD0` writer - Boot address 0"]
pub struct BOOT_ADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `BOOT_ADD1` reader - Boot address 1"]
pub struct BOOT_ADD1_R(crate::FieldReader<u16, u16>);
impl BOOT_ADD1_R {
    pub(crate) fn new(bits: u16) -> Self {
        BOOT_ADD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_ADD1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_ADD1` writer - Boot address 1"]
pub struct BOOT_ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W {
        BOOT_ADD0_W { w: self }
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W {
        BOOT_ADD1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH register with boot address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_prgr](index.html) module"]
pub struct BOOT_PRGR_SPEC;
impl crate::RegisterSpec for BOOT_PRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_prgr::R](R) reader structure"]
impl crate::Readable for BOOT_PRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_prgr::W](W) writer structure"]
impl crate::Writable for BOOT_PRGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOT_PRGR to value 0"]
impl crate::Resettable for BOOT_PRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
