#[doc = "Reader of register EXTI_TZENR2"]
pub type R = crate::R<u32, super::EXTI_TZENR2>;
#[doc = "Writer for register EXTI_TZENR2"]
pub type W = crate::W<u32, super::EXTI_TZENR2>;
#[doc = "Register EXTI_TZENR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_TZENR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TZEN41`"]
pub type TZEN41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN41`"]
pub struct TZEN41_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN41_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TZEN54`"]
pub type TZEN54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN54`"]
pub struct TZEN54_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN54_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TZEN55`"]
pub type TZEN55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN55`"]
pub struct TZEN55_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN55_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TZEN56`"]
pub type TZEN56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN56`"]
pub struct TZEN56_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN56_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TZEN57`"]
pub type TZEN57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN57`"]
pub struct TZEN57_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN57_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TZEN58`"]
pub type TZEN58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN58`"]
pub struct TZEN58_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN58_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TZEN59`"]
pub type TZEN59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN59`"]
pub struct TZEN59_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN59_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `TZEN60`"]
pub type TZEN60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN60`"]
pub struct TZEN60_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN60_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    pub fn tzen41(&self) -> TZEN41_R {
        TZEN41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    pub fn tzen54(&self) -> TZEN54_R {
        TZEN54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    pub fn tzen55(&self) -> TZEN55_R {
        TZEN55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    pub fn tzen56(&self) -> TZEN56_R {
        TZEN56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    pub fn tzen57(&self) -> TZEN57_R {
        TZEN57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    pub fn tzen58(&self) -> TZEN58_R {
        TZEN58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    pub fn tzen59(&self) -> TZEN59_R {
        TZEN59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    pub fn tzen60(&self) -> TZEN60_R {
        TZEN60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    pub fn tzen41(&mut self) -> TZEN41_W {
        TZEN41_W { w: self }
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    pub fn tzen54(&mut self) -> TZEN54_W {
        TZEN54_W { w: self }
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    pub fn tzen55(&mut self) -> TZEN55_W {
        TZEN55_W { w: self }
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    pub fn tzen56(&mut self) -> TZEN56_W {
        TZEN56_W { w: self }
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    pub fn tzen57(&mut self) -> TZEN57_W {
        TZEN57_W { w: self }
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    pub fn tzen58(&mut self) -> TZEN58_W {
        TZEN58_W { w: self }
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    pub fn tzen59(&mut self) -> TZEN59_W {
        TZEN59_W { w: self }
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    pub fn tzen60(&mut self) -> TZEN60_W {
        TZEN60_W { w: self }
    }
}
