#[doc = "Register `HNPTXSTS` reader"]
pub type R = crate::R<HNPTXSTSrs>;
#[doc = "Field `NPTXFSAV` reader - Nonperiodic TxFIFO space available"]
pub type NPTXFSAV_R = crate::FieldReader<u16>;
#[doc = "Field `NPTQXSAV` reader - Nonperiodic transmit request queue space available"]
pub type NPTQXSAV_R = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the nonperiodic transmit request queue"]
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Nonperiodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the nonperiodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HNPTXSTSrs;
impl crate::RegisterSpec for HNPTXSTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hnptxsts::R`](R) reader structure"]
impl crate::Readable for HNPTXSTSrs {}
#[doc = "`reset()` method sets HNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for HNPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0200;
}
