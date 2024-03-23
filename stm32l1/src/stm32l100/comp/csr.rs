#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `PU10K` reader - 10 kO pull-up resistor"]
pub type PU10K_R = crate::BitReader;
#[doc = "Field `PU10K` writer - 10 kO pull-up resistor"]
pub type PU10K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU400K` reader - 400 kO pull-up resistor"]
pub type PU400K_R = crate::BitReader;
#[doc = "Field `PU400K` writer - 400 kO pull-up resistor"]
pub type PU400K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10K` reader - 10 kO pull-down resistor"]
pub type PD10K_R = crate::BitReader;
#[doc = "Field `PD10K` writer - 10 kO pull-down resistor"]
pub type PD10K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD400K` reader - 400 kO pull-down resistor"]
pub type PD400K_R = crate::BitReader;
#[doc = "Field `PD400K` writer - 400 kO pull-down resistor"]
pub type PD400K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1EN` reader - Comparator 1 enable"]
pub type CMP1EN_R = crate::BitReader;
#[doc = "Field `CMP1EN` writer - Comparator 1 enable"]
pub type CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW1` reader - SW1 analog switch enable"]
pub type SW1_R = crate::BitReader;
#[doc = "Field `SW1` writer - SW1 analog switch enable"]
pub type SW1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1OUT` reader - Comparator 1 output"]
pub type CMP1OUT_R = crate::BitReader;
#[doc = "Field `SPEED` reader - Comparator 2 speed mode"]
pub type SPEED_R = crate::BitReader;
#[doc = "Field `SPEED` writer - Comparator 2 speed mode"]
pub type SPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2OUT` reader - Comparator 2 output"]
pub type CMP2OUT_R = crate::BitReader;
#[doc = "Field `VREFOUTEN` reader - VREFINT output enable"]
pub type VREFOUTEN_R = crate::BitReader;
#[doc = "Field `VREFOUTEN` writer - VREFINT output enable"]
pub type VREFOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WNDWE` reader - Window mode enable"]
pub type WNDWE_R = crate::BitReader;
#[doc = "Field `WNDWE` writer - Window mode enable"]
pub type WNDWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSEL` reader - Inverted input selection"]
pub type INSEL_R = crate::FieldReader;
#[doc = "Field `INSEL` writer - Inverted input selection"]
pub type INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUTSEL` reader - Comparator 2 output selection"]
pub type OUTSEL_R = crate::FieldReader;
#[doc = "Field `OUTSEL` writer - Comparator 2 output selection"]
pub type OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FCH3` reader - Select GPIO port PA3 as fast ADC input channel CH3."]
pub type FCH3_R = crate::BitReader;
#[doc = "Field `FCH3` writer - Select GPIO port PA3 as fast ADC input channel CH3."]
pub type FCH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCH8` reader - Select GPIO port PB0 as fast ADC input channel CH8."]
pub type FCH8_R = crate::BitReader;
#[doc = "Field `FCH8` writer - Select GPIO port PB0 as fast ADC input channel CH8."]
pub type FCH8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCH13` reader - Select GPIO port PC3 as re-routed ADC input channel CH13."]
pub type RCH13_R = crate::BitReader;
#[doc = "Field `RCH13` writer - Select GPIO port PC3 as re-routed ADC input channel CH13."]
pub type RCH13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAIE` reader - Channel Acquisition Interrupt Enable / Clear"]
pub type CAIE_R = crate::BitReader;
#[doc = "Field `CAIE` writer - Channel Acquisition Interrupt Enable / Clear"]
pub type CAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAIF` reader - Channel acquisition interrupt flag"]
pub type CAIF_R = crate::BitReader;
#[doc = "Field `TSUSP` reader - Suspend Timer Mode"]
pub type TSUSP_R = crate::BitReader;
#[doc = "Field `TSUSP` writer - Suspend Timer Mode"]
pub type TSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 10 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu10k(&self) -> PU10K_R {
        PU10K_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 400 kO pull-up resistor"]
    #[inline(always)]
    pub fn pu400k(&self) -> PU400K_R {
        PU400K_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 10 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd10k(&self) -> PD10K_R {
        PD10K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 400 kO pull-down resistor"]
    #[inline(always)]
    pub fn pd400k(&self) -> PD400K_R {
        PD400K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SW1 analog switch enable"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 output"]
    #[inline(always)]
    pub fn cmp1out(&self) -> CMP1OUT_R {
        CMP1OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator 2 speed mode"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator 2 output"]
    #[inline(always)]
    pub fn cmp2out(&self) -> CMP2OUT_R {
        CMP2OUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - VREFINT output enable"]
    #[inline(always)]
    pub fn vrefouten(&self) -> VREFOUTEN_R {
        VREFOUTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Window mode enable"]
    #[inline(always)]
    pub fn wndwe(&self) -> WNDWE_R {
        WNDWE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Inverted input selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3."]
    #[inline(always)]
    pub fn fch3(&self) -> FCH3_R {
        FCH3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8."]
    #[inline(always)]
    pub fn fch8(&self) -> FCH8_R {
        FCH8_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13."]
    #[inline(always)]
    pub fn rch13(&self) -> RCH13_R {
        RCH13_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel Acquisition Interrupt Enable / Clear"]
    #[inline(always)]
    pub fn caie(&self) -> CAIE_R {
        CAIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel acquisition interrupt flag"]
    #[inline(always)]
    pub fn caif(&self) -> CAIF_R {
        CAIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Suspend Timer Mode"]
    #[inline(always)]
    pub fn tsusp(&self) -> TSUSP_R {
        TSUSP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 10 kO pull-up resistor"]
    #[inline(always)]
    #[must_use]
    pub fn pu10k(&mut self) -> PU10K_W<CSRrs> {
        PU10K_W::new(self, 0)
    }
    #[doc = "Bit 1 - 400 kO pull-up resistor"]
    #[inline(always)]
    #[must_use]
    pub fn pu400k(&mut self) -> PU400K_W<CSRrs> {
        PU400K_W::new(self, 1)
    }
    #[doc = "Bit 2 - 10 kO pull-down resistor"]
    #[inline(always)]
    #[must_use]
    pub fn pd10k(&mut self) -> PD10K_W<CSRrs> {
        PD10K_W::new(self, 2)
    }
    #[doc = "Bit 3 - 400 kO pull-down resistor"]
    #[inline(always)]
    #[must_use]
    pub fn pd400k(&mut self) -> PD400K_W<CSRrs> {
        PD400K_W::new(self, 3)
    }
    #[doc = "Bit 4 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> CMP1EN_W<CSRrs> {
        CMP1EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - SW1 analog switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw1(&mut self) -> SW1_W<CSRrs> {
        SW1_W::new(self, 5)
    }
    #[doc = "Bit 12 - Comparator 2 speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<CSRrs> {
        SPEED_W::new(self, 12)
    }
    #[doc = "Bit 16 - VREFINT output enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefouten(&mut self) -> VREFOUTEN_W<CSRrs> {
        VREFOUTEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Window mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wndwe(&mut self) -> WNDWE_W<CSRrs> {
        WNDWE_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<CSRrs> {
        INSEL_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<CSRrs> {
        OUTSEL_W::new(self, 21)
    }
    #[doc = "Bit 26 - Select GPIO port PA3 as fast ADC input channel CH3."]
    #[inline(always)]
    #[must_use]
    pub fn fch3(&mut self) -> FCH3_W<CSRrs> {
        FCH3_W::new(self, 26)
    }
    #[doc = "Bit 27 - Select GPIO port PB0 as fast ADC input channel CH8."]
    #[inline(always)]
    #[must_use]
    pub fn fch8(&mut self) -> FCH8_W<CSRrs> {
        FCH8_W::new(self, 27)
    }
    #[doc = "Bit 28 - Select GPIO port PC3 as re-routed ADC input channel CH13."]
    #[inline(always)]
    #[must_use]
    pub fn rch13(&mut self) -> RCH13_W<CSRrs> {
        RCH13_W::new(self, 28)
    }
    #[doc = "Bit 29 - Channel Acquisition Interrupt Enable / Clear"]
    #[inline(always)]
    #[must_use]
    pub fn caie(&mut self) -> CAIE_W<CSRrs> {
        CAIE_W::new(self, 29)
    }
    #[doc = "Bit 31 - Suspend Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tsusp(&mut self) -> TSUSP_W<CSRrs> {
        TSUSP_W::new(self, 31)
    }
}
#[doc = "comparator control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
