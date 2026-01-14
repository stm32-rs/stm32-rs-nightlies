///Register `GRXSTSR_Host` reader
pub type R = crate::R<GRXSTSR_HOSTrs>;
///Field `CHNUM` reader - Endpoint number
pub type CHNUM_R = crate::FieldReader;
///Field `BCNT` reader - Byte count
pub type BCNT_R = crate::FieldReader<u16>;
///Field `DPID` reader - Data PID
pub type DPID_R = crate::FieldReader;
///Field `PKTSTS` reader - Packet status
pub type PKTSTS_R = crate::FieldReader;
///Field `STSPHST` reader - Status phase start
pub type STSPHST_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Endpoint number
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:14 - Byte count
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 15:16 - Data PID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:20 - Packet status
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bit 27 - Status phase start
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR_Host")
            .field("chnum", &self.chnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("stsphst", &self.stsphst())
            .finish()
    }
}
/**OTG_FS Receive status debug read(Host mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsr_host::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#OTG_FS_GLOBAL:GRXSTSR_Host)*/
pub struct GRXSTSR_HOSTrs;
impl crate::RegisterSpec for GRXSTSR_HOSTrs {
    type Ux = u32;
}
///`read()` method returns [`grxstsr_host::R`](R) reader structure
impl crate::Readable for GRXSTSR_HOSTrs {}
///`reset()` method sets GRXSTSR_Host to value 0
impl crate::Resettable for GRXSTSR_HOSTrs {}
