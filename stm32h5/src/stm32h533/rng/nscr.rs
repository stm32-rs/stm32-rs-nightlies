///Register `NSCR` reader
pub type R = crate::R<NSCRrs>;
///Register `NSCR` writer
pub type W = crate::W<NSCRrs>;
///Field `EN_OSC1` reader - Each bit drives one oscillator enable signal input of instance number 1, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC1_R = crate::FieldReader;
///Field `EN_OSC1` writer - Each bit drives one oscillator enable signal input of instance number 1, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_OSC2` reader - Each bit drives one oscillator enable signal input of instance number 2, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC2_R = crate::FieldReader;
///Field `EN_OSC2` writer - Each bit drives one oscillator enable signal input of instance number 2, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_OSC3` reader - Each bit drives one oscillator enable signal input of instance number 3, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC3_R = crate::FieldReader;
///Field `EN_OSC3` writer - Each bit drives one oscillator enable signal input of instance number 3, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_OSC4` reader - Each bit drives one oscillator enable signal input of instance number 4, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC4_R = crate::FieldReader;
///Field `EN_OSC4` writer - Each bit drives one oscillator enable signal input of instance number 4, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_OSC5` reader - Each bit drives one oscillator enable signal input of instance number 5, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC5_R = crate::FieldReader;
///Field `EN_OSC5` writer - Each bit drives one oscillator enable signal input of instance number 5, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_OSC6` reader - Each bit drives one oscillator enable signal input of instance number 6, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC6_R = crate::FieldReader;
///Field `EN_OSC6` writer - Each bit drives one oscillator enable signal input of instance number 6, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
pub type EN_OSC6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Each bit drives one oscillator enable signal input of instance number 1, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc1(&self) -> EN_OSC1_R {
        EN_OSC1_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Each bit drives one oscillator enable signal input of instance number 2, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc2(&self) -> EN_OSC2_R {
        EN_OSC2_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Each bit drives one oscillator enable signal input of instance number 3, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc3(&self) -> EN_OSC3_R {
        EN_OSC3_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Each bit drives one oscillator enable signal input of instance number 4, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc4(&self) -> EN_OSC4_R {
        EN_OSC4_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Each bit drives one oscillator enable signal input of instance number 5, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc5(&self) -> EN_OSC5_R {
        EN_OSC5_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Each bit drives one oscillator enable signal input of instance number 6, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc6(&self) -> EN_OSC6_R {
        EN_OSC6_R::new(((self.bits >> 15) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSCR")
            .field("en_osc1", &self.en_osc1())
            .field("en_osc2", &self.en_osc2())
            .field("en_osc3", &self.en_osc3())
            .field("en_osc4", &self.en_osc4())
            .field("en_osc5", &self.en_osc5())
            .field("en_osc6", &self.en_osc6())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Each bit drives one oscillator enable signal input of instance number 1, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc1(&mut self) -> EN_OSC1_W<'_, NSCRrs> {
        EN_OSC1_W::new(self, 0)
    }
    ///Bits 3:5 - Each bit drives one oscillator enable signal input of instance number 2, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc2(&mut self) -> EN_OSC2_W<'_, NSCRrs> {
        EN_OSC2_W::new(self, 3)
    }
    ///Bits 6:8 - Each bit drives one oscillator enable signal input of instance number 3, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc3(&mut self) -> EN_OSC3_W<'_, NSCRrs> {
        EN_OSC3_W::new(self, 6)
    }
    ///Bits 9:11 - Each bit drives one oscillator enable signal input of instance number 4, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc4(&mut self) -> EN_OSC4_W<'_, NSCRrs> {
        EN_OSC4_W::new(self, 9)
    }
    ///Bits 12:14 - Each bit drives one oscillator enable signal input of instance number 5, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc5(&mut self) -> EN_OSC5_W<'_, NSCRrs> {
        EN_OSC5_W::new(self, 12)
    }
    ///Bits 15:17 - Each bit drives one oscillator enable signal input of instance number 6, gated with the RNGEN bit in RNG_CR (set bit to enable the oscillator).
    #[inline(always)]
    pub fn en_osc6(&mut self) -> EN_OSC6_W<'_, NSCRrs> {
        EN_OSC6_W::new(self, 15)
    }
}
/**RNG noise source control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RNG:NSCR)*/
pub struct NSCRrs;
impl crate::RegisterSpec for NSCRrs {
    type Ux = u32;
}
///`read()` method returns [`nscr::R`](R) reader structure
impl crate::Readable for NSCRrs {}
///`write(|w| ..)` method takes [`nscr::W`](W) writer structure
impl crate::Writable for NSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSCR to value 0x0003_ffff
impl crate::Resettable for NSCRrs {
    const RESET_VALUE: u32 = 0x0003_ffff;
}
