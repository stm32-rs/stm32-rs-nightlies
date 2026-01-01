///Register `C2ICR` reader
pub type R = crate::R<C2ICRrs>;
///Register `C2ICR` writer
pub type W = crate::W<C2ICRrs>;
///Field `ISC` reader - ISC
pub type ISC_R = crate::FieldReader<u32>;
///Field `ISC` writer - ISC
pub type ISC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISC
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2ICR").field("isc", &self.isc()).finish()
    }
}
impl W {
    ///Bits 0:31 - ISC
    #[inline(always)]
    pub fn isc(&mut self) -> ISC_W<'_, C2ICRrs> {
        ISC_W::new(self, 0)
    }
}
/**HSEM i2terrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c2icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:C2ICR)*/
pub struct C2ICRrs;
impl crate::RegisterSpec for C2ICRrs {
    type Ux = u32;
}
///`read()` method returns [`c2icr::R`](R) reader structure
impl crate::Readable for C2ICRrs {}
///`write(|w| ..)` method takes [`c2icr::W`](W) writer structure
impl crate::Writable for C2ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2ICR to value 0
impl crate::Resettable for C2ICRrs {}
