#[doc = "Register `FAR` reader"]
pub struct R(crate::R<FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAR` writer"]
pub struct W(crate::W<FAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAR_SPEC>;
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
impl From<crate::W<FAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIL_ECC_ADDR` reader - Bank 1 ECC error address"]
pub struct FAIL_ECC_ADDR_R(crate::FieldReader<u16, u16>);
impl FAIL_ECC_ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        FAIL_ECC_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAIL_ECC_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAIL_ECC_ADDR` writer - Bank 1 ECC error address"]
pub struct FAIL_ECC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAIL_ECC_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&mut self) -> FAIL_ECC_ADDR_W {
        FAIL_ECC_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH ECC fail address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [far](index.html) module"]
pub struct FAR_SPEC;
impl crate::RegisterSpec for FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [far::R](R) reader structure"]
impl crate::Readable for FAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [far::W](W) writer structure"]
impl crate::Writable for FAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAR to value 0"]
impl crate::Resettable for FAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
