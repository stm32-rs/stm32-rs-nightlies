#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\\[1:0\\]
of the ADC_CCR register."]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\\[1:0\\]
of the ADC_CCR register."]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register."]
pub type DMACFG_R = crate::BitReader;
#[doc = "Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register."]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type RES_R = crate::FieldReader;
#[doc = "Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTSEL0` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL0_R = crate::BitReader;
#[doc = "Field `EXTSEL0` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL1` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL1_R = crate::BitReader;
#[doc = "Field `EXTSEL1` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL2` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL2_R = crate::BitReader;
#[doc = "Field `EXTSEL2` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL3` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL3_R = crate::BitReader;
#[doc = "Field `EXTSEL3` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL4` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL4_R = crate::BitReader;
#[doc = "Field `EXTSEL4` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTSEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTEN_R = crate::FieldReader;
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type OVRMOD_R = crate::BitReader;
#[doc = "Field `OVRMOD` writer - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC."]
pub type CONT_R = crate::BitReader;
#[doc = "Field `CONT` writer - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC."]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTDLY` reader - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC."]
pub type AUTDLY_R = crate::BitReader;
#[doc = "Field `AUTDLY` writer - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC."]
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ALIGN_R = crate::BitReader;
#[doc = "Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC."]
pub type DISCEN_R = crate::BitReader;
#[doc = "Field `DISCEN` writer - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC."]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCNUM` reader - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\\[2:0\\]
of the slave ADC are no more writable and their content is equal to the bits DISCNUM\\[2:0\\]
of the master ADC."]
pub type DISCNUM_R = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\\[2:0\\]
of the slave ADC are no more writable and their content is equal to the bits DISCNUM\\[2:0\\]
of the master ADC."]
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JDISCEN` reader - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC."]
pub type JDISCEN_R = crate::BitReader;
#[doc = "Field `JDISCEN` writer - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC."]
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQM` reader - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC."]
pub type JQM_R = crate::BitReader;
#[doc = "Field `JQM` writer - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC."]
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1SGL` reader - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1SGL_R = crate::BitReader;
#[doc = "Field `AWD1SGL` writer - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type AWD1EN_R = crate::BitReader;
#[doc = "Field `AWD1EN` writer - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAWD1EN` reader - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JAWD1EN_R = crate::BitReader;
#[doc = "Field `JAWD1EN` writer - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC."]
pub type JAUTO_R = crate::BitReader;
#[doc = "Field `JAUTO` writer - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC."]
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1CH` reader - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\]
setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1CH_R = crate::FieldReader;
#[doc = "Field `AWD1CH` writer - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\]
setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JQDIS` reader - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
pub type JQDIS_R = crate::BitReader;
#[doc = "Field `JQDIS` writer - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\\[1:0\\]
of the ADC_CCR register."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register."]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel0(&self) -> EXTSEL0_R {
        EXTSEL0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel1(&self) -> EXTSEL1_R {
        EXTSEL1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel2(&self) -> EXTSEL2_R {
        EXTSEL2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel3(&self) -> EXTSEL3_R {
        EXTSEL3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel4(&self) -> EXTSEL4_R {
        EXTSEL4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC."]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC."]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\\[2:0\\]
of the slave ADC are no more writable and their content is equal to the bits DISCNUM\\[2:0\\]
of the master ADC."]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC."]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC."]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC."]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\]
setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows to use the DMA to manage automatically the converted data. For more details, refer to conversions using the DMA. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bits MDMA\\[1:0\\]
of the ADC_CCR register."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CFGRrs> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DMACFG of the ADC_CCR register."]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<CFGRrs> {
        DMACFG_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<CFGRrs> {
        RES_W::new(self, 3)
    }
    #[doc = "Bit 5 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel0(&mut self) -> EXTSEL0_W<CFGRrs> {
        EXTSEL0_W::new(self, 5)
    }
    #[doc = "Bit 6 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel1(&mut self) -> EXTSEL1_W<CFGRrs> {
        EXTSEL1_W::new(self, 6)
    }
    #[doc = "Bit 7 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel2(&mut self) -> EXTSEL2_W<CFGRrs> {
        EXTSEL2_W::new(self, 7)
    }
    #[doc = "Bit 8 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel3(&mut self) -> EXTSEL3_W<CFGRrs> {
        EXTSEL3_W::new(self, 8)
    }
    #[doc = "Bit 9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel4(&mut self) -> EXTSEL4_W<CFGRrs> {
        EXTSEL4_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Single / Continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CFGRrs> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC."]
    #[inline(always)]
    #[must_use]
    pub fn autdly(&mut self) -> AUTDLY_W<CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    #[doc = "Bit 15 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to register, data alignment and offset (ADC_DR, OFFSET, OFFSET_CH, ALIGN). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CFGRrs> {
        ALIGN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both Discontinuous mode and Continuous mode enabled: it is forbidden to set both DISCEN = 1 and CONT = 1. It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC."]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in Discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bits DISCNUM\\[2:0\\]
of the slave ADC are no more writable and their content is equal to the bits DISCNUM\\[2:0\\]
of the master ADC."]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    #[doc = "Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable Discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and Discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DUAL of ADC_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC."]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC."]
    #[inline(always)]
    #[must_use]
    pub fn jqm(&mut self) -> JQM_W<CFGRrs> {
        JQM_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DUAL bits in ADC_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC."]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: reserved, must not be used Note: Some channels are not connected physically. Keep the corresponding AWD1CH\\[4:0\\]
setting to the reset value. The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<CFGRrs> {
        AWD1CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism : Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn jqdis(&mut self) -> JQDIS_W<CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
#[doc = "ADC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
