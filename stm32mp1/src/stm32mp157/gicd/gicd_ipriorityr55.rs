#[doc = "Reader of register GICD_IPRIORITYR55"]
pub type R = crate::R<u32, super::GICD_IPRIORITYR55>;
#[doc = "Writer for register GICD_IPRIORITYR55"]
pub type W = crate::W<u32, super::GICD_IPRIORITYR55>;
#[doc = "Register GICD_IPRIORITYR55 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IPRIORITYR55 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIORITY0`"]
pub type PRIORITY0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY0`"]
pub struct PRIORITY0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PRIORITY1`"]
pub type PRIORITY1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY1`"]
pub struct PRIORITY1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `PRIORITY2`"]
pub type PRIORITY2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY2`"]
pub struct PRIORITY2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `PRIORITY3`"]
pub type PRIORITY3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY3`"]
pub struct PRIORITY3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    pub fn priority0(&self) -> PRIORITY0_R {
        PRIORITY0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    pub fn priority1(&self) -> PRIORITY1_R {
        PRIORITY1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    pub fn priority2(&self) -> PRIORITY2_R {
        PRIORITY2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    pub fn priority3(&self) -> PRIORITY3_R {
        PRIORITY3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    pub fn priority0(&mut self) -> PRIORITY0_W {
        PRIORITY0_W { w: self }
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    pub fn priority1(&mut self) -> PRIORITY1_W {
        PRIORITY1_W { w: self }
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    pub fn priority2(&mut self) -> PRIORITY2_W {
        PRIORITY2_W { w: self }
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    pub fn priority3(&mut self) -> PRIORITY3_W {
        PRIORITY3_W { w: self }
    }
}
