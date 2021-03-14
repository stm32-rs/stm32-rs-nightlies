#[doc = "Reader of register JDATA13R"]
pub type R = crate::R<u32, super::JDATA13R>;
#[doc = "Reader of field `JDATA3`"]
pub type JDATA3_R = crate::R<u16, u16>;
#[doc = "Reader of field `JDATA1`"]
pub type JDATA1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Injected group conversion data for SDADC3"]
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Injected group conversion data for SDADC1"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
