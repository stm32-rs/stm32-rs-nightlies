#[doc = "Register `RNG_HTCR` reader"]
pub struct R(crate::R<RNG_HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_HTCR` writer"]
pub struct W(crate::W<RNG_HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_HTCR_SPEC>;
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
impl From<crate::W<RNG_HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTCFG` reader - health test configuration"]
pub struct HTCFG_R(crate::FieldReader<u32, u32>);
impl HTCFG_R {
    pub(crate) fn new(bits: u32) -> Self {
        HTCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTCFG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTCFG` writer - health test configuration"]
pub struct HTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HTCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&mut self) -> HTCFG_W {
        HTCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_htcr](index.html) module"]
pub struct RNG_HTCR_SPEC;
impl crate::RegisterSpec for RNG_HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_htcr::R](R) reader structure"]
impl crate::Readable for RNG_HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_htcr::W](W) writer structure"]
impl crate::Writable for RNG_HTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNG_HTCR to value 0x000c_aa74"]
impl crate::Resettable for RNG_HTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_aa74
    }
}
