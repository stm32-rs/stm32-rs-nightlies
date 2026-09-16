///Register `CSR69` reader
pub type R = crate::R<CSR69rs>;
///Register `CSR69` writer
pub type W = crate::W<CSR69rs>;
///Field `CS69` reader - Context swap x
pub type CS69_R = crate::FieldReader<u32>;
///Field `CS69` writer - Context swap x
pub type CS69_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs69(&self) -> CS69_R {
        CS69_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR69").field("cs69", &self.cs69()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs69(&mut self) -> CS69_W<'_, CSR69rs> {
        CS69_W::new(self, 0)
    }
}
/**HASH context swap register 69

You can [`read`](crate::Reg::read) this register and get [`csr69::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr69::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR69)*/
pub struct CSR69rs;
impl crate::RegisterSpec for CSR69rs {
    type Ux = u32;
}
///`read()` method returns [`csr69::R`](R) reader structure
impl crate::Readable for CSR69rs {}
///`write(|w| ..)` method takes [`csr69::W`](W) writer structure
impl crate::Writable for CSR69rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR69 to value 0
impl crate::Resettable for CSR69rs {}
