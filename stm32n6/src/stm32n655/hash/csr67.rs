///Register `CSR67` reader
pub type R = crate::R<CSR67rs>;
///Register `CSR67` writer
pub type W = crate::W<CSR67rs>;
///Field `CS67` reader - Context swap x
pub type CS67_R = crate::FieldReader<u32>;
///Field `CS67` writer - Context swap x
pub type CS67_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs67(&self) -> CS67_R {
        CS67_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR67").field("cs67", &self.cs67()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs67(&mut self) -> CS67_W<'_, CSR67rs> {
        CS67_W::new(self, 0)
    }
}
/**HASH context swap register 67

You can [`read`](crate::Reg::read) this register and get [`csr67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HASH:CSR67)*/
pub struct CSR67rs;
impl crate::RegisterSpec for CSR67rs {
    type Ux = u32;
}
///`read()` method returns [`csr67::R`](R) reader structure
impl crate::Readable for CSR67rs {}
///`write(|w| ..)` method takes [`csr67::W`](W) writer structure
impl crate::Writable for CSR67rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR67 to value 0
impl crate::Resettable for CSR67rs {}
