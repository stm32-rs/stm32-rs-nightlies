///Register `P1CSTSZR` reader
pub type R = crate::R<P1CSTSZRrs>;
///Field `HSIZE` reader - Current horizontal size, from 0 to 4094 pixels wide
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` reader - Current vertical size, from 0 to 4094 pixels high
pub type VSIZE_R = crate::FieldReader<u16>;
///Field `CROPEN` reader - Current CROPEN bit value
pub type CROPEN_R = crate::BitReader;
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
    ///Bit 31 - Current CROPEN bit value
    #[inline(always)]
    pub fn cropen(&self) -> CROPEN_R {
        CROPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CSTSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .field("cropen", &self.cropen())
            .finish()
    }
}
/**DCMIPP Pipe1 current statistics window size register

You can [`read`](crate::Reg::read) this register and get [`p1cstszr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CSTSZR)*/
pub struct P1CSTSZRrs;
impl crate::RegisterSpec for P1CSTSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cstszr::R`](R) reader structure
impl crate::Readable for P1CSTSZRrs {}
///`reset()` method sets P1CSTSZR to value 0
impl crate::Resettable for P1CSTSZRrs {}
