#[doc = "Reader of register SECCR"]
pub type R = crate::R<u32, super::SECCR>;
#[doc = "Writer for register SECCR"]
pub type W = crate::W<u32, super::SECCR>;
#[doc = "Register SECCR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::SECCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `SECPG`"]
pub type SECPG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECPG`"]
pub struct SECPG_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPG_W<'a> {
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
#[doc = "Reader of field `SECPER`"]
pub type SECPER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECPER`"]
pub struct SECPER_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPER_W<'a> {
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
#[doc = "Reader of field `SECMER1`"]
pub type SECMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECMER1`"]
pub struct SECMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> SECMER1_W<'a> {
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
#[doc = "Reader of field `SECPNB`"]
pub type SECPNB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECPNB`"]
pub struct SECPNB_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 3)) | (((value as u32) & 0x7f) << 3);
        self.w
    }
}
#[doc = "Reader of field `SECBKER`"]
pub type SECBKER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECBKER`"]
pub struct SECBKER_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBKER_W<'a> {
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
#[doc = "Reader of field `SECMER2`"]
pub type SECMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECMER2`"]
pub struct SECMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> SECMER2_W<'a> {
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
#[doc = "Reader of field `SECSTRT`"]
pub type SECSTRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECSTRT`"]
pub struct SECSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECSTRT_W<'a> {
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
#[doc = "Reader of field `SECEOPIE`"]
pub type SECEOPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECEOPIE`"]
pub struct SECEOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECEOPIE_W<'a> {
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
#[doc = "Reader of field `SECERRIE`"]
pub type SECERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECERRIE`"]
pub struct SECERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECERRIE_W<'a> {
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
#[doc = "Reader of field `SECRDERRIE`"]
pub type SECRDERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECRDERRIE`"]
pub struct SECRDERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECRDERRIE_W<'a> {
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
#[doc = "Reader of field `SECINV`"]
pub type SECINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECINV`"]
pub struct SECINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SECINV_W<'a> {
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
#[doc = "Reader of field `SECLOCK`"]
pub type SECLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECLOCK`"]
pub struct SECLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SECLOCK_W<'a> {
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
    #[doc = "Bit 0 - SECPG"]
    #[inline(always)]
    pub fn secpg(&self) -> SECPG_R {
        SECPG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SECPER"]
    #[inline(always)]
    pub fn secper(&self) -> SECPER_R {
        SECPER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SECMER1"]
    #[inline(always)]
    pub fn secmer1(&self) -> SECMER1_R {
        SECMER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:9 - SECPNB"]
    #[inline(always)]
    pub fn secpnb(&self) -> SECPNB_R {
        SECPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - SECBKER"]
    #[inline(always)]
    pub fn secbker(&self) -> SECBKER_R {
        SECBKER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SECMER2"]
    #[inline(always)]
    pub fn secmer2(&self) -> SECMER2_R {
        SECMER2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SECSTRT"]
    #[inline(always)]
    pub fn secstrt(&self) -> SECSTRT_R {
        SECSTRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SECEOPIE"]
    #[inline(always)]
    pub fn seceopie(&self) -> SECEOPIE_R {
        SECEOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SECERRIE"]
    #[inline(always)]
    pub fn secerrie(&self) -> SECERRIE_R {
        SECERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SECRDERRIE"]
    #[inline(always)]
    pub fn secrderrie(&self) -> SECRDERRIE_R {
        SECRDERRIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SECINV"]
    #[inline(always)]
    pub fn secinv(&self) -> SECINV_R {
        SECINV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SECLOCK"]
    #[inline(always)]
    pub fn seclock(&self) -> SECLOCK_R {
        SECLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SECPG"]
    #[inline(always)]
    pub fn secpg(&mut self) -> SECPG_W {
        SECPG_W { w: self }
    }
    #[doc = "Bit 1 - SECPER"]
    #[inline(always)]
    pub fn secper(&mut self) -> SECPER_W {
        SECPER_W { w: self }
    }
    #[doc = "Bit 2 - SECMER1"]
    #[inline(always)]
    pub fn secmer1(&mut self) -> SECMER1_W {
        SECMER1_W { w: self }
    }
    #[doc = "Bits 3:9 - SECPNB"]
    #[inline(always)]
    pub fn secpnb(&mut self) -> SECPNB_W {
        SECPNB_W { w: self }
    }
    #[doc = "Bit 11 - SECBKER"]
    #[inline(always)]
    pub fn secbker(&mut self) -> SECBKER_W {
        SECBKER_W { w: self }
    }
    #[doc = "Bit 15 - SECMER2"]
    #[inline(always)]
    pub fn secmer2(&mut self) -> SECMER2_W {
        SECMER2_W { w: self }
    }
    #[doc = "Bit 16 - SECSTRT"]
    #[inline(always)]
    pub fn secstrt(&mut self) -> SECSTRT_W {
        SECSTRT_W { w: self }
    }
    #[doc = "Bit 24 - SECEOPIE"]
    #[inline(always)]
    pub fn seceopie(&mut self) -> SECEOPIE_W {
        SECEOPIE_W { w: self }
    }
    #[doc = "Bit 25 - SECERRIE"]
    #[inline(always)]
    pub fn secerrie(&mut self) -> SECERRIE_W {
        SECERRIE_W { w: self }
    }
    #[doc = "Bit 26 - SECRDERRIE"]
    #[inline(always)]
    pub fn secrderrie(&mut self) -> SECRDERRIE_W {
        SECRDERRIE_W { w: self }
    }
    #[doc = "Bit 29 - SECINV"]
    #[inline(always)]
    pub fn secinv(&mut self) -> SECINV_W {
        SECINV_W { w: self }
    }
    #[doc = "Bit 31 - SECLOCK"]
    #[inline(always)]
    pub fn seclock(&mut self) -> SECLOCK_W {
        SECLOCK_W { w: self }
    }
}
