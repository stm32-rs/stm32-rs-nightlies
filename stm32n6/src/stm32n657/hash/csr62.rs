///Register `CSR62` reader
pub type R = crate::R<CSR62rs>;
///Register `CSR62` writer
pub type W = crate::W<CSR62rs>;
///Field `CS62` reader - Context swap x
pub type CS62_R = crate::FieldReader<u32>;
///Field `CS62` writer - Context swap x
pub type CS62_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs62(&self) -> CS62_R {
        CS62_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR62").field("cs62", &self.cs62()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs62(&mut self) -> CS62_W<'_, CSR62rs> {
        CS62_W::new(self, 0)
    }
}
/**HASH context swap register 62

You can [`read`](crate::Reg::read) this register and get [`csr62::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr62::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:CSR62)*/
pub struct CSR62rs;
impl crate::RegisterSpec for CSR62rs {
    type Ux = u32;
}
///`read()` method returns [`csr62::R`](R) reader structure
impl crate::Readable for CSR62rs {}
///`write(|w| ..)` method takes [`csr62::W`](W) writer structure
impl crate::Writable for CSR62rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR62 to value 0
impl crate::Resettable for CSR62rs {}
