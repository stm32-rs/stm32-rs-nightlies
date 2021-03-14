#[doc = "Reader of register D3CFGR"]
pub type R = crate::R<u32, super::D3CFGR>;
#[doc = "Writer for register D3CFGR"]
pub type W = crate::W<u32, super::D3CFGR>;
#[doc = "Register D3CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::D3CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "D3 domain APB4 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum D3PPRE_A {
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
impl From<D3PPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: D3PPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `D3PPRE`"]
pub type D3PPRE_R = crate::R<u8, D3PPRE_A>;
impl D3PPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, D3PPRE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(D3PPRE_A::DIV1),
            4 => Val(D3PPRE_A::DIV2),
            5 => Val(D3PPRE_A::DIV4),
            6 => Val(D3PPRE_A::DIV8),
            7 => Val(D3PPRE_A::DIV16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == D3PPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D3PPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D3PPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D3PPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D3PPRE_A::DIV16
    }
}
#[doc = "Write proxy for field `D3PPRE`"]
pub struct D3PPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> D3PPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D3PPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(D3PPRE_A::DIV1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(D3PPRE_A::DIV2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(D3PPRE_A::DIV4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(D3PPRE_A::DIV8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(D3PPRE_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    pub fn d3ppre(&self) -> D3PPRE_R {
        D3PPRE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    pub fn d3ppre(&mut self) -> D3PPRE_W {
        D3PPRE_W { w: self }
    }
}
