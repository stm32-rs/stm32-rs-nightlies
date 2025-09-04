///Register `CSR91` reader
pub type R = crate::R<CSR91rs>;
///Register `CSR91` writer
pub type W = crate::W<CSR91rs>;
///Field `CSx` reader - Context swap 91 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32>;
///Field `CSx` writer - Context swap 91 Refer to introduction.
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap 91 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR91").field("csx", &self.csx()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap 91 Refer to introduction.
    #[inline(always)]
    pub fn csx(&mut self) -> CSX_W<CSR91rs> {
        CSX_W::new(self, 0)
    }
}
/**HASH context swap register 91

You can [`read`](crate::Reg::read) this register and get [`csr91::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr91::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#HASH:CSR91)*/
pub struct CSR91rs;
impl crate::RegisterSpec for CSR91rs {
    type Ux = u32;
}
///`read()` method returns [`csr91::R`](R) reader structure
impl crate::Readable for CSR91rs {}
///`write(|w| ..)` method takes [`csr91::W`](W) writer structure
impl crate::Writable for CSR91rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR91 to value 0x0022_0002
impl crate::Resettable for CSR91rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}
