#[doc = "Register `OTG_GRXSTSP` reader"]
pub type R = crate::R<OTG_GRXSTSPrs>;
#[doc = "Field `EPNUM` reader - EPNUM"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - BCNT"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - DPID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - PKTSTS"]
pub type PKTSTS_R = crate::FieldReader;
#[doc = "Field `FRMNUM` reader - FRMNUM"]
pub type FRMNUM_R = crate::FieldReader;
#[doc = "Field `STSPHST` reader - STSPHST"]
pub type STSPHST_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - EPNUM"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - BCNT"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - PKTSTS"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - FRMNUM"]
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - STSPHST"]
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_grxstsp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GRXSTSPrs;
impl crate::RegisterSpec for OTG_GRXSTSPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_grxstsp::R`](R) reader structure"]
impl crate::Readable for OTG_GRXSTSPrs {}
#[doc = "`reset()` method sets OTG_GRXSTSP to value 0"]
impl crate::Resettable for OTG_GRXSTSPrs {
    const RESET_VALUE: u32 = 0;
}
