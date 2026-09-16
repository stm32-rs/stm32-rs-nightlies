///Register `CSR57` reader
pub type R = crate::R<CSR57rs>;
///Register `CSR57` writer
pub type W = crate::W<CSR57rs>;
///Field `CS57` reader - Context swap x
pub type CS57_R = crate::FieldReader<u32>;
///Field `CS57` writer - Context swap x
pub type CS57_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs57(&self) -> CS57_R {
        CS57_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR57").field("cs57", &self.cs57()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs57(&mut self) -> CS57_W<'_, CSR57rs> {
        CS57_W::new(self, 0)
    }
}
/**HASH context swap register 57

You can [`read`](crate::Reg::read) this register and get [`csr57::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr57::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:CSR57)*/
pub struct CSR57rs;
impl crate::RegisterSpec for CSR57rs {
    type Ux = u32;
}
///`read()` method returns [`csr57::R`](R) reader structure
impl crate::Readable for CSR57rs {}
///`write(|w| ..)` method takes [`csr57::W`](W) writer structure
impl crate::Writable for CSR57rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR57 to value 0
impl crate::Resettable for CSR57rs {}
