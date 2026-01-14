///Register `DATA3` reader
pub type R = crate::R<DATA3rs>;
///Register `DATA3` writer
pub type W = crate::W<DATA3rs>;
///Field `DATA3` reader - Value to be used as DATA for any COMMAND of type WRITE
pub type DATA3_R = crate::FieldReader<u32>;
///Field `DATA3` writer - Value to be used as DATA for any COMMAND of type WRITE
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA3")
            .field("data3", &self.data3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<'_, DATA3rs> {
        DATA3_W::new(self, 0)
    }
}
/**DATA3 register

You can [`read`](crate::Reg::read) this register and get [`data3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#FLASH_CTRL:DATA3)*/
pub struct DATA3rs;
impl crate::RegisterSpec for DATA3rs {
    type Ux = u32;
}
///`read()` method returns [`data3::R`](R) reader structure
impl crate::Readable for DATA3rs {}
///`write(|w| ..)` method takes [`data3::W`](W) writer structure
impl crate::Writable for DATA3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATA3 to value 0xffff_ffff
impl crate::Resettable for DATA3rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
