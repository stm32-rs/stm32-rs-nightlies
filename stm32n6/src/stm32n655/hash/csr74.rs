///Register `CSR74` reader
pub type R = crate::R<CSR74rs>;
///Register `CSR74` writer
pub type W = crate::W<CSR74rs>;
///Field `CS74` reader - Context swap x
pub type CS74_R = crate::FieldReader<u32>;
///Field `CS74` writer - Context swap x
pub type CS74_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs74(&self) -> CS74_R {
        CS74_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR74").field("cs74", &self.cs74()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs74(&mut self) -> CS74_W<'_, CSR74rs> {
        CS74_W::new(self, 0)
    }
}
/**HASH context swap register 74

You can [`read`](crate::Reg::read) this register and get [`csr74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HASH:CSR74)*/
pub struct CSR74rs;
impl crate::RegisterSpec for CSR74rs {
    type Ux = u32;
}
///`read()` method returns [`csr74::R`](R) reader structure
impl crate::Readable for CSR74rs {}
///`write(|w| ..)` method takes [`csr74::W`](W) writer structure
impl crate::Writable for CSR74rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR74 to value 0
impl crate::Resettable for CSR74rs {}
