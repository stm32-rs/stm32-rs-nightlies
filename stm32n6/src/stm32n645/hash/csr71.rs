///Register `CSR71` reader
pub type R = crate::R<CSR71rs>;
///Register `CSR71` writer
pub type W = crate::W<CSR71rs>;
///Field `CS71` reader - Context swap x
pub type CS71_R = crate::FieldReader<u32>;
///Field `CS71` writer - Context swap x
pub type CS71_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs71(&self) -> CS71_R {
        CS71_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR71").field("cs71", &self.cs71()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs71(&mut self) -> CS71_W<'_, CSR71rs> {
        CS71_W::new(self, 0)
    }
}
/**HASH context swap register 71

You can [`read`](crate::Reg::read) this register and get [`csr71::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr71::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:CSR71)*/
pub struct CSR71rs;
impl crate::RegisterSpec for CSR71rs {
    type Ux = u32;
}
///`read()` method returns [`csr71::R`](R) reader structure
impl crate::Readable for CSR71rs {}
///`write(|w| ..)` method takes [`csr71::W`](W) writer structure
impl crate::Writable for CSR71rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR71 to value 0
impl crate::Resettable for CSR71rs {}
