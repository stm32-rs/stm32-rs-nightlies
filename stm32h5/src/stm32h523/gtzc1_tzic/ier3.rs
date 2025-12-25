///Register `IER3` reader
pub type R = crate::R<IER3rs>;
///Register `IER3` writer
pub type W = crate::W<IER3rs>;
///Field `LPTIM6IE` reader - illegal access interrupt enable for LPTIM6
pub type LPTIM6IE_R = crate::BitReader;
///Field `LPTIM6IE` writer - illegal access interrupt enable for LPTIM6
pub type LPTIM6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFIE` reader - illegal access interrupt enable for VREFBUF
pub type VREFBUFIE_R = crate::BitReader;
///Field `VREFBUFIE` writer - illegal access interrupt enable for VREFBUF
pub type VREFBUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2IE` reader - illegal access interrupt enable for I3C2
pub type I3C2IE_R = crate::BitReader;
///Field `I3C2IE` writer - illegal access interrupt enable for I3C2
pub type I3C2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCIE` reader - illegal access interrupt enable for CRC
pub type CRCIE_R = crate::BitReader;
///Field `CRCIE` writer - illegal access interrupt enable for CRC
pub type CRCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICIE` reader - illegal access interrupt enable for CORDIC
pub type CORDICIE_R = crate::BitReader;
///Field `CORDICIE` writer - illegal access interrupt enable for CORDIC
pub type CORDICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACIE` reader - illegal access interrupt enable for FMAC
pub type FMACIE_R = crate::BitReader;
///Field `FMACIE` writer - illegal access interrupt enable for FMAC
pub type FMACIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHIE` reader - illegal access interrupt enable for register of ETH
pub type ETHIE_R = crate::BitReader;
///Field `ETHIE` writer - illegal access interrupt enable for register of ETH
pub type ETHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHEIE` reader - illegal access interrupt enable for ICACHE
pub type ICACHEIE_R = crate::BitReader;
///Field `ICACHEIE` writer - illegal access interrupt enable for ICACHE
pub type ICACHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHEIE` reader - illegal access interrupt enable for DCACHE
pub type DCACHEIE_R = crate::BitReader;
///Field `DCACHEIE` writer - illegal access interrupt enable for DCACHE
pub type DCACHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12IE` reader - illegal access interrupt enable for ADC1 and ADC2
pub type ADC12IE_R = crate::BitReader;
///Field `ADC12IE` writer - illegal access interrupt enable for ADC1 and ADC2
pub type ADC12IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIIE` reader - illegal access interrupt enable for DCMI
pub type DCMIIE_R = crate::BitReader;
///Field `DCMIIE` writer - illegal access interrupt enable for DCMI
pub type DCMIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESIE` reader - illegal access interrupt enable for AES
pub type AESIE_R = crate::BitReader;
///Field `AESIE` writer - illegal access interrupt enable for AES
pub type AESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHIE` reader - illegal access interrupt enable for HASH
pub type HASHIE_R = crate::BitReader;
///Field `HASHIE` writer - illegal access interrupt enable for HASH
pub type HASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGIE` reader - illegal access interrupt enable for RNG
pub type RNGIE_R = crate::BitReader;
///Field `RNGIE` writer - illegal access interrupt enable for RNG
pub type RNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESIE` reader - illegal access interrupt enable for SAES
pub type SAESIE_R = crate::BitReader;
///Field `SAESIE` writer - illegal access interrupt enable for SAES
pub type SAESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAIE` reader - illegal access interrupt enable for PKA
pub type PKAIE_R = crate::BitReader;
///Field `PKAIE` writer - illegal access interrupt enable for PKA
pub type PKAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1IE` reader - illegal access interrupt enable for SDMMC1
pub type SDMMC1IE_R = crate::BitReader;
///Field `SDMMC1IE` writer - illegal access interrupt enable for SDMMC1
pub type SDMMC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCIE` reader - illegal access interrupt enable for FMC
pub type FMCIE_R = crate::BitReader;
///Field `FMCIE` writer - illegal access interrupt enable for FMC
pub type FMCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1IE` reader - illegal access interrupt enable for OCTOSPI1
pub type OCTOSPI1IE_R = crate::BitReader;
///Field `OCTOSPI1IE` writer - illegal access interrupt enable for OCTOSPI1
pub type OCTOSPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGIE` reader - illegal access interrupt enable for RAMSCFG
pub type RAMCFGIE_R = crate::BitReader;
///Field `RAMCFGIE` writer - illegal access interrupt enable for RAMSCFG
pub type RAMCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for LPTIM6
    #[inline(always)]
    pub fn lptim6ie(&self) -> LPTIM6IE_R {
        LPTIM6IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for VREFBUF
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for I3C2
    #[inline(always)]
    pub fn i3c2ie(&self) -> I3C2IE_R {
        I3C2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for CRC
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for CORDIC
    #[inline(always)]
    pub fn cordicie(&self) -> CORDICIE_R {
        CORDICIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for FMAC
    #[inline(always)]
    pub fn fmacie(&self) -> FMACIE_R {
        FMACIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for register of ETH
    #[inline(always)]
    pub fn ethie(&self) -> ETHIE_R {
        ETHIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for ICACHE
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for DCACHE
    #[inline(always)]
    pub fn dcacheie(&self) -> DCACHEIE_R {
        DCACHEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12ie(&self) -> ADC12IE_R {
        ADC12IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for DCMI
    #[inline(always)]
    pub fn dcmiie(&self) -> DCMIIE_R {
        DCMIIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for AES
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for HASH
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for RNG
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for SAES
    #[inline(always)]
    pub fn saesie(&self) -> SAESIE_R {
        SAESIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for PKA
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for SDMMC1
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for FMC
    #[inline(always)]
    pub fn fmcie(&self) -> FMCIE_R {
        FMCIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for OCTOSPI1
    #[inline(always)]
    pub fn octospi1ie(&self) -> OCTOSPI1IE_R {
        OCTOSPI1IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for RAMSCFG
    #[inline(always)]
    pub fn ramcfgie(&self) -> RAMCFGIE_R {
        RAMCFGIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER3")
            .field("lptim6ie", &self.lptim6ie())
            .field("vrefbufie", &self.vrefbufie())
            .field("i3c2ie", &self.i3c2ie())
            .field("crcie", &self.crcie())
            .field("cordicie", &self.cordicie())
            .field("fmacie", &self.fmacie())
            .field("ethie", &self.ethie())
            .field("icacheie", &self.icacheie())
            .field("dcacheie", &self.dcacheie())
            .field("adc12ie", &self.adc12ie())
            .field("dcmiie", &self.dcmiie())
            .field("aesie", &self.aesie())
            .field("hashie", &self.hashie())
            .field("rngie", &self.rngie())
            .field("saesie", &self.saesie())
            .field("pkaie", &self.pkaie())
            .field("sdmmc1ie", &self.sdmmc1ie())
            .field("fmcie", &self.fmcie())
            .field("octospi1ie", &self.octospi1ie())
            .field("ramcfgie", &self.ramcfgie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for LPTIM6
    #[inline(always)]
    pub fn lptim6ie(&mut self) -> LPTIM6IE_W<'_, IER3rs> {
        LPTIM6IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for VREFBUF
    #[inline(always)]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W<'_, IER3rs> {
        VREFBUFIE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for I3C2
    #[inline(always)]
    pub fn i3c2ie(&mut self) -> I3C2IE_W<'_, IER3rs> {
        I3C2IE_W::new(self, 2)
    }
    ///Bit 8 - illegal access interrupt enable for CRC
    #[inline(always)]
    pub fn crcie(&mut self) -> CRCIE_W<'_, IER3rs> {
        CRCIE_W::new(self, 8)
    }
    ///Bit 9 - illegal access interrupt enable for CORDIC
    #[inline(always)]
    pub fn cordicie(&mut self) -> CORDICIE_W<'_, IER3rs> {
        CORDICIE_W::new(self, 9)
    }
    ///Bit 10 - illegal access interrupt enable for FMAC
    #[inline(always)]
    pub fn fmacie(&mut self) -> FMACIE_W<'_, IER3rs> {
        FMACIE_W::new(self, 10)
    }
    ///Bit 11 - illegal access interrupt enable for register of ETH
    #[inline(always)]
    pub fn ethie(&mut self) -> ETHIE_W<'_, IER3rs> {
        ETHIE_W::new(self, 11)
    }
    ///Bit 12 - illegal access interrupt enable for ICACHE
    #[inline(always)]
    pub fn icacheie(&mut self) -> ICACHEIE_W<'_, IER3rs> {
        ICACHEIE_W::new(self, 12)
    }
    ///Bit 13 - illegal access interrupt enable for DCACHE
    #[inline(always)]
    pub fn dcacheie(&mut self) -> DCACHEIE_W<'_, IER3rs> {
        DCACHEIE_W::new(self, 13)
    }
    ///Bit 14 - illegal access interrupt enable for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12ie(&mut self) -> ADC12IE_W<'_, IER3rs> {
        ADC12IE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for DCMI
    #[inline(always)]
    pub fn dcmiie(&mut self) -> DCMIIE_W<'_, IER3rs> {
        DCMIIE_W::new(self, 15)
    }
    ///Bit 16 - illegal access interrupt enable for AES
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W<'_, IER3rs> {
        AESIE_W::new(self, 16)
    }
    ///Bit 17 - illegal access interrupt enable for HASH
    #[inline(always)]
    pub fn hashie(&mut self) -> HASHIE_W<'_, IER3rs> {
        HASHIE_W::new(self, 17)
    }
    ///Bit 18 - illegal access interrupt enable for RNG
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W<'_, IER3rs> {
        RNGIE_W::new(self, 18)
    }
    ///Bit 19 - illegal access interrupt enable for SAES
    #[inline(always)]
    pub fn saesie(&mut self) -> SAESIE_W<'_, IER3rs> {
        SAESIE_W::new(self, 19)
    }
    ///Bit 20 - illegal access interrupt enable for PKA
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W<'_, IER3rs> {
        PKAIE_W::new(self, 20)
    }
    ///Bit 21 - illegal access interrupt enable for SDMMC1
    #[inline(always)]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W<'_, IER3rs> {
        SDMMC1IE_W::new(self, 21)
    }
    ///Bit 23 - illegal access interrupt enable for FMC
    #[inline(always)]
    pub fn fmcie(&mut self) -> FMCIE_W<'_, IER3rs> {
        FMCIE_W::new(self, 23)
    }
    ///Bit 24 - illegal access interrupt enable for OCTOSPI1
    #[inline(always)]
    pub fn octospi1ie(&mut self) -> OCTOSPI1IE_W<'_, IER3rs> {
        OCTOSPI1IE_W::new(self, 24)
    }
    ///Bit 26 - illegal access interrupt enable for RAMSCFG
    #[inline(always)]
    pub fn ramcfgie(&mut self) -> RAMCFGIE_W<'_, IER3rs> {
        RAMCFGIE_W::new(self, 26)
    }
}
/**GTZC1 TZIC interrupt enable register 3

You can [`read`](crate::Reg::read) this register and get [`ier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#GTZC1_TZIC:IER3)*/
pub struct IER3rs;
impl crate::RegisterSpec for IER3rs {
    type Ux = u32;
}
///`read()` method returns [`ier3::R`](R) reader structure
impl crate::Readable for IER3rs {}
///`write(|w| ..)` method takes [`ier3::W`](W) writer structure
impl crate::Writable for IER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER3 to value 0
impl crate::Resettable for IER3rs {}
