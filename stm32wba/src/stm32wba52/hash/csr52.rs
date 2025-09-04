///Register `CSR52` reader
pub type R = crate::R<CSR52rs>;
///Register `CSR52` writer
pub type W = crate::W<CSR52rs>;
///Field `CS52` reader - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS52_R = crate::FieldReader<u32>;
///Field `CS52` writer - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
pub type CS52_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs52(&self) -> CS52_R {
        CS52_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR52").field("cs52", &self.cs52()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x Refer to Section 25.7.7: HASH context swap registers introduction.
    #[inline(always)]
    pub fn cs52(&mut self) -> CS52_W<CSR52rs> {
        CS52_W::new(self, 0)
    }
}
/**HASH context swap register 52

You can [`read`](crate::Reg::read) this register and get [`csr52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HASH:CSR52)*/
pub struct CSR52rs;
impl crate::RegisterSpec for CSR52rs {
    type Ux = u32;
}
///`read()` method returns [`csr52::R`](R) reader structure
impl crate::Readable for CSR52rs {}
///`write(|w| ..)` method takes [`csr52::W`](W) writer structure
impl crate::Writable for CSR52rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR52 to value 0
impl crate::Resettable for CSR52rs {}
