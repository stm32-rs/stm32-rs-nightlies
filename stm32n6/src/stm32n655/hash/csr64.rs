///Register `CSR64` reader
pub type R = crate::R<CSR64rs>;
///Register `CSR64` writer
pub type W = crate::W<CSR64rs>;
///Field `CS64` reader - Context swap x
pub type CS64_R = crate::FieldReader<u32>;
///Field `CS64` writer - Context swap x
pub type CS64_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs64(&self) -> CS64_R {
        CS64_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR64").field("cs64", &self.cs64()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs64(&mut self) -> CS64_W<CSR64rs> {
        CS64_W::new(self, 0)
    }
}
/**HASH context swap register 64

You can [`read`](crate::Reg::read) this register and get [`csr64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HASH:CSR64)*/
pub struct CSR64rs;
impl crate::RegisterSpec for CSR64rs {
    type Ux = u32;
}
///`read()` method returns [`csr64::R`](R) reader structure
impl crate::Readable for CSR64rs {}
///`write(|w| ..)` method takes [`csr64::W`](W) writer structure
impl crate::Writable for CSR64rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR64 to value 0
impl crate::Resettable for CSR64rs {}
