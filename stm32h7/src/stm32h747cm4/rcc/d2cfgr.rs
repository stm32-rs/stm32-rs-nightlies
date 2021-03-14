#[doc = "Reader of register D2CFGR"]
pub type R = crate::R<u32, super::D2CFGR>;
#[doc = "Writer for register D2CFGR"]
pub type W = crate::W<u32, super::D2CFGR>;
#[doc = "Register D2CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::D2CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `D2PPRE1`"]
pub type D2PPRE1_R = crate::R<u8, D2PPRE1_A>;
impl D2PPRE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, D2PPRE1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(D2PPRE1_A::DIV1),
            4 => Val(D2PPRE1_A::DIV2),
            5 => Val(D2PPRE1_A::DIV4),
            6 => Val(D2PPRE1_A::DIV8),
            7 => Val(D2PPRE1_A::DIV16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == D2PPRE1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D2PPRE1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D2PPRE1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D2PPRE1_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D2PPRE1_A::DIV16
    }
}
#[doc = "Write proxy for field `D2PPRE1`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "D2 domain APB2 prescaler"]
pub type D2PPRE2_A = D2PPRE1_A;
#[doc = "Reader of field `D2PPRE2`"]
pub type D2PPRE2_R = crate::R<u8, D2PPRE1_A>;
#[doc = "Write proxy for field `D2PPRE2`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
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
}
