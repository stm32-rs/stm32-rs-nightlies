///Register `VVACCR` reader
pub type R = crate::R<VVACCRrs>;
///Register `VVACCR` writer
pub type W = crate::W<VVACCRrs>;
///Field `VA` reader - Vertical Active duration
pub type VA_R = crate::FieldReader<u16>;
///Field `VA` writer - Vertical Active duration
pub type VA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Vertical Active duration
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVACCR").field("va", &self.va()).finish()
    }
}
impl W {
    ///Bits 0:13 - Vertical Active duration
    #[inline(always)]
    pub fn va(&mut self) -> VA_W<VVACCRrs> {
        VA_W::new(self, 0)
    }
}
/**DSI Host Video VA Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvaccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvaccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VVACCR)*/
pub struct VVACCRrs;
impl crate::RegisterSpec for VVACCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvaccr::R`](R) reader structure
impl crate::Readable for VVACCRrs {}
///`write(|w| ..)` method takes [`vvaccr::W`](W) writer structure
impl crate::Writable for VVACCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VVACCR to value 0
impl crate::Resettable for VVACCRrs {
    const RESET_VALUE: u32 = 0;
}
