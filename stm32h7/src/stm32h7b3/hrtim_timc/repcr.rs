///Register `REPCR` reader
pub type R = crate::R<REPCRrs>;
///Register `REPCR` writer
pub type W = crate::W<REPCRrs>;
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
        f.debug_struct("REPCR").field("repx", &self.repx()).finish()
    }
}
impl W {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<REPCRrs> {
        REPX_W::new(self, 0)
    }
}
/**Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMC:REPCR)*/
pub struct REPCRrs;
impl crate::RegisterSpec for REPCRrs {
    type Ux = u32;
}
///`read()` method returns [`repcr::R`](R) reader structure
impl crate::Readable for REPCRrs {}
///`write(|w| ..)` method takes [`repcr::W`](W) writer structure
impl crate::Writable for REPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REPCR to value 0
impl crate::Resettable for REPCRrs {
    const RESET_VALUE: u32 = 0;
}
