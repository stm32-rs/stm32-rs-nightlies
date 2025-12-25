///Register `CSR83` reader
pub type R = crate::R<CSR83rs>;
///Register `CSR83` writer
pub type W = crate::W<CSR83rs>;
///Field `CS83` reader - Context swap x
pub type CS83_R = crate::FieldReader<u32>;
///Field `CS83` writer - Context swap x
pub type CS83_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs83(&self) -> CS83_R {
        CS83_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR83").field("cs83", &self.cs83()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs83(&mut self) -> CS83_W<'_, CSR83rs> {
        CS83_W::new(self, 0)
    }
}
/**HASH context swap register 83

You can [`read`](crate::Reg::read) this register and get [`csr83::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr83::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HASH:CSR83)*/
pub struct CSR83rs;
impl crate::RegisterSpec for CSR83rs {
    type Ux = u32;
}
///`read()` method returns [`csr83::R`](R) reader structure
impl crate::Readable for CSR83rs {}
///`write(|w| ..)` method takes [`csr83::W`](W) writer structure
impl crate::Writable for CSR83rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR83 to value 0
impl crate::Resettable for CSR83rs {}
