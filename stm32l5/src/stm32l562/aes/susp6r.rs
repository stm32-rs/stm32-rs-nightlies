///Register `SUSP6R` reader
pub type R = crate::R<SUSP6Rrs>;
///Register `SUSP6R` writer
pub type W = crate::W<SUSP6Rrs>;
///Field `SUSP` reader - AES suspend register 6
pub type SUSP_R = crate::FieldReader<u32>;
///Field `SUSP` writer - AES suspend register 6
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - AES suspend register 6
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP6R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES suspend register 6
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, SUSP6Rrs> {
        SUSP_W::new(self, 0)
    }
}
/**AES suspend register 6

You can [`read`](crate::Reg::read) this register and get [`susp6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#AES:SUSP6R)*/
pub struct SUSP6Rrs;
impl crate::RegisterSpec for SUSP6Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp6r::R`](R) reader structure
impl crate::Readable for SUSP6Rrs {}
///`write(|w| ..)` method takes [`susp6r::W`](W) writer structure
impl crate::Writable for SUSP6Rrs {
    type Safety = crate::Safe;
}
///`reset()` method sets SUSP6R to value 0
impl crate::Resettable for SUSP6Rrs {}
