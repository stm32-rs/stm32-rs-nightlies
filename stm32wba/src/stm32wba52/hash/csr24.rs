///Register `CSR24` reader
pub type R = crate::R<CSR24rs>;
///Register `CSR24` writer
pub type W = crate::W<CSR24rs>;
///Field `CS24` reader - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS24_R = crate::FieldReader<u32>;
///Field `CS24` writer - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS24_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs24(&self) -> CS24_R {
        CS24_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR24").field("cs24", &self.cs24()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs24(&mut self) -> CS24_W<CSR24rs> {
        CS24_W::new(self, 0)
    }
}
/**HASH context swap register 24

You can [`read`](crate::Reg::read) this register and get [`csr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HASH:CSR24)*/
pub struct CSR24rs;
impl crate::RegisterSpec for CSR24rs {
    type Ux = u32;
}
///`read()` method returns [`csr24::R`](R) reader structure
impl crate::Readable for CSR24rs {}
///`write(|w| ..)` method takes [`csr24::W`](W) writer structure
impl crate::Writable for CSR24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR24 to value 0
impl crate::Resettable for CSR24rs {}
