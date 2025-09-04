///Register `CSR17` reader
pub type R = crate::R<CSR17rs>;
///Register `CSR17` writer
pub type W = crate::W<CSR17rs>;
///Field `CS17` reader - Context swap x
pub type CS17_R = crate::FieldReader<u32>;
///Field `CS17` writer - Context swap x
pub type CS17_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs17(&self) -> CS17_R {
        CS17_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR17").field("cs17", &self.cs17()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs17(&mut self) -> CS17_W<CSR17rs> {
        CS17_W::new(self, 0)
    }
}
/**HASH context swap register 17

You can [`read`](crate::Reg::read) this register and get [`csr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:CSR17)*/
pub struct CSR17rs;
impl crate::RegisterSpec for CSR17rs {
    type Ux = u32;
}
///`read()` method returns [`csr17::R`](R) reader structure
impl crate::Readable for CSR17rs {}
///`write(|w| ..)` method takes [`csr17::W`](W) writer structure
impl crate::Writable for CSR17rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR17 to value 0
impl crate::Resettable for CSR17rs {}
