///Register `LPUART_PRESC` reader
pub type R = crate::R<LPUART_PRESCrs>;
///Register `LPUART_PRESC` writer
pub type W = crate::W<LPUART_PRESCrs>;
///Field `PRESCALER` reader - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_PRESC")
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<LPUART_PRESCrs> {
        PRESCALER_W::new(self, 0)
    }
}
/**LPUART prescaler register

You can [`read`](crate::Reg::read) this register and get [`lpuart_presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_PRESC)*/
pub struct LPUART_PRESCrs;
impl crate::RegisterSpec for LPUART_PRESCrs {
    type Ux = u32;
}
///`read()` method returns [`lpuart_presc::R`](R) reader structure
impl crate::Readable for LPUART_PRESCrs {}
///`write(|w| ..)` method takes [`lpuart_presc::W`](W) writer structure
impl crate::Writable for LPUART_PRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPUART_PRESC to value 0
impl crate::Resettable for LPUART_PRESCrs {
    const RESET_VALUE: u32 = 0;
}
