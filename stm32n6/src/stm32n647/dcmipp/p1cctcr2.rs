///Register `P1CCTCR2` reader
pub type R = crate::R<P1CCTCR2rs>;
///Field `LUM4` reader - Current luminance increase for input luminance of 128 (increase is idle with LUMx = 16)
pub type LUM4_R = crate::FieldReader;
///Field `LUM3` reader - Current luminance increase for input luminance of 96 (increase is idle with LUMx = 16)
pub type LUM3_R = crate::FieldReader;
///Field `LUM2` reader - Current luminance increase for input luminance of 64 (increase is idle with LUMx = 16)
pub type LUM2_R = crate::FieldReader;
///Field `LUM1` reader - Current luminance increase for input luminance of 32 (increase is idle with LUMx = 16)
pub type LUM1_R = crate::FieldReader;
impl R {
    ///Bits 1:6 - Current luminance increase for input luminance of 128 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum4(&self) -> LUM4_R {
        LUM4_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    ///Bits 9:14 - Current luminance increase for input luminance of 96 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum3(&self) -> LUM3_R {
        LUM3_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    ///Bits 17:22 - Current luminance increase for input luminance of 64 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum2(&self) -> LUM2_R {
        LUM2_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bits 25:30 - Current luminance increase for input luminance of 32 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum1(&self) -> LUM1_R {
        LUM1_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCTCR2")
            .field("lum4", &self.lum4())
            .field("lum3", &self.lum3())
            .field("lum2", &self.lum2())
            .field("lum1", &self.lum1())
            .finish()
    }
}
/**DCMIPP Pipe1 current contrast control register 2

You can [`read`](crate::Reg::read) this register and get [`p1cctcr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CCTCR2)*/
pub struct P1CCTCR2rs;
impl crate::RegisterSpec for P1CCTCR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1cctcr2::R`](R) reader structure
impl crate::Readable for P1CCTCR2rs {}
///`reset()` method sets P1CCTCR2 to value 0x2020_2020
impl crate::Resettable for P1CCTCR2rs {
    const RESET_VALUE: u32 = 0x2020_2020;
}
