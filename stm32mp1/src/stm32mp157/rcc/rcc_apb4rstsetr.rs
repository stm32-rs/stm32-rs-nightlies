#[doc = "Reader of register RCC_APB4RSTSETR"]
pub type R = crate::R<u32, super::RCC_APB4RSTSETR>;
#[doc = "Writer for register RCC_APB4RSTSETR"]
pub type W = crate::W<u32, super::RCC_APB4RSTSETR>;
#[doc = "Register RCC_APB4RSTSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_APB4RSTSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LTDCRST`"]
pub type LTDCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LTDCRST`"]
pub struct LTDCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCRST_W<'a> {
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
#[doc = "Reader of field `DSIRST`"]
pub type DSIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSIRST`"]
pub struct DSIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIRST_W<'a> {
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
#[doc = "Reader of field `DDRPERFMRST`"]
pub type DDRPERFMRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRPERFMRST`"]
pub struct DDRPERFMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPERFMRST_W<'a> {
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
#[doc = "Reader of field `USBPHYRST`"]
pub type USBPHYRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBPHYRST`"]
pub struct USBPHYRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&self) -> DDRPERFMRST_R {
        DDRPERFMRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&self) -> USBPHYRST_R {
        USBPHYRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W {
        LTDCRST_W { w: self }
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&mut self) -> DSIRST_W {
        DSIRST_W { w: self }
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&mut self) -> DDRPERFMRST_W {
        DDRPERFMRST_W { w: self }
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&mut self) -> USBPHYRST_W {
        USBPHYRST_W { w: self }
    }
}
