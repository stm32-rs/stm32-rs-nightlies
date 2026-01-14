///Register `P1CCTCR1` reader
pub type R = crate::R<P1CCTCR1rs>;
///Field `ENABLE` reader - Current ENABLE bit value
pub type ENABLE_R = crate::BitReader;
///Field `LUM0` reader - Current luminance increase for input luminance of 0 (increase is idle with LUMx = 16)
pub type LUM0_R = crate::FieldReader;
impl R {
    ///Bit 0 - Current ENABLE bit value
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 9:14 - Current luminance increase for input luminance of 0 (increase is idle with LUMx = 16)
    #[inline(always)]
    pub fn lum0(&self) -> LUM0_R {
        LUM0_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCTCR1")
            .field("enable", &self.enable())
            .field("lum0", &self.lum0())
            .finish()
    }
}
/**DCMIPP Pipe1 current contrast control register 1

You can [`read`](crate::Reg::read) this register and get [`p1cctcr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CCTCR1)*/
pub struct P1CCTCR1rs;
impl crate::RegisterSpec for P1CCTCR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1cctcr1::R`](R) reader structure
impl crate::Readable for P1CCTCR1rs {}
///`reset()` method sets P1CCTCR1 to value 0x2000
impl crate::Resettable for P1CCTCR1rs {
    const RESET_VALUE: u32 = 0x2000;
}
