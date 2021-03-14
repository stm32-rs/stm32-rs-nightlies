#[doc = "Reader of register GICD_CPENDSGIR1"]
pub type R = crate::R<u32, super::GICD_CPENDSGIR1>;
#[doc = "Writer for register GICD_CPENDSGIR1"]
pub type W = crate::W<u32, super::GICD_CPENDSGIR1>;
#[doc = "Register GICD_CPENDSGIR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_CPENDSGIR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SGI_CLEAR_PENDING0`"]
pub type SGI_CLEAR_PENDING0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SGI_CLEAR_PENDING0`"]
pub struct SGI_CLEAR_PENDING0_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_CLEAR_PENDING0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SGI_CLEAR_PENDING1`"]
pub type SGI_CLEAR_PENDING1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SGI_CLEAR_PENDING1`"]
pub struct SGI_CLEAR_PENDING1_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_CLEAR_PENDING1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SGI_CLEAR_PENDING2`"]
pub type SGI_CLEAR_PENDING2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SGI_CLEAR_PENDING2`"]
pub struct SGI_CLEAR_PENDING2_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_CLEAR_PENDING2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SGI_CLEAR_PENDING3`"]
pub type SGI_CLEAR_PENDING3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SGI_CLEAR_PENDING3`"]
pub struct SGI_CLEAR_PENDING3_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_CLEAR_PENDING3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SGI_CLEAR_PENDING0"]
    #[inline(always)]
    pub fn sgi_clear_pending0(&self) -> SGI_CLEAR_PENDING0_R {
        SGI_CLEAR_PENDING0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SGI_CLEAR_PENDING1"]
    #[inline(always)]
    pub fn sgi_clear_pending1(&self) -> SGI_CLEAR_PENDING1_R {
        SGI_CLEAR_PENDING1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - SGI_CLEAR_PENDING2"]
    #[inline(always)]
    pub fn sgi_clear_pending2(&self) -> SGI_CLEAR_PENDING2_R {
        SGI_CLEAR_PENDING2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - SGI_CLEAR_PENDING3"]
    #[inline(always)]
    pub fn sgi_clear_pending3(&self) -> SGI_CLEAR_PENDING3_R {
        SGI_CLEAR_PENDING3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SGI_CLEAR_PENDING0"]
    #[inline(always)]
    pub fn sgi_clear_pending0(&mut self) -> SGI_CLEAR_PENDING0_W {
        SGI_CLEAR_PENDING0_W { w: self }
    }
    #[doc = "Bits 8:9 - SGI_CLEAR_PENDING1"]
    #[inline(always)]
    pub fn sgi_clear_pending1(&mut self) -> SGI_CLEAR_PENDING1_W {
        SGI_CLEAR_PENDING1_W { w: self }
    }
    #[doc = "Bits 16:17 - SGI_CLEAR_PENDING2"]
    #[inline(always)]
    pub fn sgi_clear_pending2(&mut self) -> SGI_CLEAR_PENDING2_W {
        SGI_CLEAR_PENDING2_W { w: self }
    }
    #[doc = "Bits 24:25 - SGI_CLEAR_PENDING3"]
    #[inline(always)]
    pub fn sgi_clear_pending3(&mut self) -> SGI_CLEAR_PENDING3_W {
        SGI_CLEAR_PENDING3_W { w: self }
    }
}
