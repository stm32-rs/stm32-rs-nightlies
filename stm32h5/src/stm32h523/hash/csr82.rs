///Register `CSR82` reader
pub type R = crate::R<CSR82rs>;
///Register `CSR82` writer
pub type W = crate::W<CSR82rs>;
///Field `CS82` reader - Context swap x
pub type CS82_R = crate::FieldReader<u32>;
///Field `CS82` writer - Context swap x
pub type CS82_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs82(&self) -> CS82_R {
        CS82_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR82").field("cs82", &self.cs82()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs82(&mut self) -> CS82_W<CSR82rs> {
        CS82_W::new(self, 0)
    }
}
/**HASH context swap register 82

You can [`read`](crate::Reg::read) this register and get [`csr82::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr82::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:CSR82)*/
pub struct CSR82rs;
impl crate::RegisterSpec for CSR82rs {
    type Ux = u32;
}
///`read()` method returns [`csr82::R`](R) reader structure
impl crate::Readable for CSR82rs {}
///`write(|w| ..)` method takes [`csr82::W`](W) writer structure
impl crate::Writable for CSR82rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR82 to value 0
impl crate::Resettable for CSR82rs {}
