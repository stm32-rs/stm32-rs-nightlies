///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `DMNGT` reader - Data management configuration
pub type DMNGT_R = crate::FieldReader;
///Field `DMNGT` writer - Data management configuration
pub type DMNGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RES` reader - Data resolution
pub type RES_R = crate::FieldReader;
///Field `RES` writer - Data resolution
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EXTSEL` reader - External trigger selection for regular group
pub type EXTSEL_R = crate::FieldReader;
///Field `EXTSEL` writer - External trigger selection for regular group
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EXTEN` reader - External trigger enable and polarity selection for regular channels
pub type EXTEN_R = crate::FieldReader;
///Field `EXTEN` writer - External trigger enable and polarity selection for regular channels
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OVRMOD` reader - Overrun mode
pub type OVRMOD_R = crate::BitReader;
///Field `OVRMOD` writer - Overrun mode
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONT` reader - Single / Continuous conversion mode for regular conversions
pub type CONT_R = crate::BitReader;
///Field `CONT` writer - Single / Continuous conversion mode for regular conversions
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTDLY` reader - Delayed conversion mode
pub type AUTDLY_R = crate::BitReader;
///Field `AUTDLY` writer - Delayed conversion mode
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCEN` reader - Discontinuous mode for regular channels
pub type DISCEN_R = crate::BitReader;
///Field `DISCEN` writer - Discontinuous mode for regular channels
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCNUM` reader - Discontinuous mode channel count
pub type DISCNUM_R = crate::FieldReader;
///Field `DISCNUM` writer - Discontinuous mode channel count
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `JDISCEN` reader - Discontinuous mode on injected channels
pub type JDISCEN_R = crate::BitReader;
///Field `JDISCEN` writer - Discontinuous mode on injected channels
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JQM` reader - JSQR queue mode
pub type JQM_R = crate::BitReader;
///Field `JQM` writer - JSQR queue mode
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1SGL` reader - Enable the watchdog 1 on a single channel or on all channels
pub type AWD1SGL_R = crate::BitReader;
///Field `AWD1SGL` writer - Enable the watchdog 1 on a single channel or on all channels
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1EN` reader - Analog watchdog 1 enable on regular channels
pub type AWD1EN_R = crate::BitReader;
///Field `AWD1EN` writer - Analog watchdog 1 enable on regular channels
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JAWD1EN` reader - Analog watchdog 1 enable on injected channels
pub type JAWD1EN_R = crate::BitReader;
///Field `JAWD1EN` writer - Analog watchdog 1 enable on injected channels
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JAUTO` reader - Automatic injected group conversion
pub type JAUTO_R = crate::BitReader;
///Field `JAUTO` writer - Automatic injected group conversion
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1CH` reader - Analog watchdog 1 channel selection
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - Analog watchdog 1 channel selection
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JQDIS` reader - Injected queue disable
pub type JQDIS_R = crate::BitReader;
///Field `JQDIS` writer - Injected queue disable
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Data management configuration
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Data resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:9 - External trigger selection for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Overrun mode
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single / Continuous conversion mode for regular conversions
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Delayed conversion mode
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Discontinuous mode for regular channels
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - JSQR queue mode
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Injected queue disable
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("dmngt", &self.dmngt())
            .field("res", &self.res())
            .field("extsel", &self.extsel())
            .field("exten", &self.exten())
            .field("ovrmod", &self.ovrmod())
            .field("cont", &self.cont())
            .field("autdly", &self.autdly())
            .field("discen", &self.discen())
            .field("discnum", &self.discnum())
            .field("jdiscen", &self.jdiscen())
            .field("jqm", &self.jqm())
            .field("awd1sgl", &self.awd1sgl())
            .field("awd1en", &self.awd1en())
            .field("jawd1en", &self.jawd1en())
            .field("jauto", &self.jauto())
            .field("awd1ch", &self.awd1ch())
            .field("jqdis", &self.jqdis())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Data management configuration
    #[inline(always)]
    pub fn dmngt(&mut self) -> DMNGT_W<'_, CFGR1rs> {
        DMNGT_W::new(self, 0)
    }
    ///Bits 2:3 - Data resolution
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGR1rs> {
        RES_W::new(self, 2)
    }
    ///Bits 5:9 - External trigger selection for regular group
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGR1rs> {
        EXTSEL_W::new(self, 5)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - Overrun mode
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - Single / Continuous conversion mode for regular conversions
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGR1rs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Delayed conversion mode
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<'_, CFGR1rs> {
        AUTDLY_W::new(self, 14)
    }
    ///Bit 16 - Discontinuous mode for regular channels
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bits 17:19 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<'_, CFGR1rs> {
        DISCNUM_W::new(self, 17)
    }
    ///Bit 20 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<'_, CFGR1rs> {
        JDISCEN_W::new(self, 20)
    }
    ///Bit 21 - JSQR queue mode
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W<'_, CFGR1rs> {
        JQM_W::new(self, 21)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, CFGR1rs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, CFGR1rs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<'_, CFGR1rs> {
        JAWD1EN_W::new(self, 24)
    }
    ///Bit 25 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<'_, CFGR1rs> {
        JAUTO_W::new(self, 25)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
    ///Bit 31 - Injected queue disable
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W<'_, CFGR1rs> {
        JQDIS_W::new(self, 31)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ADC1:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
