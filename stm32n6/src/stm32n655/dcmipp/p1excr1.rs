///Register `P1EXCR1` reader
pub type R = crate::R<P1EXCR1rs>;
///Register `P1EXCR1` writer
pub type W = crate::W<P1EXCR1rs>;
///Field `ENABLE` reader - Exposure control (multiplication and shift) of all red, green and blue
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - Exposure control (multiplication and shift) of all red, green and blue
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MULTR` reader - Exposure multiplier - Red
pub type MULTR_R = crate::FieldReader;
///Field `MULTR` writer - Exposure multiplier - Red
pub type MULTR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SHFR` reader - Exposure shift - Red
pub type SHFR_R = crate::FieldReader;
///Field `SHFR` writer - Exposure shift - Red
pub type SHFR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Exposure control (multiplication and shift) of all red, green and blue
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 20:27 - Exposure multiplier - Red
    #[inline(always)]
    pub fn multr(&self) -> MULTR_R {
        MULTR_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bits 28:30 - Exposure shift - Red
    #[inline(always)]
    pub fn shfr(&self) -> SHFR_R {
        SHFR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1EXCR1")
            .field("enable", &self.enable())
            .field("multr", &self.multr())
            .field("shfr", &self.shfr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Exposure control (multiplication and shift) of all red, green and blue
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1EXCR1rs> {
        ENABLE_W::new(self, 0)
    }
    ///Bits 20:27 - Exposure multiplier - Red
    #[inline(always)]
    pub fn multr(&mut self) -> MULTR_W<'_, P1EXCR1rs> {
        MULTR_W::new(self, 20)
    }
    ///Bits 28:30 - Exposure shift - Red
    #[inline(always)]
    pub fn shfr(&mut self) -> SHFR_W<'_, P1EXCR1rs> {
        SHFR_W::new(self, 28)
    }
}
/**DCMIPP Pipe1 exposure control register 1

You can [`read`](crate::Reg::read) this register and get [`p1excr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1excr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1EXCR1)*/
pub struct P1EXCR1rs;
impl crate::RegisterSpec for P1EXCR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1excr1::R`](R) reader structure
impl crate::Readable for P1EXCR1rs {}
///`write(|w| ..)` method takes [`p1excr1::W`](W) writer structure
impl crate::Writable for P1EXCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1EXCR1 to value 0
impl crate::Resettable for P1EXCR1rs {}
