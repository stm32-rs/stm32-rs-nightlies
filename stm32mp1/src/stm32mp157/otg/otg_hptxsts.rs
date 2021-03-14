#[doc = "Reader of register OTG_HPTXSTS"]
pub type R = crate::R<u32, super::OTG_HPTXSTS>;
#[doc = "Reader of field `PTXFSAVL`"]
pub type PTXFSAVL_R = crate::R<u16, u16>;
#[doc = "Reader of field `PTXQSAV`"]
pub type PTXQSAV_R = crate::R<u8, u8>;
#[doc = "Reader of field `PTXQTOP`"]
pub type PTXQTOP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - PTXFSAVL"]
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - PTXQSAV"]
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PTXQTOP"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
