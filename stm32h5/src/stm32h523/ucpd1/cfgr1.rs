///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `HBITCLKDIV` reader - Division ratio for producing half-bit clock
pub type HBITCLKDIV_R = crate::FieldReader;
///Field `HBITCLKDIV` writer - Division ratio for producing half-bit clock
pub type HBITCLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `IFRGAP` reader - Division ratio for producing inter-frame gap timer clock
pub type IFRGAP_R = crate::FieldReader;
///Field `IFRGAP` writer - Division ratio for producing inter-frame gap timer clock
pub type IFRGAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRANSWIN` reader - Transition window duration
pub type TRANSWIN_R = crate::FieldReader;
///Field `TRANSWIN` writer - Transition window duration
pub type TRANSWIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PSC_USBPDCLK` reader - Pre-scaler division ratio for generating ucpd_clk
pub type PSC_USBPDCLK_R = crate::FieldReader;
///Field `PSC_USBPDCLK` writer - Pre-scaler division ratio for generating ucpd_clk
pub type PSC_USBPDCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXORDSETEN0` reader - SOP detection
pub type RXORDSETEN0_R = crate::BitReader;
///Field `RXORDSETEN0` writer - SOP detection
pub type RXORDSETEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN1` reader - SOP' detection
pub type RXORDSETEN1_R = crate::BitReader;
///Field `RXORDSETEN1` writer - SOP' detection
pub type RXORDSETEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN2` reader - SOP'' detection
pub type RXORDSETEN2_R = crate::BitReader;
///Field `RXORDSETEN2` writer - SOP'' detection
pub type RXORDSETEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN3` reader - Hard Reset detection
pub type RXORDSETEN3_R = crate::BitReader;
///Field `RXORDSETEN3` writer - Hard Reset detection
pub type RXORDSETEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN4` reader - Cable Detect reset
pub type RXORDSETEN4_R = crate::BitReader;
///Field `RXORDSETEN4` writer - Cable Detect reset
pub type RXORDSETEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN5` reader - SOP'_Debug
pub type RXORDSETEN5_R = crate::BitReader;
///Field `RXORDSETEN5` writer - SOP'_Debug
pub type RXORDSETEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN6` reader - SOP'' Debug
pub type RXORDSETEN6_R = crate::BitReader;
///Field `RXORDSETEN6` writer - SOP'' Debug
pub type RXORDSETEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN7` reader - SOP extension #1
pub type RXORDSETEN7_R = crate::BitReader;
///Field `RXORDSETEN7` writer - SOP extension #1
pub type RXORDSETEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDSETEN8` reader - SOP extension #2
pub type RXORDSETEN8_R = crate::BitReader;
///Field `RXORDSETEN8` writer - SOP extension #2
pub type RXORDSETEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - Transmission DMA mode enable
pub type TXDMAEN_R = crate::BitReader;
///Field `TXDMAEN` writer - Transmission DMA mode enable
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDMAEN` reader - Reception DMA mode enable
pub type RXDMAEN_R = crate::BitReader;
///Field `RXDMAEN` writer - Reception DMA mode enable
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPDEN` reader - UCPD peripheral enable
pub type UCPDEN_R = crate::BitReader;
///Field `UCPDEN` writer - UCPD peripheral enable
pub type UCPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Division ratio for producing half-bit clock
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:10 - Division ratio for producing inter-frame gap timer clock
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - Transition window duration
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 17:19 - Pre-scaler division ratio for generating ucpd_clk
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - SOP detection
    #[inline(always)]
    pub fn rxordseten0(&self) -> RXORDSETEN0_R {
        RXORDSETEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SOP' detection
    #[inline(always)]
    pub fn rxordseten1(&self) -> RXORDSETEN1_R {
        RXORDSETEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SOP'' detection
    #[inline(always)]
    pub fn rxordseten2(&self) -> RXORDSETEN2_R {
        RXORDSETEN2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Hard Reset detection
    #[inline(always)]
    pub fn rxordseten3(&self) -> RXORDSETEN3_R {
        RXORDSETEN3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Cable Detect reset
    #[inline(always)]
    pub fn rxordseten4(&self) -> RXORDSETEN4_R {
        RXORDSETEN4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SOP'_Debug
    #[inline(always)]
    pub fn rxordseten5(&self) -> RXORDSETEN5_R {
        RXORDSETEN5_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SOP'' Debug
    #[inline(always)]
    pub fn rxordseten6(&self) -> RXORDSETEN6_R {
        RXORDSETEN6_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SOP extension #1
    #[inline(always)]
    pub fn rxordseten7(&self) -> RXORDSETEN7_R {
        RXORDSETEN7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SOP extension #2
    #[inline(always)]
    pub fn rxordseten8(&self) -> RXORDSETEN8_R {
        RXORDSETEN8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Transmission DMA mode enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Reception DMA mode enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UCPD peripheral enable
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("hbitclkdiv", &self.hbitclkdiv())
            .field("ifrgap", &self.ifrgap())
            .field("transwin", &self.transwin())
            .field("psc_usbpdclk", &self.psc_usbpdclk())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("ucpden", &self.ucpden())
            .field("rxordseten0", &self.rxordseten0())
            .field("rxordseten1", &self.rxordseten1())
            .field("rxordseten2", &self.rxordseten2())
            .field("rxordseten3", &self.rxordseten3())
            .field("rxordseten4", &self.rxordseten4())
            .field("rxordseten5", &self.rxordseten5())
            .field("rxordseten6", &self.rxordseten6())
            .field("rxordseten7", &self.rxordseten7())
            .field("rxordseten8", &self.rxordseten8())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Division ratio for producing half-bit clock
    #[inline(always)]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W<'_, CFGR1rs> {
        HBITCLKDIV_W::new(self, 0)
    }
    ///Bits 6:10 - Division ratio for producing inter-frame gap timer clock
    #[inline(always)]
    pub fn ifrgap(&mut self) -> IFRGAP_W<'_, CFGR1rs> {
        IFRGAP_W::new(self, 6)
    }
    ///Bits 11:15 - Transition window duration
    #[inline(always)]
    pub fn transwin(&mut self) -> TRANSWIN_W<'_, CFGR1rs> {
        TRANSWIN_W::new(self, 11)
    }
    ///Bits 17:19 - Pre-scaler division ratio for generating ucpd_clk
    #[inline(always)]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W<'_, CFGR1rs> {
        PSC_USBPDCLK_W::new(self, 17)
    }
    ///Bit 20 - SOP detection
    #[inline(always)]
    pub fn rxordseten0(&mut self) -> RXORDSETEN0_W<'_, CFGR1rs> {
        RXORDSETEN0_W::new(self, 20)
    }
    ///Bit 21 - SOP' detection
    #[inline(always)]
    pub fn rxordseten1(&mut self) -> RXORDSETEN1_W<'_, CFGR1rs> {
        RXORDSETEN1_W::new(self, 21)
    }
    ///Bit 22 - SOP'' detection
    #[inline(always)]
    pub fn rxordseten2(&mut self) -> RXORDSETEN2_W<'_, CFGR1rs> {
        RXORDSETEN2_W::new(self, 22)
    }
    ///Bit 23 - Hard Reset detection
    #[inline(always)]
    pub fn rxordseten3(&mut self) -> RXORDSETEN3_W<'_, CFGR1rs> {
        RXORDSETEN3_W::new(self, 23)
    }
    ///Bit 24 - Cable Detect reset
    #[inline(always)]
    pub fn rxordseten4(&mut self) -> RXORDSETEN4_W<'_, CFGR1rs> {
        RXORDSETEN4_W::new(self, 24)
    }
    ///Bit 25 - SOP'_Debug
    #[inline(always)]
    pub fn rxordseten5(&mut self) -> RXORDSETEN5_W<'_, CFGR1rs> {
        RXORDSETEN5_W::new(self, 25)
    }
    ///Bit 26 - SOP'' Debug
    #[inline(always)]
    pub fn rxordseten6(&mut self) -> RXORDSETEN6_W<'_, CFGR1rs> {
        RXORDSETEN6_W::new(self, 26)
    }
    ///Bit 27 - SOP extension #1
    #[inline(always)]
    pub fn rxordseten7(&mut self) -> RXORDSETEN7_W<'_, CFGR1rs> {
        RXORDSETEN7_W::new(self, 27)
    }
    ///Bit 28 - SOP extension #2
    #[inline(always)]
    pub fn rxordseten8(&mut self) -> RXORDSETEN8_W<'_, CFGR1rs> {
        RXORDSETEN8_W::new(self, 28)
    }
    ///Bit 29 - Transmission DMA mode enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, CFGR1rs> {
        TXDMAEN_W::new(self, 29)
    }
    ///Bit 30 - Reception DMA mode enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CFGR1rs> {
        RXDMAEN_W::new(self, 30)
    }
    ///Bit 31 - UCPD peripheral enable
    #[inline(always)]
    pub fn ucpden(&mut self) -> UCPDEN_W<'_, CFGR1rs> {
        UCPDEN_W::new(self, 31)
    }
}
/**UCPD configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#UCPD1:CFGR1)*/
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
