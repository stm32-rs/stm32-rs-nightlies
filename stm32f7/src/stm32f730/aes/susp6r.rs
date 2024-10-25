///Register `SUSP6R` reader
pub type R = crate::R<SUSP6Rrs>;
///Register `SUSP6R` writer
pub type W = crate::W<SUSP6Rrs>;
///Field `AES_SUSP6R` reader - AES suspend register 6
pub type AES_SUSP6R_R = crate::FieldReader<u32>;
///Field `AES_SUSP6R` writer - AES suspend register 6
pub type AES_SUSP6R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES suspend register 6
    #[inline(always)]
    pub fn aes_susp6r(&self) -> AES_SUSP6R_R {
        AES_SUSP6R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP6R")
            .field("aes_susp6r", &self.aes_susp6r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES suspend register 6
    #[inline(always)]
    #[must_use]
    pub fn aes_susp6r(&mut self) -> AES_SUSP6R_W<SUSP6Rrs> {
        AES_SUSP6R_W::new(self, 0)
    }
}
/**AES suspend register 6

You can [`read`](crate::Reg::read) this register and get [`susp6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#AES:SUSP6R)*/
pub struct SUSP6Rrs;
impl crate::RegisterSpec for SUSP6Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp6r::R`](R) reader structure
impl crate::Readable for SUSP6Rrs {}
///`write(|w| ..)` method takes [`susp6r::W`](W) writer structure
impl crate::Writable for SUSP6Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP6R to value 0
impl crate::Resettable for SUSP6Rrs {
    const RESET_VALUE: u32 = 0;
}
