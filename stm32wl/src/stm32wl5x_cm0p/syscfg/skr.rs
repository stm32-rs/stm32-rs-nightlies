#[doc = "Register `SKR` writer"]
pub struct W(crate::W<SKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SKR_SPEC>;
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
impl From<crate::W<SKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM2 write protection key for software erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "202: Step 1 to remove SRAM2ER bits write protection"]
    STEP1 = 202,
    #[doc = "83: Step 2 to remove SRAM2ER bits write protection"]
    STEP2 = 83,
    #[doc = "17: Activate SRAM2ER bits write protection"]
    WRITEPROTECT = 17,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - SRAM2 write protection key for software erase"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Step 1 to remove SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn step1(self) -> &'a mut W {
        self.variant(KEY_AW::STEP1)
    }
    #[doc = "Step 2 to remove SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn step2(self) -> &'a mut W {
        self.variant(KEY_AW::STEP2)
    }
    #[doc = "Activate SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn write_protect(self) -> &'a mut W {
        self.variant(KEY_AW::WRITEPROTECT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - SRAM2 write protection key for software erase"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SKR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [skr](index.html) module"]
pub struct SKR_SPEC;
impl crate::RegisterSpec for SKR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [skr::W](W) writer structure"]
impl crate::Writable for SKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
