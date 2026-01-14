///Register `MACHWF1R` reader
pub type R = crate::R<MACHWF1Rrs>;
///Field `RXFIFOSIZE` reader - MTL Receive FIFO Size
pub type RXFIFOSIZE_R = crate::FieldReader;
///Field `TXFIFOSIZE` reader - MTL Transmit FIFO Size
pub type TXFIFOSIZE_R = crate::FieldReader;
///Field `OSTEN` reader - One-Step Timestamping Enable
pub type OSTEN_R = crate::BitReader;
///Field `PTOEN` reader - PTP Offload Enable
pub type PTOEN_R = crate::BitReader;
///Field `ADVTHWORD` reader - IEEE 1588 High Word Register Enable
pub type ADVTHWORD_R = crate::BitReader;
///Field `DCBEN` reader - DCB Feature Enable
pub type DCBEN_R = crate::BitReader;
///Field `SPHEN` reader - Split Header Feature Enable
pub type SPHEN_R = crate::BitReader;
///Field `TSOEN` reader - TCP Segmentation Offload Enable
pub type TSOEN_R = crate::BitReader;
///Field `DBGMEMA` reader - DMA Debug Registers Enable
pub type DBGMEMA_R = crate::BitReader;
///Field `AVSEL` reader - AV Feature Enable
pub type AVSEL_R = crate::BitReader;
///Field `HASHTBLSZ` reader - Hash Table Size
pub type HASHTBLSZ_R = crate::FieldReader;
///Field `L3L4FNUM` reader - Total number of L3 or L4 Filters
pub type L3L4FNUM_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - MTL Receive FIFO Size
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - MTL Transmit FIFO Size
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 11 - One-Step Timestamping Enable
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PTP Offload Enable
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IEEE 1588 High Word Register Enable
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - DCB Feature Enable
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Split Header Feature Enable
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TCP Segmentation Offload Enable
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DMA Debug Registers Enable
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AV Feature Enable
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 24:25 - Hash Table Size
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 27:30 - Total number of L3 or L4 Filters
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
            .field("dcben", &self.dcben())
            .field("sphen", &self.sphen())
            .field("tsoen", &self.tsoen())
            .field("dbgmema", &self.dbgmema())
            .field("avsel", &self.avsel())
            .field("hashtblsz", &self.hashtblsz())
            .field("l3l4fnum", &self.l3l4fnum())
            .finish()
    }
}
/**HW feature 1 register

You can [`read`](crate::Reg::read) this register and get [`machwf1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#Ethernet_MAC:MACHWF1R)*/
pub struct MACHWF1Rrs;
impl crate::RegisterSpec for MACHWF1Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf1r::R`](R) reader structure
impl crate::Readable for MACHWF1Rrs {}
///`reset()` method sets MACHWF1R to value 0x1184_1904
impl crate::Resettable for MACHWF1Rrs {
    const RESET_VALUE: u32 = 0x1184_1904;
}
