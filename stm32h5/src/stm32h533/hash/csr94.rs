///Register `CSR94` reader
pub type R = crate::R<CSR94rs>;
///Register `CSR94` writer
pub type W = crate::W<CSR94rs>;
///Field `CS94` reader - Context swap x
pub type CS94_R = crate::FieldReader<u32>;
///Field `CS94` writer - Context swap x
pub type CS94_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs94(&self) -> CS94_R {
        CS94_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR94").field("cs94", &self.cs94()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs94(&mut self) -> CS94_W<'_, CSR94rs> {
        CS94_W::new(self, 0)
    }
}
/**HASH context swap register 94

You can [`read`](crate::Reg::read) this register and get [`csr94::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr94::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR94)*/
pub struct CSR94rs;
impl crate::RegisterSpec for CSR94rs {
    type Ux = u32;
}
///`read()` method returns [`csr94::R`](R) reader structure
impl crate::Readable for CSR94rs {}
///`write(|w| ..)` method takes [`csr94::W`](W) writer structure
impl crate::Writable for CSR94rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR94 to value 0
impl crate::Resettable for CSR94rs {}
