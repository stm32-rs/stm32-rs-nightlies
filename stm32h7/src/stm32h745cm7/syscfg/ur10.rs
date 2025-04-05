///Register `UR10` reader
pub type R = crate::R<UR10rs>;
///Field `PA_END_2` reader - Protected area end address for bank 2
pub type PA_END_2_R = crate::FieldReader<u16>;
///Field `SA_BEG_2` reader - Secured area start address for bank 2
pub type SA_BEG_2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Protected area end address for bank 2
    #[inline(always)]
    pub fn pa_end_2(&self) -> PA_END_2_R {
        PA_END_2_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Secured area start address for bank 2
    #[inline(always)]
    pub fn sa_beg_2(&self) -> SA_BEG_2_R {
        SA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR10")
            .field("pa_end_2", &self.pa_end_2())
            .field("sa_beg_2", &self.sa_beg_2())
            .finish()
    }
}
/**SYSCFG user register 10

You can [`read`](crate::Reg::read) this register and get [`ur10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#SYSCFG:UR10)*/
pub struct UR10rs;
impl crate::RegisterSpec for UR10rs {
    type Ux = u32;
}
///`read()` method returns [`ur10::R`](R) reader structure
impl crate::Readable for UR10rs {}
///`reset()` method sets UR10 to value 0
impl crate::Resettable for UR10rs {}
