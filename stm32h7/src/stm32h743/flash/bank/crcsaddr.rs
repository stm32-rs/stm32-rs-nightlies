#[doc = "Register `CRCSADDR` reader"]
pub struct R(crate::R<CRCSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCSADDR` writer"]
pub struct W(crate::W<CRCSADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCSADDR_SPEC>;
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
impl From<crate::W<CRCSADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCSADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_START_ADDR` reader - CRC start address on bank 1"]
pub struct CRC_START_ADDR_R(crate::FieldReader<u32, u32>);
impl CRC_START_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CRC_START_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_START_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_START_ADDR` writer - CRC start address on bank 1"]
pub struct CRC_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CRC_START_ADDR_R {
        CRC_START_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&mut self) -> CRC_START_ADDR_W {
        CRC_START_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH CRC start address register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcsaddr](index.html) module"]
pub struct CRCSADDR_SPEC;
impl crate::RegisterSpec for CRCSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcsaddr::R](R) reader structure"]
impl crate::Readable for CRCSADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcsaddr::W](W) writer structure"]
impl crate::Writable for CRCSADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCSADDR to value 0"]
impl crate::Resettable for CRCSADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
