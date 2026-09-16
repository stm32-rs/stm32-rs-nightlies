///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader;
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRST` reader - IO port B reset
pub type GPIOBRST_R = crate::BitReader;
///Field `GPIOBRST` writer - IO port B reset
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCRST` reader - IO port C reset
pub type GPIOCRST_R = crate::BitReader;
///Field `GPIOCRST` writer - IO port C reset
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODRST` reader - IO port D reset
pub type GPIODRST_R = crate::BitReader;
///Field `GPIODRST` writer - IO port D reset
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOERST` reader - IO port E reset
pub type GPIOERST_R = crate::BitReader;
///Field `GPIOERST` writer - IO port E reset
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFRST` reader - IO port F reset
pub type GPIOFRST_R = crate::BitReader;
///Field `GPIOFRST` writer - IO port F reset
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGRST` reader - IO port G reset
pub type GPIOGRST_R = crate::BitReader;
///Field `GPIOGRST` writer - IO port G reset
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHRST` reader - IO port H reset
pub type GPIOHRST_R = crate::BitReader;
///Field `GPIOHRST` writer - IO port H reset
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGFSRST` reader - USB OTG FS reset
pub type OTGFSRST_R = crate::BitReader;
///Field `OTGFSRST` writer - USB OTG FS reset
pub type OTGFSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCRST` reader - ADC reset
pub type ADCRST_R = crate::BitReader;
///Field `ADCRST` writer - ADC reset
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESRST` reader - AES hardware accelerator reset
pub type AESRST_R = crate::BitReader;
///Field `AESRST` writer - AES hardware accelerator reset
pub type AESRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGRST` reader - Random number generator reset
pub type RNGRST_R = crate::BitReader;
///Field `RNGRST` writer - Random number generator reset
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("rngrst", &self.rngrst())
            .field("aesrst", &self.aesrst())
            .field("adcrst", &self.adcrst())
            .field("otgfsrst", &self.otgfsrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpioarst", &self.gpioarst())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB2RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB2RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB2RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB2RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB2RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB2RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB2RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB2RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHB2RSTRrs> {
        OTGFSRST_W::new(self, 12)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, AHB2RSTRrs> {
        ADCRST_W::new(self, 13)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W<'_, AHB2RSTRrs> {
        AESRST_W::new(self, 16)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 18)
    }
}
/**AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
