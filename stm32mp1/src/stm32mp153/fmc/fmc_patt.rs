#[doc = "Register `FMC_PATT` reader"]
pub struct R(crate::R<FMC_PATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PATT` writer"]
pub struct W(crate::W<FMC_PATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PATT_SPEC>;
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
impl From<crate::W<FMC_PATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTSET` reader - ATTSET"]
pub struct ATTSET_R(crate::FieldReader<u8, u8>);
impl ATTSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTSET` writer - ATTSET"]
pub struct ATTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ATTWAIT` reader - ATTWAIT"]
pub struct ATTWAIT_R(crate::FieldReader<u8, u8>);
impl ATTWAIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTWAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTWAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTWAIT` writer - ATTWAIT"]
pub struct ATTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ATTHOLD` reader - ATTHOLD"]
pub struct ATTHOLD_R(crate::FieldReader<u8, u8>);
impl ATTHOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTHOLD` writer - ATTHOLD"]
pub struct ATTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ATTHIZ` reader - ATTHIZ"]
pub struct ATTHIZ_R(crate::FieldReader<u8, u8>);
impl ATTHIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTHIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTHIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTHIZ` writer - ATTHIZ"]
pub struct ATTHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ATTSET"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAIT"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLD"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ATTHIZ"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATTSET"]
    #[inline(always)]
    pub fn attset(&mut self) -> ATTSET_W {
        ATTSET_W { w: self }
    }
    #[doc = "Bits 8:15 - ATTWAIT"]
    #[inline(always)]
    pub fn attwait(&mut self) -> ATTWAIT_W {
        ATTWAIT_W { w: self }
    }
    #[doc = "Bits 16:23 - ATTHOLD"]
    #[inline(always)]
    pub fn atthold(&mut self) -> ATTHOLD_W {
        ATTHOLD_W { w: self }
    }
    #[doc = "Bits 24:31 - ATTHIZ"]
    #[inline(always)]
    pub fn atthiz(&mut self) -> ATTHIZ_W {
        ATTHIZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_patt](index.html) module"]
pub struct FMC_PATT_SPEC;
impl crate::RegisterSpec for FMC_PATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_patt::R](R) reader structure"]
impl crate::Readable for FMC_PATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_patt::W](W) writer structure"]
impl crate::Writable for FMC_PATT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_PATT to value 0x0a0a_0a0a"]
impl crate::Resettable for FMC_PATT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a0a_0a0a
    }
}
