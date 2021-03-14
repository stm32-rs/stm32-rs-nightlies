#[doc = "Reader of register PCROP2ER"]
pub type R = crate::R<u32, super::PCROP2ER>;
#[doc = "Writer for register PCROP2ER"]
pub type W = crate::W<u32, super::PCROP2ER>;
#[doc = "Register PCROP2ER `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::PCROP2ER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "Reader of field `PCROP2_END`"]
pub type PCROP2_END_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCROP2_END`"]
pub struct PCROP2_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop2_end(&self) -> PCROP2_END_R {
        PCROP2_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop2_end(&mut self) -> PCROP2_END_W {
        PCROP2_END_W { w: self }
    }
}
