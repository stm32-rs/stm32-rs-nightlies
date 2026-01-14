///Register `UR6` reader
pub type R = crate::R<UR6rs>;
///Field `PA_BEG_1` reader - Protected area start address for bank 1
pub type PA_BEG_1_R = crate::FieldReader<u16>;
///Field `PA_END_1` reader - Protected area end address for bank 1
pub type PA_END_1_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Protected area start address for bank 1
    #[inline(always)]
    pub fn pa_beg_1(&self) -> PA_BEG_1_R {
        PA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Protected area end address for bank 1
    #[inline(always)]
    pub fn pa_end_1(&self) -> PA_END_1_R {
        PA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR6")
            .field("pa_beg_1", &self.pa_beg_1())
            .field("pa_end_1", &self.pa_end_1())
            .finish()
    }
}
/**SYSCFG user register 6

You can [`read`](crate::Reg::read) this register and get [`ur6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#SYSCFG:UR6)*/
pub struct UR6rs;
impl crate::RegisterSpec for UR6rs {
    type Ux = u32;
}
///`read()` method returns [`ur6::R`](R) reader structure
impl crate::Readable for UR6rs {}
///`reset()` method sets UR6 to value 0
impl crate::Resettable for UR6rs {}
