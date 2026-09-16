///Register `DIVL` reader
pub type R = crate::R<DIVLrs>;
///Field `DIVL` reader - RTC prescaler divider register Low
pub type DIVL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - RTC prescaler divider register Low
    #[inline(always)]
    pub fn divl(&self) -> DIVL_R {
        DIVL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVL").field("divl", &self.divl()).finish()
    }
}
/**RTC Prescaler Divider Register Low

You can [`read`](crate::Reg::read) this register and get [`divl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:DIVL)*/
pub struct DIVLrs;
impl crate::RegisterSpec for DIVLrs {
    type Ux = u32;
}
///`read()` method returns [`divl::R`](R) reader structure
impl crate::Readable for DIVLrs {}
///`reset()` method sets DIVL to value 0x8000
impl crate::Resettable for DIVLrs {
    const RESET_VALUE: u32 = 0x8000;
}
