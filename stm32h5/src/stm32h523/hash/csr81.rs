///Register `CSR81` reader
pub type R = crate::R<CSR81rs>;
///Register `CSR81` writer
pub type W = crate::W<CSR81rs>;
///Field `CS81` reader - Context swap x
pub type CS81_R = crate::FieldReader<u32>;
///Field `CS81` writer - Context swap x
pub type CS81_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs81(&self) -> CS81_R {
        CS81_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR81").field("cs81", &self.cs81()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs81(&mut self) -> CS81_W<'_, CSR81rs> {
        CS81_W::new(self, 0)
    }
}
/**HASH context swap register 81

You can [`read`](crate::Reg::read) this register and get [`csr81::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr81::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR81)*/
pub struct CSR81rs;
impl crate::RegisterSpec for CSR81rs {
    type Ux = u32;
}
///`read()` method returns [`csr81::R`](R) reader structure
impl crate::Readable for CSR81rs {}
///`write(|w| ..)` method takes [`csr81::W`](W) writer structure
impl crate::Writable for CSR81rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR81 to value 0
impl crate::Resettable for CSR81rs {}
