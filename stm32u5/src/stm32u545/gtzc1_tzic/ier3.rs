#[doc = "Register `IER3` reader"]
pub type R = crate::R<IER3rs>;
#[doc = "Register `IER3` writer"]
pub type W = crate::W<IER3rs>;
#[doc = "Field `MDF1IE` reader - illegal access interrupt enable for MDF1"]
pub type MDF1IE_R = crate::BitReader;
#[doc = "Field `MDF1IE` writer - illegal access interrupt enable for MDF1"]
pub type MDF1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICIE` reader - illegal access interrupt enable for CORDIC"]
pub type CORDICIE_R = crate::BitReader;
#[doc = "Field `CORDICIE` writer - illegal access interrupt enable for CORDIC"]
pub type CORDICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACIE` reader - illegal access interrupt enable for FMAC"]
pub type FMACIE_R = crate::BitReader;
#[doc = "Field `FMACIE` writer - illegal access interrupt enable for FMAC"]
pub type FMACIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCIE` reader - illegal access interrupt enable for CRC"]
pub type CRCIE_R = crate::BitReader;
#[doc = "Field `CRCIE` writer - illegal access interrupt enable for CRC"]
pub type CRCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCIE` reader - illegal access interrupt enable for TSC"]
pub type TSCIE_R = crate::BitReader;
#[doc = "Field `TSCIE` writer - illegal access interrupt enable for TSC"]
pub type TSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_REGIE` reader - illegal access interrupt enable for ICACHE registers"]
pub type ICACHE_REGIE_R = crate::BitReader;
#[doc = "Field `ICACHE_REGIE` writer - illegal access interrupt enable for ICACHE registers"]
pub type ICACHE_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE1_REGIE` reader - illegal access interrupt enable for DCACHE registers"]
pub type DCACHE1_REGIE_R = crate::BitReader;
#[doc = "Field `DCACHE1_REGIE` writer - illegal access interrupt enable for DCACHE registers"]
pub type DCACHE1_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1I2E` reader - illegal access interrupt enable for ADC1 or ADC2"]
pub type ADC1I2E_R = crate::BitReader;
#[doc = "Field `ADC1I2E` writer - illegal access interrupt enable for ADC1 or ADC2"]
pub type ADC1I2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMIIE` reader - illegal access interrupt enable for DCMI"]
pub type DCMIIE_R = crate::BitReader;
#[doc = "Field `DCMIIE` writer - illegal access interrupt enable for DCMI"]
pub type DCMIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESIE` reader - illegal access interrupt enable for AES"]
pub type AESIE_R = crate::BitReader;
#[doc = "Field `AESIE` writer - illegal access interrupt enable for AES"]
pub type AESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHIE` reader - illegal access interrupt enable for HASH"]
pub type HASHIE_R = crate::BitReader;
#[doc = "Field `HASHIE` writer - illegal access interrupt enable for HASH"]
pub type HASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGIE` reader - illegal access interrupt enable for RNG"]
pub type RNGIE_R = crate::BitReader;
#[doc = "Field `RNGIE` writer - illegal access interrupt enable for RNG"]
pub type RNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAIE` reader - illegal access interrupt enable for PKA"]
pub type PKAIE_R = crate::BitReader;
#[doc = "Field `PKAIE` writer - illegal access interrupt enable for PKA"]
pub type PKAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAESIE` reader - illegal access interrupt enable for SAES"]
pub type SAESIE_R = crate::BitReader;
#[doc = "Field `SAESIE` writer - illegal access interrupt enable for SAES"]
pub type SAESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1IE` reader - illegal access interrupt enable"]
pub type SDMMC1IE_R = crate::BitReader;
#[doc = "Field `SDMMC1IE` writer - illegal access interrupt enable"]
pub type SDMMC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1_REGIE` reader - illegal access interrupt enable for OCTOSPI1 registers"]
pub type OCTOSPI1_REGIE_R = crate::BitReader;
#[doc = "Field `OCTOSPI1_REGIE` writer - illegal access interrupt enable for OCTOSPI1 registers"]
pub type OCTOSPI1_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGIE` reader - illegal access interrupt enable for RAMCFG"]
pub type RAMCFGIE_R = crate::BitReader;
#[doc = "Field `RAMCFGIE` writer - illegal access interrupt enable for RAMCFG"]
pub type RAMCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPU2DIE` reader - GPU2DIE"]
pub type GPU2DIE_R = crate::BitReader;
#[doc = "Field `GPU2DIE` writer - GPU2DIE"]
pub type GPU2DIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPI1_REGIE` reader - HSPI1_REGIE"]
pub type HSPI1_REGIE_R = crate::BitReader;
#[doc = "Field `HSPI1_REGIE` writer - HSPI1_REGIE"]
pub type HSPI1_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for MDF1"]
    #[inline(always)]
    pub fn mdf1ie(&self) -> MDF1IE_R {
        MDF1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for CORDIC"]
    #[inline(always)]
    pub fn cordicie(&self) -> CORDICIE_R {
        CORDICIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for FMAC"]
    #[inline(always)]
    pub fn fmacie(&self) -> FMACIE_R {
        FMACIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for CRC"]
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for TSC"]
    #[inline(always)]
    pub fn tscie(&self) -> TSCIE_R {
        TSCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for ICACHE registers"]
    #[inline(always)]
    pub fn icache_regie(&self) -> ICACHE_REGIE_R {
        ICACHE_REGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for DCACHE registers"]
    #[inline(always)]
    pub fn dcache1_regie(&self) -> DCACHE1_REGIE_R {
        DCACHE1_REGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for ADC1 or ADC2"]
    #[inline(always)]
    pub fn adc1i2e(&self) -> ADC1I2E_R {
        ADC1I2E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for DCMI"]
    #[inline(always)]
    pub fn dcmiie(&self) -> DCMIIE_R {
        DCMIIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for AES"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for HASH"]
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for RNG"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for PKA"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for SAES"]
    #[inline(always)]
    pub fn saesie(&self) -> SAESIE_R {
        SAESIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access interrupt enable"]
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - illegal access interrupt enable for OCTOSPI1 registers"]
    #[inline(always)]
    pub fn octospi1_regie(&self) -> OCTOSPI1_REGIE_R {
        OCTOSPI1_REGIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - illegal access interrupt enable for RAMCFG"]
    #[inline(always)]
    pub fn ramcfgie(&self) -> RAMCFGIE_R {
        RAMCFGIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPU2DIE"]
    #[inline(always)]
    pub fn gpu2die(&self) -> GPU2DIE_R {
        GPU2DIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - HSPI1_REGIE"]
    #[inline(always)]
    pub fn hspi1_regie(&self) -> HSPI1_REGIE_R {
        HSPI1_REGIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for MDF1"]
    #[inline(always)]
    #[must_use]
    pub fn mdf1ie(&mut self) -> MDF1IE_W<IER3rs> {
        MDF1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicie(&mut self) -> CORDICIE_W<IER3rs> {
        CORDICIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacie(&mut self) -> FMACIE_W<IER3rs> {
        FMACIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcie(&mut self) -> CRCIE_W<IER3rs> {
        CRCIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for TSC"]
    #[inline(always)]
    #[must_use]
    pub fn tscie(&mut self) -> TSCIE_W<IER3rs> {
        TSCIE_W::new(self, 4)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for ICACHE registers"]
    #[inline(always)]
    #[must_use]
    pub fn icache_regie(&mut self) -> ICACHE_REGIE_W<IER3rs> {
        ICACHE_REGIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for DCACHE registers"]
    #[inline(always)]
    #[must_use]
    pub fn dcache1_regie(&mut self) -> DCACHE1_REGIE_W<IER3rs> {
        DCACHE1_REGIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for ADC1 or ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc1i2e(&mut self) -> ADC1I2E_W<IER3rs> {
        ADC1I2E_W::new(self, 8)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmiie(&mut self) -> DCMIIE_W<IER3rs> {
        DCMIIE_W::new(self, 9)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for AES"]
    #[inline(always)]
    #[must_use]
    pub fn aesie(&mut self) -> AESIE_W<IER3rs> {
        AESIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashie(&mut self) -> HASHIE_W<IER3rs> {
        HASHIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngie(&mut self) -> RNGIE_W<IER3rs> {
        RNGIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for PKA"]
    #[inline(always)]
    #[must_use]
    pub fn pkaie(&mut self) -> PKAIE_W<IER3rs> {
        PKAIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for SAES"]
    #[inline(always)]
    #[must_use]
    pub fn saesie(&mut self) -> SAESIE_W<IER3rs> {
        SAESIE_W::new(self, 15)
    }
    #[doc = "Bit 17 - illegal access interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W<IER3rs> {
        SDMMC1IE_W::new(self, 17)
    }
    #[doc = "Bit 20 - illegal access interrupt enable for OCTOSPI1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regie(&mut self) -> OCTOSPI1_REGIE_W<IER3rs> {
        OCTOSPI1_REGIE_W::new(self, 20)
    }
    #[doc = "Bit 22 - illegal access interrupt enable for RAMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgie(&mut self) -> RAMCFGIE_W<IER3rs> {
        RAMCFGIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - GPU2DIE"]
    #[inline(always)]
    #[must_use]
    pub fn gpu2die(&mut self) -> GPU2DIE_W<IER3rs> {
        GPU2DIE_W::new(self, 23)
    }
    #[doc = "Bit 26 - HSPI1_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn hspi1_regie(&mut self) -> HSPI1_REGIE_W<IER3rs> {
        HSPI1_REGIE_W::new(self, 26)
    }
}
#[doc = "TZIC interrupt enable register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER3rs;
impl crate::RegisterSpec for IER3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier3::R`](R) reader structure"]
impl crate::Readable for IER3rs {}
#[doc = "`write(|w| ..)` method takes [`ier3::W`](W) writer structure"]
impl crate::Writable for IER3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER3 to value 0"]
impl crate::Resettable for IER3rs {
    const RESET_VALUE: u32 = 0;
}
