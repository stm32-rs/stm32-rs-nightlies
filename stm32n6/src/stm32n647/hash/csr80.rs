///Register `CSR80` reader
pub type R = crate::R<CSR80rs>;
///Register `CSR80` writer
pub type W = crate::W<CSR80rs>;
///Field `CS80` reader - Context swap x
pub type CS80_R = crate::FieldReader<u32>;
///Field `CS80` writer - Context swap x
pub type CS80_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs80(&self) -> CS80_R {
        CS80_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR80").field("cs80", &self.cs80()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs80(&mut self) -> CS80_W<'_, CSR80rs> {
        CS80_W::new(self, 0)
    }
}
/**HASH context swap register 80

You can [`read`](crate::Reg::read) this register and get [`csr80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HASH:CSR80)*/
pub struct CSR80rs;
impl crate::RegisterSpec for CSR80rs {
    type Ux = u32;
}
///`read()` method returns [`csr80::R`](R) reader structure
impl crate::Readable for CSR80rs {}
///`write(|w| ..)` method takes [`csr80::W`](W) writer structure
impl crate::Writable for CSR80rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR80 to value 0
impl crate::Resettable for CSR80rs {}
