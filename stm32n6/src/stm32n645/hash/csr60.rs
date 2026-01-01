///Register `CSR60` reader
pub type R = crate::R<CSR60rs>;
///Register `CSR60` writer
pub type W = crate::W<CSR60rs>;
///Field `CS60` reader - Context swap x
pub type CS60_R = crate::FieldReader<u32>;
///Field `CS60` writer - Context swap x
pub type CS60_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs60(&self) -> CS60_R {
        CS60_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR60").field("cs60", &self.cs60()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs60(&mut self) -> CS60_W<'_, CSR60rs> {
        CS60_W::new(self, 0)
    }
}
/**HASH context swap register 60

You can [`read`](crate::Reg::read) this register and get [`csr60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:CSR60)*/
pub struct CSR60rs;
impl crate::RegisterSpec for CSR60rs {
    type Ux = u32;
}
///`read()` method returns [`csr60::R`](R) reader structure
impl crate::Readable for CSR60rs {}
///`write(|w| ..)` method takes [`csr60::W`](W) writer structure
impl crate::Writable for CSR60rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR60 to value 0
impl crate::Resettable for CSR60rs {}
