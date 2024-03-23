#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `DUAL` reader - Dual ADC mode selection These bits are written by software to select the operating mode. 0 value means Independent Mode. Values 00001 to 01001 means Dual mode, master and slave ADCs are working together. All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DUAL_R = crate::FieldReader;
#[doc = "Field `DUAL` writer - Dual ADC mode selection These bits are written by software to select the operating mode. 0 value means Independent Mode. Values 00001 to 01001 means Dual mode, master and slave ADCs are working together. All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMACFG` reader - DMA configuration (for dual ADC mode) This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DMACFG_R = crate::BitReader;
#[doc = "Field `DMACFG` writer - DMA configuration (for dual ADC mode) This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA` reader - Direct memory access mode for dual ADC mode This bitfield is set and cleared by software. Refer to the DMA controller section for more details. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type MDMA_R = crate::FieldReader;
#[doc = "Field `MDMA` writer - Direct memory access mode for dual ADC mode This bitfield is set and cleared by software. Refer to the DMA controller section for more details. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type MDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_R = crate::FieldReader;
#[doc = "Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESC` reader - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - VSENSE enable This bit is set and cleared by software to control VSENSE."]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - VSENSE enable This bit is set and cleared by software to control VSENSE."]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to control."]
pub type VBATEN_R = crate::BitReader;
#[doc = "Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to control."]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection These bits are written by software to select the operating mode. 0 value means Independent Mode. Values 00001 to 01001 means Dual mode, master and slave ADCs are working together. All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA configuration (for dual ADC mode) This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for dual ADC mode This bitfield is set and cleared by software. Refer to the DMA controller section for more details. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VSENSE enable This bit is set and cleared by software to control VSENSE."]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control."]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection These bits are written by software to select the operating mode. 0 value means Independent Mode. Values 00001 to 01001 means Dual mode, master and slave ADCs are working together. All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<CCRrs> {
        DUAL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<CCRrs> {
        DELAY_W::new(self, 8)
    }
    #[doc = "Bit 13 - DMA configuration (for dual ADC mode) This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<CCRrs> {
        DMACFG_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for dual ADC mode This bitfield is set and cleared by software. Refer to the DMA controller section for more details. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn mdma(&mut self) -> MDMA_W<CCRrs> {
        MDMA_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CCRrs> {
        CKMODE_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - VSENSE enable This bit is set and cleared by software to control VSENSE."]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CCRrs> {
        TSEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control."]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
