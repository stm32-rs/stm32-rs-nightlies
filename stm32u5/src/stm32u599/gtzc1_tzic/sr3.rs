#[doc = "Register `SR3` reader"]
pub type R = crate::R<SR3rs>;
#[doc = "Field `MDF1F` reader - illegal access flag for MDF1"]
pub type MDF1F_R = crate::BitReader;
#[doc = "Field `CORDICF` reader - illegal access flag for CORDIC"]
pub type CORDICF_R = crate::BitReader;
#[doc = "Field `FMACF` reader - illegal access flag for FMAC"]
pub type FMACF_R = crate::BitReader;
#[doc = "Field `CRCF` reader - illegal access flag for CRC"]
pub type CRCF_R = crate::BitReader;
#[doc = "Field `TSCF` reader - illegal access flag for TSC"]
pub type TSCF_R = crate::BitReader;
#[doc = "Field `DMA2DF` reader - illegal access flag for register of DMA2D"]
pub type DMA2DF_R = crate::BitReader;
#[doc = "Field `ICACHE_REGF` reader - illegal access flag for ICACHE registers"]
pub type ICACHE_REGF_R = crate::BitReader;
#[doc = "Field `DCACHE1_REGF` reader - illegal access flag for DCACHE registers"]
pub type DCACHE1_REGF_R = crate::BitReader;
#[doc = "Field `ADC12F` reader - illegal access flag for ADC1 and ADC2"]
pub type ADC12F_R = crate::BitReader;
#[doc = "Field `DCMIF` reader - illegal access flag for DCMI"]
pub type DCMIF_R = crate::BitReader;
#[doc = "Field `OTGF` reader - illegal access flag for OTG_FS or OTG_HS"]
pub type OTGF_R = crate::BitReader;
#[doc = "Field `HASHF` reader - illegal access flag for HASH"]
pub type HASHF_R = crate::BitReader;
#[doc = "Field `RNGF` reader - illegal access flag for RNG"]
pub type RNGF_R = crate::BitReader;
#[doc = "Field `OCTOSPIMF` reader - illegal access flag for OCTOSPIM"]
pub type OCTOSPIMF_R = crate::BitReader;
#[doc = "Field `SDMMC1F` reader - illegal access flag for SDMMC2"]
pub type SDMMC1F_R = crate::BitReader;
#[doc = "Field `SDMMC2F` reader - illegal access flag for SDMMC1"]
pub type SDMMC2F_R = crate::BitReader;
#[doc = "Field `FSMC_REGF` reader - illegal access flag for FSMC registers"]
pub type FSMC_REGF_R = crate::BitReader;
#[doc = "Field `OCTOSPI1_REGF` reader - illegal access flag for OCTOSPI1 registers"]
pub type OCTOSPI1_REGF_R = crate::BitReader;
#[doc = "Field `OCTOSPI2_REGF` reader - illegal access flag for OCTOSPI2 registers"]
pub type OCTOSPI2_REGF_R = crate::BitReader;
#[doc = "Field `RAMCFGF` reader - illegal access flag for RAMCFG"]
pub type RAMCFGF_R = crate::BitReader;
#[doc = "Field `GPU2DF` reader - illegal access flag for GPU2D"]
pub type GPU2DF_R = crate::BitReader;
#[doc = "Field `GFXMMUF` reader - illegal access flag for GFXMMU"]
pub type GFXMMUF_R = crate::BitReader;
#[doc = "Field `GFXMMU_REGF` reader - illegal access flag for GFXMMU registers"]
pub type GFXMMU_REGF_R = crate::BitReader;
#[doc = "Field `HSPI1_REGF` reader - illegal access flag for HSPI1 registers"]
pub type HSPI1_REGF_R = crate::BitReader;
#[doc = "Field `DCACHE2_REGF` reader - illegal access flag for DCACHE2 registers"]
pub type DCACHE2_REGF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - illegal access flag for MDF1"]
    #[inline(always)]
    pub fn mdf1f(&self) -> MDF1F_R {
        MDF1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access flag for CORDIC"]
    #[inline(always)]
    pub fn cordicf(&self) -> CORDICF_R {
        CORDICF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access flag for FMAC"]
    #[inline(always)]
    pub fn fmacf(&self) -> FMACF_R {
        FMACF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access flag for CRC"]
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access flag for TSC"]
    #[inline(always)]
    pub fn tscf(&self) -> TSCF_R {
        TSCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal access flag for register of DMA2D"]
    #[inline(always)]
    pub fn dma2df(&self) -> DMA2DF_R {
        DMA2DF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access flag for ICACHE registers"]
    #[inline(always)]
    pub fn icache_regf(&self) -> ICACHE_REGF_R {
        ICACHE_REGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - illegal access flag for DCACHE registers"]
    #[inline(always)]
    pub fn dcache1_regf(&self) -> DCACHE1_REGF_R {
        DCACHE1_REGF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access flag for ADC1 and ADC2"]
    #[inline(always)]
    pub fn adc12f(&self) -> ADC12F_R {
        ADC12F_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access flag for DCMI"]
    #[inline(always)]
    pub fn dcmif(&self) -> DCMIF_R {
        DCMIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access flag for OTG_FS or OTG_HS"]
    #[inline(always)]
    pub fn otgf(&self) -> OTGF_R {
        OTGF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access flag for HASH"]
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - illegal access flag for RNG"]
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - illegal access flag for OCTOSPIM"]
    #[inline(always)]
    pub fn octospimf(&self) -> OCTOSPIMF_R {
        OCTOSPIMF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access flag for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc1f(&self) -> SDMMC1F_R {
        SDMMC1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - illegal access flag for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc2f(&self) -> SDMMC2F_R {
        SDMMC2F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - illegal access flag for FSMC registers"]
    #[inline(always)]
    pub fn fsmc_regf(&self) -> FSMC_REGF_R {
        FSMC_REGF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - illegal access flag for OCTOSPI1 registers"]
    #[inline(always)]
    pub fn octospi1_regf(&self) -> OCTOSPI1_REGF_R {
        OCTOSPI1_REGF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - illegal access flag for OCTOSPI2 registers"]
    #[inline(always)]
    pub fn octospi2_regf(&self) -> OCTOSPI2_REGF_R {
        OCTOSPI2_REGF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - illegal access flag for RAMCFG"]
    #[inline(always)]
    pub fn ramcfgf(&self) -> RAMCFGF_R {
        RAMCFGF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - illegal access flag for GPU2D"]
    #[inline(always)]
    pub fn gpu2df(&self) -> GPU2DF_R {
        GPU2DF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access flag for GFXMMU"]
    #[inline(always)]
    pub fn gfxmmuf(&self) -> GFXMMUF_R {
        GFXMMUF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - illegal access flag for GFXMMU registers"]
    #[inline(always)]
    pub fn gfxmmu_regf(&self) -> GFXMMU_REGF_R {
        GFXMMU_REGF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - illegal access flag for HSPI1 registers"]
    #[inline(always)]
    pub fn hspi1_regf(&self) -> HSPI1_REGF_R {
        HSPI1_REGF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - illegal access flag for DCACHE2 registers"]
    #[inline(always)]
    pub fn dcache2_regf(&self) -> DCACHE2_REGF_R {
        DCACHE2_REGF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "TZIC status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr3::R`](R) reader structure"]
impl crate::Readable for SR3rs {}
#[doc = "`reset()` method sets SR3 to value 0"]
impl crate::Resettable for SR3rs {
    const RESET_VALUE: u32 = 0;
}
