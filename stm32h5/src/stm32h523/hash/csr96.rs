///Register `CSR96` reader
pub type R = crate::R<CSR96rs>;
///Register `CSR96` writer
pub type W = crate::W<CSR96rs>;
///Field `CS96` reader - Context swap x
pub type CS96_R = crate::FieldReader<u32>;
///Field `CS96` writer - Context swap x
pub type CS96_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs96(&self) -> CS96_R {
        CS96_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR96").field("cs96", &self.cs96()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs96(&mut self) -> CS96_W<'_, CSR96rs> {
        CS96_W::new(self, 0)
    }
}
/**HASH context swap register 96

You can [`read`](crate::Reg::read) this register and get [`csr96::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr96::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR96)*/
pub struct CSR96rs;
impl crate::RegisterSpec for CSR96rs {
    type Ux = u32;
}
///`read()` method returns [`csr96::R`](R) reader structure
impl crate::Readable for CSR96rs {}
///`write(|w| ..)` method takes [`csr96::W`](W) writer structure
impl crate::Writable for CSR96rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR96 to value 0
impl crate::Resettable for CSR96rs {}
