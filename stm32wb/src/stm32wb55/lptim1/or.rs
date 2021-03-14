#[doc = "Reader of register OR"]
pub type R = crate::R<u32, super::OR>;
#[doc = "Writer for register OR"]
pub type W = crate::W<u32, super::OR>;
#[doc = "Register OR `reset()`'s with value 0"]
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OR1`"]
pub type OR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OR1`"]
pub struct OR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OR1_W<'a> {
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
#[doc = "Reader of field `OR2`"]
pub type OR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OR2`"]
pub struct OR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OR2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Option register bit 1"]
    #[inline(always)]
    pub fn or1(&self) -> OR1_R {
        OR1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Option register bit 2"]
    #[inline(always)]
    pub fn or2(&self) -> OR2_R {
        OR2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option register bit 1"]
    #[inline(always)]
    pub fn or1(&mut self) -> OR1_W {
        OR1_W { w: self }
    }
    #[doc = "Bit 1 - Option register bit 2"]
    #[inline(always)]
    pub fn or2(&mut self) -> OR2_W {
        OR2_W { w: self }
    }
}
