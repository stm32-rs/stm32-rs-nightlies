#[doc = "Reader of register IMR1"]
pub type R = crate::R<u32, super::IMR1>;
#[doc = "Writer for register IMR1"]
pub type W = crate::W<u32, super::IMR1>;
#[doc = "Register IMR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM1IM`"]
pub type TIM1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1IM`"]
pub struct TIM1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIM16IM`"]
pub type TIM16IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16IM`"]
pub struct TIM16IM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TIM17IM`"]
pub type TIM17IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17IM`"]
pub struct TIM17IM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `EXIT5IM`"]
pub type EXIT5IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT5IM`"]
pub struct EXIT5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT5IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `EXIT6IM`"]
pub type EXIT6IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT6IM`"]
pub struct EXIT6IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT6IM_W<'a> {
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
#[doc = "Reader of field `EXIT7IM`"]
pub type EXIT7IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT7IM`"]
pub struct EXIT7IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT7IM_W<'a> {
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
#[doc = "Reader of field `EXIT8IM`"]
pub type EXIT8IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT8IM`"]
pub struct EXIT8IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT8IM_W<'a> {
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
#[doc = "Reader of field `EXIT9IM`"]
pub type EXIT9IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT9IM`"]
pub struct EXIT9IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT9IM_W<'a> {
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
#[doc = "Reader of field `EXIT10IM`"]
pub type EXIT10IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT10IM`"]
pub struct EXIT10IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT10IM_W<'a> {
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
#[doc = "Reader of field `EXIT11IM`"]
pub type EXIT11IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT11IM`"]
pub struct EXIT11IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT11IM_W<'a> {
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
#[doc = "Reader of field `EXIT12IM`"]
pub type EXIT12IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT12IM`"]
pub struct EXIT12IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT12IM_W<'a> {
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
#[doc = "Reader of field `EXIT13IM`"]
pub type EXIT13IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT13IM`"]
pub struct EXIT13IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT13IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `EXIT14IM`"]
pub type EXIT14IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT14IM`"]
pub struct EXIT14IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT14IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `EXIT15IM`"]
pub type EXIT15IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXIT15IM`"]
pub struct EXIT15IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT15IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Peripheral TIM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim1im(&self) -> TIM1IM_R {
        TIM1IM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral TIM16 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim16im(&self) -> TIM16IM_R {
        TIM16IM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral TIM17 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim17im(&self) -> TIM17IM_R {
        TIM17IM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral EXIT5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit5im(&self) -> EXIT5IM_R {
        EXIT5IM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral EXIT6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit6im(&self) -> EXIT6IM_R {
        EXIT6IM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Peripheral EXIT7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit7im(&self) -> EXIT7IM_R {
        EXIT7IM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral EXIT8 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit8im(&self) -> EXIT8IM_R {
        EXIT8IM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral EXIT9 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit9im(&self) -> EXIT9IM_R {
        EXIT9IM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral EXIT10 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit10im(&self) -> EXIT10IM_R {
        EXIT10IM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral EXIT11 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit11im(&self) -> EXIT11IM_R {
        EXIT11IM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral EXIT12 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit12im(&self) -> EXIT12IM_R {
        EXIT12IM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Peripheral EXIT13 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit13im(&self) -> EXIT13IM_R {
        EXIT13IM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral EXIT14 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit14im(&self) -> EXIT14IM_R {
        EXIT14IM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Peripheral EXIT15 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit15im(&self) -> EXIT15IM_R {
        EXIT15IM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Peripheral TIM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim1im(&mut self) -> TIM1IM_W {
        TIM1IM_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral TIM16 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim16im(&mut self) -> TIM16IM_W {
        TIM16IM_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral TIM17 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim17im(&mut self) -> TIM17IM_W {
        TIM17IM_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral EXIT5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit5im(&mut self) -> EXIT5IM_W {
        EXIT5IM_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral EXIT6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit6im(&mut self) -> EXIT6IM_W {
        EXIT6IM_W { w: self }
    }
    #[doc = "Bit 23 - Peripheral EXIT7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit7im(&mut self) -> EXIT7IM_W {
        EXIT7IM_W { w: self }
    }
    #[doc = "Bit 24 - Peripheral EXIT8 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit8im(&mut self) -> EXIT8IM_W {
        EXIT8IM_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral EXIT9 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit9im(&mut self) -> EXIT9IM_W {
        EXIT9IM_W { w: self }
    }
    #[doc = "Bit 26 - Peripheral EXIT10 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit10im(&mut self) -> EXIT10IM_W {
        EXIT10IM_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral EXIT11 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit11im(&mut self) -> EXIT11IM_W {
        EXIT11IM_W { w: self }
    }
    #[doc = "Bit 28 - Peripheral EXIT12 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit12im(&mut self) -> EXIT12IM_W {
        EXIT12IM_W { w: self }
    }
    #[doc = "Bit 29 - Peripheral EXIT13 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit13im(&mut self) -> EXIT13IM_W {
        EXIT13IM_W { w: self }
    }
    #[doc = "Bit 30 - Peripheral EXIT14 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit14im(&mut self) -> EXIT14IM_W {
        EXIT14IM_W { w: self }
    }
    #[doc = "Bit 31 - Peripheral EXIT15 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit15im(&mut self) -> EXIT15IM_W {
        EXIT15IM_W { w: self }
    }
}
