///Register `SECBB2R%s` reader
pub type R = crate::R<SECBB2Rrs>;
///Register `SECBB2R%s` writer
pub type W = crate::W<SECBB2Rrs>;
///Field `SECBB2` reader - SECBB2
pub type SECBB2_R = crate::FieldReader<u32>;
///Field `SECBB2` writer - SECBB2
pub type SECBB2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SECBB2
    #[inline(always)]
    pub fn secbb2(&self) -> SECBB2_R {
        SECBB2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECBB2R")
            .field("secbb2", &self.secbb2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SECBB2
    #[inline(always)]
    pub fn secbb2(&mut self) -> SECBB2_W<'_, SECBB2Rrs> {
        SECBB2_W::new(self, 0)
    }
}
/**FLASH secure block based bank 2

You can [`read`](crate::Reg::read) this register and get [`secbb2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#FLASH:SECBB2R[1])*/
pub struct SECBB2Rrs;
impl crate::RegisterSpec for SECBB2Rrs {
    type Ux = u32;
}
///`read()` method returns [`secbb2r::R`](R) reader structure
impl crate::Readable for SECBB2Rrs {}
///`write(|w| ..)` method takes [`secbb2r::W`](W) writer structure
impl crate::Writable for SECBB2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECBB2R%s to value 0
impl crate::Resettable for SECBB2Rrs {}
