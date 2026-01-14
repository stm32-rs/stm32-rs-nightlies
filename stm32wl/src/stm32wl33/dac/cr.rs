///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BON` reader - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled
pub type BON_R = crate::BitReader;
///Field `BON` writer - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled
pub type BON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEN` reader - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle.
pub type TEN_R = crate::BitReader;
///Field `TEN` writer - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle.
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL` reader - TSEL\[2:0\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled).
pub type TSEL_R = crate::FieldReader;
///Field `TSEL` writer - TSEL\[2:0\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled).
pub type TSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WAVE` reader - WAVE\[1:0\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled).
pub type WAVE_R = crate::FieldReader;
///Field `WAVE` writer - WAVE\[1:0\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled).
pub type WAVE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MAMP` reader - MAMP\[3:0\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\[1:0\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\[2:0\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\[3:0\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\[4:0\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\[5:0\] of LFSR triangle amplitude equal to 63
pub type MAMP_R = crate::FieldReader;
///Field `MAMP` writer - MAMP\[3:0\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\[1:0\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\[2:0\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\[3:0\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\[4:0\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\[5:0\] of LFSR triangle amplitude equal to 63
pub type MAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DMAEN` reader - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAUDRIE` reader - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled
pub type DMAUDRIE_R = crate::BitReader;
///Field `DMAUDRIE` writer - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled
pub type DMAUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPEN` reader - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled
pub type CMPEN_R = crate::BitReader;
///Field `CMPEN` writer - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled
pub type CMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCMEN` reader - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled
pub type VCMEN_R = crate::BitReader;
///Field `VCMEN` writer - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled
pub type VCMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCMON` reader - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON
pub type VCMON_R = crate::BitReader;
///Field `VCMON` writer - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON
pub type VCMON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled
    #[inline(always)]
    pub fn bon(&self) -> BON_R {
        BON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle.
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - TSEL\[2:0\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled).
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - WAVE\[1:0\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled).
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - MAMP\[3:0\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\[1:0\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\[2:0\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\[3:0\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\[4:0\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\[5:0\] of LFSR triangle amplitude equal to 63
    #[inline(always)]
    pub fn mamp(&self) -> MAMP_R {
        MAMP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled
    #[inline(always)]
    pub fn dmaudrie(&self) -> DMAUDRIE_R {
        DMAUDRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled
    #[inline(always)]
    pub fn vcmen(&self) -> VCMEN_R {
        VCMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON
    #[inline(always)]
    pub fn vcmon(&self) -> VCMON_R {
        VCMON_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("bon", &self.bon())
            .field("ten", &self.ten())
            .field("tsel", &self.tsel())
            .field("wave", &self.wave())
            .field("mamp", &self.mamp())
            .field("dmaen", &self.dmaen())
            .field("dmaudrie", &self.dmaudrie())
            .field("cmpen", &self.cmpen())
            .field("vcmen", &self.vcmen())
            .field("vcmon", &self.vcmon())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled
    #[inline(always)]
    pub fn bon(&mut self) -> BON_W<'_, CRrs> {
        BON_W::new(self, 1)
    }
    ///Bit 2 - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle.
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W<'_, CRrs> {
        TEN_W::new(self, 2)
    }
    ///Bits 3:5 - TSEL\[2:0\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled).
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W<'_, CRrs> {
        TSEL_W::new(self, 3)
    }
    ///Bits 6:7 - WAVE\[1:0\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled).
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W<'_, CRrs> {
        WAVE_W::new(self, 6)
    }
    ///Bits 8:11 - MAMP\[3:0\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\[1:0\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\[2:0\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\[3:0\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\[4:0\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\[5:0\] of LFSR triangle amplitude equal to 63
    #[inline(always)]
    pub fn mamp(&mut self) -> MAMP_W<'_, CRrs> {
        MAMP_W::new(self, 8)
    }
    ///Bit 12 - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CRrs> {
        DMAEN_W::new(self, 12)
    }
    ///Bit 13 - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled
    #[inline(always)]
    pub fn dmaudrie(&mut self) -> DMAUDRIE_W<'_, CRrs> {
        DMAUDRIE_W::new(self, 13)
    }
    ///Bit 14 - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W<'_, CRrs> {
        CMPEN_W::new(self, 14)
    }
    ///Bit 15 - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled
    #[inline(always)]
    pub fn vcmen(&mut self) -> VCMEN_W<'_, CRrs> {
        VCMEN_W::new(self, 15)
    }
    ///Bit 16 - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON
    #[inline(always)]
    pub fn vcmon(&mut self) -> VCMON_W<'_, CRrs> {
        VCMON_W::new(self, 16)
    }
}
/**CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
