///Register `CSR25` reader
pub type R = crate::R<CSR25rs>;
///Register `CSR25` writer
pub type W = crate::W<CSR25rs>;
///Field `CS25` reader - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS25_R = crate::FieldReader<u32>;
///Field `CS25` writer - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS25_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs25(&self) -> CS25_R {
        CS25_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR25").field("cs25", &self.cs25()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs25(&mut self) -> CS25_W<CSR25rs> {
        CS25_W::new(self, 0)
    }
}
/**HASH context swap register 25

You can [`read`](crate::Reg::read) this register and get [`csr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HASH:CSR25)*/
pub struct CSR25rs;
impl crate::RegisterSpec for CSR25rs {
    type Ux = u32;
}
///`read()` method returns [`csr25::R`](R) reader structure
impl crate::Readable for CSR25rs {}
///`write(|w| ..)` method takes [`csr25::W`](W) writer structure
impl crate::Writable for CSR25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR25 to value 0
impl crate::Resettable for CSR25rs {}
