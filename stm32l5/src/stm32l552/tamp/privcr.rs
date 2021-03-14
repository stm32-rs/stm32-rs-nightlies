#[doc = "Reader of register PRIVCR"]
pub type R = crate::R<u32, super::PRIVCR>;
#[doc = "Writer for register PRIVCR"]
pub type W = crate::W<u32, super::PRIVCR>;
#[doc = "Register PRIVCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PRIVCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKPRWPRIV`"]
pub type BKPRWPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKPRWPRIV`"]
pub struct BKPRWPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPRWPRIV_W<'a> {
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
#[doc = "Reader of field `BKPWPRIV`"]
pub type BKPWPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKPWPRIV`"]
pub struct BKPWPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPWPRIV_W<'a> {
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
#[doc = "Reader of field `TAMPPRIV`"]
pub type TAMPPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPPRIV`"]
pub struct TAMPPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRIV_W<'a> {
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
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    pub fn bkprwpriv(&self) -> BKPRWPRIV_R {
        BKPRWPRIV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    pub fn bkpwpriv(&self) -> BKPWPRIV_R {
        BKPWPRIV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Tamper privilege protection"]
    #[inline(always)]
    pub fn tamppriv(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    pub fn bkprwpriv(&mut self) -> BKPRWPRIV_W {
        BKPRWPRIV_W { w: self }
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    pub fn bkpwpriv(&mut self) -> BKPWPRIV_W {
        BKPWPRIV_W { w: self }
    }
    #[doc = "Bit 31 - Tamper privilege protection"]
    #[inline(always)]
    pub fn tamppriv(&mut self) -> TAMPPRIV_W {
        TAMPPRIV_W { w: self }
    }
}
