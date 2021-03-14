#[doc = "Reader of register SYSCFG_BOOTR"]
pub type R = crate::R<u32, super::SYSCFG_BOOTR>;
#[doc = "Writer for register SYSCFG_BOOTR"]
pub type W = crate::W<u32, super::SYSCFG_BOOTR>;
#[doc = "Register SYSCFG_BOOTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG_BOOTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOOT0`"]
pub type BOOT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOOT1`"]
pub type BOOT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOOT2`"]
pub type BOOT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOOT0_PD`"]
pub type BOOT0_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT0_PD`"]
pub struct BOOT0_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT0_PD_W<'a> {
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
#[doc = "Reader of field `BOOT1_PD`"]
pub type BOOT1_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT1_PD`"]
pub struct BOOT1_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT1_PD_W<'a> {
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
#[doc = "Reader of field `BOOT2_PD`"]
pub type BOOT2_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT2_PD`"]
pub struct BOOT2_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT2_PD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BOOT0"]
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOOT1"]
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BOOT2"]
    #[inline(always)]
    pub fn boot2(&self) -> BOOT2_R {
        BOOT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BOOT0_PD"]
    #[inline(always)]
    pub fn boot0_pd(&self) -> BOOT0_PD_R {
        BOOT0_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BOOT1_PD"]
    #[inline(always)]
    pub fn boot1_pd(&self) -> BOOT1_PD_R {
        BOOT1_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOOT2_PD"]
    #[inline(always)]
    pub fn boot2_pd(&self) -> BOOT2_PD_R {
        BOOT2_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - BOOT0_PD"]
    #[inline(always)]
    pub fn boot0_pd(&mut self) -> BOOT0_PD_W {
        BOOT0_PD_W { w: self }
    }
    #[doc = "Bit 5 - BOOT1_PD"]
    #[inline(always)]
    pub fn boot1_pd(&mut self) -> BOOT1_PD_W {
        BOOT1_PD_W { w: self }
    }
    #[doc = "Bit 6 - BOOT2_PD"]
    #[inline(always)]
    pub fn boot2_pd(&mut self) -> BOOT2_PD_W {
        BOOT2_PD_W { w: self }
    }
}
