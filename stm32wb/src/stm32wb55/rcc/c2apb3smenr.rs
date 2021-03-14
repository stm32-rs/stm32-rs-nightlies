#[doc = "Reader of register C2APB3SMENR"]
pub type R = crate::R<u32, super::C2APB3SMENR>;
#[doc = "Writer for register C2APB3SMENR"]
pub type W = crate::W<u32, super::C2APB3SMENR>;
#[doc = "Register C2APB3SMENR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::C2APB3SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `SMEN802`"]
pub type SMEN802_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMEN802`"]
pub struct SMEN802_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN802_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BLESMEN`"]
pub type BLESMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLESMEN`"]
pub struct BLESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLESMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes"]
    #[inline(always)]
    pub fn smen802(&self) -> SMEN802_R {
        SMEN802_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BLE interface clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn blesmen(&self) -> BLESMEN_R {
        BLESMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes"]
    #[inline(always)]
    pub fn smen802(&mut self) -> SMEN802_W {
        SMEN802_W { w: self }
    }
    #[doc = "Bit 0 - BLE interface clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn blesmen(&mut self) -> BLESMEN_W {
        BLESMEN_W { w: self }
    }
}
