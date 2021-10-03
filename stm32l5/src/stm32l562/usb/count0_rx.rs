#[doc = "Register `COUNT0_RX` reader"]
pub struct R(crate::R<COUNT0_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT0_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT0_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT0_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT0_RX` writer"]
pub struct W(crate::W<COUNT0_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT0_RX_SPEC>;
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
impl From<crate::W<COUNT0_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT0_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT0_RX` reader - Reception byte count"]
pub struct COUNT0_RX_R(crate::FieldReader<u16, u16>);
impl COUNT0_RX_R {
    pub(crate) fn new(bits: u16) -> Self {
        COUNT0_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT0_RX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_BLOCK` reader - Number of blocks"]
pub struct NUM_BLOCK_R(crate::FieldReader<u8, u8>);
impl NUM_BLOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_BLOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_BLOCK` writer - Number of blocks"]
pub struct NUM_BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_BLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u16 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `BL_SIZE` reader - Block size"]
pub struct BL_SIZE_R(crate::FieldReader<bool, bool>);
impl BL_SIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BL_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_SIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL_SIZE` writer - Block size"]
pub struct BL_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_SIZE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Reception byte count"]
    #[inline(always)]
    pub fn count0_rx(&self) -> COUNT0_RX_R {
        COUNT0_RX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    pub fn num_block(&self) -> NUM_BLOCK_R {
        NUM_BLOCK_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    pub fn bl_size(&self) -> BL_SIZE_R {
        BL_SIZE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    pub fn num_block(&mut self) -> NUM_BLOCK_W {
        NUM_BLOCK_W { w: self }
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    pub fn bl_size(&mut self) -> BL_SIZE_W {
        BL_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count0_rx](index.html) module"]
pub struct COUNT0_RX_SPEC;
impl crate::RegisterSpec for COUNT0_RX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [count0_rx::R](R) reader structure"]
impl crate::Readable for COUNT0_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count0_rx::W](W) writer structure"]
impl crate::Writable for COUNT0_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNT0_RX to value 0"]
impl crate::Resettable for COUNT0_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
