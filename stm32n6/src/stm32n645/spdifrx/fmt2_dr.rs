///Register `FMT2_DR` reader
pub type R = crate::R<FMT2_DRrs>;
///Field `DRNL1` reader - data value
pub type DRNL1_R = crate::FieldReader<u16>;
///Field `DRNL2` reader - data value
pub type DRNL2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - data value
    #[inline(always)]
    pub fn drnl1(&self) -> DRNL1_R {
        DRNL1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - data value
    #[inline(always)]
    pub fn drnl2(&self) -> DRNL2_R {
        DRNL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMT2_DR")
            .field("drnl1", &self.drnl1())
            .field("drnl2", &self.drnl2())
            .finish()
    }
}
/**SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt2_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SPDIFRX:FMT2_DR)*/
pub struct FMT2_DRrs;
impl crate::RegisterSpec for FMT2_DRrs {
    type Ux = u32;
}
///`read()` method returns [`fmt2_dr::R`](R) reader structure
impl crate::Readable for FMT2_DRrs {}
///`reset()` method sets FMT2_DR to value 0
impl crate::Resettable for FMT2_DRrs {}
