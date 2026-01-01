///Register `SQR1` reader
pub type R = crate::R<SQR1rs>;
///Register `SQR1` writer
pub type W = crate::W<SQR1rs>;
///Field `SQ(13-16)` reader - %s conversion in regular sequence
pub type SQ_R = crate::FieldReader;
///Field `SQ(13-16)` writer - %s conversion in regular sequence
pub type SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L` reader - Regular channel sequence length
pub type L_R = crate::FieldReader;
///Field `L` writer - Regular channel sequence length
pub type L_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///(13-16) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ13` field.</div>
    #[inline(always)]
    pub fn sq(&self, n: u8) -> SQ_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(13-16) conversion in regular sequence
    #[inline(always)]
    pub fn sq_iter(&self) -> impl Iterator<Item = SQ_R> + '_ {
        (0..4).map(move |n| SQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
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
    ///Bits 20:23 - Regular channel sequence length
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR1")
            .field("l", &self.l())
            .field("sq13", &self.sq13())
            .field("sq14", &self.sq14())
            .field("sq15", &self.sq15())
            .field("sq16", &self.sq16())
            .finish()
    }
}
impl W {
    ///(13-16) conversion in regular sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SQ13` field.</div>
    #[inline(always)]
    pub fn sq(&mut self, n: u8) -> SQ_W<'_, SQR1rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SQ_W::new(self, n * 5)
    }
    ///Bits 0:4 - 13 conversion in regular sequence
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ_W<'_, SQR1rs> {
        SQ_W::new(self, 0)
    }
    ///Bits 5:9 - 14 conversion in regular sequence
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ_W<'_, SQR1rs> {
        SQ_W::new(self, 5)
    }
    ///Bits 10:14 - 15 conversion in regular sequence
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ_W<'_, SQR1rs> {
        SQ_W::new(self, 10)
    }
    ///Bits 15:19 - 16 conversion in regular sequence
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ_W<'_, SQR1rs> {
        SQ_W::new(self, 15)
    }
    ///Bits 20:23 - Regular channel sequence length
    #[inline(always)]
    pub fn l(&mut self) -> L_W<'_, SQR1rs> {
        L_W::new(self, 20)
    }
}
/**regular sequence register 1

You can [`read`](crate::Reg::read) this register and get [`sqr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#ADC1:SQR1)*/
pub struct SQR1rs;
impl crate::RegisterSpec for SQR1rs {
    type Ux = u32;
}
///`read()` method returns [`sqr1::R`](R) reader structure
impl crate::Readable for SQR1rs {}
///`write(|w| ..)` method takes [`sqr1::W`](W) writer structure
impl crate::Writable for SQR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR1 to value 0
impl crate::Resettable for SQR1rs {}
