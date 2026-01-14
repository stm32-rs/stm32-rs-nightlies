///Register `P1CTCR2` reader
pub type R = crate::R<P1CTCR2rs>;
///Register `P1CTCR2` writer
pub type W = crate::W<P1CTCR2rs>;
///Field `LUM4` reader - Luminance increase for input luminance of 128 (increase is idle with LUMx = 16)
pub type LUM4_R = crate::FieldReader;
///Field `LUM4` writer - Luminance increase for input luminance of 128 (increase is idle with LUMx = 16)
pub type LUM4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `LUM3` reader - Luminance increase for input luminance of 96 (increase is idle with LUMx = 16)
pub type LUM3_R = crate::FieldReader;
///Field `LUM3` writer - Luminance increase for input luminance of 96 (increase is idle with LUMx = 16)
pub type LUM3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `LUM2` reader - Luminance increase for input luminance of 64 (increase is idle with LUMx = 16)
pub type LUM2_R = crate::FieldReader;
///Field `LUM2` writer - Luminance increase for input luminance of 64 (increase is idle with LUMx = 16)
pub type LUM2_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `LUM1` reader - Luminance increase for input luminance of 32 (increase is idle with LUMx = 16)
pub type LUM1_R = crate::FieldReader;
///Field `LUM1` writer - Luminance increase for input luminance of 32 (increase is idle with LUMx = 16)
pub type LUM1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 1:6 - Luminance increase for input luminance of 128 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum4(&self) -> LUM4_R {
        LUM4_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    ///Bits 9:14 - Luminance increase for input luminance of 96 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum3(&self) -> LUM3_R {
        LUM3_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    ///Bits 17:22 - Luminance increase for input luminance of 64 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum2(&self) -> LUM2_R {
        LUM2_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bits 25:30 - Luminance increase for input luminance of 32 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum1(&self) -> LUM1_R {
        LUM1_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CTCR2")
            .field("lum4", &self.lum4())
            .field("lum3", &self.lum3())
            .field("lum2", &self.lum2())
            .field("lum1", &self.lum1())
            .finish()
    }
}
impl W {
    ///Bits 1:6 - Luminance increase for input luminance of 128 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum4(&mut self) -> LUM4_W<'_, P1CTCR2rs> {
        LUM4_W::new(self, 1)
    }
    ///Bits 9:14 - Luminance increase for input luminance of 96 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum3(&mut self) -> LUM3_W<'_, P1CTCR2rs> {
        LUM3_W::new(self, 9)
    }
    ///Bits 17:22 - Luminance increase for input luminance of 64 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum2(&mut self) -> LUM2_W<'_, P1CTCR2rs> {
        LUM2_W::new(self, 17)
    }
    ///Bits 25:30 - Luminance increase for input luminance of 32 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum1(&mut self) -> LUM1_W<'_, P1CTCR2rs> {
        LUM1_W::new(self, 25)
    }
}
/**DCMIPP Pipe1 contrast control register 2

You can [`read`](crate::Reg::read) this register and get [`p1ctcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ctcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CTCR2)*/
pub struct P1CTCR2rs;
impl crate::RegisterSpec for P1CTCR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1ctcr2::R`](R) reader structure
impl crate::Readable for P1CTCR2rs {}
///`write(|w| ..)` method takes [`p1ctcr2::W`](W) writer structure
impl crate::Writable for P1CTCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CTCR2 to value 0x2020_2020
impl crate::Resettable for P1CTCR2rs {
    const RESET_VALUE: u32 = 0x2020_2020;
}
