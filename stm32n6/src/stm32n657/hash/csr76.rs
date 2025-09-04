///Register `CSR76` reader
pub type R = crate::R<CSR76rs>;
///Register `CSR76` writer
pub type W = crate::W<CSR76rs>;
///Field `CS76` reader - Context swap x
pub type CS76_R = crate::FieldReader<u32>;
///Field `CS76` writer - Context swap x
pub type CS76_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs76(&self) -> CS76_R {
        CS76_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR76").field("cs76", &self.cs76()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs76(&mut self) -> CS76_W<CSR76rs> {
        CS76_W::new(self, 0)
    }
}
/**HASH context swap register 76

You can [`read`](crate::Reg::read) this register and get [`csr76::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr76::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:CSR76)*/
pub struct CSR76rs;
impl crate::RegisterSpec for CSR76rs {
    type Ux = u32;
}
///`read()` method returns [`csr76::R`](R) reader structure
impl crate::Readable for CSR76rs {}
///`write(|w| ..)` method takes [`csr76::W`](W) writer structure
impl crate::Writable for CSR76rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR76 to value 0
impl crate::Resettable for CSR76rs {}
