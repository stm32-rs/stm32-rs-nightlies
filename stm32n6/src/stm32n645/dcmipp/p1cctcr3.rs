///Register `P1CCTCR3` reader
pub type R = crate::R<P1CCTCR3rs>;
///Field `LUM8` reader - Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)
pub type LUM8_R = crate::FieldReader;
///Field `LUM7` reader - Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)
pub type LUM7_R = crate::FieldReader;
///Field `LUM6` reader - Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)
pub type LUM6_R = crate::FieldReader;
///Field `LUM5` reader - Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)
pub type LUM5_R = crate::FieldReader;
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
        f.debug_struct("P1CCTCR3")
            .field("lum8", &self.lum8())
            .field("lum7", &self.lum7())
            .field("lum6", &self.lum6())
            .field("lum5", &self.lum5())
            .finish()
    }
}
/**DCMIPP Pipe1 current contrast control register 3

You can [`read`](crate::Reg::read) this register and get [`p1cctcr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CCTCR3)*/
pub struct P1CCTCR3rs;
impl crate::RegisterSpec for P1CCTCR3rs {
    type Ux = u32;
}
///`read()` method returns [`p1cctcr3::R`](R) reader structure
impl crate::Readable for P1CCTCR3rs {}
///`reset()` method sets P1CCTCR3 to value 0x2020_2020
impl crate::Resettable for P1CCTCR3rs {
    const RESET_VALUE: u32 = 0x2020_2020;
}
