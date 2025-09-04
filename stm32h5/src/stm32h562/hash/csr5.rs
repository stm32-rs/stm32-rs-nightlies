///Register `CSR5` reader
pub type R = crate::R<CSR5rs>;
///Register `CSR5` writer
pub type W = crate::W<CSR5rs>;
///Field `CSx` reader - Context swap 5 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32>;
///Field `CSx` writer - Context swap 5 Refer to introduction.
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap 5 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR5").field("csx", &self.csx()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap 5 Refer to introduction.
    #[inline(always)]
    pub fn csx(&mut self) -> CSX_W<CSR5rs> {
        CSX_W::new(self, 0)
    }
}
/**HASH context swap register 5

You can [`read`](crate::Reg::read) this register and get [`csr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#HASH:CSR5)*/
pub struct CSR5rs;
impl crate::RegisterSpec for CSR5rs {
    type Ux = u32;
}
///`read()` method returns [`csr5::R`](R) reader structure
impl crate::Readable for CSR5rs {}
///`write(|w| ..)` method takes [`csr5::W`](W) writer structure
impl crate::Writable for CSR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR5 to value 0x0022_0002
impl crate::Resettable for CSR5rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}
