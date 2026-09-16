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
///Field `RXORDSETEN` reader - Receiver ordered set enable
pub type RXORDSETEN_R = crate::FieldReader<u16>;
///Field `RXORDSETEN` writer - Receiver ordered set enable
pub type RXORDSETEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
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
    ///Bits 20:28 - Receiver ordered set enable
    #[inline(always)]
    pub fn rxordseten(&self) -> RXORDSETEN_R {
        RXORDSETEN_R::new(((self.bits >> 20) & 0x01ff) as u16)
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
            .field("rxordseten", &self.rxordseten())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("ucpden", &self.ucpden())
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
    ///Bits 20:28 - Receiver ordered set enable
    #[inline(always)]
    pub fn rxordseten(&mut self) -> RXORDSETEN_W<'_, CFGR1rs> {
        RXORDSETEN_W::new(self, 20)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#UCPD:CFGR1)*/
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
