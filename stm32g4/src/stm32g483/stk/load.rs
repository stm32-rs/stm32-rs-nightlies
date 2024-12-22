///Register `LOAD` reader
pub type R = crate::R<LOADrs>;
///Register `LOAD` writer
pub type W = crate::W<LOADrs>;
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
        f.debug_struct("LOAD")
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<LOADrs> {
        RELOAD_W::new(self, 0)
    }
}
/**SysTick reload value register

You can [`read`](crate::Reg::read) this register and get [`load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483xx.html#STK:LOAD)*/
pub struct LOADrs;
impl crate::RegisterSpec for LOADrs {
    type Ux = u32;
}
///`read()` method returns [`load::R`](R) reader structure
impl crate::Readable for LOADrs {}
///`write(|w| ..)` method takes [`load::W`](W) writer structure
impl crate::Writable for LOADrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOAD to value 0
impl crate::Resettable for LOADrs {
    const RESET_VALUE: u32 = 0;
}
