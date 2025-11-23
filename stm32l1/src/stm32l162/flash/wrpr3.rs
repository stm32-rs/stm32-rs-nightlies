///Register `WRPR3` reader
pub type R = crate::R<WRPR3rs>;
///Register `WRPR3` writer
pub type W = crate::W<WRPR3rs>;
///Field `WRP3` reader - WRP3
pub type WRP3_R = crate::FieldReader<u32>;
///Field `WRP3` writer - WRP3
pub type WRP3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - WRP3
    #[inline(always)]
    pub fn wrp3(&self) -> WRP3_R {
        WRP3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPR3").field("wrp3", &self.wrp3()).finish()
    }
}
impl W {
    ///Bits 0:31 - WRP3
    #[inline(always)]
    pub fn wrp3(&mut self) -> WRP3_W<'_, WRPR3rs> {
        WRP3_W::new(self, 0)
    }
}
/**Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#Flash:WRPR3)*/
pub struct WRPR3rs;
impl crate::RegisterSpec for WRPR3rs {
    type Ux = u32;
}
///`read()` method returns [`wrpr3::R`](R) reader structure
impl crate::Readable for WRPR3rs {}
///`write(|w| ..)` method takes [`wrpr3::W`](W) writer structure
impl crate::Writable for WRPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPR3 to value 0
impl crate::Resettable for WRPR3rs {}
