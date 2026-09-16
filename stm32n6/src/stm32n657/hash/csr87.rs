///Register `CSR87` reader
pub type R = crate::R<CSR87rs>;
///Register `CSR87` writer
pub type W = crate::W<CSR87rs>;
///Field `CS87` reader - Context swap x
pub type CS87_R = crate::FieldReader<u32>;
///Field `CS87` writer - Context swap x
pub type CS87_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs87(&self) -> CS87_R {
        CS87_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR87").field("cs87", &self.cs87()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs87(&mut self) -> CS87_W<'_, CSR87rs> {
        CS87_W::new(self, 0)
    }
}
/**HASH context swap register 87

You can [`read`](crate::Reg::read) this register and get [`csr87::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr87::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:CSR87)*/
pub struct CSR87rs;
impl crate::RegisterSpec for CSR87rs {
    type Ux = u32;
}
///`read()` method returns [`csr87::R`](R) reader structure
impl crate::Readable for CSR87rs {}
///`write(|w| ..)` method takes [`csr87::W`](W) writer structure
impl crate::Writable for CSR87rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR87 to value 0
impl crate::Resettable for CSR87rs {}
