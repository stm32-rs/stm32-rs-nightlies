///Register `UR11` reader
pub type R = crate::R<UR11rs>;
///Field `SA_END_2` reader - Secured area end address for bank 2
pub type SA_END_2_R = crate::FieldReader<u16>;
///Field `IWDG1M` reader - Independent Watchdog 1 mode
pub type IWDG1M_R = crate::BitReader;
impl R {
    ///Bits 0:11 - Secured area end address for bank 2
    #[inline(always)]
    pub fn sa_end_2(&self) -> SA_END_2_R {
        SA_END_2_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 16 - Independent Watchdog 1 mode
    #[inline(always)]
    pub fn iwdg1m(&self) -> IWDG1M_R {
        IWDG1M_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR11")
            .field("sa_end_2", &self.sa_end_2())
            .field("iwdg1m", &self.iwdg1m())
            .finish()
    }
}
/**SYSCFG user register 11

You can [`read`](crate::Reg::read) this register and get [`ur11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#SYSCFG:UR11)*/
pub struct UR11rs;
impl crate::RegisterSpec for UR11rs {
    type Ux = u32;
}
///`read()` method returns [`ur11::R`](R) reader structure
impl crate::Readable for UR11rs {}
///`reset()` method sets UR11 to value 0
impl crate::Resettable for UR11rs {}
