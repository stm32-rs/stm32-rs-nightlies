#[doc = "Register `DFSDM0_CR1` reader"]
pub type R = crate::R<DFSDM0_CR1rs>;
#[doc = "Register `DFSDM0_CR1` writer"]
pub type W = crate::W<DFSDM0_CR1rs>;
#[doc = "Field `DFEN` reader - DFSDM enable"]
pub type DFEN_R = crate::BitReader;
#[doc = "Field `DFEN` writer - DFSDM enable"]
pub type DFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels"]
pub type JSWSTART_R = crate::BitReader;
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels"]
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSYNC` reader - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JSYNC_R = crate::BitReader;
#[doc = "Field `JSYNC` writer - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSCAN` reader - Scanning conversion mode for injected conversions"]
pub type JSCAN_R = crate::BitReader;
#[doc = "Field `JSCAN` writer - Scanning conversion mode for injected conversions"]
pub type JSCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_R = crate::BitReader;
#[doc = "Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_R = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel"]
pub type RSWSTART_R = crate::BitReader;
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel"]
pub type RSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions"]
pub type RCONT_R = crate::BitReader;
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions"]
pub type RCONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSYNC` reader - Launch regular conversion synchronously with DFSDM0"]
pub type RSYNC_R = crate::BitReader;
#[doc = "Field `RSYNC` writer - Launch regular conversion synchronously with DFSDM0"]
pub type RSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMAEN` reader - DMA channel enabled to read data for the regular conversion"]
pub type RDMAEN_R = crate::BitReader;
#[doc = "Field `RDMAEN` writer - DMA channel enabled to read data for the regular conversion"]
pub type RDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCH` reader - Regular channel selection"]
pub type RCH_R = crate::FieldReader;
#[doc = "Field `RCH` writer - Regular channel selection"]
pub type RCH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST` reader - Fast conversion mode selection for regular conversions"]
pub type FAST_R = crate::BitReader;
#[doc = "Field `FAST` writer - Fast conversion mode selection for regular conversions"]
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWFSEL` reader - Analog watchdog fast mode select"]
pub type AWFSEL_R = crate::BitReader;
#[doc = "Field `AWFSEL` writer - Analog watchdog fast mode select"]
pub type AWFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DFEN_W<DFSDM0_CR1rs> {
        DFEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<DFSDM0_CR1rs> {
        JSWSTART_W::new(self, 1)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    #[must_use]
    pub fn jsync(&mut self) -> JSYNC_W<DFSDM0_CR1rs> {
        JSYNC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jscan(&mut self) -> JSCAN_W<DFSDM0_CR1rs> {
        JSCAN_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    #[must_use]
    pub fn jdmaen(&mut self) -> JDMAEN_W<DFSDM0_CR1rs> {
        JDMAEN_W::new(self, 5)
    }
    #[doc = "Bits 8:12 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<DFSDM0_CR1rs> {
        JEXTSEL_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<DFSDM0_CR1rs> {
        JEXTEN_W::new(self, 13)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn rswstart(&mut self) -> RSWSTART_W<DFSDM0_CR1rs> {
        RSWSTART_W::new(self, 17)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<DFSDM0_CR1rs> {
        RCONT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    #[must_use]
    pub fn rsync(&mut self) -> RSYNC_W<DFSDM0_CR1rs> {
        RSYNC_W::new(self, 19)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    #[must_use]
    pub fn rdmaen(&mut self) -> RDMAEN_W<DFSDM0_CR1rs> {
        RDMAEN_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn rch(&mut self) -> RCH_W<DFSDM0_CR1rs> {
        RCH_W::new(self, 24)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<DFSDM0_CR1rs> {
        FAST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    #[must_use]
    pub fn awfsel(&mut self) -> AWFSEL_W<DFSDM0_CR1rs> {
        AWFSEL_W::new(self, 30)
    }
}
#[doc = "DFSDM control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM0_CR1rs;
impl crate::RegisterSpec for DFSDM0_CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm0_cr1::R`](R) reader structure"]
impl crate::Readable for DFSDM0_CR1rs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm0_cr1::W`](W) writer structure"]
impl crate::Writable for DFSDM0_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM0_CR1 to value 0"]
impl crate::Resettable for DFSDM0_CR1rs {
    const RESET_VALUE: u32 = 0;
}
