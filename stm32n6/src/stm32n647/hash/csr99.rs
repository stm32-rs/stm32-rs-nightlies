///Register `CSR99` reader
pub type R = crate::R<CSR99rs>;
///Register `CSR99` writer
pub type W = crate::W<CSR99rs>;
///Field `CS99` reader - Context swap x
pub type CS99_R = crate::FieldReader<u32>;
///Field `CS99` writer - Context swap x
pub type CS99_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs99(&self) -> CS99_R {
        CS99_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR99").field("cs99", &self.cs99()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs99(&mut self) -> CS99_W<CSR99rs> {
        CS99_W::new(self, 0)
    }
}
/**HASH context swap register 99

You can [`read`](crate::Reg::read) this register and get [`csr99::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr99::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HASH:CSR99)*/
pub struct CSR99rs;
impl crate::RegisterSpec for CSR99rs {
    type Ux = u32;
}
///`read()` method returns [`csr99::R`](R) reader structure
impl crate::Readable for CSR99rs {}
///`write(|w| ..)` method takes [`csr99::W`](W) writer structure
impl crate::Writable for CSR99rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR99 to value 0
impl crate::Resettable for CSR99rs {}
