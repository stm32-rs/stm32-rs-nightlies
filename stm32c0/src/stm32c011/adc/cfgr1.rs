#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to page 355 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMACFG_R = crate::BitReader;
#[doc = "Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to page 355 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANDIR` reader - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type SCANDIR_R = crate::BitReader;
#[doc = "Field `SCANDIR` writer - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN = 0."]
pub type RES_R = crate::FieldReader;
#[doc = "Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN = 0."]
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page 353 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ALIGN_R = crate::BitReader;
#[doc = "Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page 353 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL` reader - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTSEL_R = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTEN_R = crate::FieldReader;
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRMOD_R = crate::BitReader;
#[doc = "Field `OVRMOD` writer - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CONT_R = crate::BitReader;
#[doc = "Field `CONT` writer - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT` reader - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WAIT_R = crate::BitReader;
#[doc = "Field `WAIT` writer - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOFF` reader - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AUTOFF_R = crate::BitReader;
#[doc = "Field `AUTOFF` writer - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DISCEN_R = crate::BitReader;
#[doc = "Field `DISCEN` writer - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSELRMOD` reader - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSELRMOD_R = crate::BitReader;
#[doc = "Field `CHSELRMOD` writer - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSELRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1SGL` reader - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1SGL_R = crate::BitReader;
#[doc = "Field `AWD1SGL` writer - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1EN_R = crate::BitReader;
#[doc = "Field `AWD1EN` writer - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1CH` reader - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1CH_R = crate::FieldReader;
#[doc = "Field `AWD1CH` writer - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to page 355 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN = 0."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page 353 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CFGR1rs> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to page 355 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<CFGR1rs> {
        DMACFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<CFGR1rs> {
        SCANDIR_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<CFGR1rs> {
        RES_W::new(self, 3)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page 353 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CFGR1rs> {
        ALIGN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<CFGR1rs> {
        EXTSEL_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CFGR1rs> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<CFGR1rs> {
        WAIT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<CFGR1rs> {
        AUTOFF_W::new(self, 15)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<CFGR1rs> {
        CHSELRMOD_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<CFGR1rs> {
        AWD1SGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<CFGR1rs> {
        AWD1EN_W::new(self, 23)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
