///Register `CSR61` reader
pub type R = crate::R<CSR61rs>;
///Register `CSR61` writer
pub type W = crate::W<CSR61rs>;
///Field `CS61` reader - Context swap x
pub type CS61_R = crate::FieldReader<u32>;
///Field `CS61` writer - Context swap x
pub type CS61_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs61(&self) -> CS61_R {
        CS61_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR61").field("cs61", &self.cs61()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs61(&mut self) -> CS61_W<'_, CSR61rs> {
        CS61_W::new(self, 0)
    }
}
/**HASH context swap register 61

You can [`read`](crate::Reg::read) this register and get [`csr61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR61)*/
pub struct CSR61rs;
impl crate::RegisterSpec for CSR61rs {
    type Ux = u32;
}
///`read()` method returns [`csr61::R`](R) reader structure
impl crate::Readable for CSR61rs {}
///`write(|w| ..)` method takes [`csr61::W`](W) writer structure
impl crate::Writable for CSR61rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR61 to value 0
impl crate::Resettable for CSR61rs {}
