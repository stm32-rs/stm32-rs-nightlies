#[doc = "Reader of register JDATA12R"]
pub type R = crate::R<u32, super::JDATA12R>;
#[doc = "Reader of field `JDATA2`"]
pub type JDATA2_R = crate::R<u16, u16>;
#[doc = "Reader of field `JDATA1`"]
pub type JDATA1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Injected group conversion data for SDADC2"]
    #[inline(always)]
    pub fn jdata2(&self) -> JDATA2_R {
        JDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Injected group conversion data for SDADC1"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
