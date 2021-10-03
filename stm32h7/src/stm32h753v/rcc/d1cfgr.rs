#[doc = "Register `D1CFGR` reader"]
pub struct R(crate::R<D1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D1CFGR` writer"]
pub struct W(crate::W<D1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D1CFGR_SPEC>;
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
impl From<crate::W<D1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D1 domain AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: sys_ck not divided"]
    DIV1 = 0,
    #[doc = "8: sys_ck divided by 2"]
    DIV2 = 8,
    #[doc = "9: sys_ck divided by 4"]
    DIV4 = 9,
    #[doc = "10: sys_ck divided by 8"]
    DIV8 = 10,
    #[doc = "11: sys_ck divided by 16"]
    DIV16 = 11,
    #[doc = "12: sys_ck divided by 64"]
    DIV64 = 12,
    #[doc = "13: sys_ck divided by 128"]
    DIV128 = 13,
    #[doc = "14: sys_ck divided by 256"]
    DIV256 = 14,
    #[doc = "15: sys_ck divided by 512"]
    DIV512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HPRE` reader - D1 domain AHB prescaler"]
pub struct HPRE_R(crate::FieldReader<u8, HPRE_A>);
impl HPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::DIV1),
            8 => Some(HPRE_A::DIV2),
            9 => Some(HPRE_A::DIV4),
            10 => Some(HPRE_A::DIV8),
            11 => Some(HPRE_A::DIV16),
            12 => Some(HPRE_A::DIV64),
            13 => Some(HPRE_A::DIV128),
            14 => Some(HPRE_A::DIV256),
            15 => Some(HPRE_A::DIV512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == HPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == HPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == HPRE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == HPRE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == HPRE_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == HPRE_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == HPRE_A::DIV512
    }
}
impl core::ops::Deref for HPRE_R {
    type Target = crate::FieldReader<u8, HPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPRE` writer - D1 domain AHB prescaler"]
pub struct HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "sys_ck not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::DIV1)
    }
    #[doc = "sys_ck divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::DIV2)
    }
    #[doc = "sys_ck divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::DIV4)
    }
    #[doc = "sys_ck divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::DIV8)
    }
    #[doc = "sys_ck divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::DIV16)
    }
    #[doc = "sys_ck divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::DIV64)
    }
    #[doc = "sys_ck divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::DIV128)
    }
    #[doc = "sys_ck divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::DIV256)
    }
    #[doc = "sys_ck divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::DIV512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "D1 domain APB3 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum D1PPRE_A {
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
impl From<D1PPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: D1PPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `D1PPRE` reader - D1 domain APB3 prescaler"]
pub struct D1PPRE_R(crate::FieldReader<u8, D1PPRE_A>);
impl D1PPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        D1PPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<D1PPRE_A> {
        match self.bits {
            0 => Some(D1PPRE_A::DIV1),
            4 => Some(D1PPRE_A::DIV2),
            5 => Some(D1PPRE_A::DIV4),
            6 => Some(D1PPRE_A::DIV8),
            7 => Some(D1PPRE_A::DIV16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == D1PPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == D1PPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == D1PPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == D1PPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == D1PPRE_A::DIV16
    }
}
impl core::ops::Deref for D1PPRE_R {
    type Target = crate::FieldReader<u8, D1PPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D1PPRE` writer - D1 domain APB3 prescaler"]
pub struct D1PPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> D1PPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D1PPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(D1PPRE_A::DIV1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(D1PPRE_A::DIV2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(D1PPRE_A::DIV4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(D1PPRE_A::DIV8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(D1PPRE_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "D1 domain Core prescaler"]
pub type D1CPRE_A = HPRE_A;
#[doc = "Field `D1CPRE` reader - D1 domain Core prescaler"]
pub type D1CPRE_R = HPRE_R;
#[doc = "Field `D1CPRE` writer - D1 domain Core prescaler"]
pub struct D1CPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> D1CPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D1CPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "sys_ck not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV1)
    }
    #[doc = "sys_ck divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV2)
    }
    #[doc = "sys_ck divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV4)
    }
    #[doc = "sys_ck divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV8)
    }
    #[doc = "sys_ck divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV16)
    }
    #[doc = "sys_ck divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV64)
    }
    #[doc = "sys_ck divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV128)
    }
    #[doc = "sys_ck divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV256)
    }
    #[doc = "sys_ck divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(D1CPRE_A::DIV512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    pub fn d1ppre(&self) -> D1PPRE_R {
        D1PPRE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    pub fn d1cpre(&self) -> D1CPRE_R {
        D1CPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W { w: self }
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    pub fn d1ppre(&mut self) -> D1PPRE_W {
        D1PPRE_W { w: self }
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    pub fn d1cpre(&mut self) -> D1CPRE_W {
        D1CPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Domain 1 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d1cfgr](index.html) module"]
pub struct D1CFGR_SPEC;
impl crate::RegisterSpec for D1CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d1cfgr::R](R) reader structure"]
impl crate::Readable for D1CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d1cfgr::W](W) writer structure"]
impl crate::Writable for D1CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D1CFGR to value 0"]
impl crate::Resettable for D1CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
