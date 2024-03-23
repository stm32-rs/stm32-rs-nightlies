#[doc = "Register `OTG_HPTXSTS` reader"]
pub type R = crate::R<OTG_HPTXSTSrs>;
#[doc = "Field `PTXFSAVL` reader - PTXFSAVL"]
pub type PTXFSAVL_R = crate::FieldReader<u16>;
#[doc = "Field `PTXQSAV` reader - PTXQSAV"]
pub type PTXQSAV_R = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - PTXQTOP"]
pub type PTXQTOP_R = crate::FieldReader;
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
#[doc = "This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hptxsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HPTXSTSrs;
impl crate::RegisterSpec for OTG_HPTXSTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hptxsts::R`](R) reader structure"]
impl crate::Readable for OTG_HPTXSTSrs {}
#[doc = "`reset()` method sets OTG_HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for OTG_HPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0100;
}
