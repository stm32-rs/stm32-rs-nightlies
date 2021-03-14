#[doc = "Reader of register DDRCTRL_DRAMTMG5"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG5>;
#[doc = "Writer for register DDRCTRL_DRAMTMG5"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG5>;
#[doc = "Register DDRCTRL_DRAMTMG5 `reset()`'s with value 0x0505_0403"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0505_0403
    }
}
#[doc = "Reader of field `T_CKE`"]
pub type T_CKE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKE`"]
pub struct T_CKE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `T_CKESR`"]
pub type T_CKESR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKESR`"]
pub struct T_CKESR_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKESR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `T_CKSRE`"]
pub type T_CKSRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKSRE`"]
pub struct T_CKSRE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKSRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `T_CKSRX`"]
pub type T_CKSRX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_CKSRX`"]
pub struct T_CKSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKSRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - T_CKE"]
    #[inline(always)]
    pub fn t_cke(&self) -> T_CKE_R {
        T_CKE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - T_CKESR"]
    #[inline(always)]
    pub fn t_ckesr(&self) -> T_CKESR_R {
        T_CKESR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - T_CKSRE"]
    #[inline(always)]
    pub fn t_cksre(&self) -> T_CKSRE_R {
        T_CKSRE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - T_CKSRX"]
    #[inline(always)]
    pub fn t_cksrx(&self) -> T_CKSRX_R {
        T_CKSRX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - T_CKE"]
    #[inline(always)]
    pub fn t_cke(&mut self) -> T_CKE_W {
        T_CKE_W { w: self }
    }
    #[doc = "Bits 8:13 - T_CKESR"]
    #[inline(always)]
    pub fn t_ckesr(&mut self) -> T_CKESR_W {
        T_CKESR_W { w: self }
    }
    #[doc = "Bits 16:19 - T_CKSRE"]
    #[inline(always)]
    pub fn t_cksre(&mut self) -> T_CKSRE_W {
        T_CKSRE_W { w: self }
    }
    #[doc = "Bits 24:27 - T_CKSRX"]
    #[inline(always)]
    pub fn t_cksrx(&mut self) -> T_CKSRX_W {
        T_CKSRX_W { w: self }
    }
}
