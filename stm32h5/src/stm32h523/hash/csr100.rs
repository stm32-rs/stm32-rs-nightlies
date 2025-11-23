///Register `CSR100` reader
pub type R = crate::R<CSR100rs>;
///Register `CSR100` writer
pub type W = crate::W<CSR100rs>;
///Field `CS100` reader - Context swap x
pub type CS100_R = crate::FieldReader<u32>;
///Field `CS100` writer - Context swap x
pub type CS100_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs100(&self) -> CS100_R {
        CS100_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR100")
            .field("cs100", &self.cs100())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs100(&mut self) -> CS100_W<'_, CSR100rs> {
        CS100_W::new(self, 0)
    }
}
/**HASH context swap register 100

You can [`read`](crate::Reg::read) this register and get [`csr100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR100)*/
pub struct CSR100rs;
impl crate::RegisterSpec for CSR100rs {
    type Ux = u32;
}
///`read()` method returns [`csr100::R`](R) reader structure
impl crate::Readable for CSR100rs {}
///`write(|w| ..)` method takes [`csr100::W`](W) writer structure
impl crate::Writable for CSR100rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR100 to value 0
impl crate::Resettable for CSR100rs {}
