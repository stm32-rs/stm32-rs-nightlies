///Register `CSR102` reader
pub type R = crate::R<CSR102rs>;
///Register `CSR102` writer
pub type W = crate::W<CSR102rs>;
///Field `CS102` reader - Context swap x
pub type CS102_R = crate::FieldReader<u32>;
///Field `CS102` writer - Context swap x
pub type CS102_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs102(&self) -> CS102_R {
        CS102_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR102")
            .field("cs102", &self.cs102())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs102(&mut self) -> CS102_W<'_, CSR102rs> {
        CS102_W::new(self, 0)
    }
}
/**HASH context swap register 102

You can [`read`](crate::Reg::read) this register and get [`csr102::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr102::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR102)*/
pub struct CSR102rs;
impl crate::RegisterSpec for CSR102rs {
    type Ux = u32;
}
///`read()` method returns [`csr102::R`](R) reader structure
impl crate::Readable for CSR102rs {}
///`write(|w| ..)` method takes [`csr102::W`](W) writer structure
impl crate::Writable for CSR102rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR102 to value 0
impl crate::Resettable for CSR102rs {}
