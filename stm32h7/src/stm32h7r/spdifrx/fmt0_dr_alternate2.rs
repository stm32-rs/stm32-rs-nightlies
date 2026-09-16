///Register `FMT0_DR_alternate2` reader
pub type R = crate::R<FMT0_DR_ALTERNATE2rs>;
///Field `DRNL1` reader - data value This field contains the channel B
pub type DRNL1_R = crate::FieldReader<u16>;
///Field `DRNL2` reader - data value This field contains the channel A
pub type DRNL2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - data value This field contains the channel B
    #[inline(always)]
    pub fn drnl1(&self) -> DRNL1_R {
        DRNL1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - data value This field contains the channel A
    #[inline(always)]
    pub fn drnl2(&self) -> DRNL2_R {
        DRNL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMT0_DR_alternate2")
            .field("drnl1", &self.drnl1())
            .field("drnl2", &self.drnl2())
            .finish()
    }
}
/**SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt0_dr_alternate2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SPDIFRX:FMT0_DR_alternate2)*/
pub struct FMT0_DR_ALTERNATE2rs;
impl crate::RegisterSpec for FMT0_DR_ALTERNATE2rs {
    type Ux = u32;
}
///`read()` method returns [`fmt0_dr_alternate2::R`](R) reader structure
impl crate::Readable for FMT0_DR_ALTERNATE2rs {}
///`reset()` method sets FMT0_DR_alternate2 to value 0
impl crate::Resettable for FMT0_DR_ALTERNATE2rs {}
