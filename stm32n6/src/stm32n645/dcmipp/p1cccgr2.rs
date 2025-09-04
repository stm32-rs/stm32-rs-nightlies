///Register `P1CCCGR2` reader
pub type R = crate::R<P1CCCGR2rs>;
///Field `GB` reader - Current coefficient row 2 column 3 of the matrix
pub type GB_R = crate::FieldReader<u16>;
///Field `GA` reader - Current coefficient row 2 of the added column (signed integer value)
pub type GA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - Current coefficient row 2 column 3 of the matrix
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:25 - Current coefficient row 2 of the added column (signed integer value)
    #[inline(always)]
    pub fn ga(&self) -> GA_R {
        GA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCCGR2")
            .field("gb", &self.gb())
            .field("ga", &self.ga())
            .finish()
    }
}
/**DCMIPP Pipe1 current ColorConv green coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1cccgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CCCGR2)*/
pub struct P1CCCGR2rs;
impl crate::RegisterSpec for P1CCCGR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1cccgr2::R`](R) reader structure
impl crate::Readable for P1CCCGR2rs {}
///`reset()` method sets P1CCCGR2 to value 0
impl crate::Resettable for P1CCCGR2rs {}
