///Register `JSQR` reader
pub type R = crate::R<JSQRrs>;
///Register `JSQR` writer
pub type W = crate::W<JSQRrs>;
///Field `JSQ(1-4)` reader - %s conversion in injected sequence
pub type JSQ_R = crate::FieldReader;
///Field `JSQ(1-4)` writer - %s conversion in injected sequence
pub type JSQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JL` reader - Injected sequence length
pub type JL_R = crate::FieldReader;
///Field `JL` writer - Injected sequence length
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&self, n: u8) -> JSQ_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(1-4) conversion in injected sequence
    #[inline(always)]
    pub fn jsq_iter(&self) -> impl Iterator<Item = JSQ_R> + '_ {
        (0..4).map(move |n| JSQ_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
    }
    ///Bits 0:4 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ_R {
        JSQ_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:21 - Injected sequence length
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JSQR")
            .field("jl", &self.jl())
            .field("jsq1", &self.jsq1())
            .field("jsq2", &self.jsq2())
            .field("jsq3", &self.jsq3())
            .field("jsq4", &self.jsq4())
            .finish()
    }
}
impl W {
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&mut self, n: u8) -> JSQ_W<'_, JSQRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_W::new(self, n * 5)
    }
    ///Bits 0:4 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ_W<'_, JSQRrs> {
        JSQ_W::new(self, 0)
    }
    ///Bits 5:9 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ_W<'_, JSQRrs> {
        JSQ_W::new(self, 5)
    }
    ///Bits 10:14 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ_W<'_, JSQRrs> {
        JSQ_W::new(self, 10)
    }
    ///Bits 15:19 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ_W<'_, JSQRrs> {
        JSQ_W::new(self, 15)
    }
    ///Bits 20:21 - Injected sequence length
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<'_, JSQRrs> {
        JL_W::new(self, 20)
    }
}
/**injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#ADC:JSQR)*/
pub struct JSQRrs;
impl crate::RegisterSpec for JSQRrs {
    type Ux = u32;
}
///`read()` method returns [`jsqr::R`](R) reader structure
impl crate::Readable for JSQRrs {}
///`write(|w| ..)` method takes [`jsqr::W`](W) writer structure
impl crate::Writable for JSQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JSQR to value 0
impl crate::Resettable for JSQRrs {}
