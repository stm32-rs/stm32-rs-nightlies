///Register `CSR15` reader
pub type R = crate::R<CSR15rs>;
///Register `CSR15` writer
pub type W = crate::W<CSR15rs>;
///Field `CS15` reader - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS15_R = crate::FieldReader<u32>;
///Field `CS15` writer - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS15_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs15(&self) -> CS15_R {
        CS15_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR15").field("cs15", &self.cs15()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs15(&mut self) -> CS15_W<CSR15rs> {
        CS15_W::new(self, 0)
    }
}
/**HASH context swap register 15

You can [`read`](crate::Reg::read) this register and get [`csr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HASH:CSR15)*/
pub struct CSR15rs;
impl crate::RegisterSpec for CSR15rs {
    type Ux = u32;
}
///`read()` method returns [`csr15::R`](R) reader structure
impl crate::Readable for CSR15rs {}
///`write(|w| ..)` method takes [`csr15::W`](W) writer structure
impl crate::Writable for CSR15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR15 to value 0
impl crate::Resettable for CSR15rs {}
