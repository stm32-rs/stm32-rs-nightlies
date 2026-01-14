///Register `CSR101` reader
pub type R = crate::R<CSR101rs>;
///Register `CSR101` writer
pub type W = crate::W<CSR101rs>;
///Field `CS101` reader - Context swap x
pub type CS101_R = crate::FieldReader<u32>;
///Field `CS101` writer - Context swap x
pub type CS101_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs101(&self) -> CS101_R {
        CS101_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR101")
            .field("cs101", &self.cs101())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs101(&mut self) -> CS101_W<'_, CSR101rs> {
        CS101_W::new(self, 0)
    }
}
/**HASH context swap register 101

You can [`read`](crate::Reg::read) this register and get [`csr101::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr101::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR101)*/
pub struct CSR101rs;
impl crate::RegisterSpec for CSR101rs {
    type Ux = u32;
}
///`read()` method returns [`csr101::R`](R) reader structure
impl crate::Readable for CSR101rs {}
///`write(|w| ..)` method takes [`csr101::W`](W) writer structure
impl crate::Writable for CSR101rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR101 to value 0
impl crate::Resettable for CSR101rs {}
