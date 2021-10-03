#[doc = "Register `JPEG_DIR` writer"]
pub struct W(crate::W<JPEG_DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JPEG_DIR_SPEC>;
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
impl From<crate::W<JPEG_DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JPEG_DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIN` writer - Data Input FIFO"]
pub struct DATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Input FIFO"]
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W {
        DATAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG data input register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_dir](index.html) module"]
pub struct JPEG_DIR_SPEC;
impl crate::RegisterSpec for JPEG_DIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [jpeg_dir::W](W) writer structure"]
impl crate::Writable for JPEG_DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JPEG_DIR to value 0"]
impl crate::Resettable for JPEG_DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
