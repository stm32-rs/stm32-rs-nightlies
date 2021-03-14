#[doc = "Reader of register JDATAR"]
pub type R = crate::R<u32, super::JDATAR>;
#[doc = "Reader of field `JDATACH`"]
pub type JDATACH_R = crate::R<u8, u8>;
#[doc = "Reader of field `JDATA`"]
pub type JDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 25:28 - Injected channel most recently converted"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - Injected group conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
