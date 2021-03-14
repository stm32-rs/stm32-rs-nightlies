#[doc = "Reader of register MPCBB2_VCTR11"]
pub type R = crate::R<u32, super::MPCBB2_VCTR11>;
#[doc = "Writer for register MPCBB2_VCTR11"]
pub type W = crate::W<u32, super::MPCBB2_VCTR11>;
#[doc = "Register MPCBB2_VCTR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B352`"]
pub type B352_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B352`"]
pub struct B352_W<'a> {
    w: &'a mut W,
}
impl<'a> B352_W<'a> {
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
#[doc = "Reader of field `B353`"]
pub type B353_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B353`"]
pub struct B353_W<'a> {
    w: &'a mut W,
}
impl<'a> B353_W<'a> {
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
#[doc = "Reader of field `B354`"]
pub type B354_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B354`"]
pub struct B354_W<'a> {
    w: &'a mut W,
}
impl<'a> B354_W<'a> {
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
#[doc = "Reader of field `B355`"]
pub type B355_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B355`"]
pub struct B355_W<'a> {
    w: &'a mut W,
}
impl<'a> B355_W<'a> {
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
#[doc = "Reader of field `B356`"]
pub type B356_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B356`"]
pub struct B356_W<'a> {
    w: &'a mut W,
}
impl<'a> B356_W<'a> {
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
#[doc = "Reader of field `B357`"]
pub type B357_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B357`"]
pub struct B357_W<'a> {
    w: &'a mut W,
}
impl<'a> B357_W<'a> {
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
#[doc = "Reader of field `B358`"]
pub type B358_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B358`"]
pub struct B358_W<'a> {
    w: &'a mut W,
}
impl<'a> B358_W<'a> {
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
#[doc = "Reader of field `B359`"]
pub type B359_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B359`"]
pub struct B359_W<'a> {
    w: &'a mut W,
}
impl<'a> B359_W<'a> {
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
#[doc = "Reader of field `B360`"]
pub type B360_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B360`"]
pub struct B360_W<'a> {
    w: &'a mut W,
}
impl<'a> B360_W<'a> {
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
#[doc = "Reader of field `B361`"]
pub type B361_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B361`"]
pub struct B361_W<'a> {
    w: &'a mut W,
}
impl<'a> B361_W<'a> {
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
#[doc = "Reader of field `B362`"]
pub type B362_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B362`"]
pub struct B362_W<'a> {
    w: &'a mut W,
}
impl<'a> B362_W<'a> {
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
#[doc = "Reader of field `B363`"]
pub type B363_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B363`"]
pub struct B363_W<'a> {
    w: &'a mut W,
}
impl<'a> B363_W<'a> {
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
#[doc = "Reader of field `B364`"]
pub type B364_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B364`"]
pub struct B364_W<'a> {
    w: &'a mut W,
}
impl<'a> B364_W<'a> {
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
#[doc = "Reader of field `B365`"]
pub type B365_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B365`"]
pub struct B365_W<'a> {
    w: &'a mut W,
}
impl<'a> B365_W<'a> {
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
#[doc = "Reader of field `B366`"]
pub type B366_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B366`"]
pub struct B366_W<'a> {
    w: &'a mut W,
}
impl<'a> B366_W<'a> {
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
#[doc = "Reader of field `B367`"]
pub type B367_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B367`"]
pub struct B367_W<'a> {
    w: &'a mut W,
}
impl<'a> B367_W<'a> {
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
#[doc = "Reader of field `B368`"]
pub type B368_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B368`"]
pub struct B368_W<'a> {
    w: &'a mut W,
}
impl<'a> B368_W<'a> {
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
#[doc = "Reader of field `B369`"]
pub type B369_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B369`"]
pub struct B369_W<'a> {
    w: &'a mut W,
}
impl<'a> B369_W<'a> {
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
#[doc = "Reader of field `B370`"]
pub type B370_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B370`"]
pub struct B370_W<'a> {
    w: &'a mut W,
}
impl<'a> B370_W<'a> {
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
#[doc = "Reader of field `B371`"]
pub type B371_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B371`"]
pub struct B371_W<'a> {
    w: &'a mut W,
}
impl<'a> B371_W<'a> {
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
#[doc = "Reader of field `B372`"]
pub type B372_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B372`"]
pub struct B372_W<'a> {
    w: &'a mut W,
}
impl<'a> B372_W<'a> {
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
#[doc = "Reader of field `B373`"]
pub type B373_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B373`"]
pub struct B373_W<'a> {
    w: &'a mut W,
}
impl<'a> B373_W<'a> {
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
#[doc = "Reader of field `B374`"]
pub type B374_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B374`"]
pub struct B374_W<'a> {
    w: &'a mut W,
}
impl<'a> B374_W<'a> {
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
#[doc = "Reader of field `B375`"]
pub type B375_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B375`"]
pub struct B375_W<'a> {
    w: &'a mut W,
}
impl<'a> B375_W<'a> {
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
#[doc = "Reader of field `B376`"]
pub type B376_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B376`"]
pub struct B376_W<'a> {
    w: &'a mut W,
}
impl<'a> B376_W<'a> {
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
#[doc = "Reader of field `B377`"]
pub type B377_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B377`"]
pub struct B377_W<'a> {
    w: &'a mut W,
}
impl<'a> B377_W<'a> {
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
#[doc = "Reader of field `B378`"]
pub type B378_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B378`"]
pub struct B378_W<'a> {
    w: &'a mut W,
}
impl<'a> B378_W<'a> {
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
#[doc = "Reader of field `B379`"]
pub type B379_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B379`"]
pub struct B379_W<'a> {
    w: &'a mut W,
}
impl<'a> B379_W<'a> {
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
#[doc = "Reader of field `B380`"]
pub type B380_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B380`"]
pub struct B380_W<'a> {
    w: &'a mut W,
}
impl<'a> B380_W<'a> {
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
#[doc = "Reader of field `B381`"]
pub type B381_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B381`"]
pub struct B381_W<'a> {
    w: &'a mut W,
}
impl<'a> B381_W<'a> {
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
#[doc = "Reader of field `B382`"]
pub type B382_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B382`"]
pub struct B382_W<'a> {
    w: &'a mut W,
}
impl<'a> B382_W<'a> {
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
#[doc = "Reader of field `B383`"]
pub type B383_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B383`"]
pub struct B383_W<'a> {
    w: &'a mut W,
}
impl<'a> B383_W<'a> {
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
    #[doc = "Bit 0 - B352"]
    #[inline(always)]
    pub fn b352(&self) -> B352_R {
        B352_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B353"]
    #[inline(always)]
    pub fn b353(&self) -> B353_R {
        B353_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B354"]
    #[inline(always)]
    pub fn b354(&self) -> B354_R {
        B354_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B355"]
    #[inline(always)]
    pub fn b355(&self) -> B355_R {
        B355_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B356"]
    #[inline(always)]
    pub fn b356(&self) -> B356_R {
        B356_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B357"]
    #[inline(always)]
    pub fn b357(&self) -> B357_R {
        B357_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B358"]
    #[inline(always)]
    pub fn b358(&self) -> B358_R {
        B358_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B359"]
    #[inline(always)]
    pub fn b359(&self) -> B359_R {
        B359_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B360"]
    #[inline(always)]
    pub fn b360(&self) -> B360_R {
        B360_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B361"]
    #[inline(always)]
    pub fn b361(&self) -> B361_R {
        B361_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B362"]
    #[inline(always)]
    pub fn b362(&self) -> B362_R {
        B362_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B363"]
    #[inline(always)]
    pub fn b363(&self) -> B363_R {
        B363_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B364"]
    #[inline(always)]
    pub fn b364(&self) -> B364_R {
        B364_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B365"]
    #[inline(always)]
    pub fn b365(&self) -> B365_R {
        B365_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B366"]
    #[inline(always)]
    pub fn b366(&self) -> B366_R {
        B366_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B367"]
    #[inline(always)]
    pub fn b367(&self) -> B367_R {
        B367_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B368"]
    #[inline(always)]
    pub fn b368(&self) -> B368_R {
        B368_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B369"]
    #[inline(always)]
    pub fn b369(&self) -> B369_R {
        B369_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B370"]
    #[inline(always)]
    pub fn b370(&self) -> B370_R {
        B370_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B371"]
    #[inline(always)]
    pub fn b371(&self) -> B371_R {
        B371_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B372"]
    #[inline(always)]
    pub fn b372(&self) -> B372_R {
        B372_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B373"]
    #[inline(always)]
    pub fn b373(&self) -> B373_R {
        B373_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B374"]
    #[inline(always)]
    pub fn b374(&self) -> B374_R {
        B374_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B375"]
    #[inline(always)]
    pub fn b375(&self) -> B375_R {
        B375_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B376"]
    #[inline(always)]
    pub fn b376(&self) -> B376_R {
        B376_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B377"]
    #[inline(always)]
    pub fn b377(&self) -> B377_R {
        B377_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B378"]
    #[inline(always)]
    pub fn b378(&self) -> B378_R {
        B378_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B379"]
    #[inline(always)]
    pub fn b379(&self) -> B379_R {
        B379_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B380"]
    #[inline(always)]
    pub fn b380(&self) -> B380_R {
        B380_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B381"]
    #[inline(always)]
    pub fn b381(&self) -> B381_R {
        B381_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B382"]
    #[inline(always)]
    pub fn b382(&self) -> B382_R {
        B382_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B383"]
    #[inline(always)]
    pub fn b383(&self) -> B383_R {
        B383_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B352"]
    #[inline(always)]
    pub fn b352(&mut self) -> B352_W {
        B352_W { w: self }
    }
    #[doc = "Bit 1 - B353"]
    #[inline(always)]
    pub fn b353(&mut self) -> B353_W {
        B353_W { w: self }
    }
    #[doc = "Bit 2 - B354"]
    #[inline(always)]
    pub fn b354(&mut self) -> B354_W {
        B354_W { w: self }
    }
    #[doc = "Bit 3 - B355"]
    #[inline(always)]
    pub fn b355(&mut self) -> B355_W {
        B355_W { w: self }
    }
    #[doc = "Bit 4 - B356"]
    #[inline(always)]
    pub fn b356(&mut self) -> B356_W {
        B356_W { w: self }
    }
    #[doc = "Bit 5 - B357"]
    #[inline(always)]
    pub fn b357(&mut self) -> B357_W {
        B357_W { w: self }
    }
    #[doc = "Bit 6 - B358"]
    #[inline(always)]
    pub fn b358(&mut self) -> B358_W {
        B358_W { w: self }
    }
    #[doc = "Bit 7 - B359"]
    #[inline(always)]
    pub fn b359(&mut self) -> B359_W {
        B359_W { w: self }
    }
    #[doc = "Bit 8 - B360"]
    #[inline(always)]
    pub fn b360(&mut self) -> B360_W {
        B360_W { w: self }
    }
    #[doc = "Bit 9 - B361"]
    #[inline(always)]
    pub fn b361(&mut self) -> B361_W {
        B361_W { w: self }
    }
    #[doc = "Bit 10 - B362"]
    #[inline(always)]
    pub fn b362(&mut self) -> B362_W {
        B362_W { w: self }
    }
    #[doc = "Bit 11 - B363"]
    #[inline(always)]
    pub fn b363(&mut self) -> B363_W {
        B363_W { w: self }
    }
    #[doc = "Bit 12 - B364"]
    #[inline(always)]
    pub fn b364(&mut self) -> B364_W {
        B364_W { w: self }
    }
    #[doc = "Bit 13 - B365"]
    #[inline(always)]
    pub fn b365(&mut self) -> B365_W {
        B365_W { w: self }
    }
    #[doc = "Bit 14 - B366"]
    #[inline(always)]
    pub fn b366(&mut self) -> B366_W {
        B366_W { w: self }
    }
    #[doc = "Bit 15 - B367"]
    #[inline(always)]
    pub fn b367(&mut self) -> B367_W {
        B367_W { w: self }
    }
    #[doc = "Bit 16 - B368"]
    #[inline(always)]
    pub fn b368(&mut self) -> B368_W {
        B368_W { w: self }
    }
    #[doc = "Bit 17 - B369"]
    #[inline(always)]
    pub fn b369(&mut self) -> B369_W {
        B369_W { w: self }
    }
    #[doc = "Bit 18 - B370"]
    #[inline(always)]
    pub fn b370(&mut self) -> B370_W {
        B370_W { w: self }
    }
    #[doc = "Bit 19 - B371"]
    #[inline(always)]
    pub fn b371(&mut self) -> B371_W {
        B371_W { w: self }
    }
    #[doc = "Bit 20 - B372"]
    #[inline(always)]
    pub fn b372(&mut self) -> B372_W {
        B372_W { w: self }
    }
    #[doc = "Bit 21 - B373"]
    #[inline(always)]
    pub fn b373(&mut self) -> B373_W {
        B373_W { w: self }
    }
    #[doc = "Bit 22 - B374"]
    #[inline(always)]
    pub fn b374(&mut self) -> B374_W {
        B374_W { w: self }
    }
    #[doc = "Bit 23 - B375"]
    #[inline(always)]
    pub fn b375(&mut self) -> B375_W {
        B375_W { w: self }
    }
    #[doc = "Bit 24 - B376"]
    #[inline(always)]
    pub fn b376(&mut self) -> B376_W {
        B376_W { w: self }
    }
    #[doc = "Bit 25 - B377"]
    #[inline(always)]
    pub fn b377(&mut self) -> B377_W {
        B377_W { w: self }
    }
    #[doc = "Bit 26 - B378"]
    #[inline(always)]
    pub fn b378(&mut self) -> B378_W {
        B378_W { w: self }
    }
    #[doc = "Bit 27 - B379"]
    #[inline(always)]
    pub fn b379(&mut self) -> B379_W {
        B379_W { w: self }
    }
    #[doc = "Bit 28 - B380"]
    #[inline(always)]
    pub fn b380(&mut self) -> B380_W {
        B380_W { w: self }
    }
    #[doc = "Bit 29 - B381"]
    #[inline(always)]
    pub fn b381(&mut self) -> B381_W {
        B381_W { w: self }
    }
    #[doc = "Bit 30 - B382"]
    #[inline(always)]
    pub fn b382(&mut self) -> B382_W {
        B382_W { w: self }
    }
    #[doc = "Bit 31 - B383"]
    #[inline(always)]
    pub fn b383(&mut self) -> B383_W {
        B383_W { w: self }
    }
}
