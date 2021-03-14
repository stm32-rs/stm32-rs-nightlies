#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAMP1NOER`"]
pub type TAMP1NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP1NOER`"]
pub struct TAMP1NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1NOER_W<'a> {
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
#[doc = "Reader of field `TAMP2NOER`"]
pub type TAMP2NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP2NOER`"]
pub struct TAMP2NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2NOER_W<'a> {
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
#[doc = "Reader of field `TAMP3NOER`"]
pub type TAMP3NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP3NOER`"]
pub struct TAMP3NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3NOER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TAMP4NOER`"]
pub type TAMP4NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP4NOER`"]
pub struct TAMP4NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP4NOER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TAMP5NOER`"]
pub type TAMP5NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP5NOER`"]
pub struct TAMP5NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP5NOER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TAMP6NOER`"]
pub type TAMP6NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP6NOER`"]
pub struct TAMP6NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP6NOER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TAMP7NOER`"]
pub type TAMP7NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP7NOER`"]
pub struct TAMP7NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP7NOER_W<'a> {
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
#[doc = "Reader of field `TAMP8NOER`"]
pub type TAMP8NOER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP8NOER`"]
pub struct TAMP8NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP8NOER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TAMP1MSK`"]
pub type TAMP1MSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP1MSK`"]
pub struct TAMP1MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1MSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TAMP2MSK`"]
pub type TAMP2MSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP2MSK`"]
pub struct TAMP2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2MSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TAMP3MSK`"]
pub type TAMP3MSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP3MSK`"]
pub struct TAMP3MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3MSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `BKERASE`"]
pub type BKERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKERASE`"]
pub struct BKERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKERASE_W<'a> {
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
#[doc = "Reader of field `TAMP1TRG`"]
pub type TAMP1TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP1TRG`"]
pub struct TAMP1TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1TRG_W<'a> {
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
#[doc = "Reader of field `TAMP2TRG`"]
pub type TAMP2TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP2TRG`"]
pub struct TAMP2TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2TRG_W<'a> {
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
#[doc = "Reader of field `TAMP3TRG`"]
pub type TAMP3TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP3TRG`"]
pub struct TAMP3TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3TRG_W<'a> {
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
#[doc = "Reader of field `TAMP4TRG`"]
pub type TAMP4TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP4TRG`"]
pub struct TAMP4TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP4TRG_W<'a> {
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
#[doc = "Reader of field `TAMP5TRG`"]
pub type TAMP5TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP5TRG`"]
pub struct TAMP5TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP5TRG_W<'a> {
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
#[doc = "Reader of field `TAMP6TRG`"]
pub type TAMP6TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP6TRG`"]
pub struct TAMP6TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP6TRG_W<'a> {
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
#[doc = "Reader of field `TAMP7TRG`"]
pub type TAMP7TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP7TRG`"]
pub struct TAMP7TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP7TRG_W<'a> {
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
#[doc = "Reader of field `TAMP8TRG`"]
pub type TAMP8TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMP8TRG`"]
pub struct TAMP8TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP8TRG_W<'a> {
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
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> TAMP3NOER_R {
        TAMP3NOER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TAMP4NOER"]
    #[inline(always)]
    pub fn tamp4noer(&self) -> TAMP4NOER_R {
        TAMP4NOER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TAMP5NOER"]
    #[inline(always)]
    pub fn tamp5noer(&self) -> TAMP5NOER_R {
        TAMP5NOER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TAMP6NOER"]
    #[inline(always)]
    pub fn tamp6noer(&self) -> TAMP6NOER_R {
        TAMP6NOER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TAMP7NOER"]
    #[inline(always)]
    pub fn tamp7noer(&self) -> TAMP7NOER_R {
        TAMP7NOER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TAMP8NOER"]
    #[inline(always)]
    pub fn tamp8noer(&self) -> TAMP8NOER_R {
        TAMP8NOER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 23 - BKERASE"]
    #[inline(always)]
    pub fn bkerase(&self) -> BKERASE_R {
        BKERASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TAMP4TRG"]
    #[inline(always)]
    pub fn tamp4trg(&self) -> TAMP4TRG_R {
        TAMP4TRG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TAMP5TRG"]
    #[inline(always)]
    pub fn tamp5trg(&self) -> TAMP5TRG_R {
        TAMP5TRG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TAMP6TRG"]
    #[inline(always)]
    pub fn tamp6trg(&self) -> TAMP6TRG_R {
        TAMP6TRG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TAMP7TRG"]
    #[inline(always)]
    pub fn tamp7trg(&self) -> TAMP7TRG_R {
        TAMP7TRG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TAMP8TRG"]
    #[inline(always)]
    pub fn tamp8trg(&self) -> TAMP8TRG_R {
        TAMP8TRG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W {
        TAMP1NOER_W { w: self }
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W {
        TAMP2NOER_W { w: self }
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&mut self) -> TAMP3NOER_W {
        TAMP3NOER_W { w: self }
    }
    #[doc = "Bit 3 - TAMP4NOER"]
    #[inline(always)]
    pub fn tamp4noer(&mut self) -> TAMP4NOER_W {
        TAMP4NOER_W { w: self }
    }
    #[doc = "Bit 4 - TAMP5NOER"]
    #[inline(always)]
    pub fn tamp5noer(&mut self) -> TAMP5NOER_W {
        TAMP5NOER_W { w: self }
    }
    #[doc = "Bit 5 - TAMP6NOER"]
    #[inline(always)]
    pub fn tamp6noer(&mut self) -> TAMP6NOER_W {
        TAMP6NOER_W { w: self }
    }
    #[doc = "Bit 6 - TAMP7NOER"]
    #[inline(always)]
    pub fn tamp7noer(&mut self) -> TAMP7NOER_W {
        TAMP7NOER_W { w: self }
    }
    #[doc = "Bit 7 - TAMP8NOER"]
    #[inline(always)]
    pub fn tamp8noer(&mut self) -> TAMP8NOER_W {
        TAMP8NOER_W { w: self }
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W {
        TAMP1MSK_W { w: self }
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W {
        TAMP2MSK_W { w: self }
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W {
        TAMP3MSK_W { w: self }
    }
    #[doc = "Bit 23 - BKERASE"]
    #[inline(always)]
    pub fn bkerase(&mut self) -> BKERASE_W {
        BKERASE_W { w: self }
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W {
        TAMP1TRG_W { w: self }
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W {
        TAMP2TRG_W { w: self }
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W {
        TAMP3TRG_W { w: self }
    }
    #[doc = "Bit 27 - TAMP4TRG"]
    #[inline(always)]
    pub fn tamp4trg(&mut self) -> TAMP4TRG_W {
        TAMP4TRG_W { w: self }
    }
    #[doc = "Bit 28 - TAMP5TRG"]
    #[inline(always)]
    pub fn tamp5trg(&mut self) -> TAMP5TRG_W {
        TAMP5TRG_W { w: self }
    }
    #[doc = "Bit 29 - TAMP6TRG"]
    #[inline(always)]
    pub fn tamp6trg(&mut self) -> TAMP6TRG_W {
        TAMP6TRG_W { w: self }
    }
    #[doc = "Bit 30 - TAMP7TRG"]
    #[inline(always)]
    pub fn tamp7trg(&mut self) -> TAMP7TRG_W {
        TAMP7TRG_W { w: self }
    }
    #[doc = "Bit 31 - TAMP8TRG"]
    #[inline(always)]
    pub fn tamp8trg(&mut self) -> TAMP8TRG_W {
        TAMP8TRG_W { w: self }
    }
}
