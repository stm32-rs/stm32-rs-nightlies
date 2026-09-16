///Register `SUSP7R` reader
pub type R = crate::R<SUSP7Rrs>;
///Register `SUSP7R` writer
pub type W = crate::W<SUSP7Rrs>;
///Field `SUSP7R` reader - AES Suspend
pub type SUSP7R_R = crate::FieldReader<u32>;
///Field `SUSP7R` writer - AES Suspend
pub type SUSP7R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES Suspend
    #[inline(always)]
    pub fn susp7r(&self) -> SUSP7R_R {
        SUSP7R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP7R")
            .field("susp7r", &self.susp7r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES Suspend
    #[inline(always)]
    pub fn susp7r(&mut self) -> SUSP7R_W<'_, SUSP7Rrs> {
        SUSP7R_W::new(self, 0)
    }
}
/**Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp7r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp7r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F722.html#CRYP:SUSP7R)*/
pub struct SUSP7Rrs;
impl crate::RegisterSpec for SUSP7Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp7r::R`](R) reader structure
impl crate::Readable for SUSP7Rrs {}
///`write(|w| ..)` method takes [`susp7r::W`](W) writer structure
impl crate::Writable for SUSP7Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUSP7R to value 0
impl crate::Resettable for SUSP7Rrs {}
