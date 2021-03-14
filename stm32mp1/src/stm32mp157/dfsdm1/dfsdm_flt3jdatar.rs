#[doc = "Reader of register DFSDM_FLT3JDATAR"]
pub type R = crate::R<u32, super::DFSDM_FLT3JDATAR>;
#[doc = "Reader of field `JDATACH`"]
pub type JDATACH_R = crate::R<u8, u8>;
#[doc = "Reader of field `JDATA`"]
pub type JDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - JDATACH"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - JDATA"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
