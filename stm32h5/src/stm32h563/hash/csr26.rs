///Register `CSR26` reader
pub type R = crate::R<CSR26rs>;
///Register `CSR26` writer
pub type W = crate::W<CSR26rs>;
///Field `CSx` reader - Context swap 26 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32>;
///Field `CSx` writer - Context swap 26 Refer to introduction.
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap 26 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR26").field("csx", &self.csx()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap 26 Refer to introduction.
    #[inline(always)]
    pub fn csx(&mut self) -> CSX_W<CSR26rs> {
        CSX_W::new(self, 0)
    }
}
/**HASH context swap register 26

You can [`read`](crate::Reg::read) this register and get [`csr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#HASH:CSR26)*/
pub struct CSR26rs;
impl crate::RegisterSpec for CSR26rs {
    type Ux = u32;
}
///`read()` method returns [`csr26::R`](R) reader structure
impl crate::Readable for CSR26rs {}
///`write(|w| ..)` method takes [`csr26::W`](W) writer structure
impl crate::Writable for CSR26rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR26 to value 0x0022_0002
impl crate::Resettable for CSR26rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}
