///Register `CSR30` reader
pub type R = crate::R<CSR30rs>;
///Register `CSR30` writer
pub type W = crate::W<CSR30rs>;
///Field `CSR30` reader - CSR30
pub type CSR30_R = crate::FieldReader<u32>;
///Field `CSR30` writer - CSR30
pub type CSR30_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR30
    #[inline(always)]
    pub fn csr30(&self) -> CSR30_R {
        CSR30_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR30")
            .field("csr30", &self.csr30())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSR30
    #[inline(always)]
    pub fn csr30(&mut self) -> CSR30_W<CSR30rs> {
        CSR30_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#HASH:CSR30)*/
pub struct CSR30rs;
impl crate::RegisterSpec for CSR30rs {
    type Ux = u32;
}
///`read()` method returns [`csr30::R`](R) reader structure
impl crate::Readable for CSR30rs {}
///`write(|w| ..)` method takes [`csr30::W`](W) writer structure
impl crate::Writable for CSR30rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR30 to value 0
impl crate::Resettable for CSR30rs {
    const RESET_VALUE: u32 = 0;
}