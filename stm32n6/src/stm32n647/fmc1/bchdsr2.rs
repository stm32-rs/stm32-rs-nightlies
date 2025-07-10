///Register `BCHDSR2` reader
pub type R = crate::R<BCHDSR2rs>;
///Field `EBP3` reader - Error bit position for error number 3
pub type EBP3_R = crate::FieldReader<u16>;
///Field `EBP4` reader - Error bit position for error number 4
pub type EBP4_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - Error bit position for error number 3
    #[inline(always)]
    pub fn ebp3(&self) -> EBP3_R {
        EBP3_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - Error bit position for error number 4
    #[inline(always)]
    pub fn ebp4(&self) -> EBP4_R {
        EBP4_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHDSR2")
            .field("ebp3", &self.ebp3())
            .field("ebp4", &self.ebp4())
            .finish()
    }
}
/**FMC BCH decoder status register for memory region 2

You can [`read`](crate::Reg::read) this register and get [`bchdsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FMC1:BCHDSR2)*/
pub struct BCHDSR2rs;
impl crate::RegisterSpec for BCHDSR2rs {
    type Ux = u32;
}
///`read()` method returns [`bchdsr2::R`](R) reader structure
impl crate::Readable for BCHDSR2rs {}
///`reset()` method sets BCHDSR2 to value 0
impl crate::Resettable for BCHDSR2rs {}
