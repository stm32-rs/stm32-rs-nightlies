///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
///Field `TR34` reader - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR34_R = crate::BitReader;
///Field `TR34` writer - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR46` reader - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR46_R = crate::BitReader;
///Field `TR46` writer - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR49` reader - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR49_R = crate::BitReader;
///Field `TR49` writer - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR51` reader - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR51_R = crate::BitReader;
///Field `TR51` writer - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
pub type TR51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR54` reader - Rising trigger event configuration bit of configurable event input x+32.
pub type TR54_R = crate::BitReader;
///Field `TR54` writer - Rising trigger event configuration bit of configurable event input x+32.
pub type TR54_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr34(&self) -> TR34_R {
        TR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr46(&self) -> TR46_R {
        TR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr49(&self) -> TR49_R {
        TR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr51(&self) -> TR51_R {
        TR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Rising trigger event configuration bit of configurable event input x+32.
    #[inline(always)]
    pub fn tr54(&self) -> TR54_R {
        TR54_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("tr34", &self.tr34())
            .field("tr46", &self.tr46())
            .field("tr49", &self.tr49())
            .field("tr51", &self.tr51())
            .field("tr54", &self.tr54())
            .finish()
    }
}
impl W {
    ///Bit 2 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr34(&mut self) -> TR34_W<'_, RTSR2rs> {
        TR34_W::new(self, 2)
    }
    ///Bit 14 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr46(&mut self) -> TR46_W<'_, RTSR2rs> {
        TR46_W::new(self, 14)
    }
    ///Bit 17 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr49(&mut self) -> TR49_W<'_, RTSR2rs> {
        TR49_W::new(self, 17)
    }
    ///Bit 19 - Rising trigger event configuration bit of configurable event input x+32.<sup>(1)</sup>
    #[inline(always)]
    pub fn tr51(&mut self) -> TR51_W<'_, RTSR2rs> {
        TR51_W::new(self, 19)
    }
    ///Bit 22 - Rising trigger event configuration bit of configurable event input x+32.
    #[inline(always)]
    pub fn tr54(&mut self) -> TR54_W<'_, RTSR2rs> {
        TR54_W::new(self, 22)
    }
}
/**EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:RTSR2)*/
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr2::R`](R) reader structure
impl crate::Readable for RTSR2rs {}
///`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2rs {}
