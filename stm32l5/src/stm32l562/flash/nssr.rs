#[doc = "Reader of register NSSR"]
pub type R = crate::R<u32, super::NSSR>;
#[doc = "Writer for register NSSR"]
pub type W = crate::W<u32, super::NSSR>;
#[doc = "Register NSSR `reset()`'s with value 0"]
impl crate::ResetValue for super::NSSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NSEOP`"]
pub type NSEOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSEOP`"]
pub struct NSEOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEOP_W<'a> {
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
#[doc = "Reader of field `NSOPERR`"]
pub type NSOPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSOPERR`"]
pub struct NSOPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSOPERR_W<'a> {
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
#[doc = "Reader of field `NSPROGERR`"]
pub type NSPROGERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSPROGERR`"]
pub struct NSPROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPROGERR_W<'a> {
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
#[doc = "Reader of field `NSWRPERR`"]
pub type NSWRPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSWRPERR`"]
pub struct NSWRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWRPERR_W<'a> {
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
#[doc = "Reader of field `NSPGAERR`"]
pub type NSPGAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSPGAERR`"]
pub struct NSPGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPGAERR_W<'a> {
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
#[doc = "Reader of field `NSSIZERR`"]
pub type NSSIZERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSSIZERR`"]
pub struct NSSIZERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSIZERR_W<'a> {
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
#[doc = "Reader of field `NSPGSERR`"]
pub type NSPGSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSPGSERR`"]
pub struct NSPGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPGSERR_W<'a> {
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
#[doc = "Reader of field `OPTWERR`"]
pub type OPTWERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPTWERR`"]
pub struct OPTWERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTWERR_W<'a> {
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
#[doc = "Reader of field `OPTVERR`"]
pub type OPTVERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPTVERR`"]
pub struct OPTVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTVERR_W<'a> {
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
#[doc = "Reader of field `NSBSY`"]
pub type NSBSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    pub fn nseop(&self) -> NSEOP_R {
        NSEOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    pub fn nsoperr(&self) -> NSOPERR_R {
        NSOPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    pub fn nsprogerr(&self) -> NSPROGERR_R {
        NSPROGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    pub fn nswrperr(&self) -> NSWRPERR_R {
        NSWRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    pub fn nspgaerr(&self) -> NSPGAERR_R {
        NSPGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    pub fn nssizerr(&self) -> NSSIZERR_R {
        NSSIZERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    pub fn nspgserr(&self) -> NSPGSERR_R {
        NSPGSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    pub fn optwerr(&self) -> OPTWERR_R {
        OPTWERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NSBusy"]
    #[inline(always)]
    pub fn nsbsy(&self) -> NSBSY_R {
        NSBSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    pub fn nseop(&mut self) -> NSEOP_W {
        NSEOP_W { w: self }
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    pub fn nsoperr(&mut self) -> NSOPERR_W {
        NSOPERR_W { w: self }
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    pub fn nsprogerr(&mut self) -> NSPROGERR_W {
        NSPROGERR_W { w: self }
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    pub fn nswrperr(&mut self) -> NSWRPERR_W {
        NSWRPERR_W { w: self }
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    pub fn nspgaerr(&mut self) -> NSPGAERR_W {
        NSPGAERR_W { w: self }
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    pub fn nssizerr(&mut self) -> NSSIZERR_W {
        NSSIZERR_W { w: self }
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    pub fn nspgserr(&mut self) -> NSPGSERR_W {
        NSPGSERR_W { w: self }
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    pub fn optwerr(&mut self) -> OPTWERR_W {
        OPTWERR_W { w: self }
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W {
        OPTVERR_W { w: self }
    }
}
