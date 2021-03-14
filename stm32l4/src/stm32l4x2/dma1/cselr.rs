#[doc = "Reader of register CSELR"]
pub type R = crate::R<u32, super::CSELR>;
#[doc = "Writer for register CSELR"]
pub type W = crate::W<u32, super::CSELR>;
#[doc = "Register CSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA channel 7 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C7S_A {
    #[doc = "0: Default mapping"]
    NOMAPPING = 0,
    #[doc = "1: Mapping 1"]
    MAP1 = 1,
    #[doc = "2: Mapping 2"]
    MAP2 = 2,
    #[doc = "3: Mapping 3"]
    MAP3 = 3,
    #[doc = "4: Mapping 4"]
    MAP4 = 4,
    #[doc = "5: Mapping 5"]
    MAP5 = 5,
    #[doc = "6: Mapping 6"]
    MAP6 = 6,
    #[doc = "7: Mapping 7"]
    MAP7 = 7,
    #[doc = "8: Mapping 8"]
    MAP8 = 8,
    #[doc = "9: Mapping 9"]
    MAP9 = 9,
    #[doc = "10: Mapping 10"]
    MAP10 = 10,
    #[doc = "11: Mapping 11"]
    MAP11 = 11,
    #[doc = "12: Mapping 12"]
    MAP12 = 12,
    #[doc = "13: Mapping 13"]
    MAP13 = 13,
    #[doc = "14: Mapping 14"]
    MAP14 = 14,
    #[doc = "15: Mapping 15"]
    MAP15 = 15,
}
impl From<C7S_A> for u8 {
    #[inline(always)]
    fn from(variant: C7S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C7S`"]
pub type C7S_R = crate::R<u8, C7S_A>;
impl C7S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C7S_A {
        match self.bits {
            0 => C7S_A::NOMAPPING,
            1 => C7S_A::MAP1,
            2 => C7S_A::MAP2,
            3 => C7S_A::MAP3,
            4 => C7S_A::MAP4,
            5 => C7S_A::MAP5,
            6 => C7S_A::MAP6,
            7 => C7S_A::MAP7,
            8 => C7S_A::MAP8,
            9 => C7S_A::MAP9,
            10 => C7S_A::MAP10,
            11 => C7S_A::MAP11,
            12 => C7S_A::MAP12,
            13 => C7S_A::MAP13,
            14 => C7S_A::MAP14,
            15 => C7S_A::MAP15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOMAPPING`"]
    #[inline(always)]
    pub fn is_no_mapping(&self) -> bool {
        *self == C7S_A::NOMAPPING
    }
    #[doc = "Checks if the value of the field is `MAP1`"]
    #[inline(always)]
    pub fn is_map1(&self) -> bool {
        *self == C7S_A::MAP1
    }
    #[doc = "Checks if the value of the field is `MAP2`"]
    #[inline(always)]
    pub fn is_map2(&self) -> bool {
        *self == C7S_A::MAP2
    }
    #[doc = "Checks if the value of the field is `MAP3`"]
    #[inline(always)]
    pub fn is_map3(&self) -> bool {
        *self == C7S_A::MAP3
    }
    #[doc = "Checks if the value of the field is `MAP4`"]
    #[inline(always)]
    pub fn is_map4(&self) -> bool {
        *self == C7S_A::MAP4
    }
    #[doc = "Checks if the value of the field is `MAP5`"]
    #[inline(always)]
    pub fn is_map5(&self) -> bool {
        *self == C7S_A::MAP5
    }
    #[doc = "Checks if the value of the field is `MAP6`"]
    #[inline(always)]
    pub fn is_map6(&self) -> bool {
        *self == C7S_A::MAP6
    }
    #[doc = "Checks if the value of the field is `MAP7`"]
    #[inline(always)]
    pub fn is_map7(&self) -> bool {
        *self == C7S_A::MAP7
    }
    #[doc = "Checks if the value of the field is `MAP8`"]
    #[inline(always)]
    pub fn is_map8(&self) -> bool {
        *self == C7S_A::MAP8
    }
    #[doc = "Checks if the value of the field is `MAP9`"]
    #[inline(always)]
    pub fn is_map9(&self) -> bool {
        *self == C7S_A::MAP9
    }
    #[doc = "Checks if the value of the field is `MAP10`"]
    #[inline(always)]
    pub fn is_map10(&self) -> bool {
        *self == C7S_A::MAP10
    }
    #[doc = "Checks if the value of the field is `MAP11`"]
    #[inline(always)]
    pub fn is_map11(&self) -> bool {
        *self == C7S_A::MAP11
    }
    #[doc = "Checks if the value of the field is `MAP12`"]
    #[inline(always)]
    pub fn is_map12(&self) -> bool {
        *self == C7S_A::MAP12
    }
    #[doc = "Checks if the value of the field is `MAP13`"]
    #[inline(always)]
    pub fn is_map13(&self) -> bool {
        *self == C7S_A::MAP13
    }
    #[doc = "Checks if the value of the field is `MAP14`"]
    #[inline(always)]
    pub fn is_map14(&self) -> bool {
        *self == C7S_A::MAP14
    }
    #[doc = "Checks if the value of the field is `MAP15`"]
    #[inline(always)]
    pub fn is_map15(&self) -> bool {
        *self == C7S_A::MAP15
    }
}
#[doc = "Write proxy for field `C7S`"]
pub struct C7S_W<'a> {
    w: &'a mut W,
}
impl<'a> C7S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C7S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7S_A::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7S_A::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7S_A::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7S_A::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7S_A::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7S_A::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7S_A::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7S_A::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7S_A::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7S_A::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7S_A::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7S_A::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7S_A::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7S_A::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7S_A::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7S_A::MAP15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "DMA channel 6 selection"]
pub type C6S_A = C7S_A;
#[doc = "Reader of field `C6S`"]
pub type C6S_R = crate::R<u8, C7S_A>;
#[doc = "Write proxy for field `C6S`"]
pub struct C6S_W<'a> {
    w: &'a mut W,
}
impl<'a> C6S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C6S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7S_A::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7S_A::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7S_A::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7S_A::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7S_A::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7S_A::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7S_A::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7S_A::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7S_A::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7S_A::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7S_A::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7S_A::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7S_A::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7S_A::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7S_A::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7S_A::MAP15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "DMA channel 5 selection"]
pub type C5S_A = C7S_A;
#[doc = "Reader of field `C5S`"]
pub type C5S_R = crate::R<u8, C7S_A>;
#[doc = "Write proxy for field `C5S`"]
pub struct C5S_W<'a> {
    w: &'a mut W,
}
impl<'a> C5S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C5S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7S_A::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7S_A::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7S_A::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7S_A::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7S_A::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7S_A::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7S_A::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7S_A::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7S_A::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7S_A::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7S_A::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7S_A::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7S_A::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7S_A::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7S_A::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7S_A::MAP15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "DMA channel 4 selection"]
pub type C4S_A = C7S_A;
#[doc = "Reader of field `C4S`"]
pub type C4S_R = crate::R<u8, C7S_A>;
#[doc = "Write proxy for field `C4S`"]
pub struct C4S_W<'a> {
    w: &'a mut W,
}
impl<'a> C4S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C4S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7S_A::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7S_A::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7S_A::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7S_A::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7S_A::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7S_A::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7S_A::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7S_A::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7S_A::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7S_A::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7S_A::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7S_A::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7S_A::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7S_A::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7S_A::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7S_A::MAP15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "DMA channel 3 selection"]
pub type C3S_A = C7S_A;
#[doc = "Reader of field `C3S`"]
pub type C3S_R = crate::R<u8, C7S_A>;
#[doc = "Write proxy for field `C3S`"]
pub struct C3S_W<'a> {
    w: &'a mut W,
}
impl<'a> C3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C3S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7S_A::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7S_A::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7S_A::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7S_A::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7S_A::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7S_A::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7S_A::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7S_A::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7S_A::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7S_A::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7S_A::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7S_A::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7S_A::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7S_A::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7S_A::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7S_A::MAP15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "DMA channel 2 selection"]
pub type C2S_A = C7S_A;
#[doc = "Reader of field `C2S`"]
pub type C2S_R = crate::R<u8, C7S_A>;
#[doc = "Write proxy for field `C2S`"]
pub struct C2S_W<'a> {
    w: &'a mut W,
}
impl<'a> C2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C2S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7S_A::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7S_A::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7S_A::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7S_A::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7S_A::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7S_A::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7S_A::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7S_A::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7S_A::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7S_A::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7S_A::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7S_A::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7S_A::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7S_A::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7S_A::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7S_A::MAP15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "DMA channel 1 selection"]
pub type C1S_A = C7S_A;
#[doc = "Reader of field `C1S`"]
pub type C1S_R = crate::R<u8, C7S_A>;
#[doc = "Write proxy for field `C1S`"]
pub struct C1S_W<'a> {
    w: &'a mut W,
}
impl<'a> C1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C1S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7S_A::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7S_A::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7S_A::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7S_A::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7S_A::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7S_A::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7S_A::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7S_A::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7S_A::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7S_A::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7S_A::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7S_A::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7S_A::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7S_A::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7S_A::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7S_A::MAP15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&self) -> C7S_R {
        C7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&self) -> C6S_R {
        C6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&self) -> C5S_R {
        C5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&self) -> C4S_R {
        C4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&self) -> C3S_R {
        C3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&self) -> C2S_R {
        C2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&self) -> C1S_R {
        C1S_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&mut self) -> C7S_W {
        C7S_W { w: self }
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&mut self) -> C6S_W {
        C6S_W { w: self }
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&mut self) -> C5S_W {
        C5S_W { w: self }
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&mut self) -> C4S_W {
        C4S_W { w: self }
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&mut self) -> C3S_W {
        C3S_W { w: self }
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&mut self) -> C2S_W {
        C2S_W { w: self }
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&mut self) -> C1S_W {
        C1S_W { w: self }
    }
}
