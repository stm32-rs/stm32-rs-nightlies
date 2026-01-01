///Register `HASH_CSR1` reader
pub type R = crate::R<HASH_CSR1rs>;
///Register `HASH_CSR1` writer
pub type W = crate::W<HASH_CSR1rs>;
///Field `CS1` reader - Context swap x
pub type CS1_R = crate::FieldReader<u32>;
///Field `CS1` writer - Context swap x
pub type CS1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR1")
            .field("cs1", &self.cs1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs1(&mut self) -> CS1_W<'_, HASH_CSR1rs> {
        CS1_W::new(self, 0)
    }
}
/**HASH context swap register 1

You can [`read`](crate::Reg::read) this register and get [`hash_csr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR1)*/
pub struct HASH_CSR1rs;
impl crate::RegisterSpec for HASH_CSR1rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr1::R`](R) reader structure
impl crate::Readable for HASH_CSR1rs {}
///`write(|w| ..)` method takes [`hash_csr1::W`](W) writer structure
impl crate::Writable for HASH_CSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR1 to value 0
impl crate::Resettable for HASH_CSR1rs {}
