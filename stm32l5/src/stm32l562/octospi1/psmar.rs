///Register `PSMAR` reader
pub type R = crate::R<PSMARrs>;
///Register `PSMAR` writer
pub type W = crate::W<PSMARrs>;
///Field `INTERVAL` reader - Polling interval
pub type INTERVAL_R = crate::FieldReader<u16>;
///Field `INTERVAL` writer - Polling interval
pub type INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Polling interval
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSMAR")
            .field("interval", &self.interval())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Polling interval
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W<PSMARrs> {
        INTERVAL_W::new(self, 0)
    }
}
/**polling status match register

You can [`read`](crate::Reg::read) this register and get [`psmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#OCTOSPI1:PSMAR)*/
pub struct PSMARrs;
impl crate::RegisterSpec for PSMARrs {
    type Ux = u32;
}
///`read()` method returns [`psmar::R`](R) reader structure
impl crate::Readable for PSMARrs {}
///`write(|w| ..)` method takes [`psmar::W`](W) writer structure
impl crate::Writable for PSMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PSMAR to value 0
impl crate::Resettable for PSMARrs {
    const RESET_VALUE: u32 = 0;
}
