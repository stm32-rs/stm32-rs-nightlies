#[doc = "Reader of register HWCFGR1"]
pub type R = crate::R<u32, super::HWCFGR1>;
#[doc = "Writer for register HWCFGR1"]
pub type W = crate::W<u32, super::HWCFGR1>;
#[doc = "Register HWCFGR1 `reset()`'s with value 0x0302_0100"]
impl crate::ResetValue for super::HWCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0302_0100
    }
}
#[doc = "Reader of field `CHMAP3`"]
pub type CHMAP3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP3`"]
pub struct CHMAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CHMAP2`"]
pub type CHMAP2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP2`"]
pub struct CHMAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHMAP1`"]
pub type CHMAP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP1`"]
pub struct CHMAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CHMAP0`"]
pub type CHMAP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP0`"]
pub struct CHMAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap3(&self) -> CHMAP3_R {
        CHMAP3_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap2(&self) -> CHMAP2_R {
        CHMAP2_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap1(&self) -> CHMAP1_R {
        CHMAP1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap0(&self) -> CHMAP0_R {
        CHMAP0_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap3(&mut self) -> CHMAP3_W {
        CHMAP3_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap2(&mut self) -> CHMAP2_W {
        CHMAP2_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap1(&mut self) -> CHMAP1_W {
        CHMAP1_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap0(&mut self) -> CHMAP0_W {
        CHMAP0_W { w: self }
    }
}
