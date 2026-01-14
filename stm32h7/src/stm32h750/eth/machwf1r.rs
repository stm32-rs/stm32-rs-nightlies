///Register `MACHWF1R` reader
pub type R = crate::R<MACHWF1Rrs>;
///Field `RXFIFOSIZE` reader - MTL Receive FIFO Size This field contains the configured value of MTL Rx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(RXFIFO_SIZE) -7:
pub type RXFIFOSIZE_R = crate::FieldReader;
///Field `TXFIFOSIZE` reader - MTL Transmit FIFO Size This field contains the configured value of MTL Tx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(TXFIFO_SIZE) -7:
pub type TXFIFOSIZE_R = crate::FieldReader;
///Field `OSTEN` reader - One-Step Timestamping Enable This bit is set to 1 when the Enable One-Step Timestamp Feature is selected.
pub type OSTEN_R = crate::BitReader;
///Field `PTOEN` reader - PTP Offload Enable This bit is set to 1 when the Enable PTP Timestamp Offload Feature is selected.
pub type PTOEN_R = crate::BitReader;
///Field `ADVTHWORD` reader - IEEE 1588 High Word Register Enable This bit is set to 1 when the Add IEEE 1588 Higher Word Register option is selected
pub type ADVTHWORD_R = crate::BitReader;
///Field `ADDR64` reader - Address width This field indicates the configured address width. Others: Reserved, must not be used
pub type ADDR64_R = crate::FieldReader;
///Field `DCBEN` reader - DCB Feature Enable This bit is set to 1 when the Enable Data Center Bridging option is selected
pub type DCBEN_R = crate::BitReader;
///Field `SPHEN` reader - Split Header Feature Enable This bit is set to 1 when the Enable Split Header Structure option is selected
pub type SPHEN_R = crate::BitReader;
///Field `TSOEN` reader - TCP Segmentation Offload Enable This bit is set to 1 when the Enable TCP Segmentation Offloading for TCP/IP Packets option is selected
pub type TSOEN_R = crate::BitReader;
///Field `DBGMEMA` reader - DMA Debug Registers Enable This bit is set to 1 when the Debug Mode Enable option is selected
pub type DBGMEMA_R = crate::BitReader;
///Field `AVSEL` reader - AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option is selected.
pub type AVSEL_R = crate::BitReader;
///Field `RAVSEL` reader - Rx Side Only AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option on Rx Side Only is selected.
pub type RAVSEL_R = crate::BitReader;
///Field `POUOST` reader - One Step for PTP over UDP/IP Feature Enable This bit is set to 1 when the Enable one step timestamp for PTP over UDP/IP feature is selected.
pub type POUOST_R = crate::BitReader;
///Field `HASHTBLSZ` reader - Hash Table Size This field indicates the size of the Hash table:
pub type HASHTBLSZ_R = crate::FieldReader;
///Field `L3L4FNUM` reader - Total number of L3 or L4 Filters This field indicates the total number of L3 or L4 filters: ..
pub type L3L4FNUM_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - MTL Receive FIFO Size This field contains the configured value of MTL Rx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(RXFIFO_SIZE) -7:
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - MTL Transmit FIFO Size This field contains the configured value of MTL Tx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(TXFIFO_SIZE) -7:
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 11 - One-Step Timestamping Enable This bit is set to 1 when the Enable One-Step Timestamp Feature is selected.
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PTP Offload Enable This bit is set to 1 when the Enable PTP Timestamp Offload Feature is selected.
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IEEE 1588 High Word Register Enable This bit is set to 1 when the Add IEEE 1588 Higher Word Register option is selected
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Address width This field indicates the configured address width. Others: Reserved, must not be used
    #[inline(always)]
    pub fn addr64(&self) -> ADDR64_R {
        ADDR64_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - DCB Feature Enable This bit is set to 1 when the Enable Data Center Bridging option is selected
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Split Header Feature Enable This bit is set to 1 when the Enable Split Header Structure option is selected
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TCP Segmentation Offload Enable This bit is set to 1 when the Enable TCP Segmentation Offloading for TCP/IP Packets option is selected
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DMA Debug Registers Enable This bit is set to 1 when the Debug Mode Enable option is selected
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option is selected.
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rx Side Only AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option on Rx Side Only is selected.
    #[inline(always)]
    pub fn ravsel(&self) -> RAVSEL_R {
        RAVSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - One Step for PTP over UDP/IP Feature Enable This bit is set to 1 when the Enable one step timestamp for PTP over UDP/IP feature is selected.
    #[inline(always)]
    pub fn pouost(&self) -> POUOST_R {
        POUOST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - Hash Table Size This field indicates the size of the Hash table:
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 27:30 - Total number of L3 or L4 Filters This field indicates the total number of L3 or L4 filters: ..
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3L4FNUM_R {
        L3L4FNUM_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHWF1R")
            .field("rxfifosize", &self.rxfifosize())
            .field("txfifosize", &self.txfifosize())
            .field("osten", &self.osten())
            .field("ptoen", &self.ptoen())
            .field("advthword", &self.advthword())
            .field("addr64", &self.addr64())
            .field("dcben", &self.dcben())
            .field("sphen", &self.sphen())
            .field("tsoen", &self.tsoen())
            .field("dbgmema", &self.dbgmema())
            .field("avsel", &self.avsel())
            .field("ravsel", &self.ravsel())
            .field("pouost", &self.pouost())
            .field("hashtblsz", &self.hashtblsz())
            .field("l3l4fnum", &self.l3l4fnum())
            .finish()
    }
}
/**HW feature 1 register

You can [`read`](crate::Reg::read) this register and get [`machwf1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MACHWF1R)*/
pub struct MACHWF1Rrs;
impl crate::RegisterSpec for MACHWF1Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf1r::R`](R) reader structure
impl crate::Readable for MACHWF1Rrs {}
///`reset()` method sets MACHWF1R to value 0x1104_1904
impl crate::Resettable for MACHWF1Rrs {
    const RESET_VALUE: u32 = 0x1104_1904;
}
