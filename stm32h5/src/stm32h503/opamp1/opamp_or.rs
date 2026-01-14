///Register `OPAMP_OR` reader
pub type R = crate::R<OPAMP_ORrs>;
///Register `OPAMP_OR` writer
pub type W = crate::W<OPAMP_ORrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**OPAMP option register

You can [`read`](crate::Reg::read) this register and get [`opamp_or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#OPAMP1:OPAMP_OR)*/
pub struct OPAMP_ORrs;
impl crate::RegisterSpec for OPAMP_ORrs {
    type Ux = u32;
}
///`read()` method returns [`opamp_or::R`](R) reader structure
impl crate::Readable for OPAMP_ORrs {}
///`write(|w| ..)` method takes [`opamp_or::W`](W) writer structure
impl crate::Writable for OPAMP_ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP_OR to value 0
impl crate::Resettable for OPAMP_ORrs {}
