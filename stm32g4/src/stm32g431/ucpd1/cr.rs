#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXMODE`"]
pub type TXMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXMODE`"]
pub struct TXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TXSEND`"]
pub type TXSEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSEND`"]
pub struct TXSEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSEND_W<'a> {
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
#[doc = "Reader of field `TXHRST`"]
pub type TXHRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXHRST`"]
pub struct TXHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXHRST_W<'a> {
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
#[doc = "Reader of field `RXMODE`"]
pub type RXMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMODE`"]
pub struct RXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMODE_W<'a> {
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
#[doc = "Reader of field `PHYRXEN`"]
pub type PHYRXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYRXEN`"]
pub struct PHYRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYRXEN_W<'a> {
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
#[doc = "Reader of field `PHYCCSEL`"]
pub type PHYCCSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYCCSEL`"]
pub struct PHYCCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYCCSEL_W<'a> {
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
#[doc = "Reader of field `ANASUBMODE`"]
pub type ANASUBMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANASUBMODE`"]
pub struct ANASUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANASUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `ANAMODE`"]
pub type ANAMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANAMODE`"]
pub struct ANAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAMODE_W<'a> {
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
#[doc = "Reader of field `CCENABLE`"]
pub type CCENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCENABLE`"]
pub struct CCENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `FRSRXEN`"]
pub type FRSRXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRSRXEN`"]
pub struct FRSRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSRXEN_W<'a> {
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
#[doc = "Reader of field `FRSTX`"]
pub type FRSTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRSTX`"]
pub struct FRSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTX_W<'a> {
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
#[doc = "Reader of field `RDCH`"]
pub type RDCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDCH`"]
pub struct RDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCH_W<'a> {
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
#[doc = "Reader of field `CC1TCDIS`"]
pub type CC1TCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC1TCDIS`"]
pub struct CC1TCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1TCDIS_W<'a> {
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
#[doc = "Reader of field `CC2TCDIS`"]
pub type CC2TCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC2TCDIS`"]
pub struct CC2TCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2TCDIS_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W {
        TXMODE_W { w: self }
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&mut self) -> TXSEND_W {
        TXSEND_W { w: self }
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&mut self) -> TXHRST_W {
        TXHRST_W { w: self }
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W {
        RXMODE_W { w: self }
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&mut self) -> PHYRXEN_W {
        PHYRXEN_W { w: self }
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W {
        PHYCCSEL_W { w: self }
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W {
        ANASUBMODE_W { w: self }
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&mut self) -> ANAMODE_W {
        ANAMODE_W { w: self }
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&mut self) -> CCENABLE_W {
        CCENABLE_W { w: self }
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&mut self) -> FRSRXEN_W {
        FRSRXEN_W { w: self }
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&mut self) -> FRSTX_W {
        FRSTX_W { w: self }
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&mut self) -> RDCH_W {
        RDCH_W { w: self }
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W {
        CC1TCDIS_W { w: self }
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W {
        CC2TCDIS_W { w: self }
    }
}
