#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Field `HBITCLKDIV` reader - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk)."]
pub type HBITCLKDIV_R = crate::FieldReader;
#[doc = "Field `HBITCLKDIV` writer - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk)."]
pub type HBITCLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IFRGAP` reader - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal."]
pub type IFRGAP_R = crate::FieldReader;
#[doc = "Field `IFRGAP` writer - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal."]
pub type IFRGAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRANSWIN` reader - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\\[5:0\\]
bitfield setting."]
pub type TRANSWIN_R = crate::FieldReader;
#[doc = "Field `TRANSWIN` writer - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\\[5:0\\]
bitfield setting."]
pub type TRANSWIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSC_USBPDCLK` reader - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz."]
pub type PSC_USBPDCLK_R = crate::FieldReader;
#[doc = "Field `PSC_USBPDCLK` writer - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz."]
pub type PSC_USBPDCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXORDSETEN` reader - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled"]
pub type RXORDSETEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXORDSETEN` writer - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled"]
pub type RXORDSETEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXDMAEN` reader - Transmission DMA mode enable When set, the bit enables DMA mode for transmission."]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - Transmission DMA mode enable When set, the bit enables DMA mode for transmission."]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - Reception DMA mode enable When set, the bit enables DMA mode for reception."]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Reception DMA mode enable When set, the bit enables DMA mode for reception."]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPDEN` reader - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state."]
pub type UCPDEN_R = crate::BitReader;
#[doc = "Field `UCPDEN` writer - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state."]
pub type UCPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk)."]
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal."]
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\\[5:0\\]
bitfield setting."]
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz."]
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:28 - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled"]
    #[inline(always)]
    pub fn rxordseten(&self) -> RXORDSETEN_R {
        RXORDSETEN_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - Transmission DMA mode enable When set, the bit enables DMA mode for transmission."]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reception DMA mode enable When set, the bit enables DMA mode for reception."]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state."]
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk)."]
    #[inline(always)]
    #[must_use]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W<CFGR1rs> {
        HBITCLKDIV_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal."]
    #[inline(always)]
    #[must_use]
    pub fn ifrgap(&mut self) -> IFRGAP_W<CFGR1rs> {
        IFRGAP_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\\[5:0\\]
bitfield setting."]
    #[inline(always)]
    #[must_use]
    pub fn transwin(&mut self) -> TRANSWIN_W<CFGR1rs> {
        TRANSWIN_W::new(self, 11)
    }
    #[doc = "Bits 17:19 - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz."]
    #[inline(always)]
    #[must_use]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W<CFGR1rs> {
        PSC_USBPDCLK_W::new(self, 17)
    }
    #[doc = "Bits 20:28 - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxordseten(&mut self) -> RXORDSETEN_W<CFGR1rs> {
        RXORDSETEN_W::new(self, 20)
    }
    #[doc = "Bit 29 - Transmission DMA mode enable When set, the bit enables DMA mode for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFGR1rs> {
        TXDMAEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Reception DMA mode enable When set, the bit enables DMA mode for reception."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFGR1rs> {
        RXDMAEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state."]
    #[inline(always)]
    #[must_use]
    pub fn ucpden(&mut self) -> UCPDEN_W<CFGR1rs> {
        UCPDEN_W::new(self, 31)
    }
}
#[doc = "UCPD configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
