///Register `WDATA` reader
pub type R = crate::R<WDATArs>;
///Register `WDATA` writer
pub type W = crate::W<WDATArs>;
///Field `ARG` reader - ARG
pub type ARG_R = crate::FieldReader<u32>;
///Field `ARG` writer - ARG
pub type ARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - ARG
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDATA").field("arg", &self.arg()).finish()
    }
}
impl W {
    ///Bits 0:31 - ARG
    #[inline(always)]
    pub fn arg(&mut self) -> ARG_W<'_, WDATArs> {
        ARG_W::new(self, 0)
    }
}
/**CORDIC argument register

You can [`read`](crate::Reg::read) this register and get [`wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#CORDIC:WDATA)*/
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
///`read()` method returns [`wdata::R`](R) reader structure
impl crate::Readable for WDATArs {}
///`write(|w| ..)` method takes [`wdata::W`](W) writer structure
impl crate::Writable for WDATArs {
    type Safety = crate::Safe;
}
///`reset()` method sets WDATA to value 0
impl crate::Resettable for WDATArs {}
