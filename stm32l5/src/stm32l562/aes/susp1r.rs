///Register `SUSP1R` reader
pub type R = crate::R<SUSP1Rrs>;
///Register `SUSP1R` writer
pub type W = crate::W<SUSP1Rrs>;
///Field `AES_SUSP1R` reader - AES suspend register 1
pub type AES_SUSP1R_R = crate::FieldReader<u32>;
///Field `AES_SUSP1R` writer - AES suspend register 1
pub type AES_SUSP1R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES suspend register 1
    #[inline(always)]
    pub fn aes_susp1r(&self) -> AES_SUSP1R_R {
        AES_SUSP1R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP1R")
            .field("aes_susp1r", &self.aes_susp1r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES suspend register 1
    #[inline(always)]
    #[must_use]
    pub fn aes_susp1r(&mut self) -> AES_SUSP1R_W<SUSP1Rrs> {
        AES_SUSP1R_W::new(self, 0)
    }
}
/**AES suspend register 1

You can [`read`](crate::Reg::read) this register and get [`susp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#AES:SUSP1R)*/
pub struct SUSP1Rrs;
impl crate::RegisterSpec for SUSP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp1r::R`](R) reader structure
impl crate::Readable for SUSP1Rrs {}
///`write(|w| ..)` method takes [`susp1r::W`](W) writer structure
impl crate::Writable for SUSP1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP1R to value 0
impl crate::Resettable for SUSP1Rrs {
    const RESET_VALUE: u32 = 0;
}
