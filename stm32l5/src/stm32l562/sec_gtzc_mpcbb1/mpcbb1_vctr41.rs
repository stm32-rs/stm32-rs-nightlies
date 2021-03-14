#[doc = "Reader of register MPCBB1_VCTR41"]
pub type R = crate::R<u32, super::MPCBB1_VCTR41>;
#[doc = "Writer for register MPCBB1_VCTR41"]
pub type W = crate::W<u32, super::MPCBB1_VCTR41>;
#[doc = "Register MPCBB1_VCTR41 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR41 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1312`"]
pub type B1312_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1312`"]
pub struct B1312_W<'a> {
    w: &'a mut W,
}
impl<'a> B1312_W<'a> {
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
#[doc = "Reader of field `B1313`"]
pub type B1313_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1313`"]
pub struct B1313_W<'a> {
    w: &'a mut W,
}
impl<'a> B1313_W<'a> {
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
#[doc = "Reader of field `B1314`"]
pub type B1314_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1314`"]
pub struct B1314_W<'a> {
    w: &'a mut W,
}
impl<'a> B1314_W<'a> {
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
#[doc = "Reader of field `B1315`"]
pub type B1315_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1315`"]
pub struct B1315_W<'a> {
    w: &'a mut W,
}
impl<'a> B1315_W<'a> {
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
#[doc = "Reader of field `B1316`"]
pub type B1316_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1316`"]
pub struct B1316_W<'a> {
    w: &'a mut W,
}
impl<'a> B1316_W<'a> {
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
#[doc = "Reader of field `B1317`"]
pub type B1317_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1317`"]
pub struct B1317_W<'a> {
    w: &'a mut W,
}
impl<'a> B1317_W<'a> {
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
#[doc = "Reader of field `B1318`"]
pub type B1318_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1318`"]
pub struct B1318_W<'a> {
    w: &'a mut W,
}
impl<'a> B1318_W<'a> {
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
#[doc = "Reader of field `B1319`"]
pub type B1319_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1319`"]
pub struct B1319_W<'a> {
    w: &'a mut W,
}
impl<'a> B1319_W<'a> {
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
#[doc = "Reader of field `B1320`"]
pub type B1320_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1320`"]
pub struct B1320_W<'a> {
    w: &'a mut W,
}
impl<'a> B1320_W<'a> {
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
#[doc = "Reader of field `B1321`"]
pub type B1321_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1321`"]
pub struct B1321_W<'a> {
    w: &'a mut W,
}
impl<'a> B1321_W<'a> {
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
#[doc = "Reader of field `B1322`"]
pub type B1322_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1322`"]
pub struct B1322_W<'a> {
    w: &'a mut W,
}
impl<'a> B1322_W<'a> {
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
#[doc = "Reader of field `B1323`"]
pub type B1323_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1323`"]
pub struct B1323_W<'a> {
    w: &'a mut W,
}
impl<'a> B1323_W<'a> {
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
#[doc = "Reader of field `B1324`"]
pub type B1324_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1324`"]
pub struct B1324_W<'a> {
    w: &'a mut W,
}
impl<'a> B1324_W<'a> {
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
#[doc = "Reader of field `B1325`"]
pub type B1325_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1325`"]
pub struct B1325_W<'a> {
    w: &'a mut W,
}
impl<'a> B1325_W<'a> {
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
#[doc = "Reader of field `B1326`"]
pub type B1326_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1326`"]
pub struct B1326_W<'a> {
    w: &'a mut W,
}
impl<'a> B1326_W<'a> {
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
#[doc = "Reader of field `B1327`"]
pub type B1327_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1327`"]
pub struct B1327_W<'a> {
    w: &'a mut W,
}
impl<'a> B1327_W<'a> {
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
#[doc = "Reader of field `B1328`"]
pub type B1328_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1328`"]
pub struct B1328_W<'a> {
    w: &'a mut W,
}
impl<'a> B1328_W<'a> {
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
#[doc = "Reader of field `B1329`"]
pub type B1329_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1329`"]
pub struct B1329_W<'a> {
    w: &'a mut W,
}
impl<'a> B1329_W<'a> {
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
#[doc = "Reader of field `B1330`"]
pub type B1330_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1330`"]
pub struct B1330_W<'a> {
    w: &'a mut W,
}
impl<'a> B1330_W<'a> {
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
#[doc = "Reader of field `B1331`"]
pub type B1331_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1331`"]
pub struct B1331_W<'a> {
    w: &'a mut W,
}
impl<'a> B1331_W<'a> {
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
#[doc = "Reader of field `B1332`"]
pub type B1332_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1332`"]
pub struct B1332_W<'a> {
    w: &'a mut W,
}
impl<'a> B1332_W<'a> {
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
#[doc = "Reader of field `B1333`"]
pub type B1333_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1333`"]
pub struct B1333_W<'a> {
    w: &'a mut W,
}
impl<'a> B1333_W<'a> {
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
#[doc = "Reader of field `B1334`"]
pub type B1334_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1334`"]
pub struct B1334_W<'a> {
    w: &'a mut W,
}
impl<'a> B1334_W<'a> {
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
#[doc = "Reader of field `B1335`"]
pub type B1335_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1335`"]
pub struct B1335_W<'a> {
    w: &'a mut W,
}
impl<'a> B1335_W<'a> {
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
#[doc = "Reader of field `B1336`"]
pub type B1336_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1336`"]
pub struct B1336_W<'a> {
    w: &'a mut W,
}
impl<'a> B1336_W<'a> {
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
#[doc = "Reader of field `B1337`"]
pub type B1337_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1337`"]
pub struct B1337_W<'a> {
    w: &'a mut W,
}
impl<'a> B1337_W<'a> {
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
#[doc = "Reader of field `B1338`"]
pub type B1338_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1338`"]
pub struct B1338_W<'a> {
    w: &'a mut W,
}
impl<'a> B1338_W<'a> {
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
#[doc = "Reader of field `B1339`"]
pub type B1339_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1339`"]
pub struct B1339_W<'a> {
    w: &'a mut W,
}
impl<'a> B1339_W<'a> {
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
#[doc = "Reader of field `B1340`"]
pub type B1340_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1340`"]
pub struct B1340_W<'a> {
    w: &'a mut W,
}
impl<'a> B1340_W<'a> {
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
#[doc = "Reader of field `B1341`"]
pub type B1341_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1341`"]
pub struct B1341_W<'a> {
    w: &'a mut W,
}
impl<'a> B1341_W<'a> {
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
#[doc = "Reader of field `B1342`"]
pub type B1342_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1342`"]
pub struct B1342_W<'a> {
    w: &'a mut W,
}
impl<'a> B1342_W<'a> {
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
#[doc = "Reader of field `B1343`"]
pub type B1343_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1343`"]
pub struct B1343_W<'a> {
    w: &'a mut W,
}
impl<'a> B1343_W<'a> {
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
    #[doc = "Bit 0 - B1312"]
    #[inline(always)]
    pub fn b1312(&self) -> B1312_R {
        B1312_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1313"]
    #[inline(always)]
    pub fn b1313(&self) -> B1313_R {
        B1313_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1314"]
    #[inline(always)]
    pub fn b1314(&self) -> B1314_R {
        B1314_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1315"]
    #[inline(always)]
    pub fn b1315(&self) -> B1315_R {
        B1315_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1316"]
    #[inline(always)]
    pub fn b1316(&self) -> B1316_R {
        B1316_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1317"]
    #[inline(always)]
    pub fn b1317(&self) -> B1317_R {
        B1317_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1318"]
    #[inline(always)]
    pub fn b1318(&self) -> B1318_R {
        B1318_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1319"]
    #[inline(always)]
    pub fn b1319(&self) -> B1319_R {
        B1319_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1320"]
    #[inline(always)]
    pub fn b1320(&self) -> B1320_R {
        B1320_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1321"]
    #[inline(always)]
    pub fn b1321(&self) -> B1321_R {
        B1321_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1322"]
    #[inline(always)]
    pub fn b1322(&self) -> B1322_R {
        B1322_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1323"]
    #[inline(always)]
    pub fn b1323(&self) -> B1323_R {
        B1323_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1324"]
    #[inline(always)]
    pub fn b1324(&self) -> B1324_R {
        B1324_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1325"]
    #[inline(always)]
    pub fn b1325(&self) -> B1325_R {
        B1325_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1326"]
    #[inline(always)]
    pub fn b1326(&self) -> B1326_R {
        B1326_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1327"]
    #[inline(always)]
    pub fn b1327(&self) -> B1327_R {
        B1327_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1328"]
    #[inline(always)]
    pub fn b1328(&self) -> B1328_R {
        B1328_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1329"]
    #[inline(always)]
    pub fn b1329(&self) -> B1329_R {
        B1329_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1330"]
    #[inline(always)]
    pub fn b1330(&self) -> B1330_R {
        B1330_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1331"]
    #[inline(always)]
    pub fn b1331(&self) -> B1331_R {
        B1331_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1332"]
    #[inline(always)]
    pub fn b1332(&self) -> B1332_R {
        B1332_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1333"]
    #[inline(always)]
    pub fn b1333(&self) -> B1333_R {
        B1333_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1334"]
    #[inline(always)]
    pub fn b1334(&self) -> B1334_R {
        B1334_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1335"]
    #[inline(always)]
    pub fn b1335(&self) -> B1335_R {
        B1335_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1336"]
    #[inline(always)]
    pub fn b1336(&self) -> B1336_R {
        B1336_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1337"]
    #[inline(always)]
    pub fn b1337(&self) -> B1337_R {
        B1337_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1338"]
    #[inline(always)]
    pub fn b1338(&self) -> B1338_R {
        B1338_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1339"]
    #[inline(always)]
    pub fn b1339(&self) -> B1339_R {
        B1339_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1340"]
    #[inline(always)]
    pub fn b1340(&self) -> B1340_R {
        B1340_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1341"]
    #[inline(always)]
    pub fn b1341(&self) -> B1341_R {
        B1341_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1342"]
    #[inline(always)]
    pub fn b1342(&self) -> B1342_R {
        B1342_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1343"]
    #[inline(always)]
    pub fn b1343(&self) -> B1343_R {
        B1343_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1312"]
    #[inline(always)]
    pub fn b1312(&mut self) -> B1312_W {
        B1312_W { w: self }
    }
    #[doc = "Bit 1 - B1313"]
    #[inline(always)]
    pub fn b1313(&mut self) -> B1313_W {
        B1313_W { w: self }
    }
    #[doc = "Bit 2 - B1314"]
    #[inline(always)]
    pub fn b1314(&mut self) -> B1314_W {
        B1314_W { w: self }
    }
    #[doc = "Bit 3 - B1315"]
    #[inline(always)]
    pub fn b1315(&mut self) -> B1315_W {
        B1315_W { w: self }
    }
    #[doc = "Bit 4 - B1316"]
    #[inline(always)]
    pub fn b1316(&mut self) -> B1316_W {
        B1316_W { w: self }
    }
    #[doc = "Bit 5 - B1317"]
    #[inline(always)]
    pub fn b1317(&mut self) -> B1317_W {
        B1317_W { w: self }
    }
    #[doc = "Bit 6 - B1318"]
    #[inline(always)]
    pub fn b1318(&mut self) -> B1318_W {
        B1318_W { w: self }
    }
    #[doc = "Bit 7 - B1319"]
    #[inline(always)]
    pub fn b1319(&mut self) -> B1319_W {
        B1319_W { w: self }
    }
    #[doc = "Bit 8 - B1320"]
    #[inline(always)]
    pub fn b1320(&mut self) -> B1320_W {
        B1320_W { w: self }
    }
    #[doc = "Bit 9 - B1321"]
    #[inline(always)]
    pub fn b1321(&mut self) -> B1321_W {
        B1321_W { w: self }
    }
    #[doc = "Bit 10 - B1322"]
    #[inline(always)]
    pub fn b1322(&mut self) -> B1322_W {
        B1322_W { w: self }
    }
    #[doc = "Bit 11 - B1323"]
    #[inline(always)]
    pub fn b1323(&mut self) -> B1323_W {
        B1323_W { w: self }
    }
    #[doc = "Bit 12 - B1324"]
    #[inline(always)]
    pub fn b1324(&mut self) -> B1324_W {
        B1324_W { w: self }
    }
    #[doc = "Bit 13 - B1325"]
    #[inline(always)]
    pub fn b1325(&mut self) -> B1325_W {
        B1325_W { w: self }
    }
    #[doc = "Bit 14 - B1326"]
    #[inline(always)]
    pub fn b1326(&mut self) -> B1326_W {
        B1326_W { w: self }
    }
    #[doc = "Bit 15 - B1327"]
    #[inline(always)]
    pub fn b1327(&mut self) -> B1327_W {
        B1327_W { w: self }
    }
    #[doc = "Bit 16 - B1328"]
    #[inline(always)]
    pub fn b1328(&mut self) -> B1328_W {
        B1328_W { w: self }
    }
    #[doc = "Bit 17 - B1329"]
    #[inline(always)]
    pub fn b1329(&mut self) -> B1329_W {
        B1329_W { w: self }
    }
    #[doc = "Bit 18 - B1330"]
    #[inline(always)]
    pub fn b1330(&mut self) -> B1330_W {
        B1330_W { w: self }
    }
    #[doc = "Bit 19 - B1331"]
    #[inline(always)]
    pub fn b1331(&mut self) -> B1331_W {
        B1331_W { w: self }
    }
    #[doc = "Bit 20 - B1332"]
    #[inline(always)]
    pub fn b1332(&mut self) -> B1332_W {
        B1332_W { w: self }
    }
    #[doc = "Bit 21 - B1333"]
    #[inline(always)]
    pub fn b1333(&mut self) -> B1333_W {
        B1333_W { w: self }
    }
    #[doc = "Bit 22 - B1334"]
    #[inline(always)]
    pub fn b1334(&mut self) -> B1334_W {
        B1334_W { w: self }
    }
    #[doc = "Bit 23 - B1335"]
    #[inline(always)]
    pub fn b1335(&mut self) -> B1335_W {
        B1335_W { w: self }
    }
    #[doc = "Bit 24 - B1336"]
    #[inline(always)]
    pub fn b1336(&mut self) -> B1336_W {
        B1336_W { w: self }
    }
    #[doc = "Bit 25 - B1337"]
    #[inline(always)]
    pub fn b1337(&mut self) -> B1337_W {
        B1337_W { w: self }
    }
    #[doc = "Bit 26 - B1338"]
    #[inline(always)]
    pub fn b1338(&mut self) -> B1338_W {
        B1338_W { w: self }
    }
    #[doc = "Bit 27 - B1339"]
    #[inline(always)]
    pub fn b1339(&mut self) -> B1339_W {
        B1339_W { w: self }
    }
    #[doc = "Bit 28 - B1340"]
    #[inline(always)]
    pub fn b1340(&mut self) -> B1340_W {
        B1340_W { w: self }
    }
    #[doc = "Bit 29 - B1341"]
    #[inline(always)]
    pub fn b1341(&mut self) -> B1341_W {
        B1341_W { w: self }
    }
    #[doc = "Bit 30 - B1342"]
    #[inline(always)]
    pub fn b1342(&mut self) -> B1342_W {
        B1342_W { w: self }
    }
    #[doc = "Bit 31 - B1343"]
    #[inline(always)]
    pub fn b1343(&mut self) -> B1343_W {
        B1343_W { w: self }
    }
}
