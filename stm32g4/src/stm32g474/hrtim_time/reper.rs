///Register `REPER` reader
pub type R = crate::R<REPERrs>;
///Register `REPER` writer
pub type W = crate::W<REPERrs>;
///Field `REPx` reader - Timerx Repetition counter value
pub type REPX_R = crate::FieldReader;
///Field `REPx` writer - Timerx Repetition counter value
pub type REPX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REPER").field("repx", &self.repx()).finish()
    }
}
impl W {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<REPERrs> {
        REPX_W::new(self, 0)
    }
}
/**Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`reper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:REPER)*/
pub struct REPERrs;
impl crate::RegisterSpec for REPERrs {
    type Ux = u32;
}
///`read()` method returns [`reper::R`](R) reader structure
impl crate::Readable for REPERrs {}
///`write(|w| ..)` method takes [`reper::W`](W) writer structure
impl crate::Writable for REPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REPER to value 0
impl crate::Resettable for REPERrs {
    const RESET_VALUE: u32 = 0;
}
