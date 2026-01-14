///Register `CSR77` reader
pub type R = crate::R<CSR77rs>;
///Register `CSR77` writer
pub type W = crate::W<CSR77rs>;
///Field `CS77` reader - Context swap x
pub type CS77_R = crate::FieldReader<u32>;
///Field `CS77` writer - Context swap x
pub type CS77_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs77(&self) -> CS77_R {
        CS77_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR77").field("cs77", &self.cs77()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs77(&mut self) -> CS77_W<'_, CSR77rs> {
        CS77_W::new(self, 0)
    }
}
/**HASH context swap register 77

You can [`read`](crate::Reg::read) this register and get [`csr77::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr77::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:CSR77)*/
pub struct CSR77rs;
impl crate::RegisterSpec for CSR77rs {
    type Ux = u32;
}
///`read()` method returns [`csr77::R`](R) reader structure
impl crate::Readable for CSR77rs {}
///`write(|w| ..)` method takes [`csr77::W`](W) writer structure
impl crate::Writable for CSR77rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR77 to value 0
impl crate::Resettable for CSR77rs {}
