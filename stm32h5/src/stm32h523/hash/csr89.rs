///Register `CSR89` reader
pub type R = crate::R<CSR89rs>;
///Register `CSR89` writer
pub type W = crate::W<CSR89rs>;
///Field `CS89` reader - Context swap x
pub type CS89_R = crate::FieldReader<u32>;
///Field `CS89` writer - Context swap x
pub type CS89_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs89(&self) -> CS89_R {
        CS89_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR89").field("cs89", &self.cs89()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs89(&mut self) -> CS89_W<'_, CSR89rs> {
        CS89_W::new(self, 0)
    }
}
/**HASH context swap register 89

You can [`read`](crate::Reg::read) this register and get [`csr89::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr89::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR89)*/
pub struct CSR89rs;
impl crate::RegisterSpec for CSR89rs {
    type Ux = u32;
}
///`read()` method returns [`csr89::R`](R) reader structure
impl crate::Readable for CSR89rs {}
///`write(|w| ..)` method takes [`csr89::W`](W) writer structure
impl crate::Writable for CSR89rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR89 to value 0
impl crate::Resettable for CSR89rs {}
