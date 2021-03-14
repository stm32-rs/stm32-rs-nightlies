#[doc = "Reader of register MPCBB2_VCTR9"]
pub type R = crate::R<u32, super::MPCBB2_VCTR9>;
#[doc = "Writer for register MPCBB2_VCTR9"]
pub type W = crate::W<u32, super::MPCBB2_VCTR9>;
#[doc = "Register MPCBB2_VCTR9 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B288`"]
pub type B288_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B288`"]
pub struct B288_W<'a> {
    w: &'a mut W,
}
impl<'a> B288_W<'a> {
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
#[doc = "Reader of field `B289`"]
pub type B289_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B289`"]
pub struct B289_W<'a> {
    w: &'a mut W,
}
impl<'a> B289_W<'a> {
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
#[doc = "Reader of field `B290`"]
pub type B290_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B290`"]
pub struct B290_W<'a> {
    w: &'a mut W,
}
impl<'a> B290_W<'a> {
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
#[doc = "Reader of field `B291`"]
pub type B291_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B291`"]
pub struct B291_W<'a> {
    w: &'a mut W,
}
impl<'a> B291_W<'a> {
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
#[doc = "Reader of field `B292`"]
pub type B292_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B292`"]
pub struct B292_W<'a> {
    w: &'a mut W,
}
impl<'a> B292_W<'a> {
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
#[doc = "Reader of field `B293`"]
pub type B293_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B293`"]
pub struct B293_W<'a> {
    w: &'a mut W,
}
impl<'a> B293_W<'a> {
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
#[doc = "Reader of field `B294`"]
pub type B294_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B294`"]
pub struct B294_W<'a> {
    w: &'a mut W,
}
impl<'a> B294_W<'a> {
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
#[doc = "Reader of field `B295`"]
pub type B295_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B295`"]
pub struct B295_W<'a> {
    w: &'a mut W,
}
impl<'a> B295_W<'a> {
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
#[doc = "Reader of field `B296`"]
pub type B296_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B296`"]
pub struct B296_W<'a> {
    w: &'a mut W,
}
impl<'a> B296_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `B297`"]
pub type B297_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B297`"]
pub struct B297_W<'a> {
    w: &'a mut W,
}
impl<'a> B297_W<'a> {
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
#[doc = "Reader of field `B298`"]
pub type B298_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B298`"]
pub struct B298_W<'a> {
    w: &'a mut W,
}
impl<'a> B298_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `B299`"]
pub type B299_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B299`"]
pub struct B299_W<'a> {
    w: &'a mut W,
}
impl<'a> B299_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `B300`"]
pub type B300_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B300`"]
pub struct B300_W<'a> {
    w: &'a mut W,
}
impl<'a> B300_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `B301`"]
pub type B301_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B301`"]
pub struct B301_W<'a> {
    w: &'a mut W,
}
impl<'a> B301_W<'a> {
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
#[doc = "Reader of field `B302`"]
pub type B302_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B302`"]
pub struct B302_W<'a> {
    w: &'a mut W,
}
impl<'a> B302_W<'a> {
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
#[doc = "Reader of field `B303`"]
pub type B303_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B303`"]
pub struct B303_W<'a> {
    w: &'a mut W,
}
impl<'a> B303_W<'a> {
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
#[doc = "Reader of field `B304`"]
pub type B304_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B304`"]
pub struct B304_W<'a> {
    w: &'a mut W,
}
impl<'a> B304_W<'a> {
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
#[doc = "Reader of field `B305`"]
pub type B305_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B305`"]
pub struct B305_W<'a> {
    w: &'a mut W,
}
impl<'a> B305_W<'a> {
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
#[doc = "Reader of field `B306`"]
pub type B306_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B306`"]
pub struct B306_W<'a> {
    w: &'a mut W,
}
impl<'a> B306_W<'a> {
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
#[doc = "Reader of field `B307`"]
pub type B307_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B307`"]
pub struct B307_W<'a> {
    w: &'a mut W,
}
impl<'a> B307_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `B308`"]
pub type B308_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B308`"]
pub struct B308_W<'a> {
    w: &'a mut W,
}
impl<'a> B308_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `B309`"]
pub type B309_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B309`"]
pub struct B309_W<'a> {
    w: &'a mut W,
}
impl<'a> B309_W<'a> {
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
#[doc = "Reader of field `B310`"]
pub type B310_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B310`"]
pub struct B310_W<'a> {
    w: &'a mut W,
}
impl<'a> B310_W<'a> {
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
#[doc = "Reader of field `B311`"]
pub type B311_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B311`"]
pub struct B311_W<'a> {
    w: &'a mut W,
}
impl<'a> B311_W<'a> {
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
#[doc = "Reader of field `B312`"]
pub type B312_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B312`"]
pub struct B312_W<'a> {
    w: &'a mut W,
}
impl<'a> B312_W<'a> {
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
#[doc = "Reader of field `B313`"]
pub type B313_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B313`"]
pub struct B313_W<'a> {
    w: &'a mut W,
}
impl<'a> B313_W<'a> {
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
#[doc = "Reader of field `B314`"]
pub type B314_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B314`"]
pub struct B314_W<'a> {
    w: &'a mut W,
}
impl<'a> B314_W<'a> {
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
#[doc = "Reader of field `B315`"]
pub type B315_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B315`"]
pub struct B315_W<'a> {
    w: &'a mut W,
}
impl<'a> B315_W<'a> {
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
#[doc = "Reader of field `B316`"]
pub type B316_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B316`"]
pub struct B316_W<'a> {
    w: &'a mut W,
}
impl<'a> B316_W<'a> {
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
#[doc = "Reader of field `B317`"]
pub type B317_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B317`"]
pub struct B317_W<'a> {
    w: &'a mut W,
}
impl<'a> B317_W<'a> {
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
#[doc = "Reader of field `B318`"]
pub type B318_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B318`"]
pub struct B318_W<'a> {
    w: &'a mut W,
}
impl<'a> B318_W<'a> {
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
#[doc = "Reader of field `B319`"]
pub type B319_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B319`"]
pub struct B319_W<'a> {
    w: &'a mut W,
}
impl<'a> B319_W<'a> {
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
    #[doc = "Bit 0 - B288"]
    #[inline(always)]
    pub fn b288(&self) -> B288_R {
        B288_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B289"]
    #[inline(always)]
    pub fn b289(&self) -> B289_R {
        B289_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B290"]
    #[inline(always)]
    pub fn b290(&self) -> B290_R {
        B290_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B291"]
    #[inline(always)]
    pub fn b291(&self) -> B291_R {
        B291_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B292"]
    #[inline(always)]
    pub fn b292(&self) -> B292_R {
        B292_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B293"]
    #[inline(always)]
    pub fn b293(&self) -> B293_R {
        B293_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B294"]
    #[inline(always)]
    pub fn b294(&self) -> B294_R {
        B294_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B295"]
    #[inline(always)]
    pub fn b295(&self) -> B295_R {
        B295_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B296"]
    #[inline(always)]
    pub fn b296(&self) -> B296_R {
        B296_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B297"]
    #[inline(always)]
    pub fn b297(&self) -> B297_R {
        B297_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B298"]
    #[inline(always)]
    pub fn b298(&self) -> B298_R {
        B298_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B299"]
    #[inline(always)]
    pub fn b299(&self) -> B299_R {
        B299_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B300"]
    #[inline(always)]
    pub fn b300(&self) -> B300_R {
        B300_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B301"]
    #[inline(always)]
    pub fn b301(&self) -> B301_R {
        B301_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B302"]
    #[inline(always)]
    pub fn b302(&self) -> B302_R {
        B302_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B303"]
    #[inline(always)]
    pub fn b303(&self) -> B303_R {
        B303_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B304"]
    #[inline(always)]
    pub fn b304(&self) -> B304_R {
        B304_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B305"]
    #[inline(always)]
    pub fn b305(&self) -> B305_R {
        B305_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B306"]
    #[inline(always)]
    pub fn b306(&self) -> B306_R {
        B306_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B307"]
    #[inline(always)]
    pub fn b307(&self) -> B307_R {
        B307_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B308"]
    #[inline(always)]
    pub fn b308(&self) -> B308_R {
        B308_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B309"]
    #[inline(always)]
    pub fn b309(&self) -> B309_R {
        B309_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B310"]
    #[inline(always)]
    pub fn b310(&self) -> B310_R {
        B310_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B311"]
    #[inline(always)]
    pub fn b311(&self) -> B311_R {
        B311_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B312"]
    #[inline(always)]
    pub fn b312(&self) -> B312_R {
        B312_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B313"]
    #[inline(always)]
    pub fn b313(&self) -> B313_R {
        B313_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B314"]
    #[inline(always)]
    pub fn b314(&self) -> B314_R {
        B314_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B315"]
    #[inline(always)]
    pub fn b315(&self) -> B315_R {
        B315_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B316"]
    #[inline(always)]
    pub fn b316(&self) -> B316_R {
        B316_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B317"]
    #[inline(always)]
    pub fn b317(&self) -> B317_R {
        B317_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B318"]
    #[inline(always)]
    pub fn b318(&self) -> B318_R {
        B318_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B319"]
    #[inline(always)]
    pub fn b319(&self) -> B319_R {
        B319_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B288"]
    #[inline(always)]
    pub fn b288(&mut self) -> B288_W {
        B288_W { w: self }
    }
    #[doc = "Bit 1 - B289"]
    #[inline(always)]
    pub fn b289(&mut self) -> B289_W {
        B289_W { w: self }
    }
    #[doc = "Bit 2 - B290"]
    #[inline(always)]
    pub fn b290(&mut self) -> B290_W {
        B290_W { w: self }
    }
    #[doc = "Bit 3 - B291"]
    #[inline(always)]
    pub fn b291(&mut self) -> B291_W {
        B291_W { w: self }
    }
    #[doc = "Bit 4 - B292"]
    #[inline(always)]
    pub fn b292(&mut self) -> B292_W {
        B292_W { w: self }
    }
    #[doc = "Bit 5 - B293"]
    #[inline(always)]
    pub fn b293(&mut self) -> B293_W {
        B293_W { w: self }
    }
    #[doc = "Bit 6 - B294"]
    #[inline(always)]
    pub fn b294(&mut self) -> B294_W {
        B294_W { w: self }
    }
    #[doc = "Bit 7 - B295"]
    #[inline(always)]
    pub fn b295(&mut self) -> B295_W {
        B295_W { w: self }
    }
    #[doc = "Bit 8 - B296"]
    #[inline(always)]
    pub fn b296(&mut self) -> B296_W {
        B296_W { w: self }
    }
    #[doc = "Bit 9 - B297"]
    #[inline(always)]
    pub fn b297(&mut self) -> B297_W {
        B297_W { w: self }
    }
    #[doc = "Bit 10 - B298"]
    #[inline(always)]
    pub fn b298(&mut self) -> B298_W {
        B298_W { w: self }
    }
    #[doc = "Bit 11 - B299"]
    #[inline(always)]
    pub fn b299(&mut self) -> B299_W {
        B299_W { w: self }
    }
    #[doc = "Bit 12 - B300"]
    #[inline(always)]
    pub fn b300(&mut self) -> B300_W {
        B300_W { w: self }
    }
    #[doc = "Bit 13 - B301"]
    #[inline(always)]
    pub fn b301(&mut self) -> B301_W {
        B301_W { w: self }
    }
    #[doc = "Bit 14 - B302"]
    #[inline(always)]
    pub fn b302(&mut self) -> B302_W {
        B302_W { w: self }
    }
    #[doc = "Bit 15 - B303"]
    #[inline(always)]
    pub fn b303(&mut self) -> B303_W {
        B303_W { w: self }
    }
    #[doc = "Bit 16 - B304"]
    #[inline(always)]
    pub fn b304(&mut self) -> B304_W {
        B304_W { w: self }
    }
    #[doc = "Bit 17 - B305"]
    #[inline(always)]
    pub fn b305(&mut self) -> B305_W {
        B305_W { w: self }
    }
    #[doc = "Bit 18 - B306"]
    #[inline(always)]
    pub fn b306(&mut self) -> B306_W {
        B306_W { w: self }
    }
    #[doc = "Bit 19 - B307"]
    #[inline(always)]
    pub fn b307(&mut self) -> B307_W {
        B307_W { w: self }
    }
    #[doc = "Bit 20 - B308"]
    #[inline(always)]
    pub fn b308(&mut self) -> B308_W {
        B308_W { w: self }
    }
    #[doc = "Bit 21 - B309"]
    #[inline(always)]
    pub fn b309(&mut self) -> B309_W {
        B309_W { w: self }
    }
    #[doc = "Bit 22 - B310"]
    #[inline(always)]
    pub fn b310(&mut self) -> B310_W {
        B310_W { w: self }
    }
    #[doc = "Bit 23 - B311"]
    #[inline(always)]
    pub fn b311(&mut self) -> B311_W {
        B311_W { w: self }
    }
    #[doc = "Bit 24 - B312"]
    #[inline(always)]
    pub fn b312(&mut self) -> B312_W {
        B312_W { w: self }
    }
    #[doc = "Bit 25 - B313"]
    #[inline(always)]
    pub fn b313(&mut self) -> B313_W {
        B313_W { w: self }
    }
    #[doc = "Bit 26 - B314"]
    #[inline(always)]
    pub fn b314(&mut self) -> B314_W {
        B314_W { w: self }
    }
    #[doc = "Bit 27 - B315"]
    #[inline(always)]
    pub fn b315(&mut self) -> B315_W {
        B315_W { w: self }
    }
    #[doc = "Bit 28 - B316"]
    #[inline(always)]
    pub fn b316(&mut self) -> B316_W {
        B316_W { w: self }
    }
    #[doc = "Bit 29 - B317"]
    #[inline(always)]
    pub fn b317(&mut self) -> B317_W {
        B317_W { w: self }
    }
    #[doc = "Bit 30 - B318"]
    #[inline(always)]
    pub fn b318(&mut self) -> B318_W {
        B318_W { w: self }
    }
    #[doc = "Bit 31 - B319"]
    #[inline(always)]
    pub fn b319(&mut self) -> B319_W {
        B319_W { w: self }
    }
}
