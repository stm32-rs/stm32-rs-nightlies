///Register `CSR70` reader
pub type R = crate::R<CSR70rs>;
///Register `CSR70` writer
pub type W = crate::W<CSR70rs>;
///Field `CS70` reader - Context swap x
pub type CS70_R = crate::FieldReader<u32>;
///Field `CS70` writer - Context swap x
pub type CS70_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs70(&self) -> CS70_R {
        CS70_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR70").field("cs70", &self.cs70()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs70(&mut self) -> CS70_W<'_, CSR70rs> {
        CS70_W::new(self, 0)
    }
}
/**HASH context swap register 70

You can [`read`](crate::Reg::read) this register and get [`csr70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HASH:CSR70)*/
pub struct CSR70rs;
impl crate::RegisterSpec for CSR70rs {
    type Ux = u32;
}
///`read()` method returns [`csr70::R`](R) reader structure
impl crate::Readable for CSR70rs {}
///`write(|w| ..)` method takes [`csr70::W`](W) writer structure
impl crate::Writable for CSR70rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR70 to value 0
impl crate::Resettable for CSR70rs {}
