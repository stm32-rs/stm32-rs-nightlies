#[doc = "Reader of register FDCAN_TTMLM"]
pub type R = crate::R<u32, super::FDCAN_TTMLM>;
#[doc = "Writer for register FDCAN_TTMLM"]
pub type W = crate::W<u32, super::FDCAN_TTMLM>;
#[doc = "Register FDCAN_TTMLM `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TTMLM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCM`"]
pub type CCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCM`"]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CSS`"]
pub type CSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSS`"]
pub struct CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TXEW`"]
pub type TXEW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXEW`"]
pub struct TXEW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ENTT`"]
pub type ENTT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENTT`"]
pub struct ENTT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CCM"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - CSS"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - TXEW"]
    #[inline(always)]
    pub fn txew(&self) -> TXEW_R {
        TXEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - ENTT"]
    #[inline(always)]
    pub fn entt(&self) -> ENTT_R {
        ENTT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCM"]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bits 6:7 - CSS"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W {
        CSS_W { w: self }
    }
    #[doc = "Bits 8:11 - TXEW"]
    #[inline(always)]
    pub fn txew(&mut self) -> TXEW_W {
        TXEW_W { w: self }
    }
    #[doc = "Bits 16:27 - ENTT"]
    #[inline(always)]
    pub fn entt(&mut self) -> ENTT_W {
        ENTT_W { w: self }
    }
}
