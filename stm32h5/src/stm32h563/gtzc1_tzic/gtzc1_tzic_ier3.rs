#[doc = "Register `GTZC1_TZIC_IER3` reader"]
pub type R = crate::R<GTZC1_TZIC_IER3rs>;
#[doc = "Register `GTZC1_TZIC_IER3` writer"]
pub type W = crate::W<GTZC1_TZIC_IER3rs>;
#[doc = "Field `LPTIM6IE` reader - illegal access interrupt enable for LPTIM6"]
pub type LPTIM6IE_R = crate::BitReader;
#[doc = "Field `LPTIM6IE` writer - illegal access interrupt enable for LPTIM6"]
pub type LPTIM6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFIE` reader - illegal access interrupt enable for VREFBUF"]
pub type VREFBUFIE_R = crate::BitReader;
#[doc = "Field `VREFBUFIE` writer - illegal access interrupt enable for VREFBUF"]
pub type VREFBUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCIE` reader - illegal access interrupt enable for CRC"]
pub type CRCIE_R = crate::BitReader;
#[doc = "Field `CRCIE` writer - illegal access interrupt enable for CRC"]
pub type CRCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICIE` reader - illegal access interrupt enable for CORDIC"]
pub type CORDICIE_R = crate::BitReader;
#[doc = "Field `CORDICIE` writer - illegal access interrupt enable for CORDIC"]
pub type CORDICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACIE` reader - illegal access interrupt enable for FMAC"]
pub type FMACIE_R = crate::BitReader;
#[doc = "Field `FMACIE` writer - illegal access interrupt enable for FMAC"]
pub type FMACIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHIE` reader - illegal access interrupt enable for register of ETH"]
pub type ETHIE_R = crate::BitReader;
#[doc = "Field `ETHIE` writer - illegal access interrupt enable for register of ETH"]
pub type ETHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHEIE` reader - illegal access interrupt enable for ICACHE"]
pub type ICACHEIE_R = crate::BitReader;
#[doc = "Field `ICACHEIE` writer - illegal access interrupt enable for ICACHE"]
pub type ICACHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHEIE` reader - illegal access interrupt enable for DCACHE"]
pub type DCACHEIE_R = crate::BitReader;
#[doc = "Field `DCACHEIE` writer - illegal access interrupt enable for DCACHE"]
pub type DCACHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE` reader - illegal access interrupt enable for ADC1 and ADC2"]
pub type ADC12IE_R = crate::BitReader;
#[doc = "Field `ADC12IE` writer - illegal access interrupt enable for ADC1 and ADC2"]
pub type ADC12IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMIIE` reader - illegal access interrupt enable for DCMI"]
pub type DCMIIE_R = crate::BitReader;
#[doc = "Field `DCMIIE` writer - illegal access interrupt enable for DCMI"]
pub type DCMIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHIE` reader - illegal access interrupt enable for HASH"]
pub type HASHIE_R = crate::BitReader;
#[doc = "Field `HASHIE` writer - illegal access interrupt enable for HASH"]
pub type HASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGIE` reader - illegal access interrupt enable for RNG"]
pub type RNGIE_R = crate::BitReader;
#[doc = "Field `RNGIE` writer - illegal access interrupt enable for RNG"]
pub type RNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2IE` reader - illegal access interrupt enable for SDMMC2"]
pub type SDMMC2IE_R = crate::BitReader;
#[doc = "Field `SDMMC2IE` writer - illegal access interrupt enable for SDMMC2"]
pub type SDMMC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1IE` reader - illegal access interrupt enable for SDMMC1"]
pub type SDMMC1IE_R = crate::BitReader;
#[doc = "Field `SDMMC1IE` writer - illegal access interrupt enable for SDMMC1"]
pub type SDMMC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCIE` reader - illegal access interrupt enable for FMC"]
pub type FMCIE_R = crate::BitReader;
#[doc = "Field `FMCIE` writer - illegal access interrupt enable for FMC"]
pub type FMCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1IE` reader - illegal access interrupt enable for OCTOSPI1"]
pub type OCTOSPI1IE_R = crate::BitReader;
#[doc = "Field `OCTOSPI1IE` writer - illegal access interrupt enable for OCTOSPI1"]
pub type OCTOSPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGIE` reader - illegal access interrupt enable for RAMSCFG"]
pub type RAMCFGIE_R = crate::BitReader;
#[doc = "Field `RAMCFGIE` writer - illegal access interrupt enable for RAMSCFG"]
pub type RAMCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for LPTIM6"]
    #[inline(always)]
    pub fn lptim6ie(&self) -> LPTIM6IE_R {
        LPTIM6IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for CRC"]
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for CORDIC"]
    #[inline(always)]
    pub fn cordicie(&self) -> CORDICIE_R {
        CORDICIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for FMAC"]
    #[inline(always)]
    pub fn fmacie(&self) -> FMACIE_R {
        FMACIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for register of ETH"]
    #[inline(always)]
    pub fn ethie(&self) -> ETHIE_R {
        ETHIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for ICACHE"]
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for DCACHE"]
    #[inline(always)]
    pub fn dcacheie(&self) -> DCACHEIE_R {
        DCACHEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for ADC1 and ADC2"]
    #[inline(always)]
    pub fn adc12ie(&self) -> ADC12IE_R {
        ADC12IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for DCMI"]
    #[inline(always)]
    pub fn dcmiie(&self) -> DCMIIE_R {
        DCMIIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for HASH"]
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for RNG"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - illegal access interrupt enable for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc2ie(&self) -> SDMMC2IE_R {
        SDMMC2IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - illegal access interrupt enable for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - illegal access interrupt enable for FMC"]
    #[inline(always)]
    pub fn fmcie(&self) -> FMCIE_R {
        FMCIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1ie(&self) -> OCTOSPI1IE_R {
        OCTOSPI1IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgie(&self) -> RAMCFGIE_R {
        RAMCFGIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn lptim6ie(&mut self) -> LPTIM6IE_W<GTZC1_TZIC_IER3rs> {
        LPTIM6IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W<GTZC1_TZIC_IER3rs> {
        VREFBUFIE_W::new(self, 1)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcie(&mut self) -> CRCIE_W<GTZC1_TZIC_IER3rs> {
        CRCIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicie(&mut self) -> CORDICIE_W<GTZC1_TZIC_IER3rs> {
        CORDICIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacie(&mut self) -> FMACIE_W<GTZC1_TZIC_IER3rs> {
        FMACIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for register of ETH"]
    #[inline(always)]
    #[must_use]
    pub fn ethie(&mut self) -> ETHIE_W<GTZC1_TZIC_IER3rs> {
        ETHIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icacheie(&mut self) -> ICACHEIE_W<GTZC1_TZIC_IER3rs> {
        ICACHEIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn dcacheie(&mut self) -> DCACHEIE_W<GTZC1_TZIC_IER3rs> {
        DCACHEIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie(&mut self) -> ADC12IE_W<GTZC1_TZIC_IER3rs> {
        ADC12IE_W::new(self, 14)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmiie(&mut self) -> DCMIIE_W<GTZC1_TZIC_IER3rs> {
        DCMIIE_W::new(self, 15)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashie(&mut self) -> HASHIE_W<GTZC1_TZIC_IER3rs> {
        HASHIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngie(&mut self) -> RNGIE_W<GTZC1_TZIC_IER3rs> {
        RNGIE_W::new(self, 18)
    }
    #[doc = "Bit 21 - illegal access interrupt enable for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2ie(&mut self) -> SDMMC2IE_W<GTZC1_TZIC_IER3rs> {
        SDMMC2IE_W::new(self, 21)
    }
    #[doc = "Bit 22 - illegal access interrupt enable for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W<GTZC1_TZIC_IER3rs> {
        SDMMC1IE_W::new(self, 22)
    }
    #[doc = "Bit 23 - illegal access interrupt enable for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcie(&mut self) -> FMCIE_W<GTZC1_TZIC_IER3rs> {
        FMCIE_W::new(self, 23)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1ie(&mut self) -> OCTOSPI1IE_W<GTZC1_TZIC_IER3rs> {
        OCTOSPI1IE_W::new(self, 24)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgie(&mut self) -> RAMCFGIE_W<GTZC1_TZIC_IER3rs> {
        RAMCFGIE_W::new(self, 26)
    }
}
#[doc = "GTZC1 TZIC interrupt enable register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_ier3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_ier3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_IER3rs;
impl crate::RegisterSpec for GTZC1_TZIC_IER3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_tzic_ier3::R`](R) reader structure"]
impl crate::Readable for GTZC1_TZIC_IER3rs {}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzic_ier3::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_IER3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_IER3 to value 0"]
impl crate::Resettable for GTZC1_TZIC_IER3rs {
    const RESET_VALUE: u32 = 0;
}
