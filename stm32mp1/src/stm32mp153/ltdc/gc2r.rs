///Register `GC2R` reader
pub type R = crate::R<GC2Rrs>;
///Field `EDCEN` reader - EDCEN
pub type EDCEN_R = crate::BitReader;
///Field `STSAEN` reader - STSAEN
pub type STSAEN_R = crate::BitReader;
///Field `DVAEN` reader - DVAEN
pub type DVAEN_R = crate::BitReader;
///Field `DPAEN` reader - DPAEN
pub type DPAEN_R = crate::BitReader;
///Field `BW` reader - BW
pub type BW_R = crate::FieldReader;
///Field `EDCA` reader - EDCA
pub type EDCA_R = crate::BitReader;
impl R {
    ///Bit 0 - EDCEN
    #[inline(always)]
    pub fn edcen(&self) -> EDCEN_R {
        EDCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - STSAEN
    #[inline(always)]
    pub fn stsaen(&self) -> STSAEN_R {
        STSAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DVAEN
    #[inline(always)]
    pub fn dvaen(&self) -> DVAEN_R {
        DVAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DPAEN
    #[inline(always)]
    pub fn dpaen(&self) -> DPAEN_R {
        DPAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - BW
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - EDCA
    #[inline(always)]
    pub fn edca(&self) -> EDCA_R {
        EDCA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GC2R")
            .field("edcen", &self.edcen())
            .field("stsaen", &self.stsaen())
            .field("dvaen", &self.dvaen())
            .field("dpaen", &self.dpaen())
            .field("bw", &self.bw())
            .field("edca", &self.edca())
            .finish()
    }
}
/**LTDC global configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`gc2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:GC2R)*/
pub struct GC2Rrs;
impl crate::RegisterSpec for GC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`gc2r::R`](R) reader structure
impl crate::Readable for GC2Rrs {}
///`reset()` method sets GC2R to value 0x30
impl crate::Resettable for GC2Rrs {
    const RESET_VALUE: u32 = 0x30;
}
