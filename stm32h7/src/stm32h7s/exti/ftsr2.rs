///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
///Field `TR34` reader - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR34_R = crate::BitReader;
///Field `TR34` writer - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR46` reader - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR46_R = crate::BitReader;
///Field `TR46` writer - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR49` reader - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR49_R = crate::BitReader;
///Field `TR49` writer - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR51` reader - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR51_R = crate::BitReader;
///Field `TR51` writer - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR54` reader - Falling trigger event configuration bit of configurable event input x+32.
pub type TR54_R = crate::BitReader;
///Field `TR54` writer - Falling trigger event configuration bit of configurable event input x+32.
pub type TR54_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr34(&self) -> TR34_R {
        TR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 14 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr46(&self) -> TR46_R {
        TR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr49(&self) -> TR49_R {
        TR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr51(&self) -> TR51_R {
        TR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Falling trigger event configuration bit of configurable event input x+32.
    #[inline(always)]
    pub fn tr54(&self) -> TR54_R {
        TR54_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("tr34", &self.tr34())
            .field("tr46", &self.tr46())
            .field("tr49", &self.tr49())
            .field("tr51", &self.tr51())
            .field("tr54", &self.tr54())
            .finish()
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr34(&mut self) -> TR34_W<'_, FTSR2rs> {
        TR34_W::new(self, 2)
    }
    ///Bit 14 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr46(&mut self) -> TR46_W<'_, FTSR2rs> {
        TR46_W::new(self, 14)
    }
    ///Bit 17 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr49(&mut self) -> TR49_W<'_, FTSR2rs> {
        TR49_W::new(self, 17)
    }
    ///Bit 19 - Falling trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr51(&mut self) -> TR51_W<'_, FTSR2rs> {
        TR51_W::new(self, 19)
    }
    ///Bit 22 - Falling trigger event configuration bit of configurable event input x+32.
    #[inline(always)]
    pub fn tr54(&mut self) -> TR54_W<'_, FTSR2rs> {
        TR54_W::new(self, 22)
    }
}
/**EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:FTSR2)*/
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr2::R`](R) reader structure
impl crate::Readable for FTSR2rs {}
///`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2rs {}
