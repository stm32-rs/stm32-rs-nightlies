///Register `CSR86` reader
pub type R = crate::R<CSR86rs>;
///Register `CSR86` writer
pub type W = crate::W<CSR86rs>;
///Field `CS86` reader - Context swap x
pub type CS86_R = crate::FieldReader<u32>;
///Field `CS86` writer - Context swap x
pub type CS86_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs86(&self) -> CS86_R {
        CS86_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR86").field("cs86", &self.cs86()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs86(&mut self) -> CS86_W<'_, CSR86rs> {
        CS86_W::new(self, 0)
    }
}
/**HASH context swap register 86

You can [`read`](crate::Reg::read) this register and get [`csr86::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr86::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HASH:CSR86)*/
pub struct CSR86rs;
impl crate::RegisterSpec for CSR86rs {
    type Ux = u32;
}
///`read()` method returns [`csr86::R`](R) reader structure
impl crate::Readable for CSR86rs {}
///`write(|w| ..)` method takes [`csr86::W`](W) writer structure
impl crate::Writable for CSR86rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR86 to value 0
impl crate::Resettable for CSR86rs {}
