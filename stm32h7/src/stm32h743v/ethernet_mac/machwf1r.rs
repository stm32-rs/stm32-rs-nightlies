#[doc = "Register `MACHWF1R` reader"]
pub type R = crate::R<MACHWF1Rrs>;
#[doc = "Field `RXFIFOSIZE` reader - MTL Receive FIFO Size"]
pub type RXFIFOSIZE_R = crate::FieldReader;
#[doc = "Field `TXFIFOSIZE` reader - MTL Transmit FIFO Size"]
pub type TXFIFOSIZE_R = crate::FieldReader;
#[doc = "Field `OSTEN` reader - One-Step Timestamping Enable"]
pub type OSTEN_R = crate::BitReader;
#[doc = "Field `PTOEN` reader - PTP Offload Enable"]
pub type PTOEN_R = crate::BitReader;
#[doc = "Field `ADVTHWORD` reader - IEEE 1588 High Word Register Enable"]
pub type ADVTHWORD_R = crate::BitReader;
#[doc = "Field `ADDR64` reader - Address width"]
pub type ADDR64_R = crate::FieldReader;
#[doc = "Field `DCBEN` reader - DCB Feature Enable"]
pub type DCBEN_R = crate::BitReader;
#[doc = "Field `SPHEN` reader - Split Header Feature Enable"]
pub type SPHEN_R = crate::BitReader;
#[doc = "Field `TSOEN` reader - TCP Segmentation Offload Enable"]
pub type TSOEN_R = crate::BitReader;
#[doc = "Field `DBGMEMA` reader - DMA Debug Registers Enable"]
pub type DBGMEMA_R = crate::BitReader;
#[doc = "Field `AVSEL` reader - AV Feature Enable"]
pub type AVSEL_R = crate::BitReader;
#[doc = "Field `HASHTBLSZ` reader - Hash Table Size"]
pub type HASHTBLSZ_R = crate::FieldReader;
#[doc = "Field `L3L4FNUM` reader - Total number of L3 or L4 Filters"]
pub type L3L4FNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - MTL Receive FIFO Size"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - MTL Transmit FIFO Size"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - One-Step Timestamping Enable"]
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PTP Offload Enable"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IEEE 1588 High Word Register Enable"]
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address width"]
    #[inline(always)]
    pub fn addr64(&self) -> ADDR64_R {
        ADDR64_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - DCB Feature Enable"]
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Split Header Feature Enable"]
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCP Segmentation Offload Enable"]
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA Debug Registers Enable"]
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AV Feature Enable"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Hash Table Size"]
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 27:30 - Total number of L3 or L4 Filters"]
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3L4FNUM_R {
        L3L4FNUM_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[doc = "HW feature 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machwf1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHWF1Rrs;
impl crate::RegisterSpec for MACHWF1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machwf1r::R`](R) reader structure"]
impl crate::Readable for MACHWF1Rrs {}
#[doc = "`reset()` method sets MACHWF1R to value 0x1184_1904"]
impl crate::Resettable for MACHWF1Rrs {
    const RESET_VALUE: u32 = 0x1184_1904;
}
