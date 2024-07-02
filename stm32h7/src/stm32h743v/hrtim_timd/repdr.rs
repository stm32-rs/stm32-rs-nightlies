///Register `REPDR` reader
pub type R = crate::R<REPDRrs>;
///Register `REPDR` writer
pub type W = crate::W<REPDRrs>;
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
        f.debug_struct("REPDR").field("repx", &self.repx()).finish()
    }
}
impl W {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<REPDRrs> {
        REPX_W::new(self, 0)
    }
}
/**Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMD:REPDR)*/
pub struct REPDRrs;
impl crate::RegisterSpec for REPDRrs {
    type Ux = u32;
}
///`read()` method returns [`repdr::R`](R) reader structure
impl crate::Readable for REPDRrs {}
///`write(|w| ..)` method takes [`repdr::W`](W) writer structure
impl crate::Writable for REPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REPDR to value 0
impl crate::Resettable for REPDRrs {
    const RESET_VALUE: u32 = 0;
}
