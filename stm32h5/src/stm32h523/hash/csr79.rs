///Register `CSR79` reader
pub type R = crate::R<CSR79rs>;
///Register `CSR79` writer
pub type W = crate::W<CSR79rs>;
///Field `CS79` reader - Context swap x
pub type CS79_R = crate::FieldReader<u32>;
///Field `CS79` writer - Context swap x
pub type CS79_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs79(&self) -> CS79_R {
        CS79_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR79").field("cs79", &self.cs79()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs79(&mut self) -> CS79_W<'_, CSR79rs> {
        CS79_W::new(self, 0)
    }
}
/**HASH context swap register 79

You can [`read`](crate::Reg::read) this register and get [`csr79::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr79::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR79)*/
pub struct CSR79rs;
impl crate::RegisterSpec for CSR79rs {
    type Ux = u32;
}
///`read()` method returns [`csr79::R`](R) reader structure
impl crate::Readable for CSR79rs {}
///`write(|w| ..)` method takes [`csr79::W`](W) writer structure
impl crate::Writable for CSR79rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR79 to value 0
impl crate::Resettable for CSR79rs {}
