#[doc = "Register `SDMMC_IDMABASER` reader"]
pub struct R(crate::R<SDMMC_IDMABASER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMABASER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMABASER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMABASER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMABASER` writer"]
pub struct W(crate::W<SDMMC_IDMABASER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMABASER_SPEC>;
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
impl From<crate::W<SDMMC_IDMABASER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMABASER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMABASE` reader - IDMABASE"]
pub struct IDMABASE_R(crate::FieldReader<u32, u32>);
impl IDMABASE_R {
    pub(crate) fn new(bits: u32) -> Self {
        IDMABASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABASE` writer - IDMABASE"]
pub struct IDMABASE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IDMABASE"]
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IDMABASE"]
    #[inline(always)]
    pub fn idmabase(&mut self) -> IDMABASE_W {
        IDMABASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabaser](index.html) module"]
pub struct SDMMC_IDMABASER_SPEC;
impl crate::RegisterSpec for SDMMC_IDMABASER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmabaser::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMABASER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabaser::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMABASER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_IDMABASER to value 0"]
impl crate::Resettable for SDMMC_IDMABASER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
