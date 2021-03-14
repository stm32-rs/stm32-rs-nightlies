#[doc = "Reader of register APB3FZ1"]
pub type R = crate::R<u32, super::APB3FZ1>;
#[doc = "Writer for register APB3FZ1"]
pub type W = crate::W<u32, super::APB3FZ1>;
#[doc = "Register APB3FZ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB3FZ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WWDG1`"]
pub type WWDG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDG1`"]
pub struct WWDG1_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - WWDG1 stop in debug mode"]
    #[inline(always)]
    pub fn wwdg1(&self) -> WWDG1_R {
        WWDG1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - WWDG1 stop in debug mode"]
    #[inline(always)]
    pub fn wwdg1(&mut self) -> WWDG1_W {
        WWDG1_W { w: self }
    }
}
