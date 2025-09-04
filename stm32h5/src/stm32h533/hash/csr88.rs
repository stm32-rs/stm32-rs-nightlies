///Register `CSR88` reader
pub type R = crate::R<CSR88rs>;
///Register `CSR88` writer
pub type W = crate::W<CSR88rs>;
///Field `CS88` reader - Context swap x
pub type CS88_R = crate::FieldReader<u32>;
///Field `CS88` writer - Context swap x
pub type CS88_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs88(&self) -> CS88_R {
        CS88_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR88").field("cs88", &self.cs88()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs88(&mut self) -> CS88_W<CSR88rs> {
        CS88_W::new(self, 0)
    }
}
/**HASH context swap register 88

You can [`read`](crate::Reg::read) this register and get [`csr88::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr88::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR88)*/
pub struct CSR88rs;
impl crate::RegisterSpec for CSR88rs {
    type Ux = u32;
}
///`read()` method returns [`csr88::R`](R) reader structure
impl crate::Readable for CSR88rs {}
///`write(|w| ..)` method takes [`csr88::W`](W) writer structure
impl crate::Writable for CSR88rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR88 to value 0
impl crate::Resettable for CSR88rs {}
