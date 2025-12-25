///Register `SECBB1R%s` reader
pub type R = crate::R<SECBB1Rrs>;
///Register `SECBB1R%s` writer
pub type W = crate::W<SECBB1Rrs>;
///Field `SECBB1` reader - SECBB1
pub type SECBB1_R = crate::FieldReader<u32>;
///Field `SECBB1` writer - SECBB1
pub type SECBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SECBB1
    #[inline(always)]
    pub fn secbb1(&self) -> SECBB1_R {
        SECBB1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECBB1R")
            .field("secbb1", &self.secbb1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SECBB1
    #[inline(always)]
    pub fn secbb1(&mut self) -> SECBB1_W<'_, SECBB1Rrs> {
        SECBB1_W::new(self, 0)
    }
}
/**FLASH secure block based bank 1

You can [`read`](crate::Reg::read) this register and get [`secbb1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECBB1R[1])*/
pub struct SECBB1Rrs;
impl crate::RegisterSpec for SECBB1Rrs {
    type Ux = u32;
}
///`read()` method returns [`secbb1r::R`](R) reader structure
impl crate::Readable for SECBB1Rrs {}
///`write(|w| ..)` method takes [`secbb1r::W`](W) writer structure
impl crate::Writable for SECBB1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECBB1R%s to value 0
impl crate::Resettable for SECBB1Rrs {}
