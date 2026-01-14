///Register `CSR92` reader
pub type R = crate::R<CSR92rs>;
///Register `CSR92` writer
pub type W = crate::W<CSR92rs>;
///Field `CS92` reader - Context swap x
pub type CS92_R = crate::FieldReader<u32>;
///Field `CS92` writer - Context swap x
pub type CS92_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs92(&self) -> CS92_R {
        CS92_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR92").field("cs92", &self.cs92()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs92(&mut self) -> CS92_W<'_, CSR92rs> {
        CS92_W::new(self, 0)
    }
}
/**HASH context swap register 92

You can [`read`](crate::Reg::read) this register and get [`csr92::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr92::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR92)*/
pub struct CSR92rs;
impl crate::RegisterSpec for CSR92rs {
    type Ux = u32;
}
///`read()` method returns [`csr92::R`](R) reader structure
impl crate::Readable for CSR92rs {}
///`write(|w| ..)` method takes [`csr92::W`](W) writer structure
impl crate::Writable for CSR92rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR92 to value 0
impl crate::Resettable for CSR92rs {}
