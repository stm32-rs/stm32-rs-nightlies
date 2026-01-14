///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to Section : Managing conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\[1:0\] of the ADC_CCR register.
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to Section : Managing conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\[1:0\] of the ADC_CCR register.
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section : Managing conversions using the DMA Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register.
pub type DMACFG_R = crate::BitReader;
///Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section : Managing conversions using the DMA Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register.
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADFCFG` reader - ADF mode configuration This bit is set and cleared by software to enable the ADF mode. It is effective only when DMAEN = 0. Note: To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0.
pub type ADFCFG_R = crate::BitReader;
///Field `ADFCFG` writer - ADF mode configuration This bit is set and cleared by software to enable the ADF mode. It is effective only when DMAEN = 0. Note: To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0.
pub type ADFCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type RES_R = crate::FieldReader;
///Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EXTSEL` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EXTSEL_R = crate::FieldReader;
///Field `EXTSEL` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EXTEN` reader - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EXTEN_R = crate::FieldReader;
///Field `EXTEN` writer - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OVRMOD` reader - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type OVRMOD_R = crate::BitReader;
///Field `OVRMOD` writer - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONT` reader - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
pub type CONT_R = crate::BitReader;
///Field `CONT` writer - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTDLY` reader - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.<sup>.</sup> Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
pub type AUTDLY_R = crate::BitReader;
///Field `AUTDLY` writer - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.<sup>.</sup> Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Section : Data register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type ALIGN_R = crate::BitReader;
///Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Section : Data register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCEN` reader - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
pub type DISCEN_R = crate::BitReader;
///Field `DISCEN` writer - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCNUM` reader - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
pub type DISCNUM_R = crate::FieldReader;
///Field `DISCNUM` writer - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `JDISCEN` reader - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
pub type JDISCEN_R = crate::BitReader;
///Field `JDISCEN` writer - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JQM` reader - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to Section 25.4.21: Queue of context for injected conversions for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
pub type JQM_R = crate::BitReader;
///Field `JQM` writer - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to Section 25.4.21: Queue of context for injected conversions for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1SGL` reader - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD1SGL_R = crate::BitReader;
///Field `AWD1SGL` writer - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1EN` reader - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type AWD1EN_R = crate::BitReader;
///Field `AWD1EN` writer - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JAWD1EN` reader - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JAWD1EN_R = crate::BitReader;
///Field `JAWD1EN` writer - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JAUTO` reader - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
pub type JAUTO_R = crate::BitReader;
///Field `JAUTO` writer - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1CH` reader - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\[4:0\] setting to the reset value. Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\[4:0\] setting to the reset value. Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JQDIS` reader - Injected queue disable This bit is set and cleared by software to disable the injected queue mechanism: Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
pub type JQDIS_R = crate::BitReader;
///Field `JQDIS` writer - Injected queue disable This bit is set and cleared by software to disable the injected queue mechanism: Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to Section : Managing conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\[1:0\] of the ADC_CCR register.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section : Managing conversions using the DMA Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register.
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADF mode configuration This bit is set and cleared by software to enable the ADF mode. It is effective only when DMAEN = 0. Note: To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0.
    #[inline(always)]
    pub fn adfcfg(&self) -> ADFCFG_R {
        ADFCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.<sup>.</sup> Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Section : Data register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to Section 25.4.21: Queue of context for injected conversions for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\[4:0\] setting to the reset value. Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Injected queue disable This bit is set and cleared by software to disable the injected queue mechanism: Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("dmaen", &self.dmaen())
            .field("dmacfg", &self.dmacfg())
            .field("adfcfg", &self.adfcfg())
            .field("res", &self.res())
            .field("extsel", &self.extsel())
            .field("exten", &self.exten())
            .field("ovrmod", &self.ovrmod())
            .field("cont", &self.cont())
            .field("autdly", &self.autdly())
            .field("align", &self.align())
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
    ///Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to Section : Managing conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\[1:0\] of the ADC_CCR register.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CFGRrs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section : Managing conversions using the DMA Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register.
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CFGRrs> {
        DMACFG_W::new(self, 1)
    }
    ///Bit 2 - ADF mode configuration This bit is set and cleared by software to enable the ADF mode. It is effective only when DMAEN = 0. Note: To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0.
    #[inline(always)]
    pub fn adfcfg(&mut self) -> ADFCFG_W<'_, CFGRrs> {
        ADFCFG_W::new(self, 2)
    }
    ///Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGRrs> {
        RES_W::new(self, 3)
    }
    ///Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGRrs> {
        EXTSEL_W::new(self, 5)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGRrs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.<sup>.</sup> Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<'_, CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    ///Bit 15 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Section : Data register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CFGRrs> {
        ALIGN_W::new(self, 15)
    }
    ///Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    ///Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<'_, CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    ///Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. Note: When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<'_, CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    ///Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to Section 25.4.21: Queue of context for injected conversions for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W<'_, CFGRrs> {
        JQM_W::new(self, 21)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<'_, CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    ///Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<'_, CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\[4:0\] setting to the reset value. Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, CFGRrs> {
        AWD1CH_W::new(self, 26)
    }
    ///Bit 31 - Injected queue disable This bit is set and cleared by software to disable the injected queue mechanism: Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). Note: A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W<'_, CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ADC1:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0x8000_0000
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
