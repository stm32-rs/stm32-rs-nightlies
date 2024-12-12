///Register `PRESC` reader
pub type R = crate::R<PRESCrs>;
///Register `PRESC` writer
pub type W = crate::W<PRESCrs>;
///Field `PRESCALER` reader - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESC")
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<PRESCrs> {
        PRESCALER_W::new(self, 0)
    }
}
/**USART prescaler register

You can [`read`](crate::Reg::read) this register and get [`presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USART1:PRESC)*/
pub struct PRESCrs;
impl crate::RegisterSpec for PRESCrs {
    type Ux = u32;
}
///`read()` method returns [`presc::R`](R) reader structure
impl crate::Readable for PRESCrs {}
///`write(|w| ..)` method takes [`presc::W`](W) writer structure
impl crate::Writable for PRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRESC to value 0
impl crate::Resettable for PRESCrs {
    const RESET_VALUE: u32 = 0;
}
