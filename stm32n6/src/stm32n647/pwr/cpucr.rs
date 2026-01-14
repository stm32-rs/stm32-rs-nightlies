///Register `CPUCR` reader
pub type R = crate::R<CPUCRrs>;
///Register `CPUCR` writer
pub type W = crate::W<CPUCRrs>;
///Field `PDDS` reader - Power-down deepsleep selection
pub type PDDS_R = crate::BitReader;
///Field `PDDS` writer - Power-down deepsleep selection
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSF` reader - Clear Standby and Stop flags (always read as 0)
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - Clear Standby and Stop flags (always read as 0)
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - Stop flag
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader;
///Field `SVOS` reader - System Stop mode voltage scaling selection
pub type SVOS_R = crate::BitReader;
///Field `SVOS` writer - System Stop mode voltage scaling selection
pub type SVOS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power-down deepsleep selection
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear Standby and Stop flags (always read as 0)
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Stop flag
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - System Stop mode voltage scaling selection
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUCR")
            .field("pdds", &self.pdds())
            .field("cssf", &self.cssf())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .field("svos", &self.svos())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power-down deepsleep selection
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CPUCRrs> {
        PDDS_W::new(self, 0)
    }
    ///Bit 1 - Clear Standby and Stop flags (always read as 0)
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, CPUCRrs> {
        CSSF_W::new(self, 1)
    }
    ///Bit 16 - System Stop mode voltage scaling selection
    #[inline(always)]
    pub fn svos(&mut self) -> SVOS_W<'_, CPUCRrs> {
        SVOS_W::new(self, 16)
    }
}
/**PWR CPU control register

You can [`read`](crate::Reg::read) this register and get [`cpucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#PWR:CPUCR)*/
pub struct CPUCRrs;
impl crate::RegisterSpec for CPUCRrs {
    type Ux = u32;
}
///`read()` method returns [`cpucr::R`](R) reader structure
impl crate::Readable for CPUCRrs {}
///`write(|w| ..)` method takes [`cpucr::W`](W) writer structure
impl crate::Writable for CPUCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPUCR to value 0x0001_0000
impl crate::Resettable for CPUCRrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
