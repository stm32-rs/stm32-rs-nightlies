#[doc = "Reader of register DDRCTRL_PCTRL_1"]
pub type R = crate::R<u32, super::DDRCTRL_PCTRL_1>;
#[doc = "Writer for register DDRCTRL_PCTRL_1"]
pub type W = crate::W<u32, super::DDRCTRL_PCTRL_1>;
#[doc = "Register DDRCTRL_PCTRL_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_PCTRL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORT_EN`"]
pub type PORT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT_EN`"]
pub struct PORT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_EN_W<'a> {
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
    #[doc = "Bit 0 - PORT_EN"]
    #[inline(always)]
    pub fn port_en(&self) -> PORT_EN_R {
        PORT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT_EN"]
    #[inline(always)]
    pub fn port_en(&mut self) -> PORT_EN_W {
        PORT_EN_W { w: self }
    }
}
