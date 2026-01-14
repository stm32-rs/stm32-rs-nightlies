///Register `MACHWF1R` reader
pub type R = crate::R<MACHWF1Rrs>;
///Field `RXFIFOSIZE` reader - RXFIFOSIZE
pub type RXFIFOSIZE_R = crate::FieldReader;
///Field `TXFIFOSIZE` reader - TXFIFOSIZE
pub type TXFIFOSIZE_R = crate::FieldReader;
///Field `OSTEN` reader - OSTEN
pub type OSTEN_R = crate::BitReader;
///Field `PTOEN` reader - PTOEN
pub type PTOEN_R = crate::BitReader;
///Field `ADVTHWORD` reader - ADVTHWORD
pub type ADVTHWORD_R = crate::BitReader;
///Field `ADDR64` reader - ADDR64
pub type ADDR64_R = crate::FieldReader;
///Field `DCBEN` reader - DCBEN
pub type DCBEN_R = crate::BitReader;
///Field `SPHEN` reader - SPHEN
pub type SPHEN_R = crate::BitReader;
///Field `TSOEN` reader - TSOEN
pub type TSOEN_R = crate::BitReader;
///Field `DBGMEMA` reader - DBGMEMA
pub type DBGMEMA_R = crate::BitReader;
///Field `AVSEL` reader - AVSEL
pub type AVSEL_R = crate::BitReader;
///Field `HASHTBLSZ` reader - HASHTBLSZ
pub type HASHTBLSZ_R = crate::FieldReader;
///Field `L3L4FNUM` reader - L3L4FNUM
pub type L3L4FNUM_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - RXFIFOSIZE
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - TXFIFOSIZE
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 11 - OSTEN
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PTOEN
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADVTHWORD
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - ADDR64
    #[inline(always)]
    pub fn addr64(&self) -> ADDR64_R {
        ADDR64_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - DCBEN
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SPHEN
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TSOEN
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DBGMEMA
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AVSEL
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 24:25 - HASHTBLSZ
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 27:30 - L3L4FNUM
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
            .field("hashtblsz", &self.hashtblsz())
            .field("l3l4fnum", &self.l3l4fnum())
            .finish()
    }
}
/**This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.

You can [`read`](crate::Reg::read) this register and get [`machwf1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACHWF1R)*/
pub struct MACHWF1Rrs;
impl crate::RegisterSpec for MACHWF1Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf1r::R`](R) reader structure
impl crate::Readable for MACHWF1Rrs {}
///`reset()` method sets MACHWF1R to value 0x1114_1945
impl crate::Resettable for MACHWF1Rrs {
    const RESET_VALUE: u32 = 0x1114_1945;
}
