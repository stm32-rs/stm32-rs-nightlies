///Register `CSR78` reader
pub type R = crate::R<CSR78rs>;
///Register `CSR78` writer
pub type W = crate::W<CSR78rs>;
///Field `CS78` reader - Context swap x
pub type CS78_R = crate::FieldReader<u32>;
///Field `CS78` writer - Context swap x
pub type CS78_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs78(&self) -> CS78_R {
        CS78_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR78").field("cs78", &self.cs78()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs78(&mut self) -> CS78_W<CSR78rs> {
        CS78_W::new(self, 0)
    }
}
/**HASH context swap register 78

You can [`read`](crate::Reg::read) this register and get [`csr78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HASH:CSR78)*/
pub struct CSR78rs;
impl crate::RegisterSpec for CSR78rs {
    type Ux = u32;
}
///`read()` method returns [`csr78::R`](R) reader structure
impl crate::Readable for CSR78rs {}
///`write(|w| ..)` method takes [`csr78::W`](W) writer structure
impl crate::Writable for CSR78rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR78 to value 0
impl crate::Resettable for CSR78rs {}
