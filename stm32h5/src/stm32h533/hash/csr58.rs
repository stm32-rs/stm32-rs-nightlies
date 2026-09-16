///Register `CSR58` reader
pub type R = crate::R<CSR58rs>;
///Register `CSR58` writer
pub type W = crate::W<CSR58rs>;
///Field `CS58` reader - Context swap x
pub type CS58_R = crate::FieldReader<u32>;
///Field `CS58` writer - Context swap x
pub type CS58_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs58(&self) -> CS58_R {
        CS58_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR58").field("cs58", &self.cs58()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs58(&mut self) -> CS58_W<'_, CSR58rs> {
        CS58_W::new(self, 0)
    }
}
/**HASH context swap register 58

You can [`read`](crate::Reg::read) this register and get [`csr58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR58)*/
pub struct CSR58rs;
impl crate::RegisterSpec for CSR58rs {
    type Ux = u32;
}
///`read()` method returns [`csr58::R`](R) reader structure
impl crate::Readable for CSR58rs {}
///`write(|w| ..)` method takes [`csr58::W`](W) writer structure
impl crate::Writable for CSR58rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR58 to value 0
impl crate::Resettable for CSR58rs {}
