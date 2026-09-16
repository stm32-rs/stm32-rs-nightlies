///Register `JSQR` reader
pub type R = crate::R<JSQRrs>;
///Register `JSQR` writer
pub type W = crate::W<JSQRrs>;
///Field `JSQ1` reader - 1st conversion in injected sequence
pub type JSQ1_R = crate::FieldReader;
///Field `JSQ1` writer - 1st conversion in injected sequence
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ2` reader - 2nd conversion in injected sequence
pub type JSQ2_R = crate::FieldReader;
///Field `JSQ2` writer - 2nd conversion in injected sequence
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ3` reader - 3rd conversion in injected sequence
pub type JSQ3_R = crate::FieldReader;
///Field `JSQ3` writer - 3rd conversion in injected sequence
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ4` reader - 4th conversion in injected sequence
pub type JSQ4_R = crate::FieldReader;
///Field `JSQ4` writer - 4th conversion in injected sequence
pub type JSQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JL` reader - Injected sequence length
pub type JL_R = crate::FieldReader;
///Field `JL` writer - Injected sequence length
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:4 - 1st conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 2nd conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 3rd conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 4th conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
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
            .field("jsq4", &self.jsq4())
            .field("jsq3", &self.jsq3())
            .field("jsq2", &self.jsq2())
            .field("jsq1", &self.jsq1())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - 1st conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ1_W<'_, JSQRrs> {
        JSQ1_W::new(self, 0)
    }
    ///Bits 5:9 - 2nd conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ2_W<'_, JSQRrs> {
        JSQ2_W::new(self, 5)
    }
    ///Bits 10:14 - 3rd conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ3_W<'_, JSQRrs> {
        JSQ3_W::new(self, 10)
    }
    ///Bits 15:19 - 4th conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ4_W<'_, JSQRrs> {
        JSQ4_W::new(self, 15)
    }
    ///Bits 20:21 - Injected sequence length
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<'_, JSQRrs> {
        JL_W::new(self, 20)
    }
}
/**injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#ADC1:JSQR)*/
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
