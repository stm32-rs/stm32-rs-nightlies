#[doc = "Register `HTCR` reader"]
pub struct R(crate::R<HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTCR` writer"]
pub struct W(crate::W<HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTCR_SPEC>;
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
impl From<crate::W<HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "health test configuration\n\nValue on reset: 23118"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum HTCFG_A {
    #[doc = "391711420: Magic number to be written before any write (0x1759_0ABC)"]
    MAGIC = 391711420,
    #[doc = "43636: Recommended value for RNG certification (0x0000_AA74)"]
    RECOMMENDED = 43636,
}
impl From<HTCFG_A> for u32 {
    #[inline(always)]
    fn from(variant: HTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HTCFG` reader - health test configuration"]
pub struct HTCFG_R(crate::FieldReader<u32, HTCFG_A>);
impl HTCFG_R {
    pub(crate) fn new(bits: u32) -> Self {
        HTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HTCFG_A> {
        match self.bits {
            391711420 => Some(HTCFG_A::MAGIC),
            43636 => Some(HTCFG_A::RECOMMENDED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAGIC`"]
    #[inline(always)]
    pub fn is_magic(&self) -> bool {
        **self == HTCFG_A::MAGIC
    }
    #[doc = "Checks if the value of the field is `RECOMMENDED`"]
    #[inline(always)]
    pub fn is_recommended(&self) -> bool {
        **self == HTCFG_A::RECOMMENDED
    }
}
impl core::ops::Deref for HTCFG_R {
    type Target = crate::FieldReader<u32, HTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTCFG` writer - health test configuration"]
pub struct HTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Magic number to be written before any write (0x1759_0ABC)"]
    #[inline(always)]
    pub fn magic(self) -> &'a mut W {
        self.variant(HTCFG_A::MAGIC)
    }
    #[doc = "Recommended value for RNG certification (0x0000_AA74)"]
    #[inline(always)]
    pub fn recommended(self) -> &'a mut W {
        self.variant(HTCFG_A::RECOMMENDED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&mut self) -> HTCFG_W {
        HTCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "health test control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htcr](index.html) module"]
pub struct HTCR_SPEC;
impl crate::RegisterSpec for HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htcr::R](R) reader structure"]
impl crate::Readable for HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htcr::W](W) writer structure"]
impl crate::Writable for HTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTCR to value 0x5a4e"]
impl crate::Resettable for HTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5a4e
    }
}
