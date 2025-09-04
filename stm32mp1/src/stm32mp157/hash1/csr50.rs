///Register `CSR50` reader
pub type R = crate::R<CSR50rs>;
///Register `CSR50` writer
pub type W = crate::W<CSR50rs>;
///Field `CS50` reader - CS50
pub type CS50_R = crate::FieldReader<u32>;
///Field `CS50` writer - CS50
pub type CS50_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS50
    #[inline(always)]
    pub fn cs50(&self) -> CS50_R {
        CS50_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR50").field("cs50", &self.cs50()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS50
    #[inline(always)]
    pub fn cs50(&mut self) -> CS50_W<CSR50rs> {
        CS50_W::new(self, 0)
    }
}
/**HASH context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH1:CSR50)*/
pub struct CSR50rs;
impl crate::RegisterSpec for CSR50rs {
    type Ux = u32;
}
///`read()` method returns [`csr50::R`](R) reader structure
impl crate::Readable for CSR50rs {}
///`write(|w| ..)` method takes [`csr50::W`](W) writer structure
impl crate::Writable for CSR50rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR50 to value 0
impl crate::Resettable for CSR50rs {}
