///Register `SUSP4R` reader
pub type R = crate::R<SUSP4Rrs>;
///Register `SUSP4R` writer
pub type W = crate::W<SUSP4Rrs>;
///Field `AES_SUSP4R` reader - AES suspend register 4
pub type AES_SUSP4R_R = crate::FieldReader<u32>;
///Field `AES_SUSP4R` writer - AES suspend register 4
pub type AES_SUSP4R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES suspend register 4
    #[inline(always)]
    pub fn aes_susp4r(&self) -> AES_SUSP4R_R {
        AES_SUSP4R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP4R")
            .field("aes_susp4r", &self.aes_susp4r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES suspend register 4
    #[inline(always)]
    pub fn aes_susp4r(&mut self) -> AES_SUSP4R_W<SUSP4Rrs> {
        AES_SUSP4R_W::new(self, 0)
    }
}
/**AES suspend register 4

You can [`read`](crate::Reg::read) this register and get [`susp4r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp4r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#AES:SUSP4R)*/
pub struct SUSP4Rrs;
impl crate::RegisterSpec for SUSP4Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp4r::R`](R) reader structure
impl crate::Readable for SUSP4Rrs {}
///`write(|w| ..)` method takes [`susp4r::W`](W) writer structure
impl crate::Writable for SUSP4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP4R to value 0
impl crate::Resettable for SUSP4Rrs {
    const RESET_VALUE: u32 = 0;
}
