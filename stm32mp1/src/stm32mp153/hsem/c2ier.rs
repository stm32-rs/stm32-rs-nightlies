///Register `C2IER` reader
pub type R = crate::R<C2IERrs>;
///Register `C2IER` writer
pub type W = crate::W<C2IERrs>;
///Field `ISE` reader - ISE
pub type ISE_R = crate::FieldReader<u32>;
///Field `ISE` writer - ISE
pub type ISE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISE
    #[inline(always)]
    pub fn ise(&self) -> ISE_R {
        ISE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2IER").field("ise", &self.ise()).finish()
    }
}
impl W {
    ///Bits 0:31 - ISE
    #[inline(always)]
    pub fn ise(&mut self) -> ISE_W<'_, C2IERrs> {
        ISE_W::new(self, 0)
    }
}
/**HSEM i2terrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c2ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C2IER)*/
pub struct C2IERrs;
impl crate::RegisterSpec for C2IERrs {
    type Ux = u32;
}
///`read()` method returns [`c2ier::R`](R) reader structure
impl crate::Readable for C2IERrs {}
///`write(|w| ..)` method takes [`c2ier::W`](W) writer structure
impl crate::Writable for C2IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2IER to value 0
impl crate::Resettable for C2IERrs {}
