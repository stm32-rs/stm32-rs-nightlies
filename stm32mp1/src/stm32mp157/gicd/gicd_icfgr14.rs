#[doc = "Reader of register GICD_ICFGR14"]
pub type R = crate::R<u32, super::GICD_ICFGR14>;
#[doc = "Writer for register GICD_ICFGR14"]
pub type W = crate::W<u32, super::GICD_ICFGR14>;
#[doc = "Register GICD_ICFGR14 `reset()`'s with value 0x5555_5555"]
impl crate::ResetValue for super::GICD_ICFGR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5555_5555
    }
}
#[doc = "Reader of field `INT_CONFIG0`"]
pub type INT_CONFIG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG0`"]
pub struct INT_CONFIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG1`"]
pub type INT_CONFIG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG1`"]
pub struct INT_CONFIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG2`"]
pub type INT_CONFIG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG2`"]
pub struct INT_CONFIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG3`"]
pub type INT_CONFIG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG3`"]
pub struct INT_CONFIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG4`"]
pub type INT_CONFIG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG4`"]
pub struct INT_CONFIG4_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG5`"]
pub type INT_CONFIG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG5`"]
pub struct INT_CONFIG5_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG6`"]
pub type INT_CONFIG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG6`"]
pub struct INT_CONFIG6_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG7`"]
pub type INT_CONFIG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG7`"]
pub struct INT_CONFIG7_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG8`"]
pub type INT_CONFIG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG8`"]
pub struct INT_CONFIG8_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG9`"]
pub type INT_CONFIG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG9`"]
pub struct INT_CONFIG9_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG10`"]
pub type INT_CONFIG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG10`"]
pub struct INT_CONFIG10_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG11`"]
pub type INT_CONFIG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG11`"]
pub struct INT_CONFIG11_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG12`"]
pub type INT_CONFIG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG12`"]
pub struct INT_CONFIG12_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG13`"]
pub type INT_CONFIG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG13`"]
pub struct INT_CONFIG13_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG14`"]
pub type INT_CONFIG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG14`"]
pub struct INT_CONFIG14_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `INT_CONFIG15`"]
pub type INT_CONFIG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_CONFIG15`"]
pub struct INT_CONFIG15_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - INT_CONFIG0"]
    #[inline(always)]
    pub fn int_config0(&self) -> INT_CONFIG0_R {
        INT_CONFIG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - INT_CONFIG1"]
    #[inline(always)]
    pub fn int_config1(&self) -> INT_CONFIG1_R {
        INT_CONFIG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - INT_CONFIG2"]
    #[inline(always)]
    pub fn int_config2(&self) -> INT_CONFIG2_R {
        INT_CONFIG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - INT_CONFIG3"]
    #[inline(always)]
    pub fn int_config3(&self) -> INT_CONFIG3_R {
        INT_CONFIG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - INT_CONFIG4"]
    #[inline(always)]
    pub fn int_config4(&self) -> INT_CONFIG4_R {
        INT_CONFIG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - INT_CONFIG5"]
    #[inline(always)]
    pub fn int_config5(&self) -> INT_CONFIG5_R {
        INT_CONFIG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - INT_CONFIG6"]
    #[inline(always)]
    pub fn int_config6(&self) -> INT_CONFIG6_R {
        INT_CONFIG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - INT_CONFIG7"]
    #[inline(always)]
    pub fn int_config7(&self) -> INT_CONFIG7_R {
        INT_CONFIG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - INT_CONFIG8"]
    #[inline(always)]
    pub fn int_config8(&self) -> INT_CONFIG8_R {
        INT_CONFIG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - INT_CONFIG9"]
    #[inline(always)]
    pub fn int_config9(&self) -> INT_CONFIG9_R {
        INT_CONFIG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - INT_CONFIG10"]
    #[inline(always)]
    pub fn int_config10(&self) -> INT_CONFIG10_R {
        INT_CONFIG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - INT_CONFIG11"]
    #[inline(always)]
    pub fn int_config11(&self) -> INT_CONFIG11_R {
        INT_CONFIG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - INT_CONFIG12"]
    #[inline(always)]
    pub fn int_config12(&self) -> INT_CONFIG12_R {
        INT_CONFIG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - INT_CONFIG13"]
    #[inline(always)]
    pub fn int_config13(&self) -> INT_CONFIG13_R {
        INT_CONFIG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - INT_CONFIG14"]
    #[inline(always)]
    pub fn int_config14(&self) -> INT_CONFIG14_R {
        INT_CONFIG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - INT_CONFIG15"]
    #[inline(always)]
    pub fn int_config15(&self) -> INT_CONFIG15_R {
        INT_CONFIG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - INT_CONFIG0"]
    #[inline(always)]
    pub fn int_config0(&mut self) -> INT_CONFIG0_W {
        INT_CONFIG0_W { w: self }
    }
    #[doc = "Bits 2:3 - INT_CONFIG1"]
    #[inline(always)]
    pub fn int_config1(&mut self) -> INT_CONFIG1_W {
        INT_CONFIG1_W { w: self }
    }
    #[doc = "Bits 4:5 - INT_CONFIG2"]
    #[inline(always)]
    pub fn int_config2(&mut self) -> INT_CONFIG2_W {
        INT_CONFIG2_W { w: self }
    }
    #[doc = "Bits 6:7 - INT_CONFIG3"]
    #[inline(always)]
    pub fn int_config3(&mut self) -> INT_CONFIG3_W {
        INT_CONFIG3_W { w: self }
    }
    #[doc = "Bits 8:9 - INT_CONFIG4"]
    #[inline(always)]
    pub fn int_config4(&mut self) -> INT_CONFIG4_W {
        INT_CONFIG4_W { w: self }
    }
    #[doc = "Bits 10:11 - INT_CONFIG5"]
    #[inline(always)]
    pub fn int_config5(&mut self) -> INT_CONFIG5_W {
        INT_CONFIG5_W { w: self }
    }
    #[doc = "Bits 12:13 - INT_CONFIG6"]
    #[inline(always)]
    pub fn int_config6(&mut self) -> INT_CONFIG6_W {
        INT_CONFIG6_W { w: self }
    }
    #[doc = "Bits 14:15 - INT_CONFIG7"]
    #[inline(always)]
    pub fn int_config7(&mut self) -> INT_CONFIG7_W {
        INT_CONFIG7_W { w: self }
    }
    #[doc = "Bits 16:17 - INT_CONFIG8"]
    #[inline(always)]
    pub fn int_config8(&mut self) -> INT_CONFIG8_W {
        INT_CONFIG8_W { w: self }
    }
    #[doc = "Bits 18:19 - INT_CONFIG9"]
    #[inline(always)]
    pub fn int_config9(&mut self) -> INT_CONFIG9_W {
        INT_CONFIG9_W { w: self }
    }
    #[doc = "Bits 20:21 - INT_CONFIG10"]
    #[inline(always)]
    pub fn int_config10(&mut self) -> INT_CONFIG10_W {
        INT_CONFIG10_W { w: self }
    }
    #[doc = "Bits 22:23 - INT_CONFIG11"]
    #[inline(always)]
    pub fn int_config11(&mut self) -> INT_CONFIG11_W {
        INT_CONFIG11_W { w: self }
    }
    #[doc = "Bits 24:25 - INT_CONFIG12"]
    #[inline(always)]
    pub fn int_config12(&mut self) -> INT_CONFIG12_W {
        INT_CONFIG12_W { w: self }
    }
    #[doc = "Bits 26:27 - INT_CONFIG13"]
    #[inline(always)]
    pub fn int_config13(&mut self) -> INT_CONFIG13_W {
        INT_CONFIG13_W { w: self }
    }
    #[doc = "Bits 28:29 - INT_CONFIG14"]
    #[inline(always)]
    pub fn int_config14(&mut self) -> INT_CONFIG14_W {
        INT_CONFIG14_W { w: self }
    }
    #[doc = "Bits 30:31 - INT_CONFIG15"]
    #[inline(always)]
    pub fn int_config15(&mut self) -> INT_CONFIG15_W {
        INT_CONFIG15_W { w: self }
    }
}
