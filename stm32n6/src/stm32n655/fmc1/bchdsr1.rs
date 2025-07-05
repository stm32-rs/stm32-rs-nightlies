///Register `BCHDSR1` reader
pub type R = crate::R<BCHDSR1rs>;
///Field `EBP1` reader - Error bit position for error number 1
pub type EBP1_R = crate::FieldReader<u16>;
///Field `EBP2` reader - Error bit position for error number 2
pub type EBP2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - Error bit position for error number 1
    #[inline(always)]
    pub fn ebp1(&self) -> EBP1_R {
        EBP1_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - Error bit position for error number 2
    #[inline(always)]
    pub fn ebp2(&self) -> EBP2_R {
        EBP2_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHDSR1")
            .field("ebp1", &self.ebp1())
            .field("ebp2", &self.ebp2())
            .finish()
    }
}
/**FMC BCH decoder status register for memory region 1

You can [`read`](crate::Reg::read) this register and get [`bchdsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FMC1:BCHDSR1)*/
pub struct BCHDSR1rs;
impl crate::RegisterSpec for BCHDSR1rs {
    type Ux = u32;
}
///`read()` method returns [`bchdsr1::R`](R) reader structure
impl crate::Readable for BCHDSR1rs {}
///`reset()` method sets BCHDSR1 to value 0
impl crate::Resettable for BCHDSR1rs {}
