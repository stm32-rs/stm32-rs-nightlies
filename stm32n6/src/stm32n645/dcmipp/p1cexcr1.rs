///Register `P1CEXCR1` reader
pub type R = crate::R<P1CEXCR1rs>;
///Field `ENABLE` reader - for exposure control (multiplication and shift)
pub type ENABLE_R = crate::BitReader;
///Field `MULTR` reader - Current exposure multiplier - Red
pub type MULTR_R = crate::FieldReader;
///Field `SHFR` reader - Current exposure shift - Red
pub type SHFR_R = crate::FieldReader;
impl R {
    ///Bit 0 - for exposure control (multiplication and shift)
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 20:27 - Current exposure multiplier - Red
    #[inline(always)]
    pub fn multr(&self) -> MULTR_R {
        MULTR_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bits 28:30 - Current exposure shift - Red
    #[inline(always)]
    pub fn shfr(&self) -> SHFR_R {
        SHFR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CEXCR1")
            .field("enable", &self.enable())
            .field("multr", &self.multr())
            .field("shfr", &self.shfr())
            .finish()
    }
}
/**DCMIPP Pipe1 current exposure control register 1

You can [`read`](crate::Reg::read) this register and get [`p1cexcr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CEXCR1)*/
pub struct P1CEXCR1rs;
impl crate::RegisterSpec for P1CEXCR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1cexcr1::R`](R) reader structure
impl crate::Readable for P1CEXCR1rs {}
///`reset()` method sets P1CEXCR1 to value 0
impl crate::Resettable for P1CEXCR1rs {}
