///Register `CSR95` reader
pub type R = crate::R<CSR95rs>;
///Register `CSR95` writer
pub type W = crate::W<CSR95rs>;
///Field `CS95` reader - Context swap x
pub type CS95_R = crate::FieldReader<u32>;
///Field `CS95` writer - Context swap x
pub type CS95_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs95(&self) -> CS95_R {
        CS95_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR95").field("cs95", &self.cs95()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs95(&mut self) -> CS95_W<'_, CSR95rs> {
        CS95_W::new(self, 0)
    }
}
/**HASH context swap register 95

You can [`read`](crate::Reg::read) this register and get [`csr95::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr95::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:CSR95)*/
pub struct CSR95rs;
impl crate::RegisterSpec for CSR95rs {
    type Ux = u32;
}
///`read()` method returns [`csr95::R`](R) reader structure
impl crate::Readable for CSR95rs {}
///`write(|w| ..)` method takes [`csr95::W`](W) writer structure
impl crate::Writable for CSR95rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR95 to value 0
impl crate::Resettable for CSR95rs {}
