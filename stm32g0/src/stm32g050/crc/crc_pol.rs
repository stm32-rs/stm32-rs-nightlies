#[doc = "Register `CRC_POL` reader"]
pub struct R(crate::R<CRC_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_POL` writer"]
pub struct W(crate::W<CRC_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_POL_SPEC>;
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
impl From<crate::W<CRC_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL` reader - Programmable polynomial"]
pub struct POL_R(crate::FieldReader<u32, u32>);
impl POL_R {
    pub(crate) fn new(bits: u32) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL` writer - Programmable polynomial"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "polynomial\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_pol](index.html) module"]
pub struct CRC_POL_SPEC;
impl crate::RegisterSpec for CRC_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_pol::R](R) reader structure"]
impl crate::Readable for CRC_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_pol::W](W) writer structure"]
impl crate::Writable for CRC_POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_POL to value 0x04c1_1db7"]
impl crate::Resettable for CRC_POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04c1_1db7
    }
}
