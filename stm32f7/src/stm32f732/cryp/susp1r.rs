///Register `SUSP1R` reader
pub type R = crate::R<SUSP1Rrs>;
///Register `SUSP1R` writer
pub type W = crate::W<SUSP1Rrs>;
///Field `SUSP1R` reader - AES Suspend
pub type SUSP1R_R = crate::FieldReader<u32>;
///Field `SUSP1R` writer - AES Suspend
pub type SUSP1R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES Suspend
    #[inline(always)]
    pub fn susp1r(&self) -> SUSP1R_R {
        SUSP1R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP1R")
            .field("susp1r", &self.susp1r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES Suspend
    #[inline(always)]
    pub fn susp1r(&mut self) -> SUSP1R_W<'_, SUSP1Rrs> {
        SUSP1R_W::new(self, 0)
    }
}
/**Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#CRYP:SUSP1R)*/
pub struct SUSP1Rrs;
impl crate::RegisterSpec for SUSP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp1r::R`](R) reader structure
impl crate::Readable for SUSP1Rrs {}
///`write(|w| ..)` method takes [`susp1r::W`](W) writer structure
impl crate::Writable for SUSP1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUSP1R to value 0
impl crate::Resettable for SUSP1Rrs {}
