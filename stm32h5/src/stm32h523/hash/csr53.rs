///Register `CSR53` reader
pub type R = crate::R<CSR53rs>;
///Register `CSR53` writer
pub type W = crate::W<CSR53rs>;
///Field `CS53` reader - Context swap x
pub type CS53_R = crate::FieldReader<u32>;
///Field `CS53` writer - Context swap x
pub type CS53_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs53(&self) -> CS53_R {
        CS53_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR53").field("cs53", &self.cs53()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs53(&mut self) -> CS53_W<CSR53rs> {
        CS53_W::new(self, 0)
    }
}
/**HASH context swap register 53

You can [`read`](crate::Reg::read) this register and get [`csr53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR53)*/
pub struct CSR53rs;
impl crate::RegisterSpec for CSR53rs {
    type Ux = u32;
}
///`read()` method returns [`csr53::R`](R) reader structure
impl crate::Readable for CSR53rs {}
///`write(|w| ..)` method takes [`csr53::W`](W) writer structure
impl crate::Writable for CSR53rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR53 to value 0
impl crate::Resettable for CSR53rs {}
