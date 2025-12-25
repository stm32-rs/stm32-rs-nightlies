///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `PU10K` reader - 10 kO pull-up resistor
pub type PU10K_R = crate::BitReader;
///Field `PU10K` writer - 10 kO pull-up resistor
pub type PU10K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU400K` reader - 400 kO pull-up resistor
pub type PU400K_R = crate::BitReader;
///Field `PU400K` writer - 400 kO pull-up resistor
pub type PU400K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD10K` reader - 10 kO pull-down resistor
pub type PD10K_R = crate::BitReader;
///Field `PD10K` writer - 10 kO pull-down resistor
pub type PD10K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD400K` reader - 400 kO pull-down resistor
pub type PD400K_R = crate::BitReader;
///Field `PD400K` writer - 400 kO pull-down resistor
pub type PD400K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP1EN` reader - Comparator 1 enable
pub type CMP1EN_R = crate::BitReader;
///Field `CMP1EN` writer - Comparator 1 enable
pub type CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW1` reader - SW1 analog switch enable
pub type SW1_R = crate::BitReader;
///Field `SW1` writer - SW1 analog switch enable
pub type SW1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP1OUT` reader - Comparator 1 output
pub type CMP1OUT_R = crate::BitReader;
///Field `SPEED` reader - Comparator 2 speed mode
pub type SPEED_R = crate::BitReader;
///Field `SPEED` writer - Comparator 2 speed mode
pub type SPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP2OUT` reader - Comparator 2 output
pub type CMP2OUT_R = crate::BitReader;
///Field `VREFOUTEN` reader - VREFINT output enable
pub type VREFOUTEN_R = crate::BitReader;
///Field `VREFOUTEN` writer - VREFINT output enable
pub type VREFOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WNDWE` reader - Window mode enable
pub type WNDWE_R = crate::BitReader;
///Field `WNDWE` writer - Window mode enable
pub type WNDWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INSEL` reader - Inverted input selection
pub type INSEL_R = crate::FieldReader;
///Field `INSEL` writer - Inverted input selection
pub type INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OUTSEL` reader - Comparator 2 output selection
pub type OUTSEL_R = crate::FieldReader;
///Field `OUTSEL` writer - Comparator 2 output selection
pub type OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FCH3` reader - Select GPIO port PA3 as fast ADC input channel CH3.
pub type FCH3_R = crate::BitReader;
///Field `FCH3` writer - Select GPIO port PA3 as fast ADC input channel CH3.
pub type FCH3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCH8` reader - Select GPIO port PB0 as fast ADC input channel CH8.
pub type FCH8_R = crate::BitReader;
///Field `FCH8` writer - Select GPIO port PB0 as fast ADC input channel CH8.
pub type FCH8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCH13` reader - Select GPIO port PC3 as re-routed ADC input channel CH13.
pub type RCH13_R = crate::BitReader;
///Field `RCH13` writer - Select GPIO port PC3 as re-routed ADC input channel CH13.
pub type RCH13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAIE` reader - Channel Acquisition Interrupt Enable / Clear
pub type CAIE_R = crate::BitReader;
///Field `CAIE` writer - Channel Acquisition Interrupt Enable / Clear
pub type CAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAIF` reader - Channel acquisition interrupt flag
pub type CAIF_R = crate::BitReader;
///Field `TSUSP` reader - Suspend Timer Mode
pub type TSUSP_R = crate::BitReader;
///Field `TSUSP` writer - Suspend Timer Mode
pub type TSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 10 kO pull-up resistor
    #[inline(always)]
    pub fn pu10k(&self) -> PU10K_R {
        PU10K_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 400 kO pull-up resistor
    #[inline(always)]
    pub fn pu400k(&self) -> PU400K_R {
        PU400K_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 10 kO pull-down resistor
    #[inline(always)]
    pub fn pd10k(&self) -> PD10K_R {
        PD10K_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 400 kO pull-down resistor
    #[inline(always)]
    pub fn pd400k(&self) -> PD400K_R {
        PD400K_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Comparator 1 enable
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SW1 analog switch enable
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Comparator 1 output
    #[inline(always)]
    pub fn cmp1out(&self) -> CMP1OUT_R {
        CMP1OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - Comparator 2 speed mode
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Comparator 2 output
    #[inline(always)]
    pub fn cmp2out(&self) -> CMP2OUT_R {
        CMP2OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - VREFINT output enable
    #[inline(always)]
    pub fn vrefouten(&self) -> VREFOUTEN_R {
        VREFOUTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Window mode enable
    #[inline(always)]
    pub fn wndwe(&self) -> WNDWE_R {
        WNDWE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:20 - Inverted input selection
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Comparator 2 output selection
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3.
    #[inline(always)]
    pub fn fch3(&self) -> FCH3_R {
        FCH3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8.
    #[inline(always)]
    pub fn fch8(&self) -> FCH8_R {
        FCH8_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13.
    #[inline(always)]
    pub fn rch13(&self) -> RCH13_R {
        RCH13_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Channel Acquisition Interrupt Enable / Clear
    #[inline(always)]
    pub fn caie(&self) -> CAIE_R {
        CAIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Channel acquisition interrupt flag
    #[inline(always)]
    pub fn caif(&self) -> CAIF_R {
        CAIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Suspend Timer Mode
    #[inline(always)]
    pub fn tsusp(&self) -> TSUSP_R {
        TSUSP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("tsusp", &self.tsusp())
            .field("caif", &self.caif())
            .field("caie", &self.caie())
            .field("rch13", &self.rch13())
            .field("fch8", &self.fch8())
            .field("fch3", &self.fch3())
            .field("outsel", &self.outsel())
            .field("insel", &self.insel())
            .field("wndwe", &self.wndwe())
            .field("vrefouten", &self.vrefouten())
            .field("cmp2out", &self.cmp2out())
            .field("speed", &self.speed())
            .field("cmp1out", &self.cmp1out())
            .field("sw1", &self.sw1())
            .field("cmp1en", &self.cmp1en())
            .field("pd400k", &self.pd400k())
            .field("pd10k", &self.pd10k())
            .field("pu400k", &self.pu400k())
            .field("pu10k", &self.pu10k())
            .finish()
    }
}
impl W {
    ///Bit 0 - 10 kO pull-up resistor
    #[inline(always)]
    pub fn pu10k(&mut self) -> PU10K_W<'_, CSRrs> {
        PU10K_W::new(self, 0)
    }
    ///Bit 1 - 400 kO pull-up resistor
    #[inline(always)]
    pub fn pu400k(&mut self) -> PU400K_W<'_, CSRrs> {
        PU400K_W::new(self, 1)
    }
    ///Bit 2 - 10 kO pull-down resistor
    #[inline(always)]
    pub fn pd10k(&mut self) -> PD10K_W<'_, CSRrs> {
        PD10K_W::new(self, 2)
    }
    ///Bit 3 - 400 kO pull-down resistor
    #[inline(always)]
    pub fn pd400k(&mut self) -> PD400K_W<'_, CSRrs> {
        PD400K_W::new(self, 3)
    }
    ///Bit 4 - Comparator 1 enable
    #[inline(always)]
    pub fn cmp1en(&mut self) -> CMP1EN_W<'_, CSRrs> {
        CMP1EN_W::new(self, 4)
    }
    ///Bit 5 - SW1 analog switch enable
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W<'_, CSRrs> {
        SW1_W::new(self, 5)
    }
    ///Bit 12 - Comparator 2 speed mode
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W<'_, CSRrs> {
        SPEED_W::new(self, 12)
    }
    ///Bit 16 - VREFINT output enable
    #[inline(always)]
    pub fn vrefouten(&mut self) -> VREFOUTEN_W<'_, CSRrs> {
        VREFOUTEN_W::new(self, 16)
    }
    ///Bit 17 - Window mode enable
    #[inline(always)]
    pub fn wndwe(&mut self) -> WNDWE_W<'_, CSRrs> {
        WNDWE_W::new(self, 17)
    }
    ///Bits 18:20 - Inverted input selection
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W<'_, CSRrs> {
        INSEL_W::new(self, 18)
    }
    ///Bits 21:23 - Comparator 2 output selection
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W<'_, CSRrs> {
        OUTSEL_W::new(self, 21)
    }
    ///Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3.
    #[inline(always)]
    pub fn fch3(&mut self) -> FCH3_W<'_, CSRrs> {
        FCH3_W::new(self, 26)
    }
    ///Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8.
    #[inline(always)]
    pub fn fch8(&mut self) -> FCH8_W<'_, CSRrs> {
        FCH8_W::new(self, 27)
    }
    ///Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13.
    #[inline(always)]
    pub fn rch13(&mut self) -> RCH13_W<'_, CSRrs> {
        RCH13_W::new(self, 28)
    }
    ///Bit 29 - Channel Acquisition Interrupt Enable / Clear
    #[inline(always)]
    pub fn caie(&mut self) -> CAIE_W<'_, CSRrs> {
        CAIE_W::new(self, 29)
    }
    ///Bit 31 - Suspend Timer Mode
    #[inline(always)]
    pub fn tsusp(&mut self) -> TSUSP_W<'_, CSRrs> {
        TSUSP_W::new(self, 31)
    }
}
/**comparator control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#COMP:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
