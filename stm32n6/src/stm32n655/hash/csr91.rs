///Register `CSR91` reader
pub type R = crate::R<CSR91rs>;
///Register `CSR91` writer
pub type W = crate::W<CSR91rs>;
///Field `CS91` reader - Context swap x
pub type CS91_R = crate::FieldReader<u32>;
///Field `CS91` writer - Context swap x
pub type CS91_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs91(&self) -> CS91_R {
        CS91_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR91").field("cs91", &self.cs91()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs91(&mut self) -> CS91_W<'_, CSR91rs> {
        CS91_W::new(self, 0)
    }
}
/**HASH context swap register 91

You can [`read`](crate::Reg::read) this register and get [`csr91::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr91::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HASH:CSR91)*/
pub struct CSR91rs;
impl crate::RegisterSpec for CSR91rs {
    type Ux = u32;
}
///`read()` method returns [`csr91::R`](R) reader structure
impl crate::Readable for CSR91rs {}
///`write(|w| ..)` method takes [`csr91::W`](W) writer structure
impl crate::Writable for CSR91rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR91 to value 0
impl crate::Resettable for CSR91rs {}
