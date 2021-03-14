#[doc = "Reader of register PWR_WKUPCR"]
pub type R = crate::R<u32, super::PWR_WKUPCR>;
#[doc = "Writer for register PWR_WKUPCR"]
pub type W = crate::W<u32, super::PWR_WKUPCR>;
#[doc = "Register PWR_WKUPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_WKUPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WKUPC1`"]
pub type WKUPC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPC1`"]
pub struct WKUPC1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC1_W<'a> {
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
#[doc = "Reader of field `WKUPC2`"]
pub type WKUPC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPC2`"]
pub struct WKUPC2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC2_W<'a> {
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
#[doc = "Reader of field `WKUPC3`"]
pub type WKUPC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPC3`"]
pub struct WKUPC3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC3_W<'a> {
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
#[doc = "Reader of field `WKUPC4`"]
pub type WKUPC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPC4`"]
pub struct WKUPC4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC4_W<'a> {
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
#[doc = "Reader of field `WKUPC5`"]
pub type WKUPC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPC5`"]
pub struct WKUPC5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC5_W<'a> {
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
#[doc = "Reader of field `WKUPC6`"]
pub type WKUPC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPC6`"]
pub struct WKUPC6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC6_W<'a> {
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
#[doc = "Reader of field `WKUPP1`"]
pub type WKUPP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPP1`"]
pub struct WKUPP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP1_W<'a> {
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
#[doc = "Reader of field `WKUPP2`"]
pub type WKUPP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPP2`"]
pub struct WKUPP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP2_W<'a> {
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
#[doc = "Reader of field `WKUPP3`"]
pub type WKUPP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPP3`"]
pub struct WKUPP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP3_W<'a> {
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
#[doc = "Reader of field `WKUPP4`"]
pub type WKUPP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPP4`"]
pub struct WKUPP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP4_W<'a> {
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
#[doc = "Reader of field `WKUPP5`"]
pub type WKUPP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPP5`"]
pub struct WKUPP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP5_W<'a> {
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
#[doc = "Reader of field `WKUPP6`"]
pub type WKUPP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPP6`"]
pub struct WKUPP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP6_W<'a> {
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
#[doc = "Reader of field `WKUPPUPD1`"]
pub type WKUPPUPD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WKUPPUPD1`"]
pub struct WKUPPUPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `WKUPPUPD2`"]
pub type WKUPPUPD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WKUPPUPD2`"]
pub struct WKUPPUPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `WKUPPUPD3`"]
pub type WKUPPUPD3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WKUPPUPD3`"]
pub struct WKUPPUPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `WKUPPUPD4`"]
pub type WKUPPUPD4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WKUPPUPD4`"]
pub struct WKUPPUPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `WKUPPUPD5`"]
pub type WKUPPUPD5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WKUPPUPD5`"]
pub struct WKUPPUPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `WKUPPUPD6`"]
pub type WKUPPUPD6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WKUPPUPD6`"]
pub struct WKUPPUPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WKUPC3"]
    #[inline(always)]
    pub fn wkupc3(&self) -> WKUPC3_R {
        WKUPC3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WKUPC5"]
    #[inline(always)]
    pub fn wkupc5(&self) -> WKUPC5_R {
        WKUPC5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&self) -> WKUPC6_R {
        WKUPC6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WKUPP1"]
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WKUPP2"]
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WKUPP3"]
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WKUPP4"]
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WKUPP5"]
    #[inline(always)]
    pub fn wkupp5(&self) -> WKUPP5_R {
        WKUPP5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WKUPP6"]
    #[inline(always)]
    pub fn wkupp6(&self) -> WKUPP6_R {
        WKUPP6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - WKUPPUPD1"]
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - WKUPPUPD2"]
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - WKUPPUPD3"]
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - WKUPPUPD4"]
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - WKUPPUPD5"]
    #[inline(always)]
    pub fn wkuppupd5(&self) -> WKUPPUPD5_R {
        WKUPPUPD5_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - WKUPPUPD6"]
    #[inline(always)]
    pub fn wkuppupd6(&self) -> WKUPPUPD6_R {
        WKUPPUPD6_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&mut self) -> WKUPC1_W {
        WKUPC1_W { w: self }
    }
    #[doc = "Bit 1 - WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&mut self) -> WKUPC2_W {
        WKUPC2_W { w: self }
    }
    #[doc = "Bit 2 - WKUPC3"]
    #[inline(always)]
    pub fn wkupc3(&mut self) -> WKUPC3_W {
        WKUPC3_W { w: self }
    }
    #[doc = "Bit 3 - WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&mut self) -> WKUPC4_W {
        WKUPC4_W { w: self }
    }
    #[doc = "Bit 4 - WKUPC5"]
    #[inline(always)]
    pub fn wkupc5(&mut self) -> WKUPC5_W {
        WKUPC5_W { w: self }
    }
    #[doc = "Bit 5 - WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&mut self) -> WKUPC6_W {
        WKUPC6_W { w: self }
    }
    #[doc = "Bit 8 - WKUPP1"]
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W {
        WKUPP1_W { w: self }
    }
    #[doc = "Bit 9 - WKUPP2"]
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W {
        WKUPP2_W { w: self }
    }
    #[doc = "Bit 10 - WKUPP3"]
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W {
        WKUPP3_W { w: self }
    }
    #[doc = "Bit 11 - WKUPP4"]
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W {
        WKUPP4_W { w: self }
    }
    #[doc = "Bit 12 - WKUPP5"]
    #[inline(always)]
    pub fn wkupp5(&mut self) -> WKUPP5_W {
        WKUPP5_W { w: self }
    }
    #[doc = "Bit 13 - WKUPP6"]
    #[inline(always)]
    pub fn wkupp6(&mut self) -> WKUPP6_W {
        WKUPP6_W { w: self }
    }
    #[doc = "Bits 16:17 - WKUPPUPD1"]
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W {
        WKUPPUPD1_W { w: self }
    }
    #[doc = "Bits 18:19 - WKUPPUPD2"]
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W {
        WKUPPUPD2_W { w: self }
    }
    #[doc = "Bits 20:21 - WKUPPUPD3"]
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W {
        WKUPPUPD3_W { w: self }
    }
    #[doc = "Bits 22:23 - WKUPPUPD4"]
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W {
        WKUPPUPD4_W { w: self }
    }
    #[doc = "Bits 24:25 - WKUPPUPD5"]
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> WKUPPUPD5_W {
        WKUPPUPD5_W { w: self }
    }
    #[doc = "Bits 26:27 - WKUPPUPD6"]
    #[inline(always)]
    pub fn wkuppupd6(&mut self) -> WKUPPUPD6_W {
        WKUPPUPD6_W { w: self }
    }
}
