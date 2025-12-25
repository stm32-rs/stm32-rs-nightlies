///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN1=11. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
pub type DMACFG_R = crate::BitReader;
///Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN1=11. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANDIR` reader - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type SCANDIR_R = crate::BitReader;
///Field `SCANDIR` writer - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion.
pub type RES_R = crate::FieldReader;
///Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion.
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure141: Data alignment and resolution (oversampling disabled: OVSE = 0) on page1332
pub type ALIGN_R = crate::BitReader;
///Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure141: Data alignment and resolution (oversampling disabled: OVSE = 0) on page1332
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTSEL` reader - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table160: External triggers for details):
pub type EXTSEL_R = crate::FieldReader;
///Field `EXTSEL` writer - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table160: External triggers for details):
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EXTEN` reader - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
pub type EXTEN_R = crate::FieldReader;
///Field `EXTEN` writer - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OVRMOD` reader - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
pub type OVRMOD_R = crate::BitReader;
///Field `OVRMOD` writer - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONT` reader - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
pub type CONT_R = crate::BitReader;
///Field `CONT` writer - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAIT` reader - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
pub type WAIT_R = crate::BitReader;
///Field `WAIT` writer - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOFF` reader - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
pub type AUTOFF_R = crate::BitReader;
///Field `AUTOFF` writer - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCEN` reader - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
pub type DISCEN_R = crate::BitReader;
///Field `DISCEN` writer - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSELRMOD` reader - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSELRMOD_R = crate::BitReader;
///Field `CHSELRMOD` writer - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSELRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1SGL` reader - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
pub type AWD1SGL_R = crate::BitReader;
///Field `AWD1SGL` writer - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1EN` reader - Analog watchdog enable This bit is set and cleared by software.
pub type AWD1EN_R = crate::BitReader;
///Field `AWD1EN` writer - Analog watchdog enable This bit is set and cleared by software.
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1CH` reader - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN1=11. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion.
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure141: Data alignment and resolution (oversampling disabled: OVSE = 0) on page1332
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table160: External triggers for details):
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("dmaen", &self.dmaen())
            .field("dmacfg", &self.dmacfg())
            .field("scandir", &self.scandir())
            .field("res", &self.res())
            .field("align", &self.align())
            .field("extsel", &self.extsel())
            .field("exten", &self.exten())
            .field("ovrmod", &self.ovrmod())
            .field("cont", &self.cont())
            .field("wait", &self.wait())
            .field("autoff", &self.autoff())
            .field("discen", &self.discen())
            .field("chselrmod", &self.chselrmod())
            .field("awd1sgl", &self.awd1sgl())
            .field("awd1en", &self.awd1en())
            .field("awd1ch", &self.awd1ch())
            .finish()
    }
}
impl W {
    ///Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CFGR1rs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN1=11. For more details, refer to Section113.6.5: Managing converted data using the DMA on page1333.
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CFGR1rs> {
        DMACFG_W::new(self, 1)
    }
    ///Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn scandir(&mut self) -> SCANDIR_W<'_, CFGR1rs> {
        SCANDIR_W::new(self, 2)
    }
    ///Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion.
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGR1rs> {
        RES_W::new(self, 3)
    }
    ///Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure141: Data alignment and resolution (oversampling disabled: OVSE = 0) on page1332
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CFGR1rs> {
        ALIGN_W::new(self, 5)
    }
    ///Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table160: External triggers for details):
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGR1rs> {
        EXTSEL_W::new(self, 6)
    }
    ///Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGR1rs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<'_, CFGR1rs> {
        WAIT_W::new(self, 14)
    }
    ///Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W<'_, CFGR1rs> {
        AUTOFF_W::new(self, 15)
    }
    ///Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN1=11 and CONT1=11.
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<'_, CFGR1rs> {
        CHSELRMOD_W::new(self, 21)
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, CFGR1rs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, CFGR1rs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
}
/**ADC configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#ADC:CFGR1)*/
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
