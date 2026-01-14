///Register `HASH_CSR9` reader
pub type R = crate::R<HASH_CSR9rs>;
///Register `HASH_CSR9` writer
pub type W = crate::W<HASH_CSR9rs>;
///Field `CS9` reader - Context swap x
pub type CS9_R = crate::FieldReader<u32>;
///Field `CS9` writer - Context swap x
pub type CS9_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs9(&self) -> CS9_R {
        CS9_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_CSR9")
            .field("cs9", &self.cs9())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs9(&mut self) -> CS9_W<'_, HASH_CSR9rs> {
        CS9_W::new(self, 0)
    }
}
/**HASH context swap register 9

You can [`read`](crate::Reg::read) this register and get [`hash_csr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_csr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_CSR9)*/
pub struct HASH_CSR9rs;
impl crate::RegisterSpec for HASH_CSR9rs {
    type Ux = u32;
}
///`read()` method returns [`hash_csr9::R`](R) reader structure
impl crate::Readable for HASH_CSR9rs {}
///`write(|w| ..)` method takes [`hash_csr9::W`](W) writer structure
impl crate::Writable for HASH_CSR9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_CSR9 to value 0
impl crate::Resettable for HASH_CSR9rs {}
