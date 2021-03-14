#[doc = "Reader of register C1EMR2"]
pub type R = crate::R<u32, super::C1EMR2>;
#[doc = "Writer for register C1EMR2"]
pub type W = crate::W<u32, super::C1EMR2>;
#[doc = "Register C1EMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1EMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM`"]
pub type EM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EM`"]
pub struct EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&mut self) -> EM_W {
        EM_W { w: self }
    }
}
