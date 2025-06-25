///Register `P1CEXCR2` reader
pub type R = crate::R<P1CEXCR2rs>;
///Field `MULTB` reader - Current exposure multiplier - Blue
pub type MULTB_R = crate::FieldReader;
///Field `SHFB` reader - Current exposure shift - Blue
pub type SHFB_R = crate::FieldReader;
///Field `MULTG` reader - Current exposure multiplier - Green
pub type MULTG_R = crate::FieldReader;
///Field `SHFG` reader - Current exposure shift - Green
pub type SHFG_R = crate::FieldReader;
impl R {
    ///Bits 4:11 - Current exposure multiplier - Blue
    #[inline(always)]
    pub fn multb(&self) -> MULTB_R {
        MULTB_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    ///Bits 12:14 - Current exposure shift - Blue
    #[inline(always)]
    pub fn shfb(&self) -> SHFB_R {
        SHFB_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 20:27 - Current exposure multiplier - Green
    #[inline(always)]
    pub fn multg(&self) -> MULTG_R {
        MULTG_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bits 28:30 - Current exposure shift - Green
    #[inline(always)]
    pub fn shfg(&self) -> SHFG_R {
        SHFG_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CEXCR2")
            .field("multb", &self.multb())
            .field("shfb", &self.shfb())
            .field("multg", &self.multg())
            .field("shfg", &self.shfg())
            .finish()
    }
}
/**DCMIPP Pipe1 current exposure control register 2

You can [`read`](crate::Reg::read) this register and get [`p1cexcr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CEXCR2)*/
pub struct P1CEXCR2rs;
impl crate::RegisterSpec for P1CEXCR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1cexcr2::R`](R) reader structure
impl crate::Readable for P1CEXCR2rs {}
///`reset()` method sets P1CEXCR2 to value 0
impl crate::Resettable for P1CEXCR2rs {}
