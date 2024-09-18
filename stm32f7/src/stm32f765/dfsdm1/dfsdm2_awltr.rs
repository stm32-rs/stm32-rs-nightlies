///Register `DFSDM2_AWLTR` reader
pub type R = crate::R<DFSDM2_AWLTRrs>;
///Register `DFSDM2_AWLTR` writer
pub type W = crate::W<DFSDM2_AWLTRrs>;
///Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_R = crate::FieldReader;
///Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWLT` reader - Analog watchdog low threshold
pub type AWLT_R = crate::FieldReader<u32>;
///Field `AWLT` writer - Analog watchdog low threshold
pub type AWLT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM2_AWLTR")
            .field("bkawl", &self.bkawl())
            .field("awlt", &self.awlt())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    #[must_use]
    pub fn bkawl(&mut self) -> BKAWL_W<DFSDM2_AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    #[must_use]
    pub fn awlt(&mut self) -> AWLT_W<DFSDM2_AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
/**DFSDM analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm2_awltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm2_awltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#DFSDM1:DFSDM2_AWLTR)*/
pub struct DFSDM2_AWLTRrs;
impl crate::RegisterSpec for DFSDM2_AWLTRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm2_awltr::R`](R) reader structure
impl crate::Readable for DFSDM2_AWLTRrs {}
///`write(|w| ..)` method takes [`dfsdm2_awltr::W`](W) writer structure
impl crate::Writable for DFSDM2_AWLTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM2_AWLTR to value 0
impl crate::Resettable for DFSDM2_AWLTRrs {
    const RESET_VALUE: u32 = 0;
}
