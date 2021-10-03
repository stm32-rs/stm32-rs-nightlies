#[doc = "Register `GICV_APR0` reader"]
pub struct R(crate::R<GICV_APR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_APR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_APR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_APR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICV_APR0` writer"]
pub struct W(crate::W<GICV_APR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICV_APR0_SPEC>;
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
impl From<crate::W<GICV_APR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICV_APR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APR0` reader - APR0"]
pub struct APR0_R(crate::FieldReader<u32, u32>);
impl APR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        APR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APR0` writer - APR0"]
pub struct APR0_W<'a> {
    w: &'a mut W,
}
impl<'a> APR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    pub fn apr0(&self) -> APR0_R {
        APR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    pub fn apr0(&mut self) -> APR0_W {
        APR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The GICV_APR0 is an alias of GICH_APR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_apr0](index.html) module"]
pub struct GICV_APR0_SPEC;
impl crate::RegisterSpec for GICV_APR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_apr0::R](R) reader structure"]
impl crate::Readable for GICV_APR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicv_apr0::W](W) writer structure"]
impl crate::Writable for GICV_APR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICV_APR0 to value 0"]
impl crate::Resettable for GICV_APR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
