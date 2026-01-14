///Register `CSR85` reader
pub type R = crate::R<CSR85rs>;
///Register `CSR85` writer
pub type W = crate::W<CSR85rs>;
///Field `CS85` reader - Context swap x
pub type CS85_R = crate::FieldReader<u32>;
///Field `CS85` writer - Context swap x
pub type CS85_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs85(&self) -> CS85_R {
        CS85_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR85").field("cs85", &self.cs85()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs85(&mut self) -> CS85_W<'_, CSR85rs> {
        CS85_W::new(self, 0)
    }
}
/**HASH context swap register 85

You can [`read`](crate::Reg::read) this register and get [`csr85::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr85::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR85)*/
pub struct CSR85rs;
impl crate::RegisterSpec for CSR85rs {
    type Ux = u32;
}
///`read()` method returns [`csr85::R`](R) reader structure
impl crate::Readable for CSR85rs {}
///`write(|w| ..)` method takes [`csr85::W`](W) writer structure
impl crate::Writable for CSR85rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR85 to value 0
impl crate::Resettable for CSR85rs {}
