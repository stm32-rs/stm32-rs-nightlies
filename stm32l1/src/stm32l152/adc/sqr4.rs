///Register `SQR4` reader
pub type R = crate::R<SQR4rs>;
///Register `SQR4` writer
pub type W = crate::W<SQR4rs>;
///Field `SQ(7-12)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(7-12)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///(7-12) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ7` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(7-12) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..6).map(move |n| SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
    }
    ///Bits 0:4 - 7 conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&self) -> SQ_R {
        SQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 8 conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 9 conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 10 conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:24 - 11 conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:29 - 12 conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR4")
            .field("sq7", &self.sq7())
            .field("sq8", &self.sq8())
            .field("sq9", &self.sq9())
            .field("sq10", &self.sq10())
            .field("sq11", &self.sq11())
            .field("sq12", &self.sq12())
            .finish()
    }
}
impl W {
    ///(7-12) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ7` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<SQR4rs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        SQ_W::new(self, n * 5)
    }
    ///Bits 0:4 - 7 conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 0)
    }
    ///Bits 5:9 - 8 conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 5)
    }
    ///Bits 10:14 - 9 conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 10)
    }
    ///Bits 15:19 - 10 conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 15)
    }
    ///Bits 20:24 - 11 conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 20)
    }
    ///Bits 25:29 - 12 conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ_W<SQR4rs> {
        SQ_W::new(self, 25)
    }
}
/**regular sequence register 4

You can [`read`](crate::Reg::read) this register and get [`sqr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#ADC:SQR4)*/
pub struct SQR4rs;
impl crate::RegisterSpec for SQR4rs {
    type Ux = u32;
}
///`read()` method returns [`sqr4::R`](R) reader structure
impl crate::Readable for SQR4rs {}
///`write(|w| ..)` method takes [`sqr4::W`](W) writer structure
impl crate::Writable for SQR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SQR4 to value 0
impl crate::Resettable for SQR4rs {
    const RESET_VALUE: u32 = 0;
}