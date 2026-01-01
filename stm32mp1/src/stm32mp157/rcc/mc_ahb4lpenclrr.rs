///Register `MC_AHB4LPENCLRR` reader
pub type R = crate::R<MC_AHB4LPENCLRRrs>;
///Register `MC_AHB4LPENCLRR` writer
pub type W = crate::W<MC_AHB4LPENCLRRrs>;
///Field `GPIOALPEN` reader - GPIOALPEN
pub type GPIOALPEN_R = crate::BitReader;
///Field `GPIOALPEN` writer - GPIOALPEN
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBLPEN` reader - GPIOBLPEN
pub type GPIOBLPEN_R = crate::BitReader;
///Field `GPIOBLPEN` writer - GPIOBLPEN
pub type GPIOBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCLPEN` reader - GPIOCLPEN
pub type GPIOCLPEN_R = crate::BitReader;
///Field `GPIOCLPEN` writer - GPIOCLPEN
pub type GPIOCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODLPEN` reader - GPIODLPEN
pub type GPIODLPEN_R = crate::BitReader;
///Field `GPIODLPEN` writer - GPIODLPEN
pub type GPIODLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOELPEN` reader - GPIOELPEN
pub type GPIOELPEN_R = crate::BitReader;
///Field `GPIOELPEN` writer - GPIOELPEN
pub type GPIOELPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFLPEN` reader - GPIOFLPEN
pub type GPIOFLPEN_R = crate::BitReader;
///Field `GPIOFLPEN` writer - GPIOFLPEN
pub type GPIOFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGLPEN` reader - GPIOGLPEN
pub type GPIOGLPEN_R = crate::BitReader;
///Field `GPIOGLPEN` writer - GPIOGLPEN
pub type GPIOGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHLPEN` reader - GPIOHLPEN
pub type GPIOHLPEN_R = crate::BitReader;
///Field `GPIOHLPEN` writer - GPIOHLPEN
pub type GPIOHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOILPEN` reader - GPIOILPEN
pub type GPIOILPEN_R = crate::BitReader;
///Field `GPIOILPEN` writer - GPIOILPEN
pub type GPIOILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOJLPEN` reader - GPIOJLPEN
pub type GPIOJLPEN_R = crate::BitReader;
///Field `GPIOJLPEN` writer - GPIOJLPEN
pub type GPIOJLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOKLPEN` reader - GPIOKLPEN
pub type GPIOKLPEN_R = crate::BitReader;
///Field `GPIOKLPEN` writer - GPIOKLPEN
pub type GPIOKLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOALPEN
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOBLPEN
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOCLPEN
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIODLPEN
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOELPEN
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOFLPEN
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOGLPEN
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOHLPEN
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOILPEN
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJLPEN
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOKLPEN
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AHB4LPENCLRR")
            .field("gpioalpen", &self.gpioalpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("gpioilpen", &self.gpioilpen())
            .field("gpiojlpen", &self.gpiojlpen())
            .field("gpioklpen", &self.gpioklpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOALPEN
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOBLPEN
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOCLPEN
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - GPIODLPEN
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOELPEN
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOFLPEN
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOGLPEN
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOHLPEN
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    ///Bit 8 - GPIOILPEN
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    ///Bit 9 - GPIOJLPEN
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOJLPEN_W::new(self, 9)
    }
    ///Bit 10 - GPIOKLPEN
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<'_, MC_AHB4LPENCLRRrs> {
        GPIOKLPEN_W::new(self, 10)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb4lpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb4lpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_AHB4LPENCLRR)*/
pub struct MC_AHB4LPENCLRRrs;
impl crate::RegisterSpec for MC_AHB4LPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_ahb4lpenclrr::R`](R) reader structure
impl crate::Readable for MC_AHB4LPENCLRRrs {}
///`write(|w| ..)` method takes [`mc_ahb4lpenclrr::W`](W) writer structure
impl crate::Writable for MC_AHB4LPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AHB4LPENCLRR to value 0x07ff
impl crate::Resettable for MC_AHB4LPENCLRRrs {
    const RESET_VALUE: u32 = 0x07ff;
}
