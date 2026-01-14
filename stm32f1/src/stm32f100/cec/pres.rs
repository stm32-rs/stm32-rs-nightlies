///Register `PRES` reader
pub type R = crate::R<PRESrs>;
///Register `PRES` writer
pub type W = crate::W<PRESrs>;
///Field `PRESC` reader - CEC Rx Data Register
pub type PRESC_R = crate::FieldReader<u16>;
///Field `PRESC` writer - CEC Rx Data Register
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - CEC Rx Data Register
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRES")
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - CEC Rx Data Register
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, PRESrs> {
        PRESC_W::new(self, 0)
    }
}
/**Rx Data Register

You can [`read`](crate::Reg::read) this register and get [`pres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:PRES)*/
pub struct PRESrs;
impl crate::RegisterSpec for PRESrs {
    type Ux = u32;
}
///`read()` method returns [`pres::R`](R) reader structure
impl crate::Readable for PRESrs {}
///`write(|w| ..)` method takes [`pres::W`](W) writer structure
impl crate::Writable for PRESrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRES to value 0
impl crate::Resettable for PRESrs {}
