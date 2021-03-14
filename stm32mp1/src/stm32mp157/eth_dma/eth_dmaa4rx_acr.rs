#[doc = "Reader of register ETH_DMAA4RxACR"]
pub type R = crate::R<u32, super::ETH_DMAA4RXACR>;
#[doc = "Writer for register ETH_DMAA4RxACR"]
pub type W = crate::W<u32, super::ETH_DMAA4RXACR>;
#[doc = "Register ETH_DMAA4RxACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAA4RXACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDWC`"]
pub type RDWC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDWC`"]
pub struct RDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RPC`"]
pub type RPC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC`"]
pub struct RPC_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RHC`"]
pub type RHC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RHC`"]
pub struct RHC_W<'a> {
    w: &'a mut W,
}
impl<'a> RHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RDC`"]
pub type RDC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDC`"]
pub struct RDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    pub fn rdwc(&self) -> RDWC_R {
        RDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    pub fn rpc(&self) -> RPC_R {
        RPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    pub fn rdwc(&mut self) -> RDWC_W {
        RDWC_W { w: self }
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    pub fn rpc(&mut self) -> RPC_W {
        RPC_W { w: self }
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    pub fn rhc(&mut self) -> RHC_W {
        RHC_W { w: self }
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    pub fn rdc(&mut self) -> RDC_W {
        RDC_W { w: self }
    }
}
