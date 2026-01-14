///Register `AESLEPRIVHASHREG` reader
pub type R = crate::R<AESLEPRIVHASHREGrs>;
///Register `AESLEPRIVHASHREG` writer
pub type W = crate::W<AESLEPRIVHASHREGrs>;
///Field `HASH` reader - AES Le privacy Reference Hash
pub type HASH_R = crate::FieldReader<u32>;
///Field `HASH` writer - AES Le privacy Reference Hash
pub type HASH_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - AES Le privacy Reference Hash
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESLEPRIVHASHREG")
            .field("hash", &self.hash())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - AES Le privacy Reference Hash
    #[inline(always)]
    pub fn hash(&mut self) -> HASH_W<'_, AESLEPRIVHASHREGrs> {
        HASH_W::new(self, 0)
    }
}
/**AESLEPRIVHASHREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivhashreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivhashreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVHASHREG)*/
pub struct AESLEPRIVHASHREGrs;
impl crate::RegisterSpec for AESLEPRIVHASHREGrs {
    type Ux = u32;
}
///`read()` method returns [`aesleprivhashreg::R`](R) reader structure
impl crate::Readable for AESLEPRIVHASHREGrs {}
///`write(|w| ..)` method takes [`aesleprivhashreg::W`](W) writer structure
impl crate::Writable for AESLEPRIVHASHREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AESLEPRIVHASHREG to value 0
impl crate::Resettable for AESLEPRIVHASHREGrs {}
