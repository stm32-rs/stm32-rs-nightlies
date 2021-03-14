#[doc = "Reader of register HUFFENC_AC0_60"]
pub type R = crate::R<u32, super::HUFFENC_AC0_60>;
#[doc = "Writer for register HUFFENC_AC0_60"]
pub type W = crate::W<u32, super::HUFFENC_AC0_60>;
#[doc = "Register HUFFENC_AC0_60 `reset()`'s with value 0"]
impl crate::ResetValue for super::HUFFENC_AC0_60 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DHTMem_RAM`"]
pub type DHTMEM_RAM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DHTMem_RAM`"]
pub struct DHTMEM_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DHTMEM_RAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    pub fn dhtmem_ram(&self) -> DHTMEM_RAM_R {
        DHTMEM_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W {
        DHTMEM_RAM_W { w: self }
    }
}
