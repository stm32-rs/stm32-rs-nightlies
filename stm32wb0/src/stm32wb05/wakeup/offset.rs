///Register `OFFSET` reader
pub type R = crate::R<OFFSETrs>;
///Register `OFFSET` writer
pub type W = crate::W<OFFSETrs>;
///Field `WAKEUP_OFFSET` reader - delay of anticipation of the Soc device to settle power and clock
pub type WAKEUP_OFFSET_R = crate::FieldReader;
///Field `WAKEUP_OFFSET` writer - delay of anticipation of the Soc device to settle power and clock
pub type WAKEUP_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - delay of anticipation of the Soc device to settle power and clock
    #[inline(always)]
    pub fn wakeup_offset(&self) -> WAKEUP_OFFSET_R {
        WAKEUP_OFFSET_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFFSET")
            .field("wakeup_offset", &self.wakeup_offset())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - delay of anticipation of the Soc device to settle power and clock
    #[inline(always)]
    pub fn wakeup_offset(&mut self) -> WAKEUP_OFFSET_W<'_, OFFSETrs> {
        WAKEUP_OFFSET_W::new(self, 0)
    }
}
/**WAKEUP_OFFSET register

You can [`read`](crate::Reg::read) this register and get [`offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:OFFSET)*/
pub struct OFFSETrs;
impl crate::RegisterSpec for OFFSETrs {
    type Ux = u32;
}
///`read()` method returns [`offset::R`](R) reader structure
impl crate::Readable for OFFSETrs {}
///`write(|w| ..)` method takes [`offset::W`](W) writer structure
impl crate::Writable for OFFSETrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFFSET to value 0
impl crate::Resettable for OFFSETrs {}
