#[doc = "Register `GRXSTSP_Host` reader"]
pub type R = crate::R<GRXSTSP_HOSTrs>;
#[doc = "Field `CHNUM` reader - Channel number"]
pub type CHNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PKTSTS_R = crate::FieldReader;
#[doc = "Field `STSPHST` reader - Status phase start"]
pub type STSPHST_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Channel number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Status phase start"]
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "OTG status read and pop register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_host::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXSTSP_HOSTrs;
impl crate::RegisterSpec for GRXSTSP_HOSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsp_host::R`](R) reader structure"]
impl crate::Readable for GRXSTSP_HOSTrs {}
#[doc = "`reset()` method sets GRXSTSP_Host to value 0x0200_0400"]
impl crate::Resettable for GRXSTSP_HOSTrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
