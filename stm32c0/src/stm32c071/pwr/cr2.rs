///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVM_VDDIO2` reader - supply voltage monitoring
pub type PVM_VDDIO2_R = crate::FieldReader;
///Field `PVM_VDDIO2` writer - supply voltage monitoring
pub type PVM_VDDIO2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 8:9 - supply voltage monitoring
    #[inline(always)]
    pub fn pvm_vddio2(&self) -> PVM_VDDIO2_R {
        PVM_VDDIO2_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvm_vddio2", &self.pvm_vddio2())
            .finish()
    }
}
impl W {
    ///Bits 8:9 - supply voltage monitoring
    #[inline(always)]
    pub fn pvm_vddio2(&mut self) -> PVM_VDDIO2_W<CR2rs> {
        PVM_VDDIO2_W::new(self, 8)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0x0100
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0x0100;
}
