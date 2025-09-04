///Register `CSR31` reader
pub type R = crate::R<CSR31rs>;
///Register `CSR31` writer
pub type W = crate::W<CSR31rs>;
///Field `CSx` reader - Context swap 31 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32>;
///Field `CSx` writer - Context swap 31 Refer to introduction.
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap 31 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR31").field("csx", &self.csx()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap 31 Refer to introduction.
    #[inline(always)]
    pub fn csx(&mut self) -> CSX_W<CSR31rs> {
        CSX_W::new(self, 0)
    }
}
/**HASH context swap register 31

You can [`read`](crate::Reg::read) this register and get [`csr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#HASH:CSR31)*/
pub struct CSR31rs;
impl crate::RegisterSpec for CSR31rs {
    type Ux = u32;
}
///`read()` method returns [`csr31::R`](R) reader structure
impl crate::Readable for CSR31rs {}
///`write(|w| ..)` method takes [`csr31::W`](W) writer structure
impl crate::Writable for CSR31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR31 to value 0x0022_0002
impl crate::Resettable for CSR31rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}
