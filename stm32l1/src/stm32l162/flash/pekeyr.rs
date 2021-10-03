#[doc = "Register `PEKEYR` writer"]
pub struct W(crate::W<PEKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEKEYR_SPEC>;
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
impl From<crate::W<PEKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEKEYR` writer - FLASH_PEC and data EEPROM key"]
pub struct PEKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> PEKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - FLASH_PEC and data EEPROM key"]
    #[inline(always)]
    pub fn pekeyr(&mut self) -> PEKEYR_W {
        PEKEYR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program/erase key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pekeyr](index.html) module"]
pub struct PEKEYR_SPEC;
impl crate::RegisterSpec for PEKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pekeyr::W](W) writer structure"]
impl crate::Writable for PEKEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEKEYR to value 0"]
impl crate::Resettable for PEKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
