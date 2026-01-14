///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RXNE` reader - RXNE
pub type RXNE_R = crate::BitReader;
///Field `CSRNE` reader - CSRNE
pub type CSRNE_R = crate::BitReader;
///Field `PERR` reader - PERR
pub type PERR_R = crate::BitReader;
///Field `OVR` reader - OVR
pub type OVR_R = crate::BitReader;
///Field `SBD` reader - SBD
pub type SBD_R = crate::BitReader;
///Field `SYNCD` reader - SYNCD
pub type SYNCD_R = crate::BitReader;
///Field `FERR` reader - FERR
pub type FERR_R = crate::BitReader;
///Field `SERR` reader - SERR
pub type SERR_R = crate::BitReader;
///Field `TERR` reader - TERR
pub type TERR_R = crate::BitReader;
///Field `WIDTH5` reader - WIDTH5
pub type WIDTH5_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CSRNE
    #[inline(always)]
    pub fn csrne(&self) -> CSRNE_R {
        CSRNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PERR
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OVR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SBD
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYNCD
    #[inline(always)]
    pub fn syncd(&self) -> SYNCD_R {
        SYNCD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FERR
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SERR
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TERR
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:30 - WIDTH5
    #[inline(always)]
    pub fn width5(&self) -> WIDTH5_R {
        WIDTH5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rxne", &self.rxne())
            .field("csrne", &self.csrne())
            .field("perr", &self.perr())
            .field("ovr", &self.ovr())
            .field("sbd", &self.sbd())
            .field("syncd", &self.syncd())
            .field("ferr", &self.ferr())
            .field("serr", &self.serr())
            .field("terr", &self.terr())
            .field("width5", &self.width5())
            .finish()
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
