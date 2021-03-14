#[doc = "Reader of register C2APB3ENR"]
pub type R = crate::R<u32, super::C2APB3ENR>;
#[doc = "Writer for register C2APB3ENR"]
pub type W = crate::W<u32, super::C2APB3ENR>;
#[doc = "Register C2APB3ENR `reset()`'s with value 0"]
impl crate::ResetValue for super::C2APB3ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN802`"]
pub type EN802_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN802`"]
pub struct EN802_W<'a> {
    w: &'a mut W,
}
impl<'a> EN802_W<'a> {
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
#[doc = "Reader of field `BLEEN`"]
pub type BLEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLEEN`"]
pub struct BLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEEN_W<'a> {
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
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    pub fn en802(&self) -> EN802_R {
        EN802_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    pub fn bleen(&self) -> BLEEN_R {
        BLEEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    pub fn en802(&mut self) -> EN802_W {
        EN802_W { w: self }
    }
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    pub fn bleen(&mut self) -> BLEEN_W {
        BLEEN_W { w: self }
    }
}
