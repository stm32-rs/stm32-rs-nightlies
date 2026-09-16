///Register `CSR55` reader
pub type R = crate::R<CSR55rs>;
///Register `CSR55` writer
pub type W = crate::W<CSR55rs>;
///Field `CSx` reader - Context swap 55 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32>;
///Field `CSx` writer - Context swap 55 Refer to introduction.
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap 55 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR55").field("csx", &self.csx()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap 55 Refer to introduction.
    #[inline(always)]
    pub fn csx(&mut self) -> CSX_W<'_, CSR55rs> {
        CSX_W::new(self, 0)
    }
}
/**HASH context swap register 55

You can [`read`](crate::Reg::read) this register and get [`csr55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#HASH:CSR55)*/
pub struct CSR55rs;
impl crate::RegisterSpec for CSR55rs {
    type Ux = u32;
}
///`read()` method returns [`csr55::R`](R) reader structure
impl crate::Readable for CSR55rs {}
///`write(|w| ..)` method takes [`csr55::W`](W) writer structure
impl crate::Writable for CSR55rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR55 to value 0x0022_0002
impl crate::Resettable for CSR55rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}
