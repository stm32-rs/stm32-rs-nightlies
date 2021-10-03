#[doc = "Register `OPTCR1` reader"]
pub struct R(crate::R<OPTCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR1` writer"]
pub struct W(crate::W<OPTCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR1_SPEC>;
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
impl From<crate::W<OPTCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_ADD0` reader - Boot base address when Boot pin =0"]
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
#[doc = "Field `BOOT_ADD0` writer - Boot base address when Boot pin =0"]
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
#[doc = "Field `BOOT_ADD1` reader - Boot base address when Boot pin =1"]
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
#[doc = "Field `BOOT_ADD1` writer - Boot base address when Boot pin =1"]
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
    #[doc = "Bits 0:15 - Boot base address when Boot pin =0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot base address when Boot pin =1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Boot base address when Boot pin =0"]
    #[inline(always)]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W {
        BOOT_ADD0_W { w: self }
    }
    #[doc = "Bits 16:31 - Boot base address when Boot pin =1"]
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
#[doc = "Flash option control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr1](index.html) module"]
pub struct OPTCR1_SPEC;
impl crate::RegisterSpec for OPTCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr1::R](R) reader structure"]
impl crate::Readable for OPTCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr1::W](W) writer structure"]
impl crate::Writable for OPTCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTCR1 to value 0x0fff_0000"]
impl crate::Resettable for OPTCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
