///Register `P1CTCR3` reader
pub type R = crate::R<P1CTCR3rs>;
///Register `P1CTCR3` writer
pub type W = crate::W<P1CTCR3rs>;
///Field `LUM8` reader - Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)
pub type LUM8_R = crate::FieldReader;
///Field `LUM8` writer - Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)
pub type LUM8_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `LUM7` reader - Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)
pub type LUM7_R = crate::FieldReader;
///Field `LUM7` writer - Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)
pub type LUM7_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `LUM6` reader - Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)
pub type LUM6_R = crate::FieldReader;
///Field `LUM6` writer - Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)
pub type LUM6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `LUM5` reader - Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)
pub type LUM5_R = crate::FieldReader;
///Field `LUM5` writer - Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)
pub type LUM5_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 1:6 - Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum8(&self) -> LUM8_R {
        LUM8_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    ///Bits 9:14 - Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum7(&self) -> LUM7_R {
        LUM7_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    ///Bits 17:22 - Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum6(&self) -> LUM6_R {
        LUM6_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bits 25:30 - Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum5(&self) -> LUM5_R {
        LUM5_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CTCR3")
            .field("lum8", &self.lum8())
            .field("lum7", &self.lum7())
            .field("lum6", &self.lum6())
            .field("lum5", &self.lum5())
            .finish()
    }
}
impl W {
    ///Bits 1:6 - Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum8(&mut self) -> LUM8_W<'_, P1CTCR3rs> {
        LUM8_W::new(self, 1)
    }
    ///Bits 9:14 - Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum7(&mut self) -> LUM7_W<'_, P1CTCR3rs> {
        LUM7_W::new(self, 9)
    }
    ///Bits 17:22 - Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum6(&mut self) -> LUM6_W<'_, P1CTCR3rs> {
        LUM6_W::new(self, 17)
    }
    ///Bits 25:30 - Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum5(&mut self) -> LUM5_W<'_, P1CTCR3rs> {
        LUM5_W::new(self, 25)
    }
}
/**DCMIPP Pipe1 contrast control register 3

You can [`read`](crate::Reg::read) this register and get [`p1ctcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ctcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CTCR3)*/
pub struct P1CTCR3rs;
impl crate::RegisterSpec for P1CTCR3rs {
    type Ux = u32;
}
///`read()` method returns [`p1ctcr3::R`](R) reader structure
impl crate::Readable for P1CTCR3rs {}
///`write(|w| ..)` method takes [`p1ctcr3::W`](W) writer structure
impl crate::Writable for P1CTCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CTCR3 to value 0x2020_2020
impl crate::Resettable for P1CTCR3rs {
    const RESET_VALUE: u32 = 0x2020_2020;
}
