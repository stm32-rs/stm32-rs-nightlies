#[doc = "Reader of register IPCC_C2SCR"]
pub type R = crate::R<u32, super::IPCC_C2SCR>;
#[doc = "Writer for register IPCC_C2SCR"]
pub type W = crate::W<u32, super::IPCC_C2SCR>;
#[doc = "Register IPCC_C2SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCC_C2SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHxC`"]
pub type CHXC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHxC`"]
pub struct CHXC_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CHxS`"]
pub type CHXS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHxS`"]
pub struct CHXS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    pub fn chx_c(&self) -> CHXC_R {
        CHXC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    pub fn chx_s(&self) -> CHXS_R {
        CHXS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    pub fn chx_c(&mut self) -> CHXC_W {
        CHXC_W { w: self }
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    pub fn chx_s(&mut self) -> CHXS_W {
        CHXS_W { w: self }
    }
}
