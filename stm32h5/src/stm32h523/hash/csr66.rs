///Register `CSR66` reader
pub type R = crate::R<CSR66rs>;
///Register `CSR66` writer
pub type W = crate::W<CSR66rs>;
///Field `CS66` reader - Context swap x
pub type CS66_R = crate::FieldReader<u32>;
///Field `CS66` writer - Context swap x
pub type CS66_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs66(&self) -> CS66_R {
        CS66_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR66").field("cs66", &self.cs66()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs66(&mut self) -> CS66_W<CSR66rs> {
        CS66_W::new(self, 0)
    }
}
/**HASH context swap register 66

You can [`read`](crate::Reg::read) this register and get [`csr66::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr66::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR66)*/
pub struct CSR66rs;
impl crate::RegisterSpec for CSR66rs {
    type Ux = u32;
}
///`read()` method returns [`csr66::R`](R) reader structure
impl crate::Readable for CSR66rs {}
///`write(|w| ..)` method takes [`csr66::W`](W) writer structure
impl crate::Writable for CSR66rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR66 to value 0
impl crate::Resettable for CSR66rs {}
