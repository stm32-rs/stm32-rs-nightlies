///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
///Register `CIFR` writer
pub type W = crate::W<CIFRrs>;
///Field `LSIRDYIF` reader - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator
pub type LSIRDYIF_R = crate::BitReader;
///Field `LSIRDYIF` writer - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator
pub type LSIRDYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIF` reader - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator
pub type LSERDYIF_R = crate::BitReader;
///Field `LSERDYIF` writer - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator
pub type LSERDYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIF` reader - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator
pub type HSIRDYIF_R = crate::BitReader;
///Field `HSIRDYIF` writer - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator
pub type HSIRDYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIF` reader - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator
pub type HSERDYIF_R = crate::BitReader;
///Field `HSERDYIF` writer - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator
pub type HSERDYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPLLRDYIF` reader - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator
pub type HSIPLLRDYIF_R = crate::BitReader;
///Field `HSIPLLRDYIF` writer - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator
pub type HSIPLLRDYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPLLUNLOCKDETIF` reader - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag.
pub type HSIPLLUNLOCKDETIF_R = crate::BitReader;
///Field `HSIPLLUNLOCKDETIF` writer - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag.
pub type HSIPLLUNLOCKDETIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRSTIF` reader - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type RTCRSTIF_R = crate::BitReader;
///Field `RTCRSTIF` writer - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type RTCRSTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGRSTIF` reader - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type WDGRSTIF_R = crate::BitReader;
///Field `WDGRSTIF` writer - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type WDGRSTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPURSTIF` reader - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type LPURSTIF_R = crate::BitReader;
///Field `LPURSTIF` writer - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type LPURSTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDRSTIF` reader - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type LCDRSTIF_R = crate::BitReader;
///Field `LCDRSTIF` writer - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type LCDRSTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCSCRSTIF` reader - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type LCSCRSTIF_R = crate::BitReader;
///Field `LCSCRSTIF` writer - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
pub type LCSCRSTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator
    #[inline(always)]
    pub fn lsirdyif(&self) -> LSIRDYIF_R {
        LSIRDYIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator
    #[inline(always)]
    pub fn lserdyif(&self) -> LSERDYIF_R {
        LSERDYIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator
    #[inline(always)]
    pub fn hsirdyif(&self) -> HSIRDYIF_R {
        HSIRDYIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator
    #[inline(always)]
    pub fn hserdyif(&self) -> HSERDYIF_R {
        HSERDYIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator
    #[inline(always)]
    pub fn hsipllrdyif(&self) -> HSIPLLRDYIF_R {
        HSIPLLRDYIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag.
    #[inline(always)]
    pub fn hsipllunlockdetif(&self) -> HSIPLLUNLOCKDETIF_R {
        HSIPLLUNLOCKDETIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn rtcrstif(&self) -> RTCRSTIF_R {
        RTCRSTIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn wdgrstif(&self) -> WDGRSTIF_R {
        WDGRSTIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn lpurstif(&self) -> LPURSTIF_R {
        LPURSTIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn lcdrstif(&self) -> LCDRSTIF_R {
        LCDRSTIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn lcscrstif(&self) -> LCSCRSTIF_R {
        LCSCRSTIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("lsirdyif", &self.lsirdyif())
            .field("lserdyif", &self.lserdyif())
            .field("hsirdyif", &self.hsirdyif())
            .field("hserdyif", &self.hserdyif())
            .field("hsipllrdyif", &self.hsipllrdyif())
            .field("hsipllunlockdetif", &self.hsipllunlockdetif())
            .field("rtcrstif", &self.rtcrstif())
            .field("wdgrstif", &self.wdgrstif())
            .field("lpurstif", &self.lpurstif())
            .field("lcdrstif", &self.lcdrstif())
            .field("lcscrstif", &self.lcscrstif())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator
    #[inline(always)]
    pub fn lsirdyif(&mut self) -> LSIRDYIF_W<'_, CIFRrs> {
        LSIRDYIF_W::new(self, 0)
    }
    ///Bit 1 - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator
    #[inline(always)]
    pub fn lserdyif(&mut self) -> LSERDYIF_W<'_, CIFRrs> {
        LSERDYIF_W::new(self, 1)
    }
    ///Bit 3 - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator
    #[inline(always)]
    pub fn hsirdyif(&mut self) -> HSIRDYIF_W<'_, CIFRrs> {
        HSIRDYIF_W::new(self, 3)
    }
    ///Bit 4 - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator
    #[inline(always)]
    pub fn hserdyif(&mut self) -> HSERDYIF_W<'_, CIFRrs> {
        HSERDYIF_W::new(self, 4)
    }
    ///Bit 5 - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator
    #[inline(always)]
    pub fn hsipllrdyif(&mut self) -> HSIPLLRDYIF_W<'_, CIFRrs> {
        HSIPLLRDYIF_W::new(self, 5)
    }
    ///Bit 6 - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag.
    #[inline(always)]
    pub fn hsipllunlockdetif(&mut self) -> HSIPLLUNLOCKDETIF_W<'_, CIFRrs> {
        HSIPLLUNLOCKDETIF_W::new(self, 6)
    }
    ///Bit 7 - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn rtcrstif(&mut self) -> RTCRSTIF_W<'_, CIFRrs> {
        RTCRSTIF_W::new(self, 7)
    }
    ///Bit 8 - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn wdgrstif(&mut self) -> WDGRSTIF_W<'_, CIFRrs> {
        WDGRSTIF_W::new(self, 8)
    }
    ///Bit 9 - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn lpurstif(&mut self) -> LPURSTIF_W<'_, CIFRrs> {
        LPURSTIF_W::new(self, 9)
    }
    ///Bit 10 - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn lcdrstif(&mut self) -> LCDRSTIF_W<'_, CIFRrs> {
        LCDRSTIF_W::new(self, 10)
    }
    ///Bit 13 - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock
    #[inline(always)]
    pub fn lcscrstif(&mut self) -> LCSCRSTIF_W<'_, CIFRrs> {
        LCSCRSTIF_W::new(self, 13)
    }
}
/**CIFR register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`write(|w| ..)` method takes [`cifr::W`](W) writer structure
impl crate::Writable for CIFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIFR to value 0x08
impl crate::Resettable for CIFRrs {
    const RESET_VALUE: u32 = 0x08;
}
