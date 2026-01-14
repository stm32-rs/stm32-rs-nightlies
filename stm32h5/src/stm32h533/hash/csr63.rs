///Register `CSR63` reader
pub type R = crate::R<CSR63rs>;
///Register `CSR63` writer
pub type W = crate::W<CSR63rs>;
///Field `CS63` reader - Context swap x
pub type CS63_R = crate::FieldReader<u32>;
///Field `CS63` writer - Context swap x
pub type CS63_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs63(&self) -> CS63_R {
        CS63_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR63").field("cs63", &self.cs63()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs63(&mut self) -> CS63_W<'_, CSR63rs> {
        CS63_W::new(self, 0)
    }
}
/**HASH context swap register 63

You can [`read`](crate::Reg::read) this register and get [`csr63::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr63::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR63)*/
pub struct CSR63rs;
impl crate::RegisterSpec for CSR63rs {
    type Ux = u32;
}
///`read()` method returns [`csr63::R`](R) reader structure
impl crate::Readable for CSR63rs {}
///`write(|w| ..)` method takes [`csr63::W`](W) writer structure
impl crate::Writable for CSR63rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR63 to value 0
impl crate::Resettable for CSR63rs {}
