///Register `SUSP5R` reader
pub type R = crate::R<SUSP5Rrs>;
///Register `SUSP5R` writer
pub type W = crate::W<SUSP5Rrs>;
///Field `SUSP` reader - AES suspend
pub type SUSP_R = crate::FieldReader<u32>;
///Field `SUSP` writer - AES suspend
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP5R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<SUSP5Rrs> {
        SUSP_W::new(self, 0)
    }
}
/**suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp5r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp5r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#AES:SUSP5R)*/
pub struct SUSP5Rrs;
impl crate::RegisterSpec for SUSP5Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp5r::R`](R) reader structure
impl crate::Readable for SUSP5Rrs {}
///`write(|w| ..)` method takes [`susp5r::W`](W) writer structure
impl crate::Writable for SUSP5Rrs {
    type Safety = crate::Safe;
}
///`reset()` method sets SUSP5R to value 0
impl crate::Resettable for SUSP5Rrs {}
