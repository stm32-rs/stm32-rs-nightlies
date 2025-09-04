///Register `CSR97` reader
pub type R = crate::R<CSR97rs>;
///Register `CSR97` writer
pub type W = crate::W<CSR97rs>;
///Field `CS97` reader - Context swap x
pub type CS97_R = crate::FieldReader<u32>;
///Field `CS97` writer - Context swap x
pub type CS97_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs97(&self) -> CS97_R {
        CS97_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR97").field("cs97", &self.cs97()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs97(&mut self) -> CS97_W<CSR97rs> {
        CS97_W::new(self, 0)
    }
}
/**HASH context swap register 97

You can [`read`](crate::Reg::read) this register and get [`csr97::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr97::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:CSR97)*/
pub struct CSR97rs;
impl crate::RegisterSpec for CSR97rs {
    type Ux = u32;
}
///`read()` method returns [`csr97::R`](R) reader structure
impl crate::Readable for CSR97rs {}
///`write(|w| ..)` method takes [`csr97::W`](W) writer structure
impl crate::Writable for CSR97rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR97 to value 0
impl crate::Resettable for CSR97rs {}
