///Register `SUSP3R` reader
pub type R = crate::R<SUSP3Rrs>;
///Register `SUSP3R` writer
pub type W = crate::W<SUSP3Rrs>;
///Field `SUSP3R` reader - IV127
pub type SUSP3R_R = crate::FieldReader<u32>;
///Field `SUSP3R` writer - IV127
pub type SUSP3R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IV127
    #[inline(always)]
    pub fn susp3r(&self) -> SUSP3R_R {
        SUSP3R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP3R")
            .field("susp3r", &self.susp3r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IV127
    #[inline(always)]
    pub fn susp3r(&mut self) -> SUSP3R_W<'_, SUSP3Rrs> {
        SUSP3R_W::new(self, 0)
    }
}
/**Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F722.html#CRYP:SUSP3R)*/
pub struct SUSP3Rrs;
impl crate::RegisterSpec for SUSP3Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp3r::R`](R) reader structure
impl crate::Readable for SUSP3Rrs {}
///`write(|w| ..)` method takes [`susp3r::W`](W) writer structure
impl crate::Writable for SUSP3Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUSP3R to value 0
impl crate::Resettable for SUSP3Rrs {}
