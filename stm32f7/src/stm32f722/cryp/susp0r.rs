///Register `SUSP0R` reader
pub type R = crate::R<SUSP0Rrs>;
///Register `SUSP0R` writer
pub type W = crate::W<SUSP0Rrs>;
///Field `SUSP0R` reader - AES Suspend
pub type SUSP0R_R = crate::FieldReader<u32>;
///Field `SUSP0R` writer - AES Suspend
pub type SUSP0R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES Suspend
    #[inline(always)]
    pub fn susp0r(&self) -> SUSP0R_R {
        SUSP0R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP0R")
            .field("susp0r", &self.susp0r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES Suspend
    #[inline(always)]
    pub fn susp0r(&mut self) -> SUSP0R_W<'_, SUSP0Rrs> {
        SUSP0R_W::new(self, 0)
    }
}
/**Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F722.html#CRYP:SUSP0R)*/
pub struct SUSP0Rrs;
impl crate::RegisterSpec for SUSP0Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp0r::R`](R) reader structure
impl crate::Readable for SUSP0Rrs {}
///`write(|w| ..)` method takes [`susp0r::W`](W) writer structure
impl crate::Writable for SUSP0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUSP0R to value 0
impl crate::Resettable for SUSP0Rrs {}
