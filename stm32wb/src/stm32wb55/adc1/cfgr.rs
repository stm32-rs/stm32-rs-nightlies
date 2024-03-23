#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `DMAEN` reader - ADC DMA transfer enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - ADC DMA transfer enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACFG` reader - ADC DMA transfer configuration"]
pub type DMACFG_R = crate::BitReader;
#[doc = "Field `DMACFG` writer - ADC DMA transfer configuration"]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - ADC data resolution"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `RES` writer - ADC data resolution"]
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALIGN` reader - ADC data alignement"]
pub type ALIGN_R = crate::BitReader;
#[doc = "Field `ALIGN` writer - ADC data alignement"]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL` reader - ADC group regular external trigger source"]
pub type EXTSEL_R = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - ADC group regular external trigger source"]
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTEN` reader - ADC group regular external trigger polarity"]
pub type EXTEN_R = crate::FieldReader;
#[doc = "Field `EXTEN` writer - ADC group regular external trigger polarity"]
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - ADC group regular overrun configuration"]
pub type OVRMOD_R = crate::BitReader;
#[doc = "Field `OVRMOD` writer - ADC group regular overrun configuration"]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - ADC group regular continuous conversion mode"]
pub type CONT_R = crate::BitReader;
#[doc = "Field `CONT` writer - ADC group regular continuous conversion mode"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTDLY` reader - ADC low power auto wait"]
pub type AUTDLY_R = crate::BitReader;
#[doc = "Field `AUTDLY` writer - ADC low power auto wait"]
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - ADC group regular sequencer discontinuous mode"]
pub type DISCEN_R = crate::BitReader;
#[doc = "Field `DISCEN` writer - ADC group regular sequencer discontinuous mode"]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCNUM` reader - ADC group regular sequencer discontinuous number of ranks"]
pub type DISCNUM_R = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - ADC group regular sequencer discontinuous number of ranks"]
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JDISCEN` reader - ADC group injected sequencer discontinuous mode"]
pub type JDISCEN_R = crate::BitReader;
#[doc = "Field `JDISCEN` writer - ADC group injected sequencer discontinuous mode"]
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQM` reader - ADC group injected contexts queue mode"]
pub type JQM_R = crate::BitReader;
#[doc = "Field `JQM` writer - ADC group injected contexts queue mode"]
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1SGL` reader - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub type AWD1SGL_R = crate::BitReader;
#[doc = "Field `AWD1SGL` writer - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group regular"]
pub type AWD1EN_R = crate::BitReader;
#[doc = "Field `AWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group regular"]
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group injected"]
pub type JAWD1EN_R = crate::BitReader;
#[doc = "Field `JAWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group injected"]
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - ADC group injected automatic trigger mode"]
pub type JAUTO_R = crate::BitReader;
#[doc = "Field `JAUTO` writer - ADC group injected automatic trigger mode"]
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDCH1CH` reader - ADC analog watchdog 1 monitored channel selection"]
pub type AWDCH1CH_R = crate::FieldReader;
#[doc = "Field `AWDCH1CH` writer - ADC analog watchdog 1 monitored channel selection"]
pub type AWDCH1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JQDIS` reader - ADC group injected contexts queue disable"]
pub type JQDIS_R = crate::BitReader;
#[doc = "Field `JQDIS` writer - ADC group injected contexts queue disable"]
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC DMA transfer configuration"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - ADC data resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - ADC data alignement"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    pub fn awdch1ch(&self) -> AWDCH1CH_R {
        AWDCH1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC DMA transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CFGRrs> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC DMA transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<CFGRrs> {
        DMACFG_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - ADC data resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<CFGRrs> {
        RES_W::new(self, 3)
    }
    #[doc = "Bit 5 - ADC data alignement"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CFGRrs> {
        ALIGN_W::new(self, 5)
    }
    #[doc = "Bits 6:9 - ADC group regular external trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<CFGRrs> {
        EXTSEL_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CFGRrs> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    #[must_use]
    pub fn autdly(&mut self) -> AUTDLY_W<CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    #[must_use]
    pub fn jqm(&mut self) -> JQM_W<CFGRrs> {
        JQM_W::new(self, 21)
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    #[must_use]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awdch1ch(&mut self) -> AWDCH1CH_W<CFGRrs> {
        AWDCH1CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    #[must_use]
    pub fn jqdis(&mut self) -> JQDIS_W<CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0x8000_0000"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
