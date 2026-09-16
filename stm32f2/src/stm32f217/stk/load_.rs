///Register `LOAD_` reader
pub type R = crate::R<LOAD_rs>;
///Register `LOAD_` writer
pub type W = crate::W<LOAD_rs>;
///Field `RELOAD` reader - RELOAD value
pub type RELOAD_R = crate::FieldReader<u32>;
///Field `RELOAD` writer - RELOAD value
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOAD_")
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<'_, LOAD_rs> {
        RELOAD_W::new(self, 0)
    }
}
/**SysTick reload value register

You can [`read`](crate::Reg::read) this register and get [`load_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#STK:LOAD_)*/
pub struct LOAD_rs;
impl crate::RegisterSpec for LOAD_rs {
    type Ux = u32;
}
///`read()` method returns [`load_::R`](R) reader structure
impl crate::Readable for LOAD_rs {}
///`write(|w| ..)` method takes [`load_::W`](W) writer structure
impl crate::Writable for LOAD_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOAD_ to value 0
impl crate::Resettable for LOAD_rs {}
