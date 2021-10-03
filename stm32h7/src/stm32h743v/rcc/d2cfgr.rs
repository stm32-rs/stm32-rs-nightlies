#[doc = "Register `D2CFGR` reader"]
pub struct R(crate::R<D2CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D2CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D2CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D2CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D2CFGR` writer"]
pub struct W(crate::W<D2CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D2CFGR_SPEC>;
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
impl From<crate::W<D2CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D2CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D2 domain APB1 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum D2PPRE1_A {
    #[doc = "0: rcc_hclk not divided"]
    DIV1 = 0,
    #[doc = "4: rcc_hclk divided by 2"]
    DIV2 = 4,
    #[doc = "5: rcc_hclk divided by 4"]
    DIV4 = 5,
    #[doc = "6: rcc_hclk divided by 8"]
    DIV8 = 6,
    #[doc = "7: rcc_hclk divided by 16"]
    DIV16 = 7,
}
impl From<D2PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: D2PPRE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `D2PPRE1` reader - D2 domain APB1 prescaler"]
pub struct D2PPRE1_R(crate::FieldReader<u8, D2PPRE1_A>);
impl D2PPRE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        D2PPRE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<D2PPRE1_A> {
        match self.bits {
            0 => Some(D2PPRE1_A::DIV1),
            4 => Some(D2PPRE1_A::DIV2),
            5 => Some(D2PPRE1_A::DIV4),
            6 => Some(D2PPRE1_A::DIV8),
            7 => Some(D2PPRE1_A::DIV16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == D2PPRE1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == D2PPRE1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == D2PPRE1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == D2PPRE1_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == D2PPRE1_A::DIV16
    }
}
impl core::ops::Deref for D2PPRE1_R {
    type Target = crate::FieldReader<u8, D2PPRE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D2PPRE1` writer - D2 domain APB1 prescaler"]
pub struct D2PPRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> D2PPRE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D2PPRE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(D2PPRE1_A::DIV1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(D2PPRE1_A::DIV2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(D2PPRE1_A::DIV4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(D2PPRE1_A::DIV8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(D2PPRE1_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "D2 domain APB2 prescaler"]
pub type D2PPRE2_A = D2PPRE1_A;
#[doc = "Field `D2PPRE2` reader - D2 domain APB2 prescaler"]
pub type D2PPRE2_R = D2PPRE1_R;
#[doc = "Field `D2PPRE2` writer - D2 domain APB2 prescaler"]
pub struct D2PPRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> D2PPRE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D2PPRE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(D2PPRE2_A::DIV1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(D2PPRE2_A::DIV2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(D2PPRE2_A::DIV4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(D2PPRE2_A::DIV8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(D2PPRE2_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    pub fn d2ppre1(&self) -> D2PPRE1_R {
        D2PPRE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    pub fn d2ppre2(&self) -> D2PPRE2_R {
        D2PPRE2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    pub fn d2ppre1(&mut self) -> D2PPRE1_W {
        D2PPRE1_W { w: self }
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    pub fn d2ppre2(&mut self) -> D2PPRE2_W {
        D2PPRE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Domain 2 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2cfgr](index.html) module"]
pub struct D2CFGR_SPEC;
impl crate::RegisterSpec for D2CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d2cfgr::R](R) reader structure"]
impl crate::Readable for D2CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d2cfgr::W](W) writer structure"]
impl crate::Writable for D2CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D2CFGR to value 0"]
impl crate::Resettable for D2CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
