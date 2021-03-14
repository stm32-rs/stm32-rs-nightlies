#[doc = "Reader of register MPCBB1_VCTR42"]
pub type R = crate::R<u32, super::MPCBB1_VCTR42>;
#[doc = "Writer for register MPCBB1_VCTR42"]
pub type W = crate::W<u32, super::MPCBB1_VCTR42>;
#[doc = "Register MPCBB1_VCTR42 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR42 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1344`"]
pub type B1344_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1344`"]
pub struct B1344_W<'a> {
    w: &'a mut W,
}
impl<'a> B1344_W<'a> {
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
#[doc = "Reader of field `B1345`"]
pub type B1345_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1345`"]
pub struct B1345_W<'a> {
    w: &'a mut W,
}
impl<'a> B1345_W<'a> {
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
#[doc = "Reader of field `B1346`"]
pub type B1346_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1346`"]
pub struct B1346_W<'a> {
    w: &'a mut W,
}
impl<'a> B1346_W<'a> {
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
#[doc = "Reader of field `B1347`"]
pub type B1347_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1347`"]
pub struct B1347_W<'a> {
    w: &'a mut W,
}
impl<'a> B1347_W<'a> {
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
#[doc = "Reader of field `B1348`"]
pub type B1348_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1348`"]
pub struct B1348_W<'a> {
    w: &'a mut W,
}
impl<'a> B1348_W<'a> {
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
#[doc = "Reader of field `B1349`"]
pub type B1349_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1349`"]
pub struct B1349_W<'a> {
    w: &'a mut W,
}
impl<'a> B1349_W<'a> {
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
#[doc = "Reader of field `B1350`"]
pub type B1350_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1350`"]
pub struct B1350_W<'a> {
    w: &'a mut W,
}
impl<'a> B1350_W<'a> {
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
#[doc = "Reader of field `B1351`"]
pub type B1351_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1351`"]
pub struct B1351_W<'a> {
    w: &'a mut W,
}
impl<'a> B1351_W<'a> {
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
#[doc = "Reader of field `B1352`"]
pub type B1352_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1352`"]
pub struct B1352_W<'a> {
    w: &'a mut W,
}
impl<'a> B1352_W<'a> {
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
#[doc = "Reader of field `B1353`"]
pub type B1353_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1353`"]
pub struct B1353_W<'a> {
    w: &'a mut W,
}
impl<'a> B1353_W<'a> {
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
#[doc = "Reader of field `B1354`"]
pub type B1354_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1354`"]
pub struct B1354_W<'a> {
    w: &'a mut W,
}
impl<'a> B1354_W<'a> {
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
#[doc = "Reader of field `B1355`"]
pub type B1355_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1355`"]
pub struct B1355_W<'a> {
    w: &'a mut W,
}
impl<'a> B1355_W<'a> {
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
#[doc = "Reader of field `B1356`"]
pub type B1356_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1356`"]
pub struct B1356_W<'a> {
    w: &'a mut W,
}
impl<'a> B1356_W<'a> {
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
#[doc = "Reader of field `B1357`"]
pub type B1357_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1357`"]
pub struct B1357_W<'a> {
    w: &'a mut W,
}
impl<'a> B1357_W<'a> {
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
#[doc = "Reader of field `B1358`"]
pub type B1358_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1358`"]
pub struct B1358_W<'a> {
    w: &'a mut W,
}
impl<'a> B1358_W<'a> {
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
#[doc = "Reader of field `B1359`"]
pub type B1359_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1359`"]
pub struct B1359_W<'a> {
    w: &'a mut W,
}
impl<'a> B1359_W<'a> {
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
#[doc = "Reader of field `B1360`"]
pub type B1360_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1360`"]
pub struct B1360_W<'a> {
    w: &'a mut W,
}
impl<'a> B1360_W<'a> {
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
#[doc = "Reader of field `B1361`"]
pub type B1361_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1361`"]
pub struct B1361_W<'a> {
    w: &'a mut W,
}
impl<'a> B1361_W<'a> {
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
#[doc = "Reader of field `B1362`"]
pub type B1362_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1362`"]
pub struct B1362_W<'a> {
    w: &'a mut W,
}
impl<'a> B1362_W<'a> {
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
#[doc = "Reader of field `B1363`"]
pub type B1363_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1363`"]
pub struct B1363_W<'a> {
    w: &'a mut W,
}
impl<'a> B1363_W<'a> {
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
#[doc = "Reader of field `B1364`"]
pub type B1364_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1364`"]
pub struct B1364_W<'a> {
    w: &'a mut W,
}
impl<'a> B1364_W<'a> {
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
#[doc = "Reader of field `B1365`"]
pub type B1365_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1365`"]
pub struct B1365_W<'a> {
    w: &'a mut W,
}
impl<'a> B1365_W<'a> {
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
#[doc = "Reader of field `B1366`"]
pub type B1366_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1366`"]
pub struct B1366_W<'a> {
    w: &'a mut W,
}
impl<'a> B1366_W<'a> {
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
#[doc = "Reader of field `B1367`"]
pub type B1367_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1367`"]
pub struct B1367_W<'a> {
    w: &'a mut W,
}
impl<'a> B1367_W<'a> {
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
#[doc = "Reader of field `B1368`"]
pub type B1368_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1368`"]
pub struct B1368_W<'a> {
    w: &'a mut W,
}
impl<'a> B1368_W<'a> {
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
#[doc = "Reader of field `B1369`"]
pub type B1369_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1369`"]
pub struct B1369_W<'a> {
    w: &'a mut W,
}
impl<'a> B1369_W<'a> {
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
#[doc = "Reader of field `B1370`"]
pub type B1370_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1370`"]
pub struct B1370_W<'a> {
    w: &'a mut W,
}
impl<'a> B1370_W<'a> {
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
#[doc = "Reader of field `B1371`"]
pub type B1371_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1371`"]
pub struct B1371_W<'a> {
    w: &'a mut W,
}
impl<'a> B1371_W<'a> {
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
#[doc = "Reader of field `B1372`"]
pub type B1372_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1372`"]
pub struct B1372_W<'a> {
    w: &'a mut W,
}
impl<'a> B1372_W<'a> {
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
#[doc = "Reader of field `B1373`"]
pub type B1373_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1373`"]
pub struct B1373_W<'a> {
    w: &'a mut W,
}
impl<'a> B1373_W<'a> {
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
#[doc = "Reader of field `B1374`"]
pub type B1374_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1374`"]
pub struct B1374_W<'a> {
    w: &'a mut W,
}
impl<'a> B1374_W<'a> {
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
#[doc = "Reader of field `B1375`"]
pub type B1375_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1375`"]
pub struct B1375_W<'a> {
    w: &'a mut W,
}
impl<'a> B1375_W<'a> {
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
    #[doc = "Bit 0 - B1344"]
    #[inline(always)]
    pub fn b1344(&self) -> B1344_R {
        B1344_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1345"]
    #[inline(always)]
    pub fn b1345(&self) -> B1345_R {
        B1345_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1346"]
    #[inline(always)]
    pub fn b1346(&self) -> B1346_R {
        B1346_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1347"]
    #[inline(always)]
    pub fn b1347(&self) -> B1347_R {
        B1347_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1348"]
    #[inline(always)]
    pub fn b1348(&self) -> B1348_R {
        B1348_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1349"]
    #[inline(always)]
    pub fn b1349(&self) -> B1349_R {
        B1349_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1350"]
    #[inline(always)]
    pub fn b1350(&self) -> B1350_R {
        B1350_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1351"]
    #[inline(always)]
    pub fn b1351(&self) -> B1351_R {
        B1351_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1352"]
    #[inline(always)]
    pub fn b1352(&self) -> B1352_R {
        B1352_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1353"]
    #[inline(always)]
    pub fn b1353(&self) -> B1353_R {
        B1353_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1354"]
    #[inline(always)]
    pub fn b1354(&self) -> B1354_R {
        B1354_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1355"]
    #[inline(always)]
    pub fn b1355(&self) -> B1355_R {
        B1355_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1356"]
    #[inline(always)]
    pub fn b1356(&self) -> B1356_R {
        B1356_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1357"]
    #[inline(always)]
    pub fn b1357(&self) -> B1357_R {
        B1357_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1358"]
    #[inline(always)]
    pub fn b1358(&self) -> B1358_R {
        B1358_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1359"]
    #[inline(always)]
    pub fn b1359(&self) -> B1359_R {
        B1359_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1360"]
    #[inline(always)]
    pub fn b1360(&self) -> B1360_R {
        B1360_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1361"]
    #[inline(always)]
    pub fn b1361(&self) -> B1361_R {
        B1361_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1362"]
    #[inline(always)]
    pub fn b1362(&self) -> B1362_R {
        B1362_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1363"]
    #[inline(always)]
    pub fn b1363(&self) -> B1363_R {
        B1363_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1364"]
    #[inline(always)]
    pub fn b1364(&self) -> B1364_R {
        B1364_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1365"]
    #[inline(always)]
    pub fn b1365(&self) -> B1365_R {
        B1365_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1366"]
    #[inline(always)]
    pub fn b1366(&self) -> B1366_R {
        B1366_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1367"]
    #[inline(always)]
    pub fn b1367(&self) -> B1367_R {
        B1367_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1368"]
    #[inline(always)]
    pub fn b1368(&self) -> B1368_R {
        B1368_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1369"]
    #[inline(always)]
    pub fn b1369(&self) -> B1369_R {
        B1369_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1370"]
    #[inline(always)]
    pub fn b1370(&self) -> B1370_R {
        B1370_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1371"]
    #[inline(always)]
    pub fn b1371(&self) -> B1371_R {
        B1371_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1372"]
    #[inline(always)]
    pub fn b1372(&self) -> B1372_R {
        B1372_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1373"]
    #[inline(always)]
    pub fn b1373(&self) -> B1373_R {
        B1373_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1374"]
    #[inline(always)]
    pub fn b1374(&self) -> B1374_R {
        B1374_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1375"]
    #[inline(always)]
    pub fn b1375(&self) -> B1375_R {
        B1375_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1344"]
    #[inline(always)]
    pub fn b1344(&mut self) -> B1344_W {
        B1344_W { w: self }
    }
    #[doc = "Bit 1 - B1345"]
    #[inline(always)]
    pub fn b1345(&mut self) -> B1345_W {
        B1345_W { w: self }
    }
    #[doc = "Bit 2 - B1346"]
    #[inline(always)]
    pub fn b1346(&mut self) -> B1346_W {
        B1346_W { w: self }
    }
    #[doc = "Bit 3 - B1347"]
    #[inline(always)]
    pub fn b1347(&mut self) -> B1347_W {
        B1347_W { w: self }
    }
    #[doc = "Bit 4 - B1348"]
    #[inline(always)]
    pub fn b1348(&mut self) -> B1348_W {
        B1348_W { w: self }
    }
    #[doc = "Bit 5 - B1349"]
    #[inline(always)]
    pub fn b1349(&mut self) -> B1349_W {
        B1349_W { w: self }
    }
    #[doc = "Bit 6 - B1350"]
    #[inline(always)]
    pub fn b1350(&mut self) -> B1350_W {
        B1350_W { w: self }
    }
    #[doc = "Bit 7 - B1351"]
    #[inline(always)]
    pub fn b1351(&mut self) -> B1351_W {
        B1351_W { w: self }
    }
    #[doc = "Bit 8 - B1352"]
    #[inline(always)]
    pub fn b1352(&mut self) -> B1352_W {
        B1352_W { w: self }
    }
    #[doc = "Bit 9 - B1353"]
    #[inline(always)]
    pub fn b1353(&mut self) -> B1353_W {
        B1353_W { w: self }
    }
    #[doc = "Bit 10 - B1354"]
    #[inline(always)]
    pub fn b1354(&mut self) -> B1354_W {
        B1354_W { w: self }
    }
    #[doc = "Bit 11 - B1355"]
    #[inline(always)]
    pub fn b1355(&mut self) -> B1355_W {
        B1355_W { w: self }
    }
    #[doc = "Bit 12 - B1356"]
    #[inline(always)]
    pub fn b1356(&mut self) -> B1356_W {
        B1356_W { w: self }
    }
    #[doc = "Bit 13 - B1357"]
    #[inline(always)]
    pub fn b1357(&mut self) -> B1357_W {
        B1357_W { w: self }
    }
    #[doc = "Bit 14 - B1358"]
    #[inline(always)]
    pub fn b1358(&mut self) -> B1358_W {
        B1358_W { w: self }
    }
    #[doc = "Bit 15 - B1359"]
    #[inline(always)]
    pub fn b1359(&mut self) -> B1359_W {
        B1359_W { w: self }
    }
    #[doc = "Bit 16 - B1360"]
    #[inline(always)]
    pub fn b1360(&mut self) -> B1360_W {
        B1360_W { w: self }
    }
    #[doc = "Bit 17 - B1361"]
    #[inline(always)]
    pub fn b1361(&mut self) -> B1361_W {
        B1361_W { w: self }
    }
    #[doc = "Bit 18 - B1362"]
    #[inline(always)]
    pub fn b1362(&mut self) -> B1362_W {
        B1362_W { w: self }
    }
    #[doc = "Bit 19 - B1363"]
    #[inline(always)]
    pub fn b1363(&mut self) -> B1363_W {
        B1363_W { w: self }
    }
    #[doc = "Bit 20 - B1364"]
    #[inline(always)]
    pub fn b1364(&mut self) -> B1364_W {
        B1364_W { w: self }
    }
    #[doc = "Bit 21 - B1365"]
    #[inline(always)]
    pub fn b1365(&mut self) -> B1365_W {
        B1365_W { w: self }
    }
    #[doc = "Bit 22 - B1366"]
    #[inline(always)]
    pub fn b1366(&mut self) -> B1366_W {
        B1366_W { w: self }
    }
    #[doc = "Bit 23 - B1367"]
    #[inline(always)]
    pub fn b1367(&mut self) -> B1367_W {
        B1367_W { w: self }
    }
    #[doc = "Bit 24 - B1368"]
    #[inline(always)]
    pub fn b1368(&mut self) -> B1368_W {
        B1368_W { w: self }
    }
    #[doc = "Bit 25 - B1369"]
    #[inline(always)]
    pub fn b1369(&mut self) -> B1369_W {
        B1369_W { w: self }
    }
    #[doc = "Bit 26 - B1370"]
    #[inline(always)]
    pub fn b1370(&mut self) -> B1370_W {
        B1370_W { w: self }
    }
    #[doc = "Bit 27 - B1371"]
    #[inline(always)]
    pub fn b1371(&mut self) -> B1371_W {
        B1371_W { w: self }
    }
    #[doc = "Bit 28 - B1372"]
    #[inline(always)]
    pub fn b1372(&mut self) -> B1372_W {
        B1372_W { w: self }
    }
    #[doc = "Bit 29 - B1373"]
    #[inline(always)]
    pub fn b1373(&mut self) -> B1373_W {
        B1373_W { w: self }
    }
    #[doc = "Bit 30 - B1374"]
    #[inline(always)]
    pub fn b1374(&mut self) -> B1374_W {
        B1374_W { w: self }
    }
    #[doc = "Bit 31 - B1375"]
    #[inline(always)]
    pub fn b1375(&mut self) -> B1375_W {
        B1375_W { w: self }
    }
}
