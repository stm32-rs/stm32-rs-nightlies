///Register `CSR56` reader
pub type R = crate::R<CSR56rs>;
///Register `CSR56` writer
pub type W = crate::W<CSR56rs>;
///Field `CSx` reader - Context swap 56 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32>;
///Field `CSx` writer - Context swap 56 Refer to introduction.
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap 56 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR56").field("csx", &self.csx()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap 56 Refer to introduction.
    #[inline(always)]
    pub fn csx(&mut self) -> CSX_W<CSR56rs> {
        CSX_W::new(self, 0)
    }
}
/**HASH context swap register 56

You can [`read`](crate::Reg::read) this register and get [`csr56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#HASH:CSR56)*/
pub struct CSR56rs;
impl crate::RegisterSpec for CSR56rs {
    type Ux = u32;
}
///`read()` method returns [`csr56::R`](R) reader structure
impl crate::Readable for CSR56rs {}
///`write(|w| ..)` method takes [`csr56::W`](W) writer structure
impl crate::Writable for CSR56rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR56 to value 0x0022_0002
impl crate::Resettable for CSR56rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}
