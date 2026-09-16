///Register `HASH_CSR52` reader
pub type R = crate::R<HASH_CSR52rs>;
///Register `HASH_CSR52` writer
pub type W = crate::W<HASH_CSR52rs>;
///Field `CS52` reader - Context swap x
pub type CS52_R = crate::FieldReader<u32>;
///Field `CS52` writer - Context swap x
pub type CS52_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs52(&self) -> CS52_R {
        CS52_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR52")
            .field("cs52", &self.cs52())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs52(&mut self) -> CS52_W<'_, HASH_CSR52rs> {
        CS52_W::new(self, 0)
    }
}
/**HASH context swap register 52

You can [`read`](crate::Reg::read) this register and get [`hash_csr52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_CSR52)*/
pub struct HASH_CSR52rs;
impl crate::RegisterSpec for HASH_CSR52rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr52::R`](R) reader structure
impl crate::Readable for HASH_CSR52rs {}
///`write(|w| ..)` method takes [`hash_csr52::W`](W) writer structure
impl crate::Writable for HASH_CSR52rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR52 to value 0
impl crate::Resettable for HASH_CSR52rs {}
