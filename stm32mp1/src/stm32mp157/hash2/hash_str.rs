#[doc = "Register `HASH_STR` reader"]
pub struct R(crate::R<HASH_STR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_STR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_STR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_STR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_STR` writer"]
pub struct W(crate::W<HASH_STR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_STR_SPEC>;
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
impl From<crate::W<HASH_STR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_STR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBLW` reader - NBLW"]
pub struct NBLW_R(crate::FieldReader<u8, u8>);
impl NBLW_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBLW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBLW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBLW` writer - NBLW"]
pub struct NBLW_W<'a> {
    w: &'a mut W,
}
impl<'a> NBLW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `DCAL` writer - DCAL"]
pub struct DCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - NBLW"]
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - NBLW"]
    #[inline(always)]
    pub fn nblw(&mut self) -> NBLW_W {
        NBLW_W { w: self }
    }
    #[doc = "Bit 8 - DCAL"]
    #[inline(always)]
    pub fn dcal(&mut self) -> DCAL_W {
        DCAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_str](index.html) module"]
pub struct HASH_STR_SPEC;
impl crate::RegisterSpec for HASH_STR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_str::R](R) reader structure"]
impl crate::Readable for HASH_STR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_str::W](W) writer structure"]
impl crate::Writable for HASH_STR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_STR to value 0"]
impl crate::Resettable for HASH_STR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
