///Register `JSQR` reader
pub type R = crate::R<JSQRrs>;
///Register `JSQR` writer
pub type W = crate::W<JSQRrs>;
///Field `JL` reader - JL
pub type JL_R = crate::FieldReader;
///Field `JL` writer - JL
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `JEXTSEL` reader - JEXTSEL
pub type JEXTSEL_R = crate::FieldReader;
///Field `JEXTSEL` writer - JEXTSEL
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JEXTEN` reader - JEXTEN
pub type JEXTEN_R = crate::FieldReader;
///Field `JEXTEN` writer - JEXTEN
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `JSQ1` reader - JSQ1
pub type JSQ1_R = crate::FieldReader;
///Field `JSQ1` writer - JSQ1
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ2` reader - JSQ2
pub type JSQ2_R = crate::FieldReader;
///Field `JSQ2` writer - JSQ2
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ3` reader - JSQ3
pub type JSQ3_R = crate::FieldReader;
///Field `JSQ3` writer - JSQ3
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ4` reader - JSQ4
pub type JSQ4_R = crate::FieldReader;
///Field `JSQ4` writer - JSQ4
pub type JSQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - JL
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:6 - JEXTSEL
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 7:8 - JEXTEN
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bits 9:13 - JSQ1
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 15:19 - JSQ2
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 21:25 - JSQ3
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 27:31 - JSQ4
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JSQR")
            .field("jl", &self.jl())
            .field("jextsel", &self.jextsel())
            .field("jexten", &self.jexten())
            .field("jsq1", &self.jsq1())
            .field("jsq2", &self.jsq2())
            .field("jsq3", &self.jsq3())
            .field("jsq4", &self.jsq4())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - JL
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<'_, JSQRrs> {
        JL_W::new(self, 0)
    }
    ///Bits 2:6 - JEXTSEL
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    ///Bits 7:8 - JEXTEN
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<'_, JSQRrs> {
        JEXTEN_W::new(self, 7)
    }
    ///Bits 9:13 - JSQ1
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ1_W<'_, JSQRrs> {
        JSQ1_W::new(self, 9)
    }
    ///Bits 15:19 - JSQ2
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ2_W<'_, JSQRrs> {
        JSQ2_W::new(self, 15)
    }
    ///Bits 21:25 - JSQ3
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ3_W<'_, JSQRrs> {
        JSQ3_W::new(self, 21)
    }
    ///Bits 27:31 - JSQ4
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ4_W<'_, JSQRrs> {
        JSQ4_W::new(self, 27)
    }
}
/**ADC injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC2:JSQR)*/
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
