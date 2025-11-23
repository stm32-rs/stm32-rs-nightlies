///Register `ARG` reader
pub type R = crate::R<ARGrs>;
///Register `ARG` writer
pub type W = crate::W<ARGrs>;
///Field `CMDARG` reader - Command argument
pub type CMDARG_R = crate::FieldReader<u32>;
///Field `CMDARG` writer - Command argument
pub type CMDARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Command argument
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARG")
            .field("cmdarg", &self.cmdarg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Command argument
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CMDARG_W<'_, ARGrs> {
        CMDARG_W::new(self, 0)
    }
}
/**argument register

You can [`read`](crate::Reg::read) this register and get [`arg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#SDMMC1:ARG)*/
pub struct ARGrs;
impl crate::RegisterSpec for ARGrs {
    type Ux = u32;
}
///`read()` method returns [`arg::R`](R) reader structure
impl crate::Readable for ARGrs {}
///`write(|w| ..)` method takes [`arg::W`](W) writer structure
impl crate::Writable for ARGrs {
    type Safety = crate::Safe;
}
///`reset()` method sets ARG to value 0
impl crate::Resettable for ARGrs {}
