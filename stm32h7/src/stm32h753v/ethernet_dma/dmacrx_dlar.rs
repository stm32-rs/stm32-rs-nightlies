#[doc = "Reader of register DMACRxDLAR"]
pub type R = crate::R<u32, super::DMACRXDLAR>;
#[doc = "Writer for register DMACRxDLAR"]
pub type W = crate::W<u32, super::DMACRXDLAR>;
#[doc = "Register DMACRxDLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACRXDLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDESLA`"]
pub type RDESLA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDESLA`"]
pub struct RDESLA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDESLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&mut self) -> RDESLA_W {
        RDESLA_W { w: self }
    }
}
