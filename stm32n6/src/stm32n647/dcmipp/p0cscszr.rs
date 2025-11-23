///Register `P0CSCSZR` reader
pub type R = crate::R<P0CSCSZRrs>;
///Field `HSIZE` reader - Current horizontal size, from 0 to 4094 word wide (data 32-bit).
pub type HSIZE_R = crate::FieldReader<u16>;
///Field `VSIZE` reader - Current vertical size, from 0 to 4094 pixels high.
pub type VSIZE_R = crate::FieldReader<u16>;
///Field `POSNEG` reader - Current value of the POSNEG bit
pub type POSNEG_R = crate::BitReader;
///Field `ENABLE` reader - Current value of the ENABLE bit
pub type ENABLE_R = crate::BitReader;
impl R {
    ///Bits 0:11 - Current horizontal size, from 0 to 4094 word wide (data 32-bit).
    #[inline(always)]
    pub fn hsize(&self) -> HSIZE_R {
        HSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Current vertical size, from 0 to 4094 pixels high.
    #[inline(always)]
    pub fn vsize(&self) -> VSIZE_R {
        VSIZE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 30 - Current value of the POSNEG bit
    #[inline(always)]
    pub fn posneg(&self) -> POSNEG_R {
        POSNEG_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Current value of the ENABLE bit
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0CSCSZR")
            .field("hsize", &self.hsize())
            .field("vsize", &self.vsize())
            .field("posneg", &self.posneg())
            .field("enable", &self.enable())
            .finish()
    }
}
/**DCMIPP Pipe0 current stat/crop size register

You can [`read`](crate::Reg::read) this register and get [`p0cscszr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P0CSCSZR)*/
pub struct P0CSCSZRrs;
impl crate::RegisterSpec for P0CSCSZRrs {
    type Ux = u32;
}
///`read()` method returns [`p0cscszr::R`](R) reader structure
impl crate::Readable for P0CSCSZRrs {}
///`reset()` method sets P0CSCSZR to value 0
impl crate::Resettable for P0CSCSZRrs {}
