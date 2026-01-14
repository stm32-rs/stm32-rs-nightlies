///Register `CSR55` reader
pub type R = crate::R<CSR55rs>;
///Register `CSR55` writer
pub type W = crate::W<CSR55rs>;
///Field `CS55` reader - Context swap x
pub type CS55_R = crate::FieldReader<u32>;
///Field `CS55` writer - Context swap x
pub type CS55_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs55(&self) -> CS55_R {
        CS55_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR55").field("cs55", &self.cs55()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs55(&mut self) -> CS55_W<'_, CSR55rs> {
        CS55_W::new(self, 0)
    }
}
/**HASH context swap register 55

You can [`read`](crate::Reg::read) this register and get [`csr55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:CSR55)*/
pub struct CSR55rs;
impl crate::RegisterSpec for CSR55rs {
    type Ux = u32;
}
///`read()` method returns [`csr55::R`](R) reader structure
impl crate::Readable for CSR55rs {}
///`write(|w| ..)` method takes [`csr55::W`](W) writer structure
impl crate::Writable for CSR55rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR55 to value 0
impl crate::Resettable for CSR55rs {}
