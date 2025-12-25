///Register `WRPR1` reader
pub type R = crate::R<WRPR1rs>;
///Register `WRPR1` writer
pub type W = crate::W<WRPR1rs>;
///Field `WRP1` reader - Write protection
pub type WRP1_R = crate::FieldReader<u32>;
///Field `WRP1` writer - Write protection
pub type WRP1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Write protection
    #[inline(always)]
    pub fn wrp1(&self) -> WRP1_R {
        WRP1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPR1").field("wrp1", &self.wrp1()).finish()
    }
}
impl W {
    ///Bits 0:31 - Write protection
    #[inline(always)]
    pub fn wrp1(&mut self) -> WRP1_W<'_, WRPR1rs> {
        WRP1_W::new(self, 0)
    }
}
/**Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#Flash:WRPR1)*/
pub struct WRPR1rs;
impl crate::RegisterSpec for WRPR1rs {
    type Ux = u32;
}
///`read()` method returns [`wrpr1::R`](R) reader structure
impl crate::Readable for WRPR1rs {}
///`write(|w| ..)` method takes [`wrpr1::W`](W) writer structure
impl crate::Writable for WRPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPR1 to value 0
impl crate::Resettable for WRPR1rs {}
