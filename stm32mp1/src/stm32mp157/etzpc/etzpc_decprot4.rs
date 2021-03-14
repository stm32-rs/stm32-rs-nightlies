#[doc = "Reader of register ETZPC_DECPROT4"]
pub type R = crate::R<u32, super::ETZPC_DECPROT4>;
#[doc = "Writer for register ETZPC_DECPROT4"]
pub type W = crate::W<u32, super::ETZPC_DECPROT4>;
#[doc = "Register ETZPC_DECPROT4 `reset()`'s with value 0"]
impl crate::ResetValue for super::ETZPC_DECPROT4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DECPROT0`"]
pub type DECPROT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT0`"]
pub struct DECPROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DECPROT1`"]
pub type DECPROT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT1`"]
pub struct DECPROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DECPROT2`"]
pub type DECPROT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT2`"]
pub struct DECPROT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DECPROT3`"]
pub type DECPROT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT3`"]
pub struct DECPROT3_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DECPROT4`"]
pub type DECPROT4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT4`"]
pub struct DECPROT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DECPROT5`"]
pub type DECPROT5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT5`"]
pub struct DECPROT5_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `DECPROT6`"]
pub type DECPROT6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT6`"]
pub struct DECPROT6_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DECPROT7`"]
pub type DECPROT7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT7`"]
pub struct DECPROT7_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `DECPROT8`"]
pub type DECPROT8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT8`"]
pub struct DECPROT8_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DECPROT9`"]
pub type DECPROT9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT9`"]
pub struct DECPROT9_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `DECPROT10`"]
pub type DECPROT10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT10`"]
pub struct DECPROT10_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `DECPROT11`"]
pub type DECPROT11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT11`"]
pub struct DECPROT11_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `DECPROT12`"]
pub type DECPROT12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT12`"]
pub struct DECPROT12_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DECPROT13`"]
pub type DECPROT13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT13`"]
pub struct DECPROT13_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `DECPROT14`"]
pub type DECPROT14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT14`"]
pub struct DECPROT14_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `DECPROT15`"]
pub type DECPROT15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECPROT15`"]
pub struct DECPROT15_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    pub fn decprot0(&self) -> DECPROT0_R {
        DECPROT0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    pub fn decprot1(&self) -> DECPROT1_R {
        DECPROT1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    pub fn decprot2(&self) -> DECPROT2_R {
        DECPROT2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    pub fn decprot3(&self) -> DECPROT3_R {
        DECPROT3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    pub fn decprot4(&self) -> DECPROT4_R {
        DECPROT4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    pub fn decprot5(&self) -> DECPROT5_R {
        DECPROT5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    pub fn decprot6(&self) -> DECPROT6_R {
        DECPROT6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    pub fn decprot7(&self) -> DECPROT7_R {
        DECPROT7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    pub fn decprot8(&self) -> DECPROT8_R {
        DECPROT8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    pub fn decprot9(&self) -> DECPROT9_R {
        DECPROT9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    pub fn decprot10(&self) -> DECPROT10_R {
        DECPROT10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    pub fn decprot11(&self) -> DECPROT11_R {
        DECPROT11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    pub fn decprot12(&self) -> DECPROT12_R {
        DECPROT12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    pub fn decprot13(&self) -> DECPROT13_R {
        DECPROT13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    pub fn decprot14(&self) -> DECPROT14_R {
        DECPROT14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    pub fn decprot15(&self) -> DECPROT15_R {
        DECPROT15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    pub fn decprot0(&mut self) -> DECPROT0_W {
        DECPROT0_W { w: self }
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    pub fn decprot1(&mut self) -> DECPROT1_W {
        DECPROT1_W { w: self }
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    pub fn decprot2(&mut self) -> DECPROT2_W {
        DECPROT2_W { w: self }
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    pub fn decprot3(&mut self) -> DECPROT3_W {
        DECPROT3_W { w: self }
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    pub fn decprot4(&mut self) -> DECPROT4_W {
        DECPROT4_W { w: self }
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    pub fn decprot5(&mut self) -> DECPROT5_W {
        DECPROT5_W { w: self }
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    pub fn decprot6(&mut self) -> DECPROT6_W {
        DECPROT6_W { w: self }
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    pub fn decprot7(&mut self) -> DECPROT7_W {
        DECPROT7_W { w: self }
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    pub fn decprot8(&mut self) -> DECPROT8_W {
        DECPROT8_W { w: self }
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    pub fn decprot9(&mut self) -> DECPROT9_W {
        DECPROT9_W { w: self }
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    pub fn decprot10(&mut self) -> DECPROT10_W {
        DECPROT10_W { w: self }
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    pub fn decprot11(&mut self) -> DECPROT11_W {
        DECPROT11_W { w: self }
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    pub fn decprot12(&mut self) -> DECPROT12_W {
        DECPROT12_W { w: self }
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    pub fn decprot13(&mut self) -> DECPROT13_W {
        DECPROT13_W { w: self }
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    pub fn decprot14(&mut self) -> DECPROT14_W {
        DECPROT14_W { w: self }
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    pub fn decprot15(&mut self) -> DECPROT15_W {
        DECPROT15_W { w: self }
    }
}
