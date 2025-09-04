///Register `CSR%s` reader
pub type R = crate::R<CSRrs>;
///Register `CSR%s` writer
pub type W = crate::W<CSRrs>;
///Field `CS` reader - CSR0
pub type CS_R = crate::FieldReader<u32>;
///Field `CS` writer - CSR0
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR").field("cs", &self.cs()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<CSRrs> {
        CS_W::new(self, 0)
    }
}
/**HASH context swap register %s

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#HASH:CSR[0])*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR%s to value 0
impl crate::Resettable for CSRrs {}
