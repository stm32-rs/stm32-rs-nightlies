#[doc = "Register `STR` writer"]
pub struct W(crate::W<STR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STR_SPEC>;
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
impl From<crate::W<STR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCAL` writer - Digest calculation"]
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
#[doc = "Field `NBLW` writer - Number of valid bits in the last word of the message"]
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
impl W {
    #[doc = "Bit 8 - Digest calculation"]
    #[inline(always)]
    pub fn dcal(&mut self) -> DCAL_W {
        DCAL_W { w: self }
    }
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&mut self) -> NBLW_W {
        NBLW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "start register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](index.html) module"]
pub struct STR_SPEC;
impl crate::RegisterSpec for STR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [str::W](W) writer structure"]
impl crate::Writable for STR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for STR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
