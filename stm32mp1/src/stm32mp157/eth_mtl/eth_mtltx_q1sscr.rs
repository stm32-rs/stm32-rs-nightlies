#[doc = "Reader of register ETH_MTLTxQ1SSCR"]
pub type R = crate::R<u32, super::ETH_MTLTXQ1SSCR>;
#[doc = "Writer for register ETH_MTLTxQ1SSCR"]
pub type W = crate::W<u32, super::ETH_MTLTXQ1SSCR>;
#[doc = "Register ETH_MTLTxQ1SSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MTLTXQ1SSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSC`"]
pub type SSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SSC`"]
pub struct SSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - SSC"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - SSC"]
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W {
        SSC_W { w: self }
    }
}
