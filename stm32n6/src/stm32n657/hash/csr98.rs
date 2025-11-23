///Register `CSR98` reader
pub type R = crate::R<CSR98rs>;
///Register `CSR98` writer
pub type W = crate::W<CSR98rs>;
///Field `CS98` reader - Context swap x
pub type CS98_R = crate::FieldReader<u32>;
///Field `CS98` writer - Context swap x
pub type CS98_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs98(&self) -> CS98_R {
        CS98_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR98").field("cs98", &self.cs98()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs98(&mut self) -> CS98_W<'_, CSR98rs> {
        CS98_W::new(self, 0)
    }
}
/**HASH context swap register 98

You can [`read`](crate::Reg::read) this register and get [`csr98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:CSR98)*/
pub struct CSR98rs;
impl crate::RegisterSpec for CSR98rs {
    type Ux = u32;
}
///`read()` method returns [`csr98::R`](R) reader structure
impl crate::Readable for CSR98rs {}
///`write(|w| ..)` method takes [`csr98::W`](W) writer structure
impl crate::Writable for CSR98rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR98 to value 0
impl crate::Resettable for CSR98rs {}
