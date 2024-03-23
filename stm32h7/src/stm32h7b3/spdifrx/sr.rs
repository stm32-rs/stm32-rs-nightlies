#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `RXNE` reader - Read data register not empty"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `CSRNE` reader - Control Buffer register is not empty"]
pub type CSRNE_R = crate::BitReader;
#[doc = "Field `PERR` reader - Parity error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun error"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `SBD` reader - Synchronization Block Detected"]
pub type SBD_R = crate::BitReader;
#[doc = "Field `SYNCD` reader - Synchronization Done"]
pub type SYNCD_R = crate::BitReader;
#[doc = "Field `FERR` reader - Framing error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `SERR` reader - Synchronization error"]
pub type SERR_R = crate::BitReader;
#[doc = "Field `TERR` reader - Time-out error"]
pub type TERR_R = crate::BitReader;
#[doc = "Field `WIDTH5` reader - Duration of 5 symbols counted with SPDIF_CLK"]
pub type WIDTH5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Buffer register is not empty"]
    #[inline(always)]
    pub fn csrne(&self) -> CSRNE_R {
        CSRNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Block Detected"]
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncd(&self) -> SYNCD_R {
        SYNCD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization error"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time-out error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Duration of 5 symbols counted with SPDIF_CLK"]
    #[inline(always)]
    pub fn width5(&self) -> WIDTH5_R {
        WIDTH5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
