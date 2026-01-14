///Register `DATA0` reader
pub type R = crate::R<DATA0rs>;
///Register `DATA0` writer
pub type W = crate::W<DATA0rs>;
///Field `DATA0` reader - Value to be used as DATA for any COMMAND of type WRITE and compare value for MASSREAD
pub type DATA0_R = crate::FieldReader<u32>;
///Field `DATA0` writer - Value to be used as DATA for any COMMAND of type WRITE and compare value for MASSREAD
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE and compare value for MASSREAD
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA0")
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE and compare value for MASSREAD
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, DATA0rs> {
        DATA0_W::new(self, 0)
    }
}
/**DATA0 register

You can [`read`](crate::Reg::read) this register and get [`data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#FLASH_CTRL:DATA0)*/
pub struct DATA0rs;
impl crate::RegisterSpec for DATA0rs {
    type Ux = u32;
}
///`read()` method returns [`data0::R`](R) reader structure
impl crate::Readable for DATA0rs {}
///`write(|w| ..)` method takes [`data0::W`](W) writer structure
impl crate::Writable for DATA0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATA0 to value 0xffff_ffff
impl crate::Resettable for DATA0rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
