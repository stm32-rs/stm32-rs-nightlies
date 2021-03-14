#[doc = "Reader of register PCROP2SR"]
pub type R = crate::R<u32, super::PCROP2SR>;
#[doc = "Writer for register PCROP2SR"]
pub type W = crate::W<u32, super::PCROP2SR>;
#[doc = "Register PCROP2SR `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::PCROP2SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "Reader of field `PCROP2_STRT`"]
pub type PCROP2_STRT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCROP2_STRT`"]
pub struct PCROP2_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop2_strt(&self) -> PCROP2_STRT_R {
        PCROP2_STRT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop2_strt(&mut self) -> PCROP2_STRT_W {
        PCROP2_STRT_W { w: self }
    }
}
