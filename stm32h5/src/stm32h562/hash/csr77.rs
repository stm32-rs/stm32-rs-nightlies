///Register `CSR77` reader
pub type R = crate::R<CSR77rs>;
///Register `CSR77` writer
pub type W = crate::W<CSR77rs>;
///Field `CSx` reader - Context swap 77 Refer to introduction.
pub type CSX_R = crate::FieldReader<u32>;
///Field `CSx` writer - Context swap 77 Refer to introduction.
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap 77 Refer to introduction.
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR77").field("csx", &self.csx()).finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap 77 Refer to introduction.
    #[inline(always)]
    pub fn csx(&mut self) -> CSX_W<CSR77rs> {
        CSX_W::new(self, 0)
    }
}
/**HASH context swap register 77

You can [`read`](crate::Reg::read) this register and get [`csr77::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr77::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#HASH:CSR77)*/
pub struct CSR77rs;
impl crate::RegisterSpec for CSR77rs {
    type Ux = u32;
}
///`read()` method returns [`csr77::R`](R) reader structure
impl crate::Readable for CSR77rs {}
///`write(|w| ..)` method takes [`csr77::W`](W) writer structure
impl crate::Writable for CSR77rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR77 to value 0x0022_0002
impl crate::Resettable for CSR77rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}