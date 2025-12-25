///Register `P1CDSSZR` reader
pub type R = crate::R<P1CDSSZRrs>;
///Field `HSIZE` reader - Current horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` reader - Current vertical size, from 0 to 4094 pixels high
pub type VSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Current horizontal size, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hsize(&self) -> HSIZE_R {
        HSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Current vertical size, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vsize(&self) -> VSIZE_R {
        VSIZE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CDSSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .finish()
    }
}
/**DCMIPP Pipex current downsize destination size register

You can [`read`](crate::Reg::read) this register and get [`p1cdsszr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CDSSZR)*/
pub struct P1CDSSZRrs;
impl crate::RegisterSpec for P1CDSSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cdsszr::R`](R) reader structure
impl crate::Readable for P1CDSSZRrs {}
///`reset()` method sets P1CDSSZR to value 0
impl crate::Resettable for P1CDSSZRrs {}
