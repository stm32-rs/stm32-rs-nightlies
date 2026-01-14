///Register `DR_10` reader
pub type R = crate::R<DR_10rs>;
///Field `DRNL1` reader - Data value
pub type DRNL1_R = crate::FieldReader<u16>;
///Field `DRNL2` reader - Data value
pub type DRNL2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Data value
    #[inline(always)]
    pub fn drnl1(&self) -> DRNL1_R {
        DRNL1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Data value
    #[inline(always)]
    pub fn drnl2(&self) -> DRNL2_R {
        DRNL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR_10")
            .field("drnl1", &self.drnl1())
            .field("drnl2", &self.drnl2())
            .finish()
    }
}
/**Data input register

You can [`read`](crate::Reg::read) this register and get [`dr_10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#SPDIFRX:DR_10)*/
pub struct DR_10rs;
impl crate::RegisterSpec for DR_10rs {
    type Ux = u32;
}
///`read()` method returns [`dr_10::R`](R) reader structure
impl crate::Readable for DR_10rs {}
///`reset()` method sets DR_10 to value 0
impl crate::Resettable for DR_10rs {}
