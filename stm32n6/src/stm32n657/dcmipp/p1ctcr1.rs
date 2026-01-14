///Register `P1CTCR1` reader
pub type R = crate::R<P1CTCR1rs>;
///Register `P1CTCR1` writer
pub type W = crate::W<P1CTCR1rs>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LUM0` reader - Luminance increase for input luminance of 0 (increase is idle with LUMx = 16)
pub type LUM0_R = crate::FieldReader;
///Field `LUM0` writer - Luminance increase for input luminance of 0 (increase is idle with LUMx = 16)
pub type LUM0_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 9:14 - Luminance increase for input luminance of 0 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum0(&self) -> LUM0_R {
        LUM0_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CTCR1")
            .field("enable", &self.enable())
            .field("lum0", &self.lum0())
            .finish()
    }
}
impl W {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1CTCR1rs> {
        ENABLE_W::new(self, 0)
    }
    ///Bits 9:14 - Luminance increase for input luminance of 0 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum0(&mut self) -> LUM0_W<'_, P1CTCR1rs> {
        LUM0_W::new(self, 9)
    }
}
/**DCMIPP Pipe1 contrast control register 1

You can [`read`](crate::Reg::read) this register and get [`p1ctcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ctcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CTCR1)*/
pub struct P1CTCR1rs;
impl crate::RegisterSpec for P1CTCR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1ctcr1::R`](R) reader structure
impl crate::Readable for P1CTCR1rs {}
///`write(|w| ..)` method takes [`p1ctcr1::W`](W) writer structure
impl crate::Writable for P1CTCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CTCR1 to value 0x2000
impl crate::Resettable for P1CTCR1rs {
    const RESET_VALUE: u32 = 0x2000;
}
