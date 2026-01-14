///Register `CSR84` reader
pub type R = crate::R<CSR84rs>;
///Register `CSR84` writer
pub type W = crate::W<CSR84rs>;
///Field `CS84` reader - Context swap x
pub type CS84_R = crate::FieldReader<u32>;
///Field `CS84` writer - Context swap x
pub type CS84_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs84(&self) -> CS84_R {
        CS84_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR84").field("cs84", &self.cs84()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs84(&mut self) -> CS84_W<'_, CSR84rs> {
        CS84_W::new(self, 0)
    }
}
/**HASH context swap register 84

You can [`read`](crate::Reg::read) this register and get [`csr84::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr84::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:CSR84)*/
pub struct CSR84rs;
impl crate::RegisterSpec for CSR84rs {
    type Ux = u32;
}
///`read()` method returns [`csr84::R`](R) reader structure
impl crate::Readable for CSR84rs {}
///`write(|w| ..)` method takes [`csr84::W`](W) writer structure
impl crate::Writable for CSR84rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR84 to value 0
impl crate::Resettable for CSR84rs {}
