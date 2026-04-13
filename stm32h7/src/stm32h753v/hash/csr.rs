///Register `CSR%s` reader
pub type R = crate::R<CSRrs>;
///Register `CSR%s` writer
pub type W = crate::W<CSRrs>;
///Field `CSR` reader - CSR0
pub type CSR_R = crate::FieldReader<u32>;
///Field `CSR` writer - CSR0
pub type CSR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR").field("csr", &self.csr()).finish()
    }
}
impl W {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<'_, CSRrs> {
        CSR_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HASH:CSR[0])*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR%s to value 0
impl crate::Resettable for CSRrs {}
