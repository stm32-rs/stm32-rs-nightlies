#[doc = "Reader of register OTG_HNPTXFSIZ"]
pub type R = crate::R<u32, super::OTG_HNPTXFSIZ>;
#[doc = "Writer for register OTG_HNPTXFSIZ"]
pub type W = crate::W<u32, super::OTG_HNPTXFSIZ>;
#[doc = "Register OTG_HNPTXFSIZ `reset()`'s with value 0x0200_0200"]
impl crate::ResetValue for super::OTG_HNPTXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0200
    }
}
#[doc = "Reader of field `NPTXFSA`"]
pub type NPTXFSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NPTXFSA`"]
pub struct NPTXFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `NPTXFD`"]
pub type NPTXFD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NPTXFD`"]
pub struct NPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - NPTXFSA"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - NPTXFD"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NPTXFSA"]
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W {
        NPTXFSA_W { w: self }
    }
    #[doc = "Bits 16:31 - NPTXFD"]
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NPTXFD_W {
        NPTXFD_W { w: self }
    }
}
