///Register `SQR3` reader
pub type R = crate::R<SQR3rs>;
///Register `SQR3` writer
pub type W = crate::W<SQR3rs>;
///Field `SQ(13-18)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(13-18)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///(13-18) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ13` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(13-18) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..6).map(move |n| SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
    }
    ///Bits 0:4 - 13 conversion in regular sequence
    #[inline(always)]
    pub fn sq13(&self) -> SQ_R {
        SQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 14 conversion in regular sequence
    #[inline(always)]
    pub fn sq14(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 15 conversion in regular sequence
    #[inline(always)]
    pub fn sq15(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 16 conversion in regular sequence
    #[inline(always)]
    pub fn sq16(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:24 - 17 conversion in regular sequence
    #[inline(always)]
    pub fn sq17(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:29 - 18 conversion in regular sequence
    #[inline(always)]
    pub fn sq18(&self) -> SQ_R {
        SQ_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR3")
            .field("sq13", &self.sq13())
            .field("sq14", &self.sq14())
            .field("sq15", &self.sq15())
            .field("sq16", &self.sq16())
            .field("sq17", &self.sq17())
            .field("sq18", &self.sq18())
            .finish()
    }
}
impl W {
    ///(13-18) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ13` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<SQR3rs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        SQ_W::new(self, n * 5)
    }
    ///Bits 0:4 - 13 conversion in regular sequence
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 0)
    }
    ///Bits 5:9 - 14 conversion in regular sequence
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 5)
    }
    ///Bits 10:14 - 15 conversion in regular sequence
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 10)
    }
    ///Bits 15:19 - 16 conversion in regular sequence
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 15)
    }
    ///Bits 20:24 - 17 conversion in regular sequence
    #[inline(always)]
    pub fn sq17(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 20)
    }
    ///Bits 25:29 - 18 conversion in regular sequence
    #[inline(always)]
    pub fn sq18(&mut self) -> SQ_W<SQR3rs> {
        SQ_W::new(self, 25)
    }
}
/**regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#ADC:SQR3)*/
pub struct SQR3rs;
impl crate::RegisterSpec for SQR3rs {
    type Ux = u32;
}
///`read()` method returns [`sqr3::R`](R) reader structure
impl crate::Readable for SQR3rs {}
///`write(|w| ..)` method takes [`sqr3::W`](W) writer structure
impl crate::Writable for SQR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SQR3 to value 0
impl crate::Resettable for SQR3rs {
    const RESET_VALUE: u32 = 0;
}