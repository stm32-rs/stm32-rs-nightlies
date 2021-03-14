#[doc = "Reader of register NSCR"]
pub type R = crate::R<u32, super::NSCR>;
#[doc = "Writer for register NSCR"]
pub type W = crate::W<u32, super::NSCR>;
#[doc = "Register NSCR `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::NSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `NSPG`"]
pub type NSPG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSPG`"]
pub struct NSPG_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPG_W<'a> {
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
#[doc = "Reader of field `NSPER`"]
pub type NSPER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSPER`"]
pub struct NSPER_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPER_W<'a> {
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
#[doc = "Reader of field `NSMER1`"]
pub type NSMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSMER1`"]
pub struct NSMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> NSMER1_W<'a> {
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
#[doc = "Reader of field `NSPNB`"]
pub type NSPNB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NSPNB`"]
pub struct NSPNB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 3)) | (((value as u32) & 0x7f) << 3);
        self.w
    }
}
#[doc = "Reader of field `NSBKER`"]
pub type NSBKER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSBKER`"]
pub struct NSBKER_W<'a> {
    w: &'a mut W,
}
impl<'a> NSBKER_W<'a> {
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
#[doc = "Reader of field `NSMER2`"]
pub type NSMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSMER2`"]
pub struct NSMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> NSMER2_W<'a> {
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
#[doc = "Reader of field `NSSTRT`"]
pub type NSSTRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSSTRT`"]
pub struct NSSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSTRT_W<'a> {
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
#[doc = "Reader of field `OPTSTRT`"]
pub type OPTSTRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPTSTRT`"]
pub struct OPTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTSTRT_W<'a> {
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
#[doc = "Reader of field `NSEOPIE`"]
pub type NSEOPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSEOPIE`"]
pub struct NSEOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEOPIE_W<'a> {
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
#[doc = "Reader of field `NSERRIE`"]
pub type NSERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSERRIE`"]
pub struct NSERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NSERRIE_W<'a> {
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
#[doc = "Reader of field `OBL_LAUNCH`"]
pub type OBL_LAUNCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OBL_LAUNCH`"]
pub struct OBL_LAUNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OBL_LAUNCH_W<'a> {
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
#[doc = "Reader of field `OPTLOCK`"]
pub type OPTLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPTLOCK`"]
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
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
#[doc = "Reader of field `NSLOCK`"]
pub type NSLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSLOCK`"]
pub struct NSLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NSLOCK_W<'a> {
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
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    pub fn nspg(&self) -> NSPG_R {
        NSPG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    pub fn nsper(&self) -> NSPER_R {
        NSPER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    pub fn nsmer1(&self) -> NSMER1_R {
        NSMER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    pub fn nspnb(&self) -> NSPNB_R {
        NSPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    pub fn nsbker(&self) -> NSBKER_R {
        NSBKER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    pub fn nsmer2(&self) -> NSMER2_R {
        NSMER2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    pub fn nsstrt(&self) -> NSSTRT_R {
        NSSTRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    pub fn nseopie(&self) -> NSEOPIE_R {
        NSEOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    pub fn nserrie(&self) -> NSERRIE_R {
        NSERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    pub fn nslock(&self) -> NSLOCK_R {
        NSLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    pub fn nspg(&mut self) -> NSPG_W {
        NSPG_W { w: self }
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    pub fn nsper(&mut self) -> NSPER_W {
        NSPER_W { w: self }
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    pub fn nsmer1(&mut self) -> NSMER1_W {
        NSMER1_W { w: self }
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    pub fn nspnb(&mut self) -> NSPNB_W {
        NSPNB_W { w: self }
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    pub fn nsbker(&mut self) -> NSBKER_W {
        NSBKER_W { w: self }
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    pub fn nsmer2(&mut self) -> NSMER2_W {
        NSMER2_W { w: self }
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    pub fn nsstrt(&mut self) -> NSSTRT_W {
        NSSTRT_W { w: self }
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W {
        OPTSTRT_W { w: self }
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    pub fn nseopie(&mut self) -> NSEOPIE_W {
        NSEOPIE_W { w: self }
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    pub fn nserrie(&mut self) -> NSERRIE_W {
        NSERRIE_W { w: self }
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W {
        OBL_LAUNCH_W { w: self }
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    pub fn nslock(&mut self) -> NSLOCK_W {
        NSLOCK_W { w: self }
    }
}
