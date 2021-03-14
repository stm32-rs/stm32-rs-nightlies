#[doc = "Reader of register OTG_FS_GOTGCTL"]
pub type R = crate::R<u32, super::OTG_FS_GOTGCTL>;
#[doc = "Writer for register OTG_FS_GOTGCTL"]
pub type W = crate::W<u32, super::OTG_FS_GOTGCTL>;
#[doc = "Register OTG_FS_GOTGCTL `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::OTG_FS_GOTGCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "Reader of field `SRQSCS`"]
pub type SRQSCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRQ`"]
pub type SRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRQ`"]
pub struct SRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQ_W<'a> {
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
#[doc = "Reader of field `HNGSCS`"]
pub type HNGSCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HNPRQ`"]
pub type HNPRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPRQ`"]
pub struct HNPRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPRQ_W<'a> {
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
#[doc = "Reader of field `HSHNPEN`"]
pub type HSHNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSHNPEN`"]
pub struct HSHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSHNPEN_W<'a> {
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
#[doc = "Reader of field `DHNPEN`"]
pub type DHNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DHNPEN`"]
pub struct DHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DHNPEN_W<'a> {
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
#[doc = "Reader of field `CIDSTS`"]
pub type CIDSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBCT`"]
pub type DBCT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASVLD`"]
pub type ASVLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSVLD`"]
pub type BSVLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBVALOEN`"]
pub type VBVALOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBVALOEN`"]
pub struct VBVALOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALOEN_W<'a> {
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
#[doc = "Reader of field `VBVALOVAL`"]
pub type VBVALOVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBVALOVAL`"]
pub struct VBVALOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALOVAL_W<'a> {
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
#[doc = "Reader of field `AVALOEN`"]
pub type AVALOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVALOEN`"]
pub struct AVALOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALOEN_W<'a> {
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
#[doc = "Reader of field `AVALOVAL`"]
pub type AVALOVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVALOVAL`"]
pub struct AVALOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALOVAL_W<'a> {
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
#[doc = "Reader of field `BVALOEN`"]
pub type BVALOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BVALOEN`"]
pub struct BVALOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALOEN_W<'a> {
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
#[doc = "Reader of field `BVALOVAL`"]
pub type BVALOVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BVALOVAL`"]
pub struct BVALOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALOVAL_W<'a> {
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
#[doc = "Reader of field `EHEN`"]
pub type EHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EHEN`"]
pub struct EHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EHEN_W<'a> {
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
#[doc = "Reader of field `OTGVER`"]
pub type OTGVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGVER`"]
pub struct OTGVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGVER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W {
        SRQ_W { w: self }
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W {
        HNPRQ_W { w: self }
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W {
        HSHNPEN_W { w: self }
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W {
        DHNPEN_W { w: self }
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W {
        VBVALOEN_W { w: self }
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W {
        VBVALOVAL_W { w: self }
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    pub fn avaloen(&mut self) -> AVALOEN_W {
        AVALOEN_W { w: self }
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    pub fn avaloval(&mut self) -> AVALOVAL_W {
        AVALOVAL_W { w: self }
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    pub fn bvaloen(&mut self) -> BVALOEN_W {
        BVALOEN_W { w: self }
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    pub fn bvaloval(&mut self) -> BVALOVAL_W {
        BVALOVAL_W { w: self }
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&mut self) -> EHEN_W {
        EHEN_W { w: self }
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W {
        OTGVER_W { w: self }
    }
}
