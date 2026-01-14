///Register `CSR54` reader
pub type R = crate::R<CSR54rs>;
///Register `CSR54` writer
pub type W = crate::W<CSR54rs>;
///Field `CS54` reader - Context swap x
pub type CS54_R = crate::FieldReader<u32>;
///Field `CS54` writer - Context swap x
pub type CS54_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs54(&self) -> CS54_R {
        CS54_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR54").field("cs54", &self.cs54()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap x
    #[inline(always)]
    pub fn cs54(&mut self) -> CS54_W<'_, CSR54rs> {
        CS54_W::new(self, 0)
    }
}
/**HASH context swap register 54

You can [`read`](crate::Reg::read) this register and get [`csr54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:CSR54)*/
pub struct CSR54rs;
impl crate::RegisterSpec for CSR54rs {
    type Ux = u32;
}
///`read()` method returns [`csr54::R`](R) reader structure
impl crate::Readable for CSR54rs {}
///`write(|w| ..)` method takes [`csr54::W`](W) writer structure
impl crate::Writable for CSR54rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR54 to value 0
impl crate::Resettable for CSR54rs {}
