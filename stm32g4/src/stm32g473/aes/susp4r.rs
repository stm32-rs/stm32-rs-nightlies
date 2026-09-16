///Register `SUSP4R` reader
pub type R = crate::R<SUSP4Rrs>;
///Register `SUSP4R` writer
pub type W = crate::W<SUSP4Rrs>;
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
        f.debug_struct("SUSP4R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, SUSP4Rrs> {
        SUSP_W::new(self, 0)
    }
}
/**suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp4r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp4r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#AES:SUSP4R)*/
pub struct SUSP4Rrs;
impl crate::RegisterSpec for SUSP4Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp4r::R`](R) reader structure
impl crate::Readable for SUSP4Rrs {}
///`write(|w| ..)` method takes [`susp4r::W`](W) writer structure
impl crate::Writable for SUSP4Rrs {
    type Safety = crate::Safe;
}
///`reset()` method sets SUSP4R to value 0
impl crate::Resettable for SUSP4Rrs {}
