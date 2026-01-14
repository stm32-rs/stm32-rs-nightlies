///Register `CSR93` reader
pub type R = crate::R<CSR93rs>;
///Register `CSR93` writer
pub type W = crate::W<CSR93rs>;
///Field `CS93` reader - Context swap x
pub type CS93_R = crate::FieldReader<u32>;
///Field `CS93` writer - Context swap x
pub type CS93_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs93(&self) -> CS93_R {
        CS93_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR93").field("cs93", &self.cs93()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs93(&mut self) -> CS93_W<'_, CSR93rs> {
        CS93_W::new(self, 0)
    }
}
/**HASH context swap register 93

You can [`read`](crate::Reg::read) this register and get [`csr93::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr93::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HASH:CSR93)*/
pub struct CSR93rs;
impl crate::RegisterSpec for CSR93rs {
    type Ux = u32;
}
///`read()` method returns [`csr93::R`](R) reader structure
impl crate::Readable for CSR93rs {}
///`write(|w| ..)` method takes [`csr93::W`](W) writer structure
impl crate::Writable for CSR93rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR93 to value 0
impl crate::Resettable for CSR93rs {}
