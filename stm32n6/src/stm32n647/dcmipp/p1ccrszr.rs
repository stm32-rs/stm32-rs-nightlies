///Register `P1CCRSZR` reader
pub type R = crate::R<P1CCRSZRrs>;
///Field `HSIZE` reader - Current horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` reader - Current vertical size, from 0 to 4094 pixels high
pub type VSIZE_R = crate::FieldReader<u16>;
///Field `ENABLE` reader - Current ENABLE bit value.
pub type ENABLE_R = crate::BitReader;
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
    ///Bit 31 - Current ENABLE bit value.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCRSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .field("enable", &self.enable())
            .finish()
    }
}
/**DCMIPP Pipex current crop window size register

You can [`read`](crate::Reg::read) this register and get [`p1ccrszr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CCRSZR)*/
pub struct P1CCRSZRrs;
impl crate::RegisterSpec for P1CCRSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p1ccrszr::R`](R) reader structure
impl crate::Readable for P1CCRSZRrs {}
///`reset()` method sets P1CCRSZR to value 0
impl crate::Resettable for P1CCRSZRrs {}
