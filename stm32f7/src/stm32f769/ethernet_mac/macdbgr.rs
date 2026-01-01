///Register `MACDBGR` reader
pub type R = crate::R<MACDBGRrs>;
///Field `MMRPEA` reader - MAC MII receive protocol engine active
pub type MMRPEA_R = crate::BitReader;
///Field `MSFRWCS` reader - MAC small FIFO read/write controllers status
pub type MSFRWCS_R = crate::FieldReader;
///Field `RFWRA` reader - Rx FIFO write controller active
pub type RFWRA_R = crate::BitReader;
///Field `RFRCS` reader - Rx FIFO read controller status
pub type RFRCS_R = crate::FieldReader;
///Field `RFFL` reader - Rx FIFO fill level
pub type RFFL_R = crate::FieldReader;
///Field `MMTEA` reader - MAC MII transmit engine active
pub type MMTEA_R = crate::BitReader;
///Field `MTFCS` reader - MAC transmit frame controller status
pub type MTFCS_R = crate::FieldReader;
///Field `MTP` reader - MAC transmitter in pause
pub type MTP_R = crate::BitReader;
///Field `TFRS` reader - Tx FIFO read status
pub type TFRS_R = crate::FieldReader;
///Field `TFWA` reader - Tx FIFO write active
pub type TFWA_R = crate::BitReader;
///Field `TFNE` reader - Tx FIFO not empty
pub type TFNE_R = crate::BitReader;
///Field `TFF` reader - Tx FIFO full
pub type TFF_R = crate::BitReader;
impl R {
    ///Bit 0 - MAC MII receive protocol engine active
    #[inline(always)]
    pub fn mmrpea(&self) -> MMRPEA_R {
        MMRPEA_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - MAC small FIFO read/write controllers status
    #[inline(always)]
    pub fn msfrwcs(&self) -> MSFRWCS_R {
        MSFRWCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 4 - Rx FIFO write controller active
    #[inline(always)]
    pub fn rfwra(&self) -> RFWRA_R {
        RFWRA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Rx FIFO read controller status
    #[inline(always)]
    pub fn rfrcs(&self) -> RFRCS_R {
        RFRCS_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bits 8:9 - Rx FIFO fill level
    #[inline(always)]
    pub fn rffl(&self) -> RFFL_R {
        RFFL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 16 - MAC MII transmit engine active
    #[inline(always)]
    pub fn mmtea(&self) -> MMTEA_R {
        MMTEA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - MAC transmit frame controller status
    #[inline(always)]
    pub fn mtfcs(&self) -> MTFCS_R {
        MTFCS_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - MAC transmitter in pause
    #[inline(always)]
    pub fn mtp(&self) -> MTP_R {
        MTP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Tx FIFO read status
    #[inline(always)]
    pub fn tfrs(&self) -> TFRS_R {
        TFRS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Tx FIFO write active
    #[inline(always)]
    pub fn tfwa(&self) -> TFWA_R {
        TFWA_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Tx FIFO not empty
    #[inline(always)]
    pub fn tfne(&self) -> TFNE_R {
        TFNE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Tx FIFO full
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACDBGR")
            .field("tff", &self.tff())
            .field("tfne", &self.tfne())
            .field("tfwa", &self.tfwa())
            .field("tfrs", &self.tfrs())
            .field("mtp", &self.mtp())
            .field("mtfcs", &self.mtfcs())
            .field("mmtea", &self.mmtea())
            .field("rffl", &self.rffl())
            .field("rfrcs", &self.rfrcs())
            .field("rfwra", &self.rfwra())
            .field("msfrwcs", &self.msfrwcs())
            .field("mmrpea", &self.mmrpea())
            .finish()
    }
}
/**Ethernet MAC debug register

You can [`read`](crate::Reg::read) this register and get [`macdbgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#Ethernet_MAC:MACDBGR)*/
pub struct MACDBGRrs;
impl crate::RegisterSpec for MACDBGRrs {
    type Ux = u32;
}
///`read()` method returns [`macdbgr::R`](R) reader structure
impl crate::Readable for MACDBGRrs {}
///`reset()` method sets MACDBGR to value 0
impl crate::Resettable for MACDBGRrs {}
