///Register `C1ICR` reader
pub type R = crate::R<C1ICRrs>;
///Register `C1ICR` writer
pub type W = crate::W<C1ICRrs>;
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
        f.debug_struct("C1ICR").field("isc", &self.isc()).finish()
    }
}
impl W {
    ///Bits 0:31 - ISC
    #[inline(always)]
    pub fn isc(&mut self) -> ISC_W<'_, C1ICRrs> {
        ISC_W::new(self, 0)
    }
}
/**HSEM i1terrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c1icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C1ICR)*/
pub struct C1ICRrs;
impl crate::RegisterSpec for C1ICRrs {
    type Ux = u32;
}
///`read()` method returns [`c1icr::R`](R) reader structure
impl crate::Readable for C1ICRrs {}
///`write(|w| ..)` method takes [`c1icr::W`](W) writer structure
impl crate::Writable for C1ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1ICR to value 0
impl crate::Resettable for C1ICRrs {}
