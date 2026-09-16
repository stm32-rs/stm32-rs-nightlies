///Register `WRPR2` reader
pub type R = crate::R<WRPR2rs>;
///Register `WRPR2` writer
pub type W = crate::W<WRPR2rs>;
///Field `WRP2` reader - WRP2
pub type WRP2_R = crate::FieldReader<u32>;
///Field `WRP2` writer - WRP2
pub type WRP2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - WRP2
    #[inline(always)]
    pub fn wrp2(&self) -> WRP2_R {
        WRP2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPR2").field("wrp2", &self.wrp2()).finish()
    }
}
impl W {
    ///Bits 0:31 - WRP2
    #[inline(always)]
    pub fn wrp2(&mut self) -> WRP2_W<'_, WRPR2rs> {
        WRP2_W::new(self, 0)
    }
}
/**Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#Flash:WRPR2)*/
pub struct WRPR2rs;
impl crate::RegisterSpec for WRPR2rs {
    type Ux = u32;
}
///`read()` method returns [`wrpr2::R`](R) reader structure
impl crate::Readable for WRPR2rs {}
///`write(|w| ..)` method takes [`wrpr2::W`](W) writer structure
impl crate::Writable for WRPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPR2 to value 0
impl crate::Resettable for WRPR2rs {}
