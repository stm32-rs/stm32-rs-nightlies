///Register `CSR68` reader
pub type R = crate::R<CSR68rs>;
///Register `CSR68` writer
pub type W = crate::W<CSR68rs>;
///Field `CS68` reader - Context swap x
pub type CS68_R = crate::FieldReader<u32>;
///Field `CS68` writer - Context swap x
pub type CS68_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs68(&self) -> CS68_R {
        CS68_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR68").field("cs68", &self.cs68()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs68(&mut self) -> CS68_W<CSR68rs> {
        CS68_W::new(self, 0)
    }
}
/**HASH context swap register 68

You can [`read`](crate::Reg::read) this register and get [`csr68::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr68::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR68)*/
pub struct CSR68rs;
impl crate::RegisterSpec for CSR68rs {
    type Ux = u32;
}
///`read()` method returns [`csr68::R`](R) reader structure
impl crate::Readable for CSR68rs {}
///`write(|w| ..)` method takes [`csr68::W`](W) writer structure
impl crate::Writable for CSR68rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR68 to value 0
impl crate::Resettable for CSR68rs {}
