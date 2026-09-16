///Register `SQR5` reader
pub type R = crate::R<SQR5rs>;
///Register `SQR5` writer
pub type W = crate::W<SQR5rs>;
///Field `SQ(1-6)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(1-6)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///(1-6) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ1` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(1-6) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..6).map(move |n| SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
    }
    ///Bits 0:4 - 1 conversion in regular sequence
    #[inline(always)]
    pub fn sq1(&self) -> SQ_R {
        SQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 2 conversion in regular sequence
    #[inline(always)]
    pub fn sq2(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 3 conversion in regular sequence
    #[inline(always)]
    pub fn sq3(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 4 conversion in regular sequence
    #[inline(always)]
    pub fn sq4(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:24 - 5 conversion in regular sequence
    #[inline(always)]
    pub fn sq5(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:29 - 6 conversion in regular sequence
    #[inline(always)]
    pub fn sq6(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR5")
            .field("sq1", &self.sq1())
            .field("sq2", &self.sq2())
            .field("sq3", &self.sq3())
            .field("sq4", &self.sq4())
            .field("sq5", &self.sq5())
            .field("sq6", &self.sq6())
            .finish()
    }
}
impl W {
    ///(1-6) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ1` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<'_, SQR5rs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        SQ_W::new(self, n * 5)
    }
    ///Bits 0:4 - 1 conversion in regular sequence
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ_W<'_, SQR5rs> {
        SQ_W::new(self, 0)
    }
    ///Bits 5:9 - 2 conversion in regular sequence
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ_W<'_, SQR5rs> {
        SQ_W::new(self, 5)
    }
    ///Bits 10:14 - 3 conversion in regular sequence
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ_W<'_, SQR5rs> {
        SQ_W::new(self, 10)
    }
    ///Bits 15:19 - 4 conversion in regular sequence
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ_W<'_, SQR5rs> {
        SQ_W::new(self, 15)
    }
    ///Bits 20:24 - 5 conversion in regular sequence
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ_W<'_, SQR5rs> {
        SQ_W::new(self, 20)
    }
    ///Bits 25:29 - 6 conversion in regular sequence
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ_W<'_, SQR5rs> {
        SQ_W::new(self, 25)
    }
}
/**regular sequence register 5

You can [`read`](crate::Reg::read) this register and get [`sqr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#ADC:SQR5)*/
pub struct SQR5rs;
impl crate::RegisterSpec for SQR5rs {
    type Ux = u32;
}
///`read()` method returns [`sqr5::R`](R) reader structure
impl crate::Readable for SQR5rs {}
///`write(|w| ..)` method takes [`sqr5::W`](W) writer structure
impl crate::Writable for SQR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR5 to value 0
impl crate::Resettable for SQR5rs {}
