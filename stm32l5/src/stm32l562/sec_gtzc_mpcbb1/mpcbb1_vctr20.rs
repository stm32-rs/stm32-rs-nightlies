#[doc = "Reader of register MPCBB1_VCTR20"]
pub type R = crate::R<u32, super::MPCBB1_VCTR20>;
#[doc = "Writer for register MPCBB1_VCTR20"]
pub type W = crate::W<u32, super::MPCBB1_VCTR20>;
#[doc = "Register MPCBB1_VCTR20 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B640`"]
pub type B640_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B640`"]
pub struct B640_W<'a> {
    w: &'a mut W,
}
impl<'a> B640_W<'a> {
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
#[doc = "Reader of field `B641`"]
pub type B641_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B641`"]
pub struct B641_W<'a> {
    w: &'a mut W,
}
impl<'a> B641_W<'a> {
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
#[doc = "Reader of field `B642`"]
pub type B642_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B642`"]
pub struct B642_W<'a> {
    w: &'a mut W,
}
impl<'a> B642_W<'a> {
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
#[doc = "Reader of field `B643`"]
pub type B643_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B643`"]
pub struct B643_W<'a> {
    w: &'a mut W,
}
impl<'a> B643_W<'a> {
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
#[doc = "Reader of field `B644`"]
pub type B644_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B644`"]
pub struct B644_W<'a> {
    w: &'a mut W,
}
impl<'a> B644_W<'a> {
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
#[doc = "Reader of field `B645`"]
pub type B645_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B645`"]
pub struct B645_W<'a> {
    w: &'a mut W,
}
impl<'a> B645_W<'a> {
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
#[doc = "Reader of field `B646`"]
pub type B646_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B646`"]
pub struct B646_W<'a> {
    w: &'a mut W,
}
impl<'a> B646_W<'a> {
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
#[doc = "Reader of field `B647`"]
pub type B647_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B647`"]
pub struct B647_W<'a> {
    w: &'a mut W,
}
impl<'a> B647_W<'a> {
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
#[doc = "Reader of field `B648`"]
pub type B648_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B648`"]
pub struct B648_W<'a> {
    w: &'a mut W,
}
impl<'a> B648_W<'a> {
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
#[doc = "Reader of field `B649`"]
pub type B649_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B649`"]
pub struct B649_W<'a> {
    w: &'a mut W,
}
impl<'a> B649_W<'a> {
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
#[doc = "Reader of field `B650`"]
pub type B650_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B650`"]
pub struct B650_W<'a> {
    w: &'a mut W,
}
impl<'a> B650_W<'a> {
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
#[doc = "Reader of field `B651`"]
pub type B651_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B651`"]
pub struct B651_W<'a> {
    w: &'a mut W,
}
impl<'a> B651_W<'a> {
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
#[doc = "Reader of field `B652`"]
pub type B652_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B652`"]
pub struct B652_W<'a> {
    w: &'a mut W,
}
impl<'a> B652_W<'a> {
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
#[doc = "Reader of field `B653`"]
pub type B653_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B653`"]
pub struct B653_W<'a> {
    w: &'a mut W,
}
impl<'a> B653_W<'a> {
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
#[doc = "Reader of field `B654`"]
pub type B654_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B654`"]
pub struct B654_W<'a> {
    w: &'a mut W,
}
impl<'a> B654_W<'a> {
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
#[doc = "Reader of field `B655`"]
pub type B655_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B655`"]
pub struct B655_W<'a> {
    w: &'a mut W,
}
impl<'a> B655_W<'a> {
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
#[doc = "Reader of field `B656`"]
pub type B656_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B656`"]
pub struct B656_W<'a> {
    w: &'a mut W,
}
impl<'a> B656_W<'a> {
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
#[doc = "Reader of field `B657`"]
pub type B657_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B657`"]
pub struct B657_W<'a> {
    w: &'a mut W,
}
impl<'a> B657_W<'a> {
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
#[doc = "Reader of field `B658`"]
pub type B658_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B658`"]
pub struct B658_W<'a> {
    w: &'a mut W,
}
impl<'a> B658_W<'a> {
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
#[doc = "Reader of field `B659`"]
pub type B659_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B659`"]
pub struct B659_W<'a> {
    w: &'a mut W,
}
impl<'a> B659_W<'a> {
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
#[doc = "Reader of field `B660`"]
pub type B660_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B660`"]
pub struct B660_W<'a> {
    w: &'a mut W,
}
impl<'a> B660_W<'a> {
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
#[doc = "Reader of field `B661`"]
pub type B661_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B661`"]
pub struct B661_W<'a> {
    w: &'a mut W,
}
impl<'a> B661_W<'a> {
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
#[doc = "Reader of field `B662`"]
pub type B662_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B662`"]
pub struct B662_W<'a> {
    w: &'a mut W,
}
impl<'a> B662_W<'a> {
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
#[doc = "Reader of field `B663`"]
pub type B663_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B663`"]
pub struct B663_W<'a> {
    w: &'a mut W,
}
impl<'a> B663_W<'a> {
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
#[doc = "Reader of field `B664`"]
pub type B664_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B664`"]
pub struct B664_W<'a> {
    w: &'a mut W,
}
impl<'a> B664_W<'a> {
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
#[doc = "Reader of field `B665`"]
pub type B665_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B665`"]
pub struct B665_W<'a> {
    w: &'a mut W,
}
impl<'a> B665_W<'a> {
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
#[doc = "Reader of field `B666`"]
pub type B666_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B666`"]
pub struct B666_W<'a> {
    w: &'a mut W,
}
impl<'a> B666_W<'a> {
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
#[doc = "Reader of field `B667`"]
pub type B667_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B667`"]
pub struct B667_W<'a> {
    w: &'a mut W,
}
impl<'a> B667_W<'a> {
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
#[doc = "Reader of field `B668`"]
pub type B668_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B668`"]
pub struct B668_W<'a> {
    w: &'a mut W,
}
impl<'a> B668_W<'a> {
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
#[doc = "Reader of field `B669`"]
pub type B669_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B669`"]
pub struct B669_W<'a> {
    w: &'a mut W,
}
impl<'a> B669_W<'a> {
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
#[doc = "Reader of field `B670`"]
pub type B670_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B670`"]
pub struct B670_W<'a> {
    w: &'a mut W,
}
impl<'a> B670_W<'a> {
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
#[doc = "Reader of field `B671`"]
pub type B671_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B671`"]
pub struct B671_W<'a> {
    w: &'a mut W,
}
impl<'a> B671_W<'a> {
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
    #[doc = "Bit 0 - B640"]
    #[inline(always)]
    pub fn b640(&self) -> B640_R {
        B640_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B641"]
    #[inline(always)]
    pub fn b641(&self) -> B641_R {
        B641_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B642"]
    #[inline(always)]
    pub fn b642(&self) -> B642_R {
        B642_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B643"]
    #[inline(always)]
    pub fn b643(&self) -> B643_R {
        B643_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B644"]
    #[inline(always)]
    pub fn b644(&self) -> B644_R {
        B644_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B645"]
    #[inline(always)]
    pub fn b645(&self) -> B645_R {
        B645_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B646"]
    #[inline(always)]
    pub fn b646(&self) -> B646_R {
        B646_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B647"]
    #[inline(always)]
    pub fn b647(&self) -> B647_R {
        B647_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B648"]
    #[inline(always)]
    pub fn b648(&self) -> B648_R {
        B648_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B649"]
    #[inline(always)]
    pub fn b649(&self) -> B649_R {
        B649_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B650"]
    #[inline(always)]
    pub fn b650(&self) -> B650_R {
        B650_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B651"]
    #[inline(always)]
    pub fn b651(&self) -> B651_R {
        B651_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B652"]
    #[inline(always)]
    pub fn b652(&self) -> B652_R {
        B652_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B653"]
    #[inline(always)]
    pub fn b653(&self) -> B653_R {
        B653_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B654"]
    #[inline(always)]
    pub fn b654(&self) -> B654_R {
        B654_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B655"]
    #[inline(always)]
    pub fn b655(&self) -> B655_R {
        B655_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B656"]
    #[inline(always)]
    pub fn b656(&self) -> B656_R {
        B656_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B657"]
    #[inline(always)]
    pub fn b657(&self) -> B657_R {
        B657_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B658"]
    #[inline(always)]
    pub fn b658(&self) -> B658_R {
        B658_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B659"]
    #[inline(always)]
    pub fn b659(&self) -> B659_R {
        B659_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B660"]
    #[inline(always)]
    pub fn b660(&self) -> B660_R {
        B660_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B661"]
    #[inline(always)]
    pub fn b661(&self) -> B661_R {
        B661_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B662"]
    #[inline(always)]
    pub fn b662(&self) -> B662_R {
        B662_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B663"]
    #[inline(always)]
    pub fn b663(&self) -> B663_R {
        B663_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B664"]
    #[inline(always)]
    pub fn b664(&self) -> B664_R {
        B664_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B665"]
    #[inline(always)]
    pub fn b665(&self) -> B665_R {
        B665_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B666"]
    #[inline(always)]
    pub fn b666(&self) -> B666_R {
        B666_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B667"]
    #[inline(always)]
    pub fn b667(&self) -> B667_R {
        B667_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B668"]
    #[inline(always)]
    pub fn b668(&self) -> B668_R {
        B668_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B669"]
    #[inline(always)]
    pub fn b669(&self) -> B669_R {
        B669_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B670"]
    #[inline(always)]
    pub fn b670(&self) -> B670_R {
        B670_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B671"]
    #[inline(always)]
    pub fn b671(&self) -> B671_R {
        B671_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B640"]
    #[inline(always)]
    pub fn b640(&mut self) -> B640_W {
        B640_W { w: self }
    }
    #[doc = "Bit 1 - B641"]
    #[inline(always)]
    pub fn b641(&mut self) -> B641_W {
        B641_W { w: self }
    }
    #[doc = "Bit 2 - B642"]
    #[inline(always)]
    pub fn b642(&mut self) -> B642_W {
        B642_W { w: self }
    }
    #[doc = "Bit 3 - B643"]
    #[inline(always)]
    pub fn b643(&mut self) -> B643_W {
        B643_W { w: self }
    }
    #[doc = "Bit 4 - B644"]
    #[inline(always)]
    pub fn b644(&mut self) -> B644_W {
        B644_W { w: self }
    }
    #[doc = "Bit 5 - B645"]
    #[inline(always)]
    pub fn b645(&mut self) -> B645_W {
        B645_W { w: self }
    }
    #[doc = "Bit 6 - B646"]
    #[inline(always)]
    pub fn b646(&mut self) -> B646_W {
        B646_W { w: self }
    }
    #[doc = "Bit 7 - B647"]
    #[inline(always)]
    pub fn b647(&mut self) -> B647_W {
        B647_W { w: self }
    }
    #[doc = "Bit 8 - B648"]
    #[inline(always)]
    pub fn b648(&mut self) -> B648_W {
        B648_W { w: self }
    }
    #[doc = "Bit 9 - B649"]
    #[inline(always)]
    pub fn b649(&mut self) -> B649_W {
        B649_W { w: self }
    }
    #[doc = "Bit 10 - B650"]
    #[inline(always)]
    pub fn b650(&mut self) -> B650_W {
        B650_W { w: self }
    }
    #[doc = "Bit 11 - B651"]
    #[inline(always)]
    pub fn b651(&mut self) -> B651_W {
        B651_W { w: self }
    }
    #[doc = "Bit 12 - B652"]
    #[inline(always)]
    pub fn b652(&mut self) -> B652_W {
        B652_W { w: self }
    }
    #[doc = "Bit 13 - B653"]
    #[inline(always)]
    pub fn b653(&mut self) -> B653_W {
        B653_W { w: self }
    }
    #[doc = "Bit 14 - B654"]
    #[inline(always)]
    pub fn b654(&mut self) -> B654_W {
        B654_W { w: self }
    }
    #[doc = "Bit 15 - B655"]
    #[inline(always)]
    pub fn b655(&mut self) -> B655_W {
        B655_W { w: self }
    }
    #[doc = "Bit 16 - B656"]
    #[inline(always)]
    pub fn b656(&mut self) -> B656_W {
        B656_W { w: self }
    }
    #[doc = "Bit 17 - B657"]
    #[inline(always)]
    pub fn b657(&mut self) -> B657_W {
        B657_W { w: self }
    }
    #[doc = "Bit 18 - B658"]
    #[inline(always)]
    pub fn b658(&mut self) -> B658_W {
        B658_W { w: self }
    }
    #[doc = "Bit 19 - B659"]
    #[inline(always)]
    pub fn b659(&mut self) -> B659_W {
        B659_W { w: self }
    }
    #[doc = "Bit 20 - B660"]
    #[inline(always)]
    pub fn b660(&mut self) -> B660_W {
        B660_W { w: self }
    }
    #[doc = "Bit 21 - B661"]
    #[inline(always)]
    pub fn b661(&mut self) -> B661_W {
        B661_W { w: self }
    }
    #[doc = "Bit 22 - B662"]
    #[inline(always)]
    pub fn b662(&mut self) -> B662_W {
        B662_W { w: self }
    }
    #[doc = "Bit 23 - B663"]
    #[inline(always)]
    pub fn b663(&mut self) -> B663_W {
        B663_W { w: self }
    }
    #[doc = "Bit 24 - B664"]
    #[inline(always)]
    pub fn b664(&mut self) -> B664_W {
        B664_W { w: self }
    }
    #[doc = "Bit 25 - B665"]
    #[inline(always)]
    pub fn b665(&mut self) -> B665_W {
        B665_W { w: self }
    }
    #[doc = "Bit 26 - B666"]
    #[inline(always)]
    pub fn b666(&mut self) -> B666_W {
        B666_W { w: self }
    }
    #[doc = "Bit 27 - B667"]
    #[inline(always)]
    pub fn b667(&mut self) -> B667_W {
        B667_W { w: self }
    }
    #[doc = "Bit 28 - B668"]
    #[inline(always)]
    pub fn b668(&mut self) -> B668_W {
        B668_W { w: self }
    }
    #[doc = "Bit 29 - B669"]
    #[inline(always)]
    pub fn b669(&mut self) -> B669_W {
        B669_W { w: self }
    }
    #[doc = "Bit 30 - B670"]
    #[inline(always)]
    pub fn b670(&mut self) -> B670_W {
        B670_W { w: self }
    }
    #[doc = "Bit 31 - B671"]
    #[inline(always)]
    pub fn b671(&mut self) -> B671_W {
        B671_W { w: self }
    }
}
